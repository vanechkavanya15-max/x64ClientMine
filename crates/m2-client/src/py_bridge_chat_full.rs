use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

use m2_client::chat_buffer_v2::get_chat_buffer;

#[derive(Debug, Clone, Default)]
pub struct WhisperLineFull {
    pub whisper_type: u8,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct WhisperStateFull {
    pub name: String,
    pub lines: Vec<WhisperLineFull>,
    pub position_x: f32,
    pub position_y: f32,
}

static FULL_WHISPER_MANAGER: OnceLock<Mutex<HashMap<String, WhisperStateFull>>> = OnceLock::new();

fn get_full_whisper_manager() -> &'static Mutex<HashMap<String, WhisperStateFull>> {
    FULL_WHISPER_MANAGER.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn m2_py_chat_full_append_chat(chat_type: u8, text: *const c_char) -> bool {
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
pub extern "C" fn m2_py_chat_full_append_whisper(
    whisper_type: u8,
    name: *const c_char,
    text: *const c_char,
) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() || text.is_null() {
            return false;
        }

        let c_name = unsafe { CStr::from_ptr(name) };
        let c_text = unsafe { CStr::from_ptr(text) };

        let name_str = match c_name.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };

        let text_str = match c_text.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };

        let mut map = match get_full_whisper_manager().lock() {
            Ok(m) => m,
            Err(e) => e.into_inner(),
        };

        let conversation = map
            .entry(name_str.clone())
            .or_insert_with(|| WhisperStateFull {
                name: name_str,
                lines: Vec::new(),
                position_x: 0.0,
                position_y: 0.0,
            });

        conversation.lines.push(WhisperLineFull {
            whisper_type,
            text: text_str,
        });

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_full_set_whisper_position(
    name: *const c_char,
    x: f32,
    y: f32,
) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }

        let c_str = unsafe { CStr::from_ptr(name) };
        let name_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let mut map = match get_full_whisper_manager().lock() {
            Ok(m) => m,
            Err(e) => e.into_inner(),
        };

        if let Some(conversation) = map.get_mut(name_str) {
            conversation.position_x = x;
            conversation.position_y = y;
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_full_clear_chat() {
    safe_ffi_call((), || {
        let mut buffer = match get_chat_buffer().lock() {
            Ok(b) => b,
            Err(e) => e.into_inner(),
        };
        buffer.clear();
    });
}

#[no_mangle]
pub extern "C" fn m2_py_chat_full_is_open_whisper(name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }

        let c_str = unsafe { CStr::from_ptr(name) };
        let name_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let map = match get_full_whisper_manager().lock() {
            Ok(m) => m,
            Err(e) => e.into_inner(),
        };

        map.contains_key(name_str)
    })
}
