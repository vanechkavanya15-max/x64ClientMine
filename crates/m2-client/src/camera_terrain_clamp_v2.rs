use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::panic::catch_unwind;

use glam::Vec3;

// Zmienne kolizyjne odpowiadajace tym z CCamera w silniku Metin2
const TERRAIN_COLLISION_RADIUS: f32 = 50.0;
const EPSILON: f32 = 1e-5;

/// Funkcja obliczajaca wysokosc terenu za pomoca Raycastingu w dol ze skokami LOD, 
/// dokladnie tak jak robi to CMapOutdoor::__PickTerrainHeight w oryginalnym C++.
/// Promien ray'a `fRayRange` i dlugosc `fLimitRange` sprawdzana na kazdym kroku fStep.
fn pick_terrain_height<F>(
    start: Vec3,
    end: Vec3,
    step: f32,
    ray_range: f32,
    limit_range: f32,
    get_terrain_height: &F,
) -> Option<Vec3>
where
    F: Fn(f32, f32) -> f32,
{
    if ray_range <= EPSILON {
        return None;
    }
    
    let mut f_pos = 0.0;
    let ray_range_inv = 1.0 / ray_range;

    while f_pos < ray_range && f_pos < limit_range {
        let cur_pos = start.lerp(end, f_pos * ray_range_inv);
        let map_height = get_terrain_height(cur_pos.x, cur_pos.y);

        if map_height >= cur_pos.z {
            return Some(cur_pos);
        } else {
            let diff = cur_pos.z - map_height;
            let mut multiplier = 0.01 * diff;
            if multiplier < 1.0 {
                multiplier = 1.0;
            }
            f_pos += step * multiplier;
        }
    }

    None
}

/// Odpowiednik CMapOutdoor::GetPickingPointWithRayOnlyTerrain.
/// Rzuca promien z startu w kierunku dir (znormalizowanym) na dlugosc distance.
/// Jesli przetnie teren (jego wysokosc > Z na promieniu), zwraca wektor kolizji.
fn get_picking_point_with_ray_only_terrain<F>(
    start: Vec3,
    dir: Vec3,
    distance: f32,
    get_terrain_height: &F,
) -> Option<Vec3>
where
    F: Fn(f32, f32) -> f32,
{
    if distance <= EPSILON {
        return None;
    }

    let end = start + dir * distance;
    
    // 3 poziomy LOD: step 5 (zasieg 5000), step 10 (zasieg 10000), step 100 (zasieg 100000)
    if let Some(hit) = pick_terrain_height(start, end, 5.0, distance, 5000.0, get_terrain_height) {
        return Some(hit);
    }
    if let Some(hit) = pick_terrain_height(start, end, 10.0, distance, 10000.0, get_terrain_height) {
        return Some(hit);
    }
    if let Some(hit) = pick_terrain_height(start, end, 100.0, distance, 100000.0, get_terrain_height) {
        return Some(hit);
    }

    None
}

