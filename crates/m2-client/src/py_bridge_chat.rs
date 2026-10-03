use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

use crate::chat_buffer_v2::get_chat_buffer;
use m2_client_bridge::ffi_guard::safe_ffi_call;

static CHAT_COLORS: OnceLock<Mutex<HashMap<u8, u32>>> = OnceLock::new();

pub fn get_chat_colors() -> &'static Mutex<HashMap<u8, u32>> {
    CHAT_COLORS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn m2_py_chat_append_chat(chat_type: u8, text: *const c_char) -> bool {
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
pub extern "C" fn m2_py_chat_clear_chat() {
    safe_ffi_call((), || {
        let mut buffer = match get_chat_buffer().lock() {
            Ok(b) => b,
            Err(e) => e.into_inner(),
        };
        buffer.clear();
    });
}

#[no_mangle]
pub extern "C" fn m2_py_chat_get_chat_line_count() -> usize {
    safe_ffi_call(0, || {
        let buffer = match get_chat_buffer().lock() {
            Ok(b) => b,
            Err(e) => e.into_inner(),
        };
        buffer.lines.len()
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_set_chat_color(chat_type: u8, color_argb: u32) {
    safe_ffi_call((), || {
        let mut colors = match get_chat_colors().lock() {
            Ok(c) => c,
            Err(e) => e.into_inner(),
        };
        colors.insert(chat_type, color_argb);
    });
}

#[no_mangle]
pub extern "C" fn m2_py_chat_get_chat_color(chat_type: u8) -> u32 {
    safe_ffi_call(0xFFFFFFFF, || {
        let colors = match get_chat_colors().lock() {
            Ok(c) => c,
            Err(e) => e.into_inner(),
        };
        *colors.get(&chat_type).unwrap_or(&0xFFFFFFFF) // Default to white
    })
}

#[derive(Debug, Clone, Default)]
pub struct WhisperConversation {
    pub name: String,
    pub lines: Vec<String>,
}

static WHISPER_MAP: OnceLock<Mutex<HashMap<String, WhisperConversation>>> = OnceLock::new();

pub fn get_whisper_map() -> &'static Mutex<HashMap<String, WhisperConversation>> {
    WHISPER_MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Tworzy nowe okno lub bufor szeptu dla podanej nazwy gracza (chat.CreateWhisper).
#[no_mangle]
pub extern "C" fn m2_py_chat_create_whisper(name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(name) };
        let name_str = match c_str.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };
        let mut map = match get_whisper_map().lock() {
            Ok(m) => m,
            Err(e) => e.into_inner(),
        };
        map.entry(name_str.clone()).or_insert(WhisperConversation {
            name: name_str,
            lines: Vec::new(),
        });
        true
    })
}

/// Renderuje zawartosc okna szeptu (chat.RenderWhisper).
#[no_mangle]
pub extern "C" fn m2_py_chat_render_whisper(name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(name) };
        let name_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };
        let map = match get_whisper_map().lock() {
            Ok(m) => m,
            Err(e) => e.into_inner(),
        };
        map.contains_key(name_str)
    })
}
