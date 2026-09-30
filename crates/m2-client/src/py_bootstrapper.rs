// py_bootstrapper.rs
use std::ffi::CString;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGISTERED_MODULES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PyMethodDef {
    pub ml_name: *const std::ffi::c_char,
    pub ml_meth: *const std::ffi::c_void,
    pub ml_flags: std::ffi::c_int,
    pub ml_doc: *const std::ffi::c_char,
}

unsafe impl Sync for PyMethodDef {}
unsafe impl Send for PyMethodDef {}

#[derive(Debug, PartialEq, Eq)]
pub enum BootstrapperError {
    EmptyRootScript,
    ModuleImportFailed(String),
    ExecutionFailed(String),
}

#[derive(Debug, Clone)]
pub struct BootstrapperConfig {
    pub root_script: String,
    pub import_modules: Vec<String>,
}

impl Default for BootstrapperConfig {
    fn default() -> Self {
        Self {
            root_script: "prototype.py".to_string(),
            import_modules: vec!["system".to_string()],
        }
    }
}

pub fn get_registered_modules() -> Vec<String> {
    REGISTERED_MODULES.lock().unwrap().clone()
}

pub fn clear_registered_modules() {
    REGISTERED_MODULES.lock().unwrap().clear();
}

pub unsafe fn py_init_module(name: &str, methods: *const PyMethodDef) -> *mut std::ffi::c_void {
    let mut sym = get_py_sym("Py_InitModule");
    if sym.is_null() {
        sym = get_py_sym("Py_InitModule4");
    }
    if sym.is_null() {
        sym = get_py_sym("Py_InitModule4_64");
    }

    if !sym.is_null() {
        if let Ok(c_name) = CString::new(name) {
            let func: extern "C" fn(
                *const std::ffi::c_char,
                *const PyMethodDef,
                *const std::ffi::c_char,
                *mut std::ffi::c_void,
                std::ffi::c_int,
            ) -> *mut std::ffi::c_void = std::mem::transmute(sym);
            return func(c_name.as_ptr(), methods, std::ptr::null(), std::ptr::null_mut(), 1013);
        }
    }
    std::ptr::null_mut()
}

pub fn register_module_app() -> *mut std::ffi::c_void {
    static METHODS: &[PyMethodDef] = &[
        PyMethodDef {
            ml_name: b"GetWidth\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_app::m2_app_get_width as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetWidth() -> int\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"GetHeight\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_app::m2_app_get_height as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetHeight() -> int\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"GetTime\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_app::m2_app_get_time as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetTime() -> float\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"GetGlobalTime\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_app::m2_app_get_global_time as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetGlobalTime() -> int\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"Exit\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_app::m2_app_exit as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"Exit() -> None\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"SetCamera\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_app::m2_app_set_camera as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"SetCamera(distance, pitch, rotation, target_height) -> None\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"IsPressed\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_app::m2_app_is_pressed as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"IsPressed(key_code) -> bool\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"EnableSpecialCameraMode\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_app::m2_app_enable_special_camera_mode as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"EnableSpecialCameraMode() -> None\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: std::ptr::null(),
            ml_meth: std::ptr::null(),
            ml_flags: 0,
            ml_doc: std::ptr::null(),
        },
    ];

    if let Ok(mut list) = REGISTERED_MODULES.lock() {
        if !list.contains(&"app".to_string()) {
            list.push("app".to_string());
        }
    }

    unsafe { py_init_module("app", METHODS.as_ptr()) }
}

