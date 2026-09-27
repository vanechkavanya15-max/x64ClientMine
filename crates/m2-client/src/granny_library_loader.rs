use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::sync::RwLock;

#[derive(Debug, Clone, Default)]
pub struct GrannyApi {
    pub load_file: usize,
    pub free_file: usize,
    pub get_mesh: usize,
    pub sample_model: usize,
    pub is_loaded: bool,
}

pub type GrannyLoadFileInfo = extern "system" fn(*const c_char) -> *mut c_void;
pub type GrannyFreeFileInfo = extern "system" fn(*mut c_void);
pub type GrannyGetMesh = extern "system" fn(*mut c_void, i32) -> *mut c_void;
pub type GrannySampleModelAnimationsAccelerated = extern "system" fn(*mut c_void, i32, *mut c_void, *mut c_void) -> i32;

static GLOBAL_GRANNY_API: RwLock<Option<GrannyApi>> = RwLock::new(None);
static GLOBAL_LIBRARY_HANDLE: RwLock<Option<usize>> = RwLock::new(None);

#[cfg(unix)]
mod platform {
    use std::os::raw::{c_char, c_int, c_void};
    pub const RTLD_NOW: c_int = 2;
    extern "C" {
        pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
        pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        pub fn dlclose(handle: *mut c_void) -> c_int;
    }
}

#[cfg(windows)]
mod platform {
    use std::os::raw::{c_char, c_int, c_void};
    extern "system" {
        pub fn LoadLibraryA(lpLibFileName: *const c_char) -> *mut c_void;
        pub fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void;
        pub fn FreeLibrary(hLibModule: *mut c_void) -> c_int;
    }
}

pub fn load_granny2_library(dll_path: &str) -> Result<GrannyApi, String> {
    if is_granny_available() {
        if let Ok(guard) = GLOBAL_GRANNY_API.read() {
            if let Some(api) = guard.as_ref() {
                return Ok(api.clone());
            }
        }
    }

    let path_c = CString::new(dll_path).map_err(|_| "Invalid DLL path".to_string())?;

    let handle = unsafe {
        #[cfg(unix)]
        {
            platform::dlopen(path_c.as_ptr(), platform::RTLD_NOW)
        }
        #[cfg(windows)]
        {
            platform::LoadLibraryA(path_c.as_ptr())
        }
    };

    if handle.is_null() {
        return Err("Failed to load library".to_string());
    }

    let load_symbol = |name: &str| -> Result<usize, String> {
        let sym_name = CString::new(name).unwrap();
        let sym = unsafe {
            #[cfg(unix)]
            {
                platform::dlsym(handle, sym_name.as_ptr())
            }
            #[cfg(windows)]
            {
                platform::GetProcAddress(handle, sym_name.as_ptr())
            }
        };
        if sym.is_null() {
            Err(format!("Failed to load symbol: {}", name))
        } else {
            Ok(sym as usize)
        }
    };

    let load_file = load_symbol("GrannyLoadFileInfo")?;
    let free_file = load_symbol("GrannyFreeFileInfo")?;
    let get_mesh = load_symbol("GrannyGetMesh")?;
    let sample_model = load_symbol("GrannySampleModelAnimationsAccelerated")?;

    let api = GrannyApi {
        load_file,
        free_file,
        get_mesh,
        sample_model,
        is_loaded: true,
    };

    if let Ok(mut guard) = GLOBAL_GRANNY_API.write() {
        *guard = Some(api.clone());
    }
    if let Ok(mut guard) = GLOBAL_LIBRARY_HANDLE.write() {
        *guard = Some(handle as usize);
    }

    Ok(api)
}

pub fn is_granny_available() -> bool {
    if let Ok(guard) = GLOBAL_GRANNY_API.read() {
        guard.is_some()
    } else {
        false
    }
}

pub fn unload_granny() {
    if let Ok(mut handle_guard) = GLOBAL_LIBRARY_HANDLE.write() {
        if let Some(handle) = handle_guard.take() {
            unsafe {
                #[cfg(unix)]
                {
                    platform::dlclose(handle as *mut c_void);
                }
                #[cfg(windows)]
                {
                    platform::FreeLibrary(handle as *mut c_void);
                }
            }
        }
    }
    
    if let Ok(mut api_guard) = GLOBAL_GRANNY_API.write() {
        *api_guard = None;
    }
}
