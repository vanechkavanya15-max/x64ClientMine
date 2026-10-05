// wgpu_raycast_picker.rs - Wyznaczanie promienia 3D z pozycji myszy (Raycast Picking)
// Odwzorowuje CCamera::CalculateRay oraz CPythonCharacterManager::Pick w kliencie Rust.
// Pozwala na klikanie i wybieranie aktorow (graczy, potworow, NPC) oraz wskazywanie celu ruchu na ziemi.
// Komentarze w jezyku polskim bez polskich znakow diakrytycznych.

use glam::{Mat4, Vec3, Vec4};
use std::sync::{Mutex, OnceLock};

/// Promien w przestrzeni trojwymiarowej
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray3D {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray3D {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Pobiera punkt na promieniu dla odleglosci t
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Test przeciecia ze sfera (bounding sphere postaci)
    /// Zwraca odleglosc t do punktu wejscia, jesli przecina sfere
    pub fn intersect_sphere(&self, center: Vec3, radius: f32) -> Option<f32> {
        let oc = self.origin - center;
        let b = oc.dot(self.direction);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_disc = discriminant.sqrt();
        let t1 = -b - sqrt_disc;
        let t2 = -b + sqrt_disc;

        if t1 >= 0.0 {
            Some(t1)
        } else if t2 >= 0.0 {
            Some(t2)
        } else {
            None
        }
    }

    /// Test przeciecia z prostopadloscianem AABB
    pub fn intersect_aabb(&self, min: Vec3, max: Vec3) -> Option<f32> {
        let inv_dir = Vec3::new(1.0 / self.direction.x, 1.0 / self.direction.y, 1.0 / self.direction.z);
        let t0 = (min - self.origin) * inv_dir;
        let t1 = (max - self.origin) * inv_dir;

        let tmin = t0.min(t1);
        let tmax = t0.max(t1);

        let t_enter = tmin.max_element();
        let t_exit = tmax.min_element();

        if t_enter <= t_exit && t_exit >= 0.0 {
            Some(if t_enter >= 0.0 { t_enter } else { 0.0 })
        } else {
            None
        }
    }
}

/// Globalna macierz View-Projection uzywana do rzutowania promienia
static GLOBAL_INV_VIEW_PROJ: OnceLock<Mutex<Mat4>> = OnceLock::new();

fn get_inv_view_proj() -> &'static Mutex<Mat4> {
    GLOBAL_INV_VIEW_PROJ.get_or_init(|| Mutex::new(Mat4::IDENTITY))
}

pub fn update_picker_view_projection(view_proj: Mat4) {
    if let Ok(mut mat) = get_inv_view_proj().lock() {
        *mat = view_proj.inverse();
    }
}

/// Przelicza wspolrzedne ekranowe myszy (np. 512, 384 w oknie 1024x768) na promien Ray3D
pub fn screen_coords_to_ray(screen_x: f32, screen_y: f32, width: f32, height: f32) -> Ray3D {
    let inv_mat = if let Ok(guard) = get_inv_view_proj().lock() {
        *guard
    } else {
        Mat4::IDENTITY
    };

    // Przeliczenie do przestrzeni NDC [-1, 1]
    let ndc_x = (2.0 * screen_x) / width.max(1.0) - 1.0;
    let ndc_y = 1.0 - (2.0 * screen_y) / height.max(1.0); // odwrocenie osi Y dla ekranu

    // Punkt bliski (near plane: z = 0.0 lub -1.0)
    let near_ndc = Vec4::new(ndc_x, ndc_y, 0.0, 1.0);
    let far_ndc = Vec4::new(ndc_x, ndc_y, 1.0, 1.0);

    let near_world = inv_mat * near_ndc;
    let far_world = inv_mat * far_ndc;

    let near_pt = if near_world.w.abs() > 1e-6 {
        near_world.truncate() / near_world.w
    } else {
        near_world.truncate()
    };

    let far_pt = if far_world.w.abs() > 1e-6 {
        far_world.truncate() / far_world.w
    } else {
        far_world.truncate()
    };

    let dir = (far_pt - near_pt).normalize();
    Ray3D {
        origin: near_pt,
        direction: dir,
    }
}

/// Rekord aktora do testu kolizji kursora
#[derive(Debug, Clone, Copy)]
pub struct ActorPickTarget {
    pub vid: u32,
    pub center: Vec3,
    pub radius: f32,
}