pub fn register_module_player() -> *mut std::ffi::c_void {
    static METHODS: &[PyMethodDef] = &[
        PyMethodDef {
            ml_name: b"GetStatus\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_player::m2_py_player_get_status as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetStatus(type) -> int\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"SetStatus\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_player::m2_py_player_set_status as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"SetStatus(type, val) -> bool\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"GetItemIndex\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_player::m2_py_player_get_item_index as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetItemIndex(window, slot) -> int\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"GetItemCount\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_player::m2_py_player_get_item_count as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetItemCount(window, slot) -> int\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"GetItemCountByVnum\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_player::m2_py_player_get_item_count_by_vnum as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetItemCountByVnum(vnum) -> int\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"PickCloseItem\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_player::m2_py_player_pick_close_item as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"PickCloseItem() -> None\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"SendClickItem\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_player::m2_py_player_send_click_item as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"SendClickItem(window, slot) -> bool\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"GetMainCharacterPosition\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_player::m2_py_player_get_main_character_position as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetMainCharacterPosition(out_x, out_y, out_z) -> bool\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: std::ptr::null(),
            ml_meth: std::ptr::null(),
            ml_flags: 0,
            ml_doc: std::ptr::null(),
        },
    ];

    if let Ok(mut list) = REGISTERED_MODULES.lock() {
        if !list.contains(&"player".to_string()) {
            list.push("player".to_string());
        }
    }

    unsafe { py_init_module("player", METHODS.as_ptr()) }
}

pub fn register_module_chr() -> *mut std::ffi::c_void {
    static METHODS: &[PyMethodDef] = &[
        PyMethodDef {
            ml_name: b"GetInstance\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_chr::m2_chr_get_instance as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetInstance(vid) -> int\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"SelectInstance\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_chr::m2_chr_select_instance as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"SelectInstance(vid) -> bool\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"BlendLoopMotion\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_chr::m2_chr_blend_loop_motion as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"BlendLoopMotion(vid, motion_id) -> bool\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"GetPixelPosition\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_chr::m2_chr_get_pixel_position as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetPixelPosition(vid, out_x, out_y, out_z) -> bool\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"GetName\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_chr::m2_chr_get_name as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetName(vid, out_buf, len) -> int\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: std::ptr::null(),
            ml_meth: std::ptr::null(),
            ml_flags: 0,
            ml_doc: std::ptr::null(),
        },
    ];

    if let Ok(mut list) = REGISTERED_MODULES.lock() {
        if !list.contains(&"chr".to_string()) {
            list.push("chr".to_string());
        }
    }

    unsafe { py_init_module("chr", METHODS.as_ptr()) }
}

pub fn register_module_net() -> *mut std::ffi::c_void {
    static METHODS: &[PyMethodDef] = &[
        PyMethodDef {
            ml_name: b"Connect\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_net::m2_net_Connect as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"Connect(ip, port) -> bool\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"Disconnect\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_net::m2_net_Disconnect as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"Disconnect() -> None\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"GetMainStream\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_net::m2_net_GetMainStream as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"GetMainStream() -> bool\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: b"SendChatPacket\0".as_ptr() as *const std::ffi::c_char,
            ml_meth: crate::py_bridge_net::m2_net_SendChatPacket as *const std::ffi::c_void,
            ml_flags: 1,
            ml_doc: b"SendChatPacket(chat_str, chat_type, out_buf) -> int\0".as_ptr() as *const std::ffi::c_char,
        },
        PyMethodDef {
            ml_name: std::ptr::null(),
            ml_meth: std::ptr::null(),
            ml_flags: 0,
            ml_doc: std::ptr::null(),
        },
    ];

    if let Ok(mut list) = REGISTERED_MODULES.lock() {
        if !list.contains(&"net".to_string()) {
            list.push("net".to_string());
        }
    }

    unsafe { py_init_module("net", METHODS.as_ptr()) }
}

/// Zwraca liste nazw metod zarejestrowanych dla danego modulu gry
pub fn get_module_methods(module_name: &str) -> Vec<String> {
    match module_name {
        "app" => vec![
            "GetWidth", "GetHeight", "GetTime", "GetGlobalTime", "Exit",
            "SetCamera", "IsPressed", "EnableSpecialCameraMode",
        ].into_iter().map(String::from).collect(),
        "player" => vec![
            "GetStatus", "SetStatus", "GetItemIndex", "GetItemCount",
            "GetItemCountByVnum", "PickCloseItem", "SendClickItem", "GetMainCharacterPosition",
        ].into_iter().map(String::from).collect(),
        "chr" => vec![
            "GetInstance", "SelectInstance", "BlendLoopMotion", "GetPixelPosition", "GetName",
        ].into_iter().map(String::from).collect(),
        "net" => vec![
            "Connect", "Disconnect", "GetMainStream", "SendChatPacket",
        ].into_iter().map(String::from).collect(),
        _ => Vec::new(),
    }
}

