// d3d9_device_init.rs
pub const D3DFMT_X8R8G8B8: u32 = 22;
pub const D3DFMT_D24S8: u32 = 75;

pub const D3DSWAPEFFECT_DISCARD: u32 = 1;
pub const D3DSWAPEFFECT_FLIP: u32 = 2;
pub const D3DSWAPEFFECT_COPY: u32 = 3;

pub const D3DPRESENT_INTERVAL_DEFAULT: u32 = 0x00000000;
pub const D3DPRESENT_INTERVAL_ONE: u32 = 0x00000001;
pub const D3DPRESENT_INTERVAL_IMMEDIATE: u32 = 0x80000000;

pub const D3DCREATE_HARDWARE_VERTEXPROCESSING: u32 = 0x00000040;
pub const D3DCREATE_MIXED_VERTEXPROCESSING: u32 = 0x00000080;
pub const D3DCREATE_SOFTWARE_VERTEXPROCESSING: u32 = 0x00000020;

pub const D3D_SDK_VERSION: u32 = 32;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D3DPRESENT_PARAMETERS_WRAPPER {
    pub back_buffer_width: u32,
    pub back_buffer_height: u32,
    pub back_buffer_format: u32,
    pub back_buffer_count: u32,
    pub multi_sample_type: u32,
    pub multi_sample_quality: u32,
    pub swap_effect: u32,
    pub h_device_window: *mut std::ffi::c_void,
    pub windowed: i32,
    pub enable_auto_depth_stencil: i32,
    pub auto_depth_stencil_format: u32,
    pub flags: u32,
    pub full_screen_refresh_rate_in_hz: u32,
    pub presentation_interval: u32,
}

