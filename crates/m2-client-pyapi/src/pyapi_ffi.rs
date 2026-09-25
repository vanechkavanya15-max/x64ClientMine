//! Modul pyapi_ffi dla skrzynki m2-client-pyapi
//! Mostek C-FFI dla integracji interpretera Pythona
//! Eksporty C-FFI m2_pyapi_init, m2_pyapi_dispatch_event, m2_pyapi_shutdown

use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::py_runtime::PyRuntime;

static PY_INITIALIZED: AtomicBool = AtomicBool::new(false);
static RUNTIME: Mutex<Option<PyRuntime>> = Mutex::new(None);

/// Inicjalizacja podsystemu Pythona przez FFI
#[no_mangle]
pub unsafe extern "C" fn m2_pyapi_init() -> bool {
    let mut rt = RUNTIME.lock().unwrap();
    if rt.is_none() {
        let mut runtime = PyRuntime::new();
        let _ = runtime.initialize_gil();
        *rt = Some(runtime);
        PY_INITIALIZED.store(true, Ordering::SeqCst);
        true
    } else {
        true
    }
}

/// Przekazanie zdarzenia sieciowego lub systemowego do skryptu Pythona
#[no_mangle]
pub unsafe extern "C" fn m2_pyapi_dispatch_event(event_name: *const c_char, json_args: *const c_char) -> bool {
    if !PY_INITIALIZED.load(Ordering::SeqCst) {
        return false;
    }
    if event_name.is_null() {
        return false;
    }

    let c_event = match CStr::from_ptr(event_name).to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let args_str = if json_args.is_null() {
        ""
    } else {
        match CStr::from_ptr(json_args).to_str() {
            Ok(s) => s,
            Err(_) => return false,
        }
    };

    let mut rt = RUNTIME.lock().unwrap();
    if let Some(ref mut runtime) = *rt {
        let script = format!("event_{}('{}')", c_event, args_str);
        runtime.execute_script(&script).is_ok()
    } else {
        false
    }
}

/// Zamkniecie podsystemu Pythona i zwolnienie zasobow
#[no_mangle]
pub unsafe extern "C" fn m2_pyapi_shutdown() {
    let mut rt = RUNTIME.lock().unwrap();
    if let Some(mut runtime) = rt.take() {
        let _ = runtime.finalize_gil();
    }
    PY_INITIALIZED.store(false, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_pyapi_ffi_lifecycle() {
        unsafe {
            assert!(m2_pyapi_init());
            
            let ev = CString::new("BINARY_OnServerCommand").unwrap();
            let args = CString::new("test_cmd").unwrap();
            assert!(m2_pyapi_dispatch_event(ev.as_ptr(), args.as_ptr()));

            // Null event check
            assert!(!m2_pyapi_dispatch_event(std::ptr::null(), args.as_ptr()));

            m2_pyapi_shutdown();
            assert!(!m2_pyapi_dispatch_event(ev.as_ptr(), args.as_ptr()));
        }
    }
}
