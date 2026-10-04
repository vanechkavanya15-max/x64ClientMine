use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{OnceLock, RwLock};

use crate::bridge::ffi_guard::safe_ffi_call;

const TYPE_WAYPOINT: u8 = 6;

#[derive(Debug, Clone, PartialEq)]
pub struct AtlasMarkInfo {
    pub by_type: u8,
    pub dw_id: u32,
    pub x: f32,
    pub y: f32,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignalPoint {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct MinimapSignalsState {
    pub waypoints: Vec<AtlasMarkInfo>,
    pub signals: Vec<SignalPoint>,
    pub next_signal_id: u32,
}

impl Default for MinimapSignalsState {
    fn default() -> Self {
        Self {
            waypoints: Vec::new(),
            signals: Vec::new(),
            next_signal_id: 255,
        }
    }
}

static MINIMAP_SIGNALS_STATE: OnceLock<RwLock<MinimapSignalsState>> = OnceLock::new();

pub fn get_minimap_signals_state() -> &'static RwLock<MinimapSignalsState> {
    MINIMAP_SIGNALS_STATE.get_or_init(|| RwLock::new(MinimapSignalsState::default()))
}

pub fn reset_minimap_signals_state() {
    let mut state = get_minimap_signals_state().write().unwrap();
    state.waypoints.clear();
    state.signals.clear();
    state.next_signal_id = 255;
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_add_way_point(
    id: u32,
    x: f32,
    y: f32,
    name: *const c_char,
) -> bool {
    safe_ffi_call(false, || {
        let name_str = if name.is_null() {
            String::new()
        } else {
            match unsafe { CStr::from_ptr(name) }.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => String::new(),
            }
        };

        let mut state = get_minimap_signals_state().write().unwrap();

        // Check if waypoint with id already exists
        if state.waypoints.iter().any(|wp| wp.dw_id == id) {
            return false;
        }

        state.waypoints.push(AtlasMarkInfo {
            by_type: TYPE_WAYPOINT,
            dw_id: id,
            x,
            y,
            text: name_str,
        });

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_remove_way_point(id: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_minimap_signals_state().write().unwrap();
        
        let initial_len = state.waypoints.len();
        state.waypoints.retain(|wp| wp.dw_id != id);
        
        // Return true if an element was removed, false otherwise.
        state.waypoints.len() < initial_len
    })
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_add_signal_point(x: f32, y: f32) -> u32 {
    safe_ffi_call(0, || {
        let id;
        {
            let mut state = get_minimap_signals_state().write().unwrap();
            id = state.next_signal_id;
            state.next_signal_id += 1;
            
            state.signals.push(SignalPoint { id, x, y });
        }
        
        // Call add_way_point internally
        m2_py_minimap_add_way_point(id, x, y, std::ptr::null());
        
        id
    })
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_clear_all_signal_point() -> bool {
    safe_ffi_call(false, || {
        let mut state = get_minimap_signals_state().write().unwrap();
        
        let signal_ids: Vec<u32> = state.signals.iter().map(|s| s.id).collect();
        state.waypoints.retain(|wp| !signal_ids.contains(&wp.dw_id));
        state.signals.clear();
        
        true
    })
}
