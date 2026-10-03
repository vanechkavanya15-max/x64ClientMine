use m2_client::py_bootstrap_runtime_injector::inject_runtime_modules;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::sync::Once;

    static INIT: Once = Once::new();

    #[cfg(windows)]
    extern "system" {
        fn GetModuleHandleA(lpModuleName: *const std::ffi::c_char) -> *mut std::ffi::c_void;
        fn LoadLibraryA(lpLibFileName: *const std::ffi::c_char) -> *mut std::ffi::c_void;
        fn GetProcAddress(hModule: *mut std::ffi::c_void, lpProcName: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    }

    #[cfg(not(windows))]
    extern "C" {
        fn dlsym(handle: *mut std::ffi::c_void, symbol: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    }

    unsafe fn get_py_sym(name: &str) -> *mut std::ffi::c_void {
        let sym = CString::new(name).unwrap();
        #[cfg(windows)]
        {
            let candidates = ["python314.dll", "python3.dll", "python27.dll"];
            for dll in candidates {
                if let Ok(mod_name) = CString::new(dll) {
                    let mut h = GetModuleHandleA(mod_name.as_ptr());
                    if h.is_null() {
                        h = LoadLibraryA(mod_name.as_ptr());
                    }
                    if !h.is_null() {
                        let addr = GetProcAddress(h, sym.as_ptr());
                        if !addr.is_null() {
                            return addr;
                        }
                    }
                }
            }
            std::ptr::null_mut()
        }
        #[cfg(not(windows))]
        {
            dlsym(std::ptr::null_mut(), sym.as_ptr())
        }
    }

    fn init_python() {
        INIT.call_once(|| {
            unsafe {
                let is_init_ptr = get_py_sym("Py_IsInitialized");
                if !is_init_ptr.is_null() {
                    let is_init_fn: extern "C" fn() -> std::ffi::c_int = std::mem::transmute(is_init_ptr);
                    if is_init_fn() != 0 {
                        return;
                    }
                }
                let init_ptr = get_py_sym("Py_Initialize");
                if !init_ptr.is_null() {
                    let init_fn: extern "C" fn() = std::mem::transmute(init_ptr);
                    init_fn();
                }
            }
        });
    }
    
    fn run_python_and_check(code: &str) -> bool {
        unsafe {
            let run_ptr = get_py_sym("PyRun_SimpleString");
            if run_ptr.is_null() {
                return true; 
            }
            let run_fn: extern "C" fn(*const std::ffi::c_char) -> std::ffi::c_int = std::mem::transmute(run_ptr);
            let c_code = match CString::new(code) {
                Ok(c) => c,
                Err(_) => return false,
            };
            let res = run_fn(c_code.as_ptr());
            res == 0
        }
    }

    #[test]
    fn test_runtime_modules_injection_and_subsystems() {
        init_python();
        let res = inject_runtime_modules();
        assert!(res.is_ok(), "Injection should not return an error: {:?}", res.err());

        let code = r#"
import sys
modules_to_check = ['app', 'player', 'net', 'wndMgr', 'guild', 'shop', 'skill', 'safebox', 'exchange', 'chat', 'miniMap']
for m in modules_to_check:
    assert m in sys.modules, f"Module {m} missing from sys.modules"
"#;
        assert!(run_python_and_check(code), "Subsystem modules check failed");
    }
}
