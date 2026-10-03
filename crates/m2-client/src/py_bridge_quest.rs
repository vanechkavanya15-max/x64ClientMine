//! C-FFI bridge for Python `quest` module exports.

use std::ffi::{CStr, c_char};
use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::ui_quest_dialog::get_ui_quest_dialog;

#[no_mangle]
pub extern "C" fn m2_py_quest_clear() {
    safe_ffi_call((), || {
        if let Ok(mut state) = get_ui_quest_dialog().lock() {
            state.clear();
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_skin(skin_type: u8) {
    safe_ffi_call((), || {
        if let Ok(mut state) = get_ui_quest_dialog().lock() {
            state.skin_type = skin_type;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_title(title: *const c_char) {
    safe_ffi_call((), || {
        if title.is_null() {
            return;
        }
        let title_str = unsafe { CStr::from_ptr(title) }.to_string_lossy().into_owned();
        if let Ok(mut state) = get_ui_quest_dialog().lock() {
            state.title = title_str;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_clock_title(title: *const c_char) {
    safe_ffi_call((), || {
        if title.is_null() {
            return;
        }
        let title_str = unsafe { CStr::from_ptr(title) }.to_string_lossy().into_owned();
        if let Ok(mut state) = get_ui_quest_dialog().lock() {
            state.clock_title = title_str;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_clock_value(seconds: u32) {
    safe_ffi_call((), || {
        if let Ok(mut state) = get_ui_quest_dialog().lock() {
            state.clock_value = seconds;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_counter_name(name: *const c_char) {
    safe_ffi_call((), || {
        if name.is_null() {
            return;
        }
        let name_str = unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned();
        if let Ok(mut state) = get_ui_quest_dialog().lock() {
            state.counter_name = name_str;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_counter_value(val: u32) {
    safe_ffi_call((), || {
        if let Ok(mut state) = get_ui_quest_dialog().lock() {
            state.counter_value = val;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_get_quest_count() -> u32 {
    0
}

static QUEST_NAME: &[u8] = b"Quest\0";

#[no_mangle]
pub extern "C" fn m2_py_quest_get_quest_name(_idx: u32) -> *const c_char {
    QUEST_NAME.as_ptr() as *const c_char
}