/// Pelna logika CCamera::ProcessTerrainCollision odtworzona w Rust.
/// Implementuje tez wyznaczanie odpowiednich wektorow promieni, 
/// bazujac na obliczeniach view/cross/up z CCamera.
pub fn process_terrain_collision<F>(
    eye: Vec3,
    target: Vec3,
    get_terrain_height: &F,
) -> Vec3
where
    F: Fn(f32, f32) -> f32,
{
    // Obliczanie wektorow View, Cross (Right), Up z CCamera::SetViewMatrix()
    let mut view = target - eye;
    
    // W C++ CenterRay obliczane jest z NIEZNORMALIZOWANEGO wektora view
    let center_ray = -view;

    let distance = view.length();
    if distance > EPSILON {
        view = view.normalize();
    } else {
        view = Vec3::new(0.0, 0.0, 1.0);
    }

    let mut up = Vec3::new(0.0, 1.0, 0.0);
    
    // Jesli View idzie niemal calkowicie wzdluz osi Z to standardowy wektor Up w gore. 
    // W C++: D3DXVec3Cross(&m_v3Cross, &m_v3Up, &m_v3View) gdzie default Up to (0,0,1). Ale tutaj zrobmy dokladnie
    // Default m_v3Up to (0,0,1) dla cross product z view. 
    // C++ CCamera: D3DXVec3Cross(&m_v3Cross, &m_v3Up, &m_v3View)
    let initial_up = Vec3::new(0.0, 0.0, 1.0);
    
    // Oblicz Right (Cross)
    let mut cross = initial_up.cross(view);
    if cross.length() > EPSILON {
        cross = cross.normalize();
    } else {
        cross = Vec3::new(1.0, 0.0, 0.0);
    }

    // Recompute Up
    up = view.cross(cross);
    if up.length() > EPSILON {
        up = up.normalize();
    } else {
        up = Vec3::new(0.0, 1.0, 0.0);
    }

    // Konstrukcja m_kTargetToCameraBottomRay
    // zrodlo C++: m_kTargetToCameraBottomRay.SetStartPoint(m_v3Target);
    // temp = (v3CenterRay - m_fTerrainCollisionRadius * m_v3Up);
    // m_kTargetToCameraBottomRay.SetDirection(temp, D3DXVec3Length(&temp));
    let tcb_dir_unnorm = center_ray - up * TERRAIN_COLLISION_RADIUS;
    let tcb_dist = tcb_dir_unnorm.length();
    let tcb_dir = if tcb_dist > EPSILON { tcb_dir_unnorm / tcb_dist } else { Vec3::new(0.0, 0.0, -1.0) };
    
    // Konstrukcja m_kCameraBottomToTerrainRay
    // zrodlo C++: m_kCameraBottomToTerrainRay.SetStartPoint(m_v3Eye);
    // m_kCameraBottomToTerrainRay.SetDirection(-m_v3Up, 2.0f * m_fTerrainCollisionRadius);
    let cbt_dir = -up;
    let cbt_dist = 2.0 * TERRAIN_COLLISION_RADIUS;

    // Aplikacja logiki kolizyjnej (ProcessTerrainCollision)
    
    let mut current_eye = eye;

    // 1. Sprawdz m_kTargetToCameraBottomRay
    if let Some(_hit) = get_picking_point_with_ray_only_terrain(target, tcb_dir, tcb_dist, get_terrain_height) {
        let mut check_vector = current_eye - up * 2.0 * TERRAIN_COLLISION_RADIUS;
        
        // Zgodnie z C++: v3CheckVector.z = rPythonBackground.GetHeight(floorf(v3CheckVector.x), floorf(v3CheckVector.y));
        check_vector.z = get_terrain_height(check_vector.x.floor(), check_vector.y.floor());
        
        let new_eye = check_vector + up * 2.0 * TERRAIN_COLLISION_RADIUS;
        if new_eye.z > current_eye.z {
            current_eye = new_eye;
        }
    }

    // 2. Sprawdz m_kCameraBottomToTerrainRay
    // Nalezy uzyc current_eye dla promienia jesli oczko juz zostalo zmodyfikowane w 1. kroku?
    // W C++ SetEye() wywolane jest bezposrednio wiec oko zaktualizowane.
    // C++ set eye sets m_v3Eye ale nie recomputes ray (unless SetEye immediately calls SetViewMatrix() that changes ray?)
    // Tak, SetEye calls SetViewMatrix. But C++ logic just uses current m_kCameraBottomToTerrainRay directly from previous frame unless updated.
    // W C++ m_kCameraBottomToTerrainRay juz jest zdefiniowane, uzywa go od razu. Ale tu uzyjemy najswiezszego eye dla logicznej poprawnosci tak jak robi C++ (czyli jesli oczko sie podnioslo, kolizja od dolu i tak jest juz policzona na STARYM promieniu). 
    // Odwzorujmy C++: promien startuje ze starego eye, tak jak zdefiniowano przed wejsciem do metody.
    if let Some(hit) = get_picking_point_with_ray_only_terrain(eye, cbt_dir, cbt_dist, get_terrain_height) {
        let diff = current_eye - hit;
        if diff.length() < 2.0 * TERRAIN_COLLISION_RADIUS {
            let new_eye = hit + up * 2.0 * TERRAIN_COLLISION_RADIUS;
            // set new_eye wbrew ewentualnemu krokowi 1, gdyz w C++ if/else jest na tym samym poziomie i 2 check wywoluje nowe SetEye.
            current_eye = new_eye;
        }
    }

    current_eye
}

#[repr(C)]
pub struct FfiVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[no_mangle]
pub extern "C" fn m2_camera_clamp_to_terrain(
    eye_x: f32, eye_y: f32, eye_z: f32, 
    target_x: f32, target_y: f32, target_z: f32,
    get_height_cb: extern "C" fn(f32, f32) -> f32
) -> FfiVec3 {
    safe_ffi_call(FfiVec3 { x: eye_x, y: eye_y, z: eye_z }, || {
        let eye = Vec3::new(eye_x, eye_y, eye_z);
        let target = Vec3::new(target_x, target_y, target_z);
        
        let result = process_terrain_collision(eye, target, &|x, y| {
            get_height_cb(x, y)
        });
        
        FfiVec3 {
            x: result.x,
            y: result.y,
            z: result.z,
        }
    })
}