/// Wybiera najblizszego aktora pod promieniem
pub fn pick_closest_actor(ray: &Ray3D, targets: &[ActorPickTarget]) -> Option<u32> {
    let mut closest_vid = None;
    let mut closest_t = f32::MAX;

    for target in targets {
        if let Some(t) = ray.intersect_sphere(target.center, target.radius) {
            if t < closest_t {
                closest_t = t;
                closest_vid = Some(target.vid);
            }
        }
    }

    closest_vid
}

/// Wyznacza punkt uderzenia promienia w ziemie na podstawie probkowania wysokosci terenu
pub fn pick_ground_hit(ray: &Ray3D, max_dist: f32, step_size: f32) -> Option<Vec3> {
    let mut current_dist = 0.0;
    while current_dist <= max_dist {
        let pt = ray.at(current_dist);
        // Pobieramy wysokosc terenu dla wspolrzednych (x, y)
        let terrain_z = crate::terrain_height_evaluator::get_terrain_height_at(pt.x, pt.y);
        
        // Jesli promien zszedl ponizej lub na wysokosc terenu, znalezlismy punkt uderzenia
        if pt.z <= terrain_z {
            return Some(Vec3::new(pt.x, pt.y, terrain_z));
        }

        current_dist += step_size;
    }

    None
}

// ============================================================================
// Eksporty C-ABI (FFI)
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_picker_set_view_projection(mat_ptr: *const f32) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if mat_ptr.is_null() {
            return false;
        }
        let slice = unsafe { std::slice::from_raw_parts(mat_ptr, 16) };
        let mat = Mat4::from_cols_slice(slice);
        update_picker_view_projection(mat);
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_picker_pick_ground_position(
    screen_x: f32,
    screen_y: f32,
    width: f32,
    height: f32,
    out_x: *mut f32,
    out_y: *mut f32,
    out_z: *mut f32,
) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_z.is_null() {
            return false;
        }
        let ray = screen_coords_to_ray(screen_x, screen_y, width, height);
        if let Some(hit) = pick_ground_hit(&ray, 10000.0, 50.0) {
            unsafe {
                *out_x = hit.x;
                *out_y = hit.y;
                *out_z = hit.z;
            }
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_background_get_picking_point(
    screen_x: f32,
    screen_y: f32,
    out_x: *mut f32,
    out_y: *mut f32,
    out_z: *mut f32,
) -> bool {
    m2_picker_pick_ground_position(screen_x, screen_y, 1024.0, 768.0, out_x, out_y, out_z)
}

#[no_mangle]
pub extern "C" fn m2_chr_Pick() -> u32 {
    let (mut cx, mut cy) = (512, 384);
    if let Ok(state) = crate::py_bridge_app_cursor::PY_APP_CURSOR.lock() {
        cx = state.mouse_x;
        cy = state.mouse_y;
    }
    let ray = screen_coords_to_ray(cx as f32, cy as f32, 1024.0, 768.0);
    if let Ok(mgr) = crate::character_manager_core::get_character_manager().read() {
        let targets: Vec<ActorPickTarget> = mgr.alive_instances.values().map(|a| ActorPickTarget {
            vid: a.vid,
            center: Vec3::new(a.x, a.y, a.z + 100.0),
            radius: 80.0,
        }).collect();
        if let Some(vid) = pick_closest_actor(&ray, &targets) {
            return vid;
        }
    }
    crate::py_bridge_chr_transforms::m2_chr_get_selected_instance_vid()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_sphere_intersection_hit() {
        let ray = Ray3D::new(Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 0.0, 1.0));
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 2.0;

        let hit = ray.intersect_sphere(center, radius);
        assert!(hit.is_some());
        let t = hit.unwrap();
        assert!((t - 8.0).abs() < 1e-4); // Odleglosc od -10 do sfery o promieniu 2 to 8
    }

    #[test]
    fn test_ray_sphere_intersection_miss() {
        let ray = Ray3D::new(Vec3::new(0.0, 10.0, -10.0), Vec3::new(0.0, 0.0, 1.0));
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 2.0;

        let hit = ray.intersect_sphere(center, radius);
        assert!(hit.is_none());
    }

    #[test]
    fn test_pick_closest_actor() {
        let ray = Ray3D::new(Vec3::new(0.0, 0.0, -100.0), Vec3::new(0.0, 0.0, 1.0));
        let targets = vec![
            ActorPickTarget { vid: 101, center: Vec3::new(0.0, 0.0, 50.0), radius: 5.0 },
            ActorPickTarget { vid: 102, center: Vec3::new(0.0, 0.0, 10.0), radius: 5.0 }, // Blizszy
        ];

        let picked = pick_closest_actor(&ray, &targets);
        assert_eq!(picked, Some(102));
    }
}
