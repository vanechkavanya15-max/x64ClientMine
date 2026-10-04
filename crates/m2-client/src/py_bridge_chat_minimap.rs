use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{OnceLock, RwLock};

use crate::bridge::ffi_guard::safe_ffi_call;
use crate::chat_buffer_v2::get_chat_buffer;

// -----------------------------------------------------------------------------
// CHAT MODULE
// -----------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn m2_py_chat_append_chat_v2(chat_type: u8, text: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if text.is_null() {
            return false;
        }

        let c_str = unsafe { CStr::from_ptr(text) };
        let string_text = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u32;

        let mut buffer = match get_chat_buffer().lock() {
            Ok(b) => b,
            Err(e) => e.into_inner(),
        };
        buffer.append_line(chat_type, string_text, timestamp)
    })
}


#[no_mangle]
pub extern "C" fn m2_py_chat_get_line_count() -> usize {
    safe_ffi_call(0, || {
        let buffer = match get_chat_buffer().lock() {
            Ok(b) => b,
            Err(e) => e.into_inner(),
        };
        buffer.lines.len()
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_get_line(
    idx: usize,
    out_buf: *mut c_char,
    buf_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || buf_len == 0 {
            return false;
        }

        let buffer = match get_chat_buffer().lock() {
            Ok(b) => b,
            Err(e) => e.into_inner(),
        };

        if idx >= buffer.lines.len() {
            return false;
        }

        let line = &buffer.lines[idx];
        let text_to_copy = if line.sender_name.is_empty() {
            line.text.clone()
        } else {
            format!("{} : {}", line.sender_name, line.text)
        };

        let bytes = text_to_copy.as_bytes();
        let copy_len = std::cmp::min(bytes.len(), buf_len - 1);

        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, copy_len);
            *out_buf.add(copy_len) = 0; // Null terminator
        }

        true
    })
}

// -----------------------------------------------------------------------------
// MINIMAP MODULE
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Waypoint {
    pub x: f32,
    pub y: f32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct MinimapModuleState {
    pub center_x: f32,
    pub center_y: f32,
    pub scale: f32,
    pub is_visible: bool,
    pub waypoints: Vec<Waypoint>,
}

impl Default for MinimapModuleState {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_y: 0.0,
            scale: 1.0,
            is_visible: true,
            waypoints: Vec::new(),
        }
    }
}

static MINIMAP_MODULE_STATE: OnceLock<RwLock<MinimapModuleState>> = OnceLock::new();

pub fn get_minimap_module_state() -> &'static RwLock<MinimapModuleState> {
    MINIMAP_MODULE_STATE.get_or_init(|| RwLock::new(MinimapModuleState::default()))
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_set_minimap_position(x: f32, y: f32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_minimap_module_state().write().unwrap();
        state.center_x = x;
        state.center_y = y;
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_set_scale(scale: f32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_minimap_module_state().write().unwrap();
        state.scale = scale.max(0.0);
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_show_minimap() -> bool {
    safe_ffi_call(false, || {
        let mut state = get_minimap_module_state().write().unwrap();
        state.is_visible = true;
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_hide_minimap() -> bool {
    safe_ffi_call(false, || {
        let mut state = get_minimap_module_state().write().unwrap();
        state.is_visible = false;
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_add_waypoint(x: f32, y: f32, name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }

        let name_str = match unsafe { CStr::from_ptr(name) }.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };

        let mut state = get_minimap_module_state().write().unwrap();
        state.waypoints.push(Waypoint { x, y, name: name_str });
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_minimap_clear_waypoints() -> bool {
    safe_ffi_call(false, || {
        let mut state = get_minimap_module_state().write().unwrap();
        state.waypoints.clear();
        true
    })
}