use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::sync::Mutex;
use crate::terrain_height_calc::{TerrainHeightCalculator, RawHeightMap, HEIGHTMAP_RAW_SIZE};
use crate::terrain_attr_map::TerrainAttrMap;

pub struct MovementState {
    pub target_x: f32,
    pub target_y: f32,
    pub is_moving: bool,
    pub current_dir: f32,
}

static MOVEMENT_STATE: Mutex<MovementState> = Mutex::new(MovementState {
    target_x: 0.0,
    target_y: 0.0,
    is_moving: false,
    current_dir: 0.0,
});

/// Global terrain height calculator (loaded when map changes).
/// If None, returns 0.0 (flat ground) as safe fallback.
static TERRAIN_HEIGHT: Mutex<Option<TerrainHeightCalculator>> = Mutex::new(None);

/// Global terrain attribute map (loaded when map changes).
/// If None, all positions are walkable as safe fallback.
static TERRAIN_ATTR: Mutex<Option<TerrainAttrMap>> = Mutex::new(None);

/// Set the active terrain height data (called when loading a new map sector).
pub fn set_terrain_height(calc: TerrainHeightCalculator) {
    if let Ok(mut guard) = TERRAIN_HEIGHT.lock() {
        *guard = Some(calc);
    }
}

/// Set the active terrain attribute data (called when loading a new map sector).
pub fn set_terrain_attr(attr: TerrainAttrMap) {
    if let Ok(mut guard) = TERRAIN_ATTR.lock() {
        *guard = Some(attr);
    }
}

pub fn get_movement_state() -> (f32, f32, bool) {
    let state = MOVEMENT_STATE.lock().unwrap();
    (state.target_x, state.target_y, state.is_moving)
}

// Metin2 map coordinate system:
// One sector = 25600 units (256 * CELL_SIZE where CELL_SIZE = 100)
// Height map is 131x131 per sector
// Local coords within sector: world_coord % 25600.0, then scale to 0..128 range
const SECTOR_SIZE: f32 = 25600.0;
const CELL_SIZE: f32 = 200.0; // CELLSCALE in C++

#[no_mangle]
pub extern "C" fn m2_move_to(target_x: f32, target_y: f32) -> bool {
    safe_ffi_call(false, || {
        if !m2_is_walkable(target_x, target_y) {
            return false;
        }
        
        let mut state = MOVEMENT_STATE.lock().unwrap();
        state.target_x = target_x;
        state.target_y = target_y;
        state.is_moving = true;
        
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_get_height_at(x: f32, y: f32) -> f32 {
    safe_ffi_call(0.0, || {
        if !x.is_finite() || !y.is_finite() {
            return 0.0;
        }

        // Try real terrain data first
        if let Ok(guard) = TERRAIN_HEIGHT.lock() {
            if let Some(ref calc) = *guard {
                // Convert world coords to local sector coords (0..128 range)
                let local_x = (x % SECTOR_SIZE) / CELL_SIZE;
                let local_y = (y % SECTOR_SIZE) / CELL_SIZE;
                return calc.get_height_triangle(local_x, local_y);
            }
        }

        // Fallback: flat ground at 0.0 (safe default when no terrain loaded)
        0.0
    })
}

#[no_mangle]
pub extern "C" fn m2_is_walkable(x: f32, y: f32) -> bool {
    safe_ffi_call(false, || {
        if !x.is_finite() || !y.is_finite() {
            return false;
        }
        
        // Basic world bounds check (Metin2 maps are typically 2-4 sectors wide)
        if x < 0.0 || y < 0.0 || x > SECTOR_SIZE * 8.0 || y > SECTOR_SIZE * 8.0 {
            return false;
        }

        // Try real terrain attribute data
        if let Ok(guard) = TERRAIN_ATTR.lock() {
            if let Some(ref attr) = *guard {
                // Convert world coords to attribute grid coords
                let ax = (x / CELL_SIZE) as usize;
                let ay = (y / CELL_SIZE) as usize;
                return attr.is_walkable(ax, ay);
            }
        }

        // Fallback: walkable everywhere when no terrain attr loaded
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_move_to_direction(dir_rot: f32) -> bool {
    safe_ffi_call(false, || {
        if !dir_rot.is_finite() {
            return false;
        }
        let mut state = MOVEMENT_STATE.lock().unwrap();
        state.is_moving = true;
        state.current_dir = dir_rot;
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_stop_movement() {
    safe_ffi_call((), || {
        let mut state = MOVEMENT_STATE.lock().unwrap();
        state.is_moving = false;
    })
}
