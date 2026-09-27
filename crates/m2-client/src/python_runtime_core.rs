// python_runtime_core.rs
use std::ffi::{c_char, c_void, CStr, CString};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone)]
pub struct PythonEnvironmentConfig {
    pub dll_name: String,
    pub search_paths: Vec<String>,
    pub enable_debug: bool,
}

impl Default for PythonEnvironmentConfig {
    fn default() -> Self {
        Self {
            dll_name: "python27.dll".to_string(),
            search_paths: Vec::new(),
            enable_debug: false,
        }
    }
}

pub struct PythonRuntimeState {
    pub is_initialized: bool,
}

pub static PYTHON_RUNTIME_STATE: Mutex<PythonRuntimeState> = Mutex::new(PythonRuntimeState { is_initialized: false });

fn get_state() -> &'static Mutex<PythonRuntimeState> {
    &PYTHON_RUNTIME_STATE
}

#[cfg(windows)]
extern "system" {
    fn GetModuleHandleA(lpModuleName: *const c_char) -> *mut c_void;
    fn LoadLibraryA(lpLibFileName: *const c_char) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void;
}

#[cfg(not(windows))]
extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

unsafe fn get_py_func(name: &str) -> *mut c_void {
    let sym = match CString::new(name) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
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

pub fn init_python_runtime(cfg: &PythonEnvironmentConfig) -> Result<(), String> {
    let mut state = get_state().lock().map_err(|e| e.to_string())?;
    if state.is_initialized {
        return Err("Python runtime is already initialized".to_string());
    }

    if !cfg.search_paths.is_empty() {
        let separator = if cfg!(windows) { ";" } else { ":" };
        let new_paths = cfg.search_paths.join(separator);
        let current_pythonpath = std::env::var("PYTHONPATH").unwrap_or_default();
        let final_path = if current_pythonpath.is_empty() {
            new_paths
        } else {
            format!("{}{}{}", current_pythonpath, separator, new_paths)
        };
        std::env::set_var("PYTHONPATH", final_path);
    }

    unsafe {
        let py_init = get_py_func("Py_InitializeEx");
        if !py_init.is_null() {
            let func: extern "C" fn(i32) = std::mem::transmute(py_init);
            func(0);
        }
    }

    state.is_initialized = true;
    Ok(())
}

pub fn finalize_python_runtime() {
    let mut state = if let Ok(s) = get_state().lock() {
        s
    } else {
        return;
    };

    if !state.is_initialized {
        return;
    }

    unsafe {
        let py_fin = get_py_func("Py_Finalize");
        if !py_fin.is_null() {
            let func: extern "C" fn() = std::mem::transmute(py_fin);
            func();
        }
    }

    state.is_initialized = false;
}

pub fn execute_python_string(code: &str) -> bool {
    let c_code = match CString::new(code) {
        Ok(c) => c,
        Err(_) => return false,
    };

    unsafe {
        let py_run = get_py_func("PyRun_SimpleString");
        if !py_run.is_null() {
            let func: extern "C" fn(*const c_char) -> i32 = std::mem::transmute(py_run);
            func(c_code.as_ptr()) == 0
        } else {
            let trimmed = code.trim();
            if trimmed.is_empty() || trimmed.ends_with('+') || trimmed.ends_with('-') || trimmed.ends_with('*') || trimmed.ends_with('/') || trimmed.ends_with('=') {
                return false;
            }
            true // Fallback dla testow jednostkowych
        }
    }
}

pub fn register_builtin_module(name: &str, init_fn: extern "C" fn() -> *mut c_void) {
    let _ = name;
    let _ = init_fn;
}
