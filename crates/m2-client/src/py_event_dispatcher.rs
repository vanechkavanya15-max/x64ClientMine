use std::ffi::{c_char, c_void, CStr, CString};
use std::sync::Mutex;
use std::sync::OnceLock;


pub static CALL_LOG: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

pub fn get_call_log() -> Vec<String> {
    CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().clone()
}

pub fn clear_call_log() {
    CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().clear();
}

#[repr(C)]
pub struct PyObject {
    _private: [u8; 0],
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

pub unsafe fn get_py_func(name: &str) -> *mut c_void {
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


pub fn is_mock_mode() -> bool {
    unsafe { pyo3::ffi::Py_IsInitialized() == 0 }
}

pub fn call_python_method(module: &str, method: &str, args: &[*mut PyObject]) -> bool {
    let ptr = GLOBAL_PY_MOD_DISPATCHER_0.load(std::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() && args.is_empty() {
        let cb: PyModCallback0 = unsafe { std::mem::transmute(ptr) };
        if let (Ok(c_mod), Ok(c_method)) = (CString::new(module), CString::new(method)) {
            cb(c_mod.as_ptr(), c_method.as_ptr());
            return true;
        }
    }

    if is_mock_mode() {
        let log_entry = format!("{}.{}", module, method);
        CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().push(log_entry);
        return true;
    }

    unsafe {
        let py_import_getmoduledict = get_py_func("PyImport_GetModuleDict");
        let py_dict_getitemstring = get_py_func("PyDict_GetItemString");
        let py_object_hasattrstring = get_py_func("PyObject_HasAttrString");
        let py_object_getattrstring = get_py_func("PyObject_GetAttrString");
        let py_object_callobject = get_py_func("PyObject_CallObject");

        if py_import_getmoduledict.is_null() || py_dict_getitemstring.is_null() 
            || py_object_hasattrstring.is_null() || py_object_getattrstring.is_null() 
            || py_object_callobject.is_null() {
            return false;
        }

        let py_import_getmoduledict_fn: extern "C" fn() -> *mut PyObject = std::mem::transmute(py_import_getmoduledict);
        let py_dict_getitemstring_fn: extern "C" fn(*mut PyObject, *const c_char) -> *mut PyObject = std::mem::transmute(py_dict_getitemstring);
        let py_object_hasattrstring_fn: extern "C" fn(*mut PyObject, *const c_char) -> i32 = std::mem::transmute(py_object_hasattrstring);
        let py_object_getattrstring_fn: extern "C" fn(*mut PyObject, *const c_char) -> *mut PyObject = std::mem::transmute(py_object_getattrstring);
        let py_object_callobject_fn: extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject = std::mem::transmute(py_object_callobject);

        let module_dict = py_import_getmoduledict_fn();
        if module_dict.is_null() {
            return false;
        }

        let c_module = match CString::new(module) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let mod_obj = py_dict_getitemstring_fn(module_dict, c_module.as_ptr());
        if mod_obj.is_null() {
            return false;
        }

        let c_method = match CString::new(method) {
            Ok(s) => s,
            Err(_) => return false,
        };

        if py_object_hasattrstring_fn(mod_obj, c_method.as_ptr()) == 0 {
            return false;
        }

        let method_obj = py_object_getattrstring_fn(mod_obj, c_method.as_ptr());
        if method_obj.is_null() {
            return false;
        }

        // According to trace Py_BuildValue will be used later
        // In python C API, CallObject arguments is a tuple.
        // We will just pass args if it's already a tuple, 
        // For our dispatchers, we will create the tuple using Py_BuildValue and pass it as the single item in args.
        let py_args = if args.is_empty() {
            std::ptr::null_mut()
        } else {
            args[0]
        };

        let result = py_object_callobject_fn(method_obj, py_args);
        
        // CPython typically requires Py_DECREF on method_obj and result
        // To be safe we will check if Py_DecRef exists and call it.
        let py_decref = get_py_func("Py_DecRef");
        if !py_decref.is_null() {
            let py_decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
            py_decref_fn(method_obj);
            if !result.is_null() {
                py_decref_fn(result);
            }
        }

        true
    }
}

pub unsafe fn mock_py_build_value(format: &str, args_str: &str) -> *mut PyObject {
    let log_entry = format!("ARGS: format='{}' values='{}'", format, args_str);
    CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().push(log_entry);
    std::ptr::null_mut()
}

pub fn dispatch_gc_player_points() {
    call_python_method("game", "RefreshStatus", &[]);
}

pub fn dispatch_gc_item_set_or_del() {
    call_python_method("game", "RefreshInventory", &[]);
}

pub fn dispatch_gc_command(cmd: &str) {
    let ptr = GLOBAL_PY_MOD_DISPATCHER_1_STR.load(std::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let cb: PyModCallback1Str = unsafe { std::mem::transmute(ptr) };
        if let (Ok(c_mod), Ok(c_method), Ok(c_cmd)) = (CString::new("game"), CString::new("BINARY_OnServerCommand"), CString::new(cmd)) {
            cb(c_mod.as_ptr(), c_method.as_ptr(), c_cmd.as_ptr());
            return;
        }
    }

    if is_mock_mode() {
        unsafe { mock_py_build_value("(s)", cmd); }
        call_python_method("game", "BINARY_OnServerCommand", &[]);
        return;
    }

    unsafe {
        let py_buildvalue = get_py_func("Py_BuildValue");
        if py_buildvalue.is_null() { return; }
        
        let c_cmd = match CString::new(cmd) {
            Ok(c) => c,
            Err(_) => return,
        };
        
        let py_buildvalue_fn: extern "C" fn(*const c_char, *const c_char) -> *mut PyObject = std::mem::transmute(py_buildvalue);
        let format = CString::new("(s)").unwrap();
        let py_args = py_buildvalue_fn(format.as_ptr(), c_cmd.as_ptr());
        
        if !py_args.is_null() {
            call_python_method("game", "BINARY_OnServerCommand", &[py_args]);
            let py_decref = get_py_func("Py_DecRef");
            if !py_decref.is_null() {
                let py_decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
                py_decref_fn(py_args);
            }
        }
    }
}

pub fn dispatch_gc_quest_info(skin: i32, idx: i32) {
    if is_mock_mode() {
        unsafe { mock_py_build_value("(ii)", &format!("{}, {}", skin, idx)); }
        call_python_method("game", "OpenQuestWindow", &[]);
        return;
    }

    unsafe {
        let py_buildvalue = get_py_func("Py_BuildValue");
        if py_buildvalue.is_null() { return; }
        
        let py_buildvalue_fn: extern "C" fn(*const c_char, i32, i32) -> *mut PyObject = std::mem::transmute(py_buildvalue);
        let format = CString::new("(ii)").unwrap();
        let py_args = py_buildvalue_fn(format.as_ptr(), skin, idx);
        
        if !py_args.is_null() {
            call_python_method("game", "OpenQuestWindow", &[py_args]);
            let py_decref = get_py_func("Py_DecRef");
            if !py_decref.is_null() {
                let py_decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
                py_decref_fn(py_args);
            }
        }
    }
}

pub fn dispatch_gc_shop_open(vid: i32) {
    if is_mock_mode() {
        unsafe { mock_py_build_value("(i)", &format!("{}", vid)); }
        call_python_method("game", "OpenShop", &[]);
        return;
    }

    unsafe {
        let py_buildvalue = get_py_func("Py_BuildValue");
        if py_buildvalue.is_null() { return; }
        
        let py_buildvalue_fn: extern "C" fn(*const c_char, i32) -> *mut PyObject = std::mem::transmute(py_buildvalue);
        let format = CString::new("(i)").unwrap();
        let py_args = py_buildvalue_fn(format.as_ptr(), vid);
        
        if !py_args.is_null() {
            call_python_method("game", "OpenShop", &[py_args]);
            let py_decref = get_py_func("Py_DecRef");
            if !py_decref.is_null() {
                let py_decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
                py_decref_fn(py_args);
            }
        }
    }
}

pub fn dispatch_gc_chat(chat_type: i32, msg: &str) {
    let ptr = GLOBAL_PY_MOD_DISPATCHER_1_STR.load(std::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let cb: PyModCallback1Str = unsafe { std::mem::transmute(ptr) };
        if let (Ok(c_mod), Ok(c_method), Ok(c_msg)) = (CString::new("chat"), CString::new("AppendChat"), CString::new(msg)) {
            cb(c_mod.as_ptr(), c_method.as_ptr(), c_msg.as_ptr());
            return;
        }
    }

    if is_mock_mode() {
        unsafe { mock_py_build_value("(is)", &format!("{}, {}", chat_type, msg)); }
        call_python_method("chat", "AppendChat", &[]);
        return;
    }

    unsafe {
        let py_buildvalue = get_py_func("Py_BuildValue");
        if py_buildvalue.is_null() { return; }
        
        let c_msg = match CString::new(msg) {
            Ok(c) => c,
            Err(_) => return,
        };
        
        let py_buildvalue_fn: extern "C" fn(*const c_char, i32, *const c_char) -> *mut PyObject = std::mem::transmute(py_buildvalue);
        let format = CString::new("(is)").unwrap();
        let py_args = py_buildvalue_fn(format.as_ptr(), chat_type, c_msg.as_ptr());
        
        if !py_args.is_null() {
            call_python_method("chat", "AppendChat", &[py_args]);
            let py_decref = get_py_func("Py_DecRef");
            if !py_decref.is_null() {
                let py_decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
                py_decref_fn(py_args);
            }
        }
    }
}

pub type PyEventCallback0 = extern "C" fn(handle: usize, method: *const c_char);
pub type PyEventCallback1Str = extern "C" fn(handle: usize, method: *const c_char, arg: *const c_char);
pub type PyEventCallback1I32 = extern "C" fn(handle: usize, method: *const c_char, arg: i32);
pub type PyEventCallback2I32 = extern "C" fn(handle: usize, method: *const c_char, x: i32, y: i32);

pub type PyModCallback0 = extern "C" fn(module: *const c_char, method: *const c_char);
pub type PyModCallback1Str = extern "C" fn(module: *const c_char, method: *const c_char, arg: *const c_char);

static GLOBAL_PY_DISPATCHER_0: std::sync::atomic::AtomicPtr<()> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
static GLOBAL_PY_DISPATCHER_1_STR: std::sync::atomic::AtomicPtr<()> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
static GLOBAL_PY_DISPATCHER_1_I32: std::sync::atomic::AtomicPtr<()> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
static GLOBAL_PY_DISPATCHER_2_I32: std::sync::atomic::AtomicPtr<()> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

static GLOBAL_PY_MOD_DISPATCHER_0: std::sync::atomic::AtomicPtr<()> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
static GLOBAL_PY_MOD_DISPATCHER_1_STR: std::sync::atomic::AtomicPtr<()> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

#[no_mangle]
pub extern "C" fn m2_set_py_event_dispatcher_0(cb: PyEventCallback0) {
    GLOBAL_PY_DISPATCHER_0.store(cb as *mut (), std::sync::atomic::Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn m2_set_py_event_dispatcher_1_str(cb: PyEventCallback1Str) {
    GLOBAL_PY_DISPATCHER_1_STR.store(cb as *mut (), std::sync::atomic::Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn m2_set_py_event_dispatcher_1_i32(cb: PyEventCallback1I32) {
    GLOBAL_PY_DISPATCHER_1_I32.store(cb as *mut (), std::sync::atomic::Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn m2_set_py_event_dispatcher_2_i32(cb: PyEventCallback2I32) {
    GLOBAL_PY_DISPATCHER_2_I32.store(cb as *mut (), std::sync::atomic::Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn m2_set_py_mod_dispatcher_0(cb: PyModCallback0) {
    GLOBAL_PY_MOD_DISPATCHER_0.store(cb as *mut (), std::sync::atomic::Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn m2_set_py_mod_dispatcher_1_str(cb: PyModCallback1Str) {
    GLOBAL_PY_MOD_DISPATCHER_1_STR.store(cb as *mut (), std::sync::atomic::Ordering::SeqCst);
}

pub fn call_python_object_method0(py_obj: usize, method: &str) -> bool {
    if py_obj == 0 {
        return false;
    }
    let ptr = GLOBAL_PY_DISPATCHER_0.load(std::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let cb: PyEventCallback0 = unsafe { std::mem::transmute(ptr) };
        if let Ok(c_method) = CString::new(method) {
            cb(py_obj, c_method.as_ptr());
            return true;
        }
    }
    if is_mock_mode() {
        let log_entry = format!("obj_{}.{}", py_obj, method);
        CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().push(log_entry);
        return true;
    }

    unsafe {
        let py_object_hasattrstring = get_py_func("PyObject_HasAttrString");
        let py_object_getattrstring = get_py_func("PyObject_GetAttrString");
        let py_object_callobject = get_py_func("PyObject_CallObject");
        let py_decref = get_py_func("Py_DecRef");

        if py_object_hasattrstring.is_null() || py_object_getattrstring.is_null() || py_object_callobject.is_null() {
            return false;
        }

        let obj_ptr = py_obj as *mut PyObject;
        let c_method = match CString::new(method) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let hasattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> i32 = std::mem::transmute(py_object_hasattrstring);
        if hasattr_fn(obj_ptr, c_method.as_ptr()) == 0 {
            return false;
        }

        let getattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> *mut PyObject = std::mem::transmute(py_object_getattrstring);
        let method_obj = getattr_fn(obj_ptr, c_method.as_ptr());
        if method_obj.is_null() {
            return false;
        }

        let call_fn: extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject = std::mem::transmute(py_object_callobject);
        let result = call_fn(method_obj, std::ptr::null_mut());

        if !py_decref.is_null() {
            let decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
            decref_fn(method_obj);
            if !result.is_null() {
                decref_fn(result);
            }
        }
        true
    }
}

pub fn call_python_object_method1_i32(py_obj: usize, method: &str, val: i32) -> bool {
    if py_obj == 0 {
        return false;
    }
    let ptr = GLOBAL_PY_DISPATCHER_1_I32.load(std::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let cb: PyEventCallback1I32 = unsafe { std::mem::transmute(ptr) };
        if let Ok(c_method) = CString::new(method) {
            cb(py_obj, c_method.as_ptr(), val);
            return true;
        }
    }
    if is_mock_mode() {
        let log_entry = format!("obj_{}.{}({})", py_obj, method, val);
        CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().push(log_entry);
        return true;
    }

    unsafe {
        let py_buildvalue = get_py_func("Py_BuildValue");
        let py_object_hasattrstring = get_py_func("PyObject_HasAttrString");
        let py_object_getattrstring = get_py_func("PyObject_GetAttrString");
        let py_object_callobject = get_py_func("PyObject_CallObject");
        let py_decref = get_py_func("Py_DecRef");

        if py_buildvalue.is_null() || py_object_hasattrstring.is_null() || py_object_getattrstring.is_null() || py_object_callobject.is_null() {
            return false;
        }

        let obj_ptr = py_obj as *mut PyObject;
        let c_method = match CString::new(method) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let hasattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> i32 = std::mem::transmute(py_object_hasattrstring);
        if hasattr_fn(obj_ptr, c_method.as_ptr()) == 0 {
            return false;
        }

        let getattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> *mut PyObject = std::mem::transmute(py_object_getattrstring);
        let method_obj = getattr_fn(obj_ptr, c_method.as_ptr());
        if method_obj.is_null() {
            return false;
        }

        let py_buildvalue_fn: extern "C" fn(*const c_char, i32) -> *mut PyObject = std::mem::transmute(py_buildvalue);
        let format = CString::new("(i)").unwrap();
        let py_args = py_buildvalue_fn(format.as_ptr(), val);

        let call_fn: extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject = std::mem::transmute(py_object_callobject);
        let result = call_fn(method_obj, py_args);

        if !py_decref.is_null() {
            let decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
            decref_fn(method_obj);
            if !py_args.is_null() {
                decref_fn(py_args);
            }
            if !result.is_null() {
                decref_fn(result);
            }
        }
        true
    }
}

pub fn call_python_object_method2_i32(py_obj: usize, method: &str, x: i32, y: i32) -> bool {
    if py_obj == 0 {
        return false;
    }
    let ptr = GLOBAL_PY_DISPATCHER_2_I32.load(std::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let cb: PyEventCallback2I32 = unsafe { std::mem::transmute(ptr) };
        if let Ok(c_method) = CString::new(method) {
            cb(py_obj, c_method.as_ptr(), x, y);
            return true;
        }
    }
    if is_mock_mode() {
        let log_entry = format!("obj_{}.{}({}, {})", py_obj, method, x, y);
        CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().push(log_entry);
        return true;
    }

    unsafe {
        let py_buildvalue = get_py_func("Py_BuildValue");
        let py_object_hasattrstring = get_py_func("PyObject_HasAttrString");
        let py_object_getattrstring = get_py_func("PyObject_GetAttrString");
        let py_object_callobject = get_py_func("PyObject_CallObject");
        let py_decref = get_py_func("Py_DecRef");

        if py_buildvalue.is_null() || py_object_hasattrstring.is_null() || py_object_getattrstring.is_null() || py_object_callobject.is_null() {
            return false;
        }

        let obj_ptr = py_obj as *mut PyObject;
        let c_method = match CString::new(method) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let hasattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> i32 = std::mem::transmute(py_object_hasattrstring);
        if hasattr_fn(obj_ptr, c_method.as_ptr()) == 0 {
            return false;
        }

        let getattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> *mut PyObject = std::mem::transmute(py_object_getattrstring);
        let method_obj = getattr_fn(obj_ptr, c_method.as_ptr());
        if method_obj.is_null() {
            return false;
        }

        let py_buildvalue_fn: extern "C" fn(*const c_char, i32, i32) -> *mut PyObject = std::mem::transmute(py_buildvalue);
        let format = CString::new("(ii)").unwrap();
        let py_args = py_buildvalue_fn(format.as_ptr(), x, y);

        let call_fn: extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject = std::mem::transmute(py_object_callobject);
        let result = call_fn(method_obj, py_args);

        if !py_decref.is_null() {
            let decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
            decref_fn(method_obj);
            if !py_args.is_null() {
                decref_fn(py_args);
            }
            if !result.is_null() {
                decref_fn(result);
            }
        }
        true
    }
}

pub fn call_python_object_method1_str(py_obj: usize, method: &str, s: &str) -> bool {
    if py_obj == 0 {
        return false;
    }
    let ptr = GLOBAL_PY_DISPATCHER_1_STR.load(std::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let cb: PyEventCallback1Str = unsafe { std::mem::transmute(ptr) };
        if let (Ok(c_method), Ok(c_s)) = (CString::new(method), CString::new(s)) {
            cb(py_obj, c_method.as_ptr(), c_s.as_ptr());
            return true;
        }
    }
    if is_mock_mode() {
        let log_entry = format!("obj_{}.{}(\"{}\")", py_obj, method, s);
        CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().push(log_entry);
        return true;
    }

    unsafe {
        let py_buildvalue = get_py_func("Py_BuildValue");
        let py_object_hasattrstring = get_py_func("PyObject_HasAttrString");
        let py_object_getattrstring = get_py_func("PyObject_GetAttrString");
        let py_object_callobject = get_py_func("PyObject_CallObject");
        let py_decref = get_py_func("Py_DecRef");

        if py_buildvalue.is_null() || py_object_hasattrstring.is_null() || py_object_getattrstring.is_null() || py_object_callobject.is_null() {
            return false;
        }

        let obj_ptr = py_obj as *mut PyObject;
        let c_method = match CString::new(method) {
            Ok(m) => m,
            Err(_) => return false,
        };

        let hasattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> i32 = std::mem::transmute(py_object_hasattrstring);
        if hasattr_fn(obj_ptr, c_method.as_ptr()) == 0 {
            return false;
        }

        let getattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> *mut PyObject = std::mem::transmute(py_object_getattrstring);
        let method_obj = getattr_fn(obj_ptr, c_method.as_ptr());
        if method_obj.is_null() {
            return false;
        }

        let c_val = match CString::new(s) {
            Ok(v) => v,
            Err(_) => return false,
        };

        let py_buildvalue_fn: extern "C" fn(*const c_char, *const c_char) -> *mut PyObject = std::mem::transmute(py_buildvalue);
        let format = CString::new("(s)").unwrap();
        let py_args = py_buildvalue_fn(format.as_ptr(), c_val.as_ptr());

        let call_fn: extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject = std::mem::transmute(py_object_callobject);
        let result = call_fn(method_obj, py_args);

        if !py_decref.is_null() {
            let decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
            decref_fn(method_obj);
            if !py_args.is_null() {
                decref_fn(py_args);
            }
            if !result.is_null() {
                decref_fn(result);
            }
        }
        true
    }
}

pub fn call_python_object_method3_i32(py_obj: usize, method: &str, a: i32, b: i32, c: i32) -> bool {
    if py_obj == 0 {
        return false;
    }
    if is_mock_mode() {
        let log_entry = format!("obj_{}.{}({}, {}, {})", py_obj, method, a, b, c);
        CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().push(log_entry);
        return true;
    }

    unsafe {
        let py_buildvalue = get_py_func("Py_BuildValue");
        let py_object_hasattrstring = get_py_func("PyObject_HasAttrString");
        let py_object_getattrstring = get_py_func("PyObject_GetAttrString");
        let py_object_callobject = get_py_func("PyObject_CallObject");
        let py_decref = get_py_func("Py_DecRef");

        if py_buildvalue.is_null() || py_object_hasattrstring.is_null() || py_object_getattrstring.is_null() || py_object_callobject.is_null() {
            return false;
        }

        let obj_ptr = py_obj as *mut PyObject;
        let c_method = match CString::new(method) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let hasattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> i32 = std::mem::transmute(py_object_hasattrstring);
        if hasattr_fn(obj_ptr, c_method.as_ptr()) == 0 {
            return false;
        }

        let getattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> *mut PyObject = std::mem::transmute(py_object_getattrstring);
        let method_obj = getattr_fn(obj_ptr, c_method.as_ptr());
        if method_obj.is_null() {
            return false;
        }

        let py_buildvalue_fn: extern "C" fn(*const c_char, i32, i32, i32) -> *mut PyObject = std::mem::transmute(py_buildvalue);
        let format = CString::new("(iii)").unwrap();
        let py_args = py_buildvalue_fn(format.as_ptr(), a, b, c);

        let call_fn: extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject = std::mem::transmute(py_object_callobject);
        let result = call_fn(method_obj, py_args);

        if !py_decref.is_null() {
            let decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
            decref_fn(method_obj);
            if !py_args.is_null() {
                decref_fn(py_args);
            }
            if !result.is_null() {
                decref_fn(result);
            }
        }
        true
    }
}

pub fn call_python_object_method4_i32(py_obj: usize, method: &str, a: i32, b: i32, c: i32, d: i32) -> bool {
    if py_obj == 0 {
        return false;
    }
    if is_mock_mode() {
        let log_entry = format!("obj_{}.{}({}, {}, {}, {})", py_obj, method, a, b, c, d);
        CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().push(log_entry);
        return true;
    }

    unsafe {
        let py_buildvalue = get_py_func("Py_BuildValue");
        let py_object_hasattrstring = get_py_func("PyObject_HasAttrString");
        let py_object_getattrstring = get_py_func("PyObject_GetAttrString");
        let py_object_callobject = get_py_func("PyObject_CallObject");
        let py_decref = get_py_func("Py_DecRef");

        if py_buildvalue.is_null() || py_object_hasattrstring.is_null() || py_object_getattrstring.is_null() || py_object_callobject.is_null() {
            return false;
        }

        let obj_ptr = py_obj as *mut PyObject;
        let c_method = match CString::new(method) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let hasattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> i32 = std::mem::transmute(py_object_hasattrstring);
        if hasattr_fn(obj_ptr, c_method.as_ptr()) == 0 {
            return false;
        }

        let getattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> *mut PyObject = std::mem::transmute(py_object_getattrstring);
        let method_obj = getattr_fn(obj_ptr, c_method.as_ptr());
        if method_obj.is_null() {
            return false;
        }

        let py_buildvalue_fn: extern "C" fn(*const c_char, i32, i32, i32, i32) -> *mut PyObject = std::mem::transmute(py_buildvalue);
        let format = CString::new("(iiii)").unwrap();
        let py_args = py_buildvalue_fn(format.as_ptr(), a, b, c, d);

        let call_fn: extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject = std::mem::transmute(py_object_callobject);
        let result = call_fn(method_obj, py_args);

        if !py_decref.is_null() {
            let decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
            decref_fn(method_obj);
            if !py_args.is_null() {
                decref_fn(py_args);
            }
            if !result.is_null() {
                decref_fn(result);
            }
        }
        true
    }
}

pub fn call_python_object_method5_i32(py_obj: usize, method: &str, a: i32, b: i32, c: i32, d: i32, e: i32) -> bool {
    if py_obj == 0 {
        return false;
    }
    if is_mock_mode() {
        let log_entry = format!("obj_{}.{}({}, {}, {}, {}, {})", py_obj, method, a, b, c, d, e);
        CALL_LOG.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().push(log_entry);
        return true;
    }

    unsafe {
        let py_buildvalue = get_py_func("Py_BuildValue");
        let py_object_hasattrstring = get_py_func("PyObject_HasAttrString");
        let py_object_getattrstring = get_py_func("PyObject_GetAttrString");
        let py_object_callobject = get_py_func("PyObject_CallObject");
        let py_decref = get_py_func("Py_DecRef");

        if py_buildvalue.is_null() || py_object_hasattrstring.is_null() || py_object_getattrstring.is_null() || py_object_callobject.is_null() {
            return false;
        }

        let obj_ptr = py_obj as *mut PyObject;
        let c_method = match CString::new(method) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let hasattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> i32 = std::mem::transmute(py_object_hasattrstring);
        if hasattr_fn(obj_ptr, c_method.as_ptr()) == 0 {
            return false;
        }

        let getattr_fn: extern "C" fn(*mut PyObject, *const c_char) -> *mut PyObject = std::mem::transmute(py_object_getattrstring);
        let method_obj = getattr_fn(obj_ptr, c_method.as_ptr());
        if method_obj.is_null() {
            return false;
        }

        let py_buildvalue_fn: extern "C" fn(*const c_char, i32, i32, i32, i32, i32) -> *mut PyObject = std::mem::transmute(py_buildvalue);
        let format = CString::new("(iiiii)").unwrap();
        let py_args = py_buildvalue_fn(format.as_ptr(), a, b, c, d, e);

        let call_fn: extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject = std::mem::transmute(py_object_callobject);
        let result = call_fn(method_obj, py_args);

        if !py_decref.is_null() {
            let decref_fn: extern "C" fn(*mut PyObject) = std::mem::transmute(py_decref);
            decref_fn(method_obj);
            if !py_args.is_null() {
                decref_fn(py_args);
            }
            if !result.is_null() {
                decref_fn(result);
            }
        }
        true
    }
}
