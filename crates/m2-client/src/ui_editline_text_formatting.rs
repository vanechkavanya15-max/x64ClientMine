// ui_editline_text_formatting.rs - Text Formatting FFI overrides for Metin2 Pure Rust
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{LazyLock, RwLock};

#[derive(Debug, Clone)]
pub struct TextFormatState {
    pub limit_width: f32,
    pub multi_line: bool,
    pub is_secret: bool,
    pub max_chars: i32,
    pub is_outline: bool,
    pub is_feather: bool,
    pub font_name: String,
}

impl Default for TextFormatState {
    fn default() -> Self {
        Self {
            limit_width: 0.0,
            multi_line: false,
            is_secret: false,
            max_chars: 0,
            is_outline: false,
            is_feather: false,
            font_name: String::new(),
        }
    }
}

pub static TEXT_FORMAT_STATES: LazyLock<RwLock<HashMap<u32, TextFormatState>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

fn get_or_create_state(handle: u32) -> TextFormatState {
    let map = TEXT_FORMAT_STATES.read().unwrap();
    map.get(&handle).cloned().unwrap_or_default()
}

fn update_state<F>(handle: u32, mut update_fn: F)
where
    F: FnMut(&mut TextFormatState),
{
    let mut map = TEXT_FORMAT_STATES.write().unwrap();
    let state = map.entry(handle).or_insert_with(TextFormatState::default);
    update_fn(state);
}

fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

// -----------------------------------------------------------------------------
// FFI Overrides replacing stubs from PythonBridge
// The instructions specify exact naming: wndMgr_SetLimitWidth, etc.
// -----------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn wndMgr_SetLimitWidth(handle: u32, width: f32) {
    update_state(handle, |state| state.limit_width = width);
}

#[no_mangle]
pub extern "C" fn wndMgr_SetMultiLine(handle: u32, enable: bool) {
    update_state(handle, |state| state.multi_line = enable);
}

#[no_mangle]
pub extern "C" fn wndMgr_SetSecret(handle: u32, enable: bool) {
    update_state(handle, |state| state.is_secret = enable);
}

#[no_mangle]
pub extern "C" fn wndMgr_SetMax(handle: u32, max: i32) {
    update_state(handle, |state| state.max_chars = max);
}

#[no_mangle]
pub extern "C" fn wndMgr_SetOutline(handle: u32, enable: bool) {
    update_state(handle, |state| state.is_outline = enable);
}

#[no_mangle]
pub extern "C" fn wndMgr_SetFeather(handle: u32, enable: bool) {
    update_state(handle, |state| state.is_feather = enable);
}

#[no_mangle]
pub extern "C" fn wndMgr_SetFontName(handle: u32, name: *const c_char) {
    let font_name = cstr_to_string(name);
    update_state(handle, |state| state.font_name = font_name.clone());
}