/// Rejestracja wbudowanych modulow gry dla srodowiska Pythona (app, player, chr, net)
pub fn register_builtin_modules() {
    register_module_app();
    register_module_player();
    register_module_chr();
    register_module_net();
}

pub fn run_python_bootstrap(cfg: &BootstrapperConfig) -> Result<(), BootstrapperError> {
    if cfg.root_script.trim().is_empty() {
        let err = "Root script cannot be empty".to_string();
        format_syserr(&err);
        return Err(BootstrapperError::EmptyRootScript);
    }

    for module in &cfg.import_modules {
        if module.trim().is_empty() {
            return Err(BootstrapperError::ModuleImportFailed("empty module".to_string()));
        }
    }

    unsafe {
        call_py_initialize();
    }

    register_builtin_modules();

    for module in &cfg.import_modules {
        let code = format!("import {}\n", module);
        let c_code = match CString::new(code) {
            Ok(c) => c,
            Err(e) => return Err(BootstrapperError::ModuleImportFailed(e.to_string())),
        };
        let res = unsafe { call_py_run_simple_string(c_code.as_ptr()) };
        if res != 0 {
            let err = format!("Failed to import module: {}", module);
            format_syserr(&err);
            unsafe { call_py_err_print(); }
            return Err(BootstrapperError::ModuleImportFailed(module.clone()));
        }
    }

    let root_mod = cfg.root_script.strip_suffix(".py").unwrap_or(&cfg.root_script);
    let main_code = format!(
        "import {}\nif hasattr({}, 'Main'):\n    {}.Main()\n",
        root_mod, root_mod, root_mod
    );
    let c_main_code = match CString::new(main_code) {
        Ok(c) => c,
        Err(e) => return Err(BootstrapperError::ExecutionFailed(e.to_string())),
    };
    let res = unsafe { call_py_run_simple_string(c_main_code.as_ptr()) };
    if res != 0 {
        let err = format!("Failed to execute root script Main(): {}", cfg.root_script);
        format_syserr(&err);
        unsafe { call_py_err_print(); }
        return Err(BootstrapperError::ExecutionFailed(cfg.root_script.clone()));
    }

    Ok(())
}

pub fn load_initial_game_scripts() -> Result<(), BootstrapperError> {
    let cfg = BootstrapperConfig {
        root_script: "prototype.py".to_string(),
        import_modules: vec!["system".to_string()],
    };
    run_python_bootstrap(&cfg)
}

fn format_syserr(msg: &str) {
    eprintln!("SYSERR: {}", msg);
}

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

unsafe fn call_py_initialize() {
    let ptr = get_py_sym("Py_Initialize");
    if !ptr.is_null() {
        let func: extern "C" fn() = std::mem::transmute(ptr);
        func();
    }
}

unsafe fn call_py_run_simple_string(command: *const std::ffi::c_char) -> std::ffi::c_int {
    let ptr = get_py_sym("PyRun_SimpleString");
    if !ptr.is_null() {
        let func: extern "C" fn(*const std::ffi::c_char) -> std::ffi::c_int = std::mem::transmute(ptr);
        func(command)
    } else {
        -1 // W trybie testowym bez python27.dll zwracamy -1 (blad symulowany)
    }
}

unsafe fn call_py_err_print() {
    let ptr = get_py_sym("PyErr_Print");
    if !ptr.is_null() {
        let func: extern "C" fn() = std::mem::transmute(ptr);
        func();
    }
}
