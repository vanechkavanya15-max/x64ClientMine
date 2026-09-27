use std::panic::catch_unwind;
use std::sync::{OnceLock, RwLock};

use crate::atlas_map_scaler_v2::m2_atlas_calc_pos;
use crate::minimap_compass_mapper_v2::{MinimapCompassMapperV2, m2_minimap_world_to_screen, MinimapScreenPos};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MinimapWaypoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct MinimapState {
    pub scale: f32,
    pub is_atlas_open: bool,
    pub is_minimap_visible: bool,
    pub waypoints: Vec<MinimapWaypoint>,
}

impl Default for MinimapState {
    fn default() -> Self {
        Self {
            scale: 1.0,
            is_atlas_open: false,
            is_minimap_visible: true,
            waypoints: Vec::new(),
        }
    }
}

static MINIMAP_STATE: OnceLock<RwLock<MinimapState>> = OnceLock::new();

pub fn get_minimap_state() -> &'static RwLock<MinimapState> {
    MINIMAP_STATE.get_or_init(|| RwLock::new(MinimapState::default()))
}

/// Reset state (useful for tests)
pub fn reset_minimap_state() {
    let mut state = get_minimap_state().write().unwrap();
    state.scale = 1.0;
    state.is_atlas_open = false;
    state.is_minimap_visible = true;
    state.waypoints.clear();
}

#[no_mangle]
pub extern "C" fn m2_minimap_set_scale(scale: f32) -> bool {
    catch_unwind(|| {
        let mut state = get_minimap_state().write().unwrap();
        state.scale = scale.max(0.0);
    })
    .is_ok()
}

#[no_mangle]
pub extern "C" fn m2_minimap_get_scale() -> f32 {
    catch_unwind(|| {
        let state = get_minimap_state().read().unwrap();
        state.scale
    })
    .unwrap_or(1.0)
}

#[no_mangle]
pub extern "C" fn m2_minimap_open_atlas_window() -> bool {
    catch_unwind(|| {
        let mut state = get_minimap_state().write().unwrap();
        state.is_atlas_open = true;
    })
    .is_ok()
}

#[no_mangle]
pub extern "C" fn m2_minimap_close_atlas_window() -> bool {
    catch_unwind(|| {
        let mut state = get_minimap_state().write().unwrap();
        state.is_atlas_open = false;
    })
    .is_ok()
}

#[no_mangle]
pub extern "C" fn m2_minimap_is_atlas_open() -> bool {
    catch_unwind(|| {
        let state = get_minimap_state().read().unwrap();
        state.is_atlas_open
    })
    .unwrap_or(false)
}

/// Utility function demonstrating integration with minimap compass mapper.
/// Calculates the compass rotation based on the camera roll.
#[no_mangle]
pub extern "C" fn m2_minimap_get_compass_rotation(camera_rotation: f32) -> f32 {
    catch_unwind(|| {
        MinimapCompassMapperV2::calculate_compass_rotation(camera_rotation)
    })
    .unwrap_or(0.0)
}

/// Utility function demonstrating integration with minimap compass mapper.
/// Calculates screen pos offset for a waypoint relative to player on the minimap compass.
#[no_mangle]
pub extern "C" fn m2_minimap_get_waypoint_compass_screen_pos(
    index: usize,
    player_x: f32,
    player_y: f32,
    radius: f32,
    out_screen_x: *mut f32,
    out_screen_y: *mut f32,
) -> bool {
    catch_unwind(|| {
        let state = get_minimap_state().read().unwrap();
        if index < state.waypoints.len() {
            let wp = &state.waypoints[index];
            let pos = m2_minimap_world_to_screen(player_x, player_y, wp.x, wp.y, radius);
            unsafe {
                if !out_screen_x.is_null() {
                    *out_screen_x = pos.x;
                }
                if !out_screen_y.is_null() {
                    *out_screen_y = pos.y;
                }
            }
            return true;
        }
        false
    })
    .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_minimap_show_minimap() -> bool {
    catch_unwind(|| {
        let mut state = get_minimap_state().write().unwrap();
        state.is_minimap_visible = true;
    })
    .is_ok()
}

#[no_mangle]
pub extern "C" fn m2_minimap_hide_minimap() -> bool {
    catch_unwind(|| {
        let mut state = get_minimap_state().write().unwrap();
        state.is_minimap_visible = false;
    })
    .is_ok()
}

#[no_mangle]
pub extern "C" fn m2_minimap_is_visible() -> bool {
    catch_unwind(|| {
        let state = get_minimap_state().read().unwrap();
        state.is_minimap_visible
    })
    .unwrap_or(true)
}

#[no_mangle]
pub extern "C" fn m2_minimap_add_waypoint(x: f32, y: f32, z: f32) -> bool {
    catch_unwind(|| {
        let mut state = get_minimap_state().write().unwrap();
        state.waypoints.push(MinimapWaypoint { x, y, z });
    })
    .is_ok()
}

#[no_mangle]
pub extern "C" fn m2_minimap_clear_waypoints() -> bool {
    catch_unwind(|| {
        let mut state = get_minimap_state().write().unwrap();
        state.waypoints.clear();
    })
    .is_ok()
}

#[no_mangle]
pub extern "C" fn m2_minimap_get_waypoint(
    index: usize,
    out_x: *mut f32,
    out_y: *mut f32,
    out_z: *mut f32,
) -> bool {
    catch_unwind(|| {
        let state = get_minimap_state().read().unwrap();
        if index < state.waypoints.len() {
            let wp = &state.waypoints[index];
            unsafe {
                if !out_x.is_null() {
                    *out_x = wp.x;
                }
                if !out_y.is_null() {
                    *out_y = wp.y;
                }
                if !out_z.is_null() {
                    *out_z = wp.z;
                }
            }
            return true;
        }
        false
    })
    .unwrap_or(false)
}

/// Utility function demonstrating integration with atlas map scaler.
/// This translates a waypoint to an atlas position.
#[no_mangle]
pub extern "C" fn m2_minimap_get_waypoint_atlas_pos(
    index: usize,
    map_w: f32,
    map_h: f32,
    out_atlas_x: *mut f32,
    out_atlas_y: *mut f32,
) -> bool {
    catch_unwind(|| {
        let state = get_minimap_state().read().unwrap();
        if index < state.waypoints.len() {
            let wp = &state.waypoints[index];
            let atlas_pos = m2_atlas_calc_pos(wp.x, wp.y, map_w, map_h);
            unsafe {
                if !out_atlas_x.is_null() {
                    *out_atlas_x = atlas_pos.x;
                }
                if !out_atlas_y.is_null() {
                    *out_atlas_y = atlas_pos.y;
                }
            }
            return true;
        }
        false
    })
    .unwrap_or(false)
}
