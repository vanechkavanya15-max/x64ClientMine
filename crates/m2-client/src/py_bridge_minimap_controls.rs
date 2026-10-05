use std::panic::catch_unwind;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone)]
pub struct MinimapControlsState {
    pub width: f32,
    pub height: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub center_cell_x: f32,
    pub center_cell_y: f32,
    pub scale: f32,
    pub is_created: bool,
}

impl Default for MinimapControlsState {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            center_x: 0.0,
            center_y: 0.0,
            center_cell_x: 0.0,
            center_cell_y: 0.0,
            scale: 1.0,
            is_created: false,
        }
    }
}

static MINIMAP_CONTROLS_STATE: OnceLock<RwLock<MinimapControlsState>> = OnceLock::new();

pub fn get_minimap_controls_state() -> &'static RwLock<MinimapControlsState> {
    MINIMAP_CONTROLS_STATE.get_or_init(|| RwLock::new(MinimapControlsState::default()))
}

pub fn reset_minimap_controls_state() {
    let mut state = get_minimap_controls_state().write().unwrap();
    *state = MinimapControlsState::default();
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_set_minimap_size(width: f32, height: f32) {
    let _ = catch_unwind(|| {
        let mut state = get_minimap_controls_state().write().unwrap();
        state.width = width;
        state.height = height;
    });
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_scale_up() {
    let _ = catch_unwind(|| {
        let mut state = get_minimap_controls_state().write().unwrap();
        state.scale *= 2.0;
        if state.scale >= 4.0 {
            state.scale = 4.0;
        }
        // __SetPosition(); // In complete impl, would recalculate things
    });
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_scale_down() {
    let _ = catch_unwind(|| {
        let mut state = get_minimap_controls_state().write().unwrap();
        state.scale *= 0.5;
        if state.scale <= 0.5 {
            state.scale = 0.5;
        }
        // __SetPosition();
    });
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_create() -> bool {
    catch_unwind(|| {
        let mut state = get_minimap_controls_state().write().unwrap();
        state.is_created = true;
        
        // CPythonMiniMap::Create sets up resources, we just track status
        true
    }).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_destroy() {
    let _ = catch_unwind(|| {
        let mut state = get_minimap_controls_state().write().unwrap();
        state.is_created = false;
    });
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_set_center_position(center_x: f32, center_y: f32) {
    let _ = catch_unwind(|| {
        let mut state = get_minimap_controls_state().write().unwrap();
        state.center_x = center_x;
        state.center_y = center_y;
        
        // CTerrainImpl::TERRAIN_XSIZE = 128, CTerrainImpl::CELLSCALE = 200 => 25600
        // We calculate center cell X/Y
        let terrain_size_x = 25600.0;
        let terrain_size_y = 25600.0;
        let cell_scale = 200.0;
        
        let terrain_coord_x = (center_x / terrain_size_x).floor() as i32;
        let terrain_coord_y = (center_y / terrain_size_y).floor() as i32;
        
        state.center_cell_x = (center_x - (terrain_coord_x as f32 * terrain_size_x)) / cell_scale;
        state.center_cell_y = (center_y - (terrain_coord_y as f32 * terrain_size_y)) / cell_scale;
        
        // __SetPosition();
    });
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_update(center_x: f32, center_y: f32) {
    let _ = catch_unwind(|| {
        let mut needs_update = false;
        {
            let state = get_minimap_controls_state().read().unwrap();
            if state.center_x != center_x || state.center_y != center_y {
                needs_update = true;
            }
        }
        
        if needs_update {
            m2_py_minimap_set_center_position(center_x, center_y);
        }
    });
}
