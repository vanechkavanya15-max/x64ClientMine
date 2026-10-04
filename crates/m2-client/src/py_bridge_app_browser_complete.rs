//! Python `app` module web browser integration, window flashing, and external URL handling.

use std::ffi::{CStr, c_char};
use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Default)]
pub struct AppBrowserState {
    pub last_url: String,
    pub is_open: bool,
    pub flash_count: u32,
}

static APP_BROWSER_STATE: OnceLock<Mutex<AppBrowserState>> = OnceLock::new();

pub fn get_app_browser_state() -> &'static Mutex<AppBrowserState> {
    APP_BROWSER_STATE.get_or_init(|| Mutex::new(AppBrowserState::default()))
}

pub fn sanitize_url(url: &str) -> bool {
    let lower = url.trim().to_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

// C-FFI exports
#[no_mangle]
pub extern "C" fn m2_py_app_open_web_browser(url: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if url.is_null() {
            return false;
        }
        let url_str = unsafe { CStr::from_ptr(url) }.to_string_lossy().into_owned();
        if !sanitize_url(&url_str) {
            return false;
        }

        let mut state = get_app_browser_state().lock().unwrap();
        state.last_url = url_str;
        state.is_open = true;

        // In Windows desktop runtime, open default browser if not in headless/test mode
        #[cfg(target_os = "windows")]
        {
            if std::env::var("M2_HEADLESS_TEST").is_err() {
                let _ = std::process::Command::new("cmd")
                    .args(["/C", "start", "", &state.last_url])
                    .spawn();
            }
        }

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_app_is_web_browser_open() -> bool {
    safe_ffi_call(false, || {
        let state = get_app_browser_state().lock().unwrap();
        state.is_open
    })
}

#[no_mangle]
pub extern "C" fn m2_py_app_close_web_browser() -> bool {
    safe_ffi_call(false, || {
        let mut state = get_app_browser_state().lock().unwrap();
        state.is_open = false;
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_app_flash_window() -> bool {
    safe_ffi_call(false, || {
        let mut state = get_app_browser_state().lock().unwrap();
        state.flash_count += 1;
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_app_get_last_opened_url(out_buf: *mut c_char, max_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || max_len == 0 {
            return false;
        }
        let state = get_app_browser_state().lock().unwrap();
        let bytes = state.last_url.as_bytes();
        let copy_len = bytes.len().min(max_len - 1);
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, out_buf, copy_len);
            *out_buf.add(copy_len) = 0;
        }
        true
    })
}