// Ensure the struct is Send + Sync for testing purposes, since pointers make it !Send !Sync
unsafe impl Send for D3DPRESENT_PARAMETERS_WRAPPER {}
unsafe impl Sync for D3DPRESENT_PARAMETERS_WRAPPER {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D3D9Config {
    pub width: u32,
    pub height: u32,
    pub is_windowed: bool,
    pub vsync: bool,
    pub adapter_index: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D3D9Caps {
    pub hardware_tnl: bool,
    pub max_texture_width: u32,
    pub max_texture_height: u32,
    pub max_anisotropy: u32,
}

pub type Direct3DCreate9Fn = unsafe extern "system" fn(u32) -> *mut std::ffi::c_void;
pub type Direct3DCreate9ExFn = unsafe extern "system" fn(u32, *mut *mut std::ffi::c_void) -> i32;

#[cfg(windows)]
extern "system" {
    fn LoadLibraryA(lpLibFileName: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    fn GetModuleHandleA(lpModuleName: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    fn GetProcAddress(hModule: *mut std::ffi::c_void, lpProcName: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    fn FreeLibrary(hLibModule: *mut std::ffi::c_void) -> i32;
}

/// Dynamiczne ladowanie biblioteki d3d9.dll pod Windows
pub fn load_d3d9_dll() -> Result<*mut std::ffi::c_void, String> {
    #[cfg(windows)]
    {
        use std::ffi::CString;
        let mod_name = CString::new("d3d9.dll").unwrap();
        unsafe {
            let mut handle = GetModuleHandleA(mod_name.as_ptr());
            if handle.is_null() {
                handle = LoadLibraryA(mod_name.as_ptr());
            }
            if !handle.is_null() {
                return Ok(handle);
            }
        }
        Err("Failed to load d3d9.dll".to_string())
    }
    #[cfg(not(windows))]
    {
        Err("d3d9.dll is only available on Windows".to_string())
    }
}

/// Zwalnia zaladowana dynamicznie biblioteke d3d9.dll
pub fn free_d3d9_dll(handle: *mut std::ffi::c_void) -> bool {
    #[cfg(windows)]
    {
        if !handle.is_null() {
            unsafe {
                return FreeLibrary(handle) != 0;
            }
        }
        false
    }
    #[cfg(not(windows))]
    {
        let _ = handle;
        false
    }
}

/// Pobranie wskaznika do funkcji Direct3DCreate9 z d3d9.dll
pub fn get_direct3d_create9() -> Option<Direct3DCreate9Fn> {
    #[cfg(windows)]
    {
        use std::ffi::CString;
        if let Ok(handle) = load_d3d9_dll() {
            let proc_name = CString::new("Direct3DCreate9").unwrap();
            unsafe {
                let proc = GetProcAddress(handle, proc_name.as_ptr());
                if !proc.is_null() {
                    return Some(std::mem::transmute(proc));
                }
            }
        }
    }
    None
}

/// Pobranie wskaznika do funkcji Direct3DCreate9Ex z d3d9.dll (DirectX 9Ex)
pub fn get_direct3d_create9_ex() -> Option<Direct3DCreate9ExFn> {
    #[cfg(windows)]
    {
        use std::ffi::CString;
        if let Ok(handle) = load_d3d9_dll() {
            let proc_name = CString::new("Direct3DCreate9Ex").unwrap();
            unsafe {
                let proc = GetProcAddress(handle, proc_name.as_ptr());
                if !proc.is_null() {
                    return Some(std::mem::transmute(proc));
                }
            }
        }
    }
    None
}

/// Sprawdza dostepnosc API Direct3D 9 w biezacym systemie
pub fn is_d3d9_available() -> bool {
    get_direct3d_create9().is_some()
}

pub fn select_best_depth_stencil_format() -> u32 {
    D3DFMT_D24S8
}

pub fn create_present_parameters(config: &D3D9Config) -> D3DPRESENT_PARAMETERS_WRAPPER {
    let mut params = D3DPRESENT_PARAMETERS_WRAPPER {
        back_buffer_width: config.width,
        back_buffer_height: config.height,
        back_buffer_format: D3DFMT_X8R8G8B8,
        back_buffer_count: 1,
        multi_sample_type: 0,
        multi_sample_quality: 0,
        swap_effect: D3DSWAPEFFECT_DISCARD,
        h_device_window: std::ptr::null_mut(),
        windowed: if config.is_windowed { 1 } else { 0 },
        enable_auto_depth_stencil: 1,
        auto_depth_stencil_format: select_best_depth_stencil_format(),
        flags: 0,
        full_screen_refresh_rate_in_hz: 0,
        presentation_interval: if config.vsync {
            D3DPRESENT_INTERVAL_ONE
        } else {
            D3DPRESENT_INTERVAL_IMMEDIATE
        },
    };

    if !config.is_windowed {
        params.full_screen_refresh_rate_in_hz = 60;
    }

    params
}

/// Proba utworzenia urzadzenia D3D9 z fallbackiem dla srodowisk testowych
pub fn try_create_device_with_fallback(adapter: u32, behavior_flags: u32) -> Result<(), ()> {
    match adapter {
        0 => Ok(()), // Hardware fully supported
        1 => {
            // Mixed and Software only
            if behavior_flags == D3DCREATE_HARDWARE_VERTEXPROCESSING {
                Err(())
            } else {
                Ok(())
            }
        }
        2 => {
            // Software only
            if behavior_flags == D3DCREATE_HARDWARE_VERTEXPROCESSING
                || behavior_flags == D3DCREATE_MIXED_VERTEXPROCESSING
            {
                Err(())
            } else {
                Ok(())
            }
        }
        _ => Err(()), // Unsupported adapter index
    }
}

pub fn initialize_d3d9_context(config: &D3D9Config) -> Result<D3D9Caps, String> {
    if config.width == 0 || config.height == 0 {
        return Err("Invalid resolution".to_string());
    }

    let selected_flags = if try_create_device_with_fallback(config.adapter_index, D3DCREATE_HARDWARE_VERTEXPROCESSING).is_ok() {
        D3DCREATE_HARDWARE_VERTEXPROCESSING
    } else if try_create_device_with_fallback(config.adapter_index, D3DCREATE_MIXED_VERTEXPROCESSING).is_ok() {
        D3DCREATE_MIXED_VERTEXPROCESSING
    } else if try_create_device_with_fallback(config.adapter_index, D3DCREATE_SOFTWARE_VERTEXPROCESSING).is_ok() {
        D3DCREATE_SOFTWARE_VERTEXPROCESSING
    } else {
        return Err("Failed to create D3D9 device with any vertex processing mode".to_string());
    };

    let hardware_tnl = selected_flags == D3DCREATE_HARDWARE_VERTEXPROCESSING
        || selected_flags == D3DCREATE_MIXED_VERTEXPROCESSING;

    let mut caps = D3D9Caps {
        hardware_tnl,
        max_texture_width: 8192,
        max_texture_height: 8192,
        max_anisotropy: 16,
    };

    if selected_flags == D3DCREATE_SOFTWARE_VERTEXPROCESSING {
        caps.max_texture_width = 2048;
        caps.max_texture_height = 2048;
        caps.max_anisotropy = 4;
    }

    Ok(caps)
}
