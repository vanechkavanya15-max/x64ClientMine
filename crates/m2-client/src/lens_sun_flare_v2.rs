use glam::{Mat4, Vec3, Vec4};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlarePiece {
    pub position_x: f32,
    pub position_y: f32,
    pub size: f32,
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub color_a: f32,
}

impl Default for FlarePiece {
    fn default() -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
            size: 0.0,
            color_r: 0.0,
            color_g: 0.0,
            color_b: 0.0,
            color_a: 0.0,
        }
    }
}

pub type TerrainHeightCallback = extern "C" fn(f32, f32) -> f32;

const FLARE_POSITIONS: [f32; 8] = [-0.55, -0.5, -0.45, 0.2, 0.3, 0.95, 0.9, 1.0];
const FLARE_WIDTHS: [f32; 8] = [20.0, 32.0, 20.0, 32.0, 100.0, 32.0, 20.0, 250.0];
const FLARE_COLORS: [[f32; 4]; 8] = [
    [1.0, 1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0, 1.0],
    [0.0, 1.0, 0.0, 0.8],
    [0.3, 0.5, 1.0, 0.9],
    [0.3, 0.5, 1.0, 0.6],
    [1.0, 0.6, 0.9, 0.4],
    [1.0, 0.0, 0.0, 0.5],
    [1.0, 0.6, 0.3, 0.4],
];

const DISTANCE_SCALE: f32 = 1.0;

fn interpolate(start: f32, end: f32, percent: f32) -> f32 {
    start + (end - start) * percent
}

#[no_mangle]
pub extern "C" fn m2_lens_flare_calc(
    target_pos: *const [f32; 3],
    eye_pos: *const [f32; 3],
    camera_dir: *const [f32; 3],
    light_dir: *const [f32; 3],
    view_proj_matrix: *const [f32; 16],
    screen_width: f32,
    screen_height: f32,
    terrain_callback: TerrainHeightCallback,
    out_sun_screen_x: *mut f32,
    out_sun_screen_y: *mut f32,
    out_pieces: *mut FlarePiece,
    out_after_bright: *mut f32,
) -> bool {
    if target_pos.is_null() || eye_pos.is_null() || camera_dir.is_null() || light_dir.is_null() || view_proj_matrix.is_null() {
        return false;
    }

    let target = Vec3::from_array(unsafe { *target_pos });
    let eye = Vec3::from_array(unsafe { *eye_pos });
    let cam_dir = Vec3::from_array(unsafe { *camera_dir }).normalize_or_zero();
    let l_dir = Vec3::from_array(unsafe { *light_dir }).normalize_or_zero();
    
    // Oblicz pozycje slonca (bardzo daleko w kierunku przeciwnym do wektora swiatla)
    let sun_pos = target - l_dir * 99999999.0;
    
    // Test widocznosci - dot product kamery i wektora do slonca
    let sun_dir = (sun_pos - eye).normalize_or_zero();
    let is_in_front = cam_dir.dot(sun_dir) > 0.0;
    
    if !is_in_front {
        if !out_after_bright.is_null() {
            unsafe { *out_after_bright = 0.0; }
        }
        return false;
    }

    // Raycast ze srodka ekranu do slonca przeciwko geometrii terenu (test okluzji)
    let step_size = 200.0;
    let mut current_dist = 0.0;
    let max_dist = 25600.0; // Maksymalny dystans rysowania / lapania terenu
    let mut occluded = false;

    while current_dist <= max_dist {
        let check_pos = eye + sun_dir * current_dist;
        let terrain_h = terrain_callback(check_pos.x, check_pos.y);
        if check_pos.z < terrain_h {
            occluded = true;
            break;
        }
        // Jeśli jestesmy wysoko nad max wysokoscia terenu, przerwij
        if check_pos.z > 50000.0 {
            break;
        }
        current_dist += step_size;
    }

    if occluded {
        if !out_after_bright.is_null() {
            unsafe { *out_after_bright = 0.0; }
        }
        return false;
    }

    // Rzutowanie pozycji slonca na ekran 2D
    let vp_array = unsafe { *view_proj_matrix };
    let vp_mat = Mat4::from_cols_array(&vp_array);
    
    let mut clip_pos = vp_mat * Vec4::new(sun_pos.x, sun_pos.y, sun_pos.z, 1.0);
    if clip_pos.w != 0.0 {
        clip_pos /= clip_pos.w;
    }

    // Transformacja z NDC [-1, 1] do koordynatow ekranu (origin w lewym gornym rogu)
    let screen_x = (clip_pos.x + 1.0) / 2.0 * screen_width;
    let screen_y = (1.0 - clip_pos.y) / 2.0 * screen_height;

    if !out_sun_screen_x.is_null() {
        unsafe { *out_sun_screen_x = screen_x; }
    }
    if !out_sun_screen_y.is_null() {
        unsafe { *out_sun_screen_y = screen_y; }
    }

    // Obliczanie jasnosci zaleznej od odleglosci od srodka ekranu
    let ndc_x = screen_x / screen_width;
    let ndc_y = screen_y / screen_height;
    
    let dist_x = 0.5 - ndc_x;
    let dist_y = 0.5 - ndc_y;
    let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();
    
    let mut after_bright = interpolate(0.0, 1.0, 1.0 - (distance * DISTANCE_SCALE));
    after_bright = after_bright.clamp(0.0, 1.0);

    if !out_after_bright.is_null() {
        unsafe { *out_after_bright = after_bright; }
    }

    if after_bright <= 0.0 {
        return false;
    }

    // Wyznaczanie polozenia pierscieni flary wzdluz osi optycznej
    if !out_pieces.is_null() {
        let f_dx = screen_x - screen_width / 2.0;
        let f_dy = screen_y - screen_height / 2.0;
        
        let pieces_slice = unsafe { std::slice::from_raw_parts_mut(out_pieces, 8) };
        for i in 0..8 {
            let center_x = screen_x - (FLARE_POSITIONS[i] + 1.0) * f_dx;
            let center_y = screen_y - (FLARE_POSITIONS[i] + 1.0) * f_dy;
            
            pieces_slice[i] = FlarePiece {
                position_x: center_x,
                position_y: center_y,
                size: FLARE_WIDTHS[i],
                color_r: FLARE_COLORS[i][0],
                color_g: FLARE_COLORS[i][1],
                color_b: FLARE_COLORS[i][2],
                color_a: FLARE_COLORS[i][3],
            };
        }
    }

    true
}
