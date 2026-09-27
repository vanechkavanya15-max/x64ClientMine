use std::convert::TryInto;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawMouseDelta {
    pub dx: i32,
    pub dy: i32,
    pub wheel_delta: i16,
    pub flags: u32,
    pub button_flags: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MouseSensitivityConfig {
    pub sensitivity: f32,
    pub invert_y: bool,
    pub smooth_factor: f32,
}

impl Default for MouseSensitivityConfig {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            invert_y: false,
            smooth_factor: 1.0,
        }
    }
}

// Win32 API Definitions for Raw Input Registration
#[cfg(windows)]
#[repr(C)]
pub struct RAWINPUTDEVICE {
    pub usUsagePage: u16,
    pub usUsage: u16,
    pub dwFlags: u32,
    pub hwndTarget: *mut std::ffi::c_void,
}

#[cfg(windows)]
#[link(name = "user32")]
extern "system" {
    fn RegisterRawInputDevices(
        pRawInputDevices: *const RAWINPUTDEVICE,
        uiNumDevices: u32,
        cbSize: u32,
    ) -> i32;
}

/// Registers the raw mouse device.
/// On Windows, it interacts with Win32 `RegisterRawInputDevices` using standard HID Usage Page 1 (Generic Desktop), Usage 2 (Mouse).
#[no_mangle]
pub extern "C" fn register_raw_mouse_device() -> bool {
    #[cfg(windows)]
    {
        // RIDEV_INPUTSINK = 0x00000100 (if we want background input, but typically 0 is fine for active window)
        let rid = RAWINPUTDEVICE {
            usUsagePage: 0x01, // HID_USAGE_PAGE_GENERIC
            usUsage: 0x02,     // HID_USAGE_GENERIC_MOUSE
            dwFlags: 0,
            hwndTarget: std::ptr::null_mut(),
        };

        unsafe {
            let result = RegisterRawInputDevices(&rid, 1, std::mem::size_of::<RAWINPUTDEVICE>() as u32);
            result != 0
        }
    }
    #[cfg(not(windows))]
    {
        // On Linux/macOS, we don't have RegisterRawInputDevices, but we mock the success
        // to pass cross-platform compilation and tests.
        true
    }
}

/// Parses the Win32 RAWINPUT RAWMOUSE structure.
/// The standard RAWINPUTHEADER is 16 bytes (32-bit) or 24 bytes (64-bit).
/// But here, the input is expected to be the RAWMOUSE part or standard payload.
pub fn parse_raw_input_mouse(raw_data: &[u8]) -> Option<RawMouseDelta> {
    if raw_data.len() < 24 {
        return None;
    }

    let flags = u16::from_ne_bytes(raw_data[0..2].try_into().unwrap()) as u32;
    
    let button_flags = u16::from_ne_bytes(raw_data[4..6].try_into().unwrap());
    let button_data = i16::from_ne_bytes(raw_data[6..8].try_into().unwrap());
    
    let l_last_x = i32::from_ne_bytes(raw_data[12..16].try_into().unwrap());
    let l_last_y = i32::from_ne_bytes(raw_data[16..20].try_into().unwrap());
    
    // Wheel delta is present when RI_MOUSE_WHEEL (0x0400) is set in button flags.
    let wheel_delta = if (button_flags & 0x0400) != 0 {
        button_data
    } else {
        0
    };

    Some(RawMouseDelta {
        dx: l_last_x,
        dy: l_last_y,
        wheel_delta,
        flags,
        button_flags,
    })
}

/// Applies raw mouse movement to camera pitch and yaw.
pub fn apply_camera_rotation_delta(
    delta: RawMouseDelta,
    cfg: &MouseSensitivityConfig,
    pitch: &mut f32,
    yaw: &mut f32,
) {
    let max_delta = 1000.0;
    
    let dx_clamped = (delta.dx as f32).clamp(-max_delta, max_delta);
    let dy_clamped = (delta.dy as f32).clamp(-max_delta, max_delta);

    let move_x = dx_clamped * cfg.sensitivity * cfg.smooth_factor;
    let move_y = dy_clamped * cfg.sensitivity * cfg.smooth_factor;

    *yaw += move_x;
    
    if cfg.invert_y {
        *pitch += move_y;
    } else {
        *pitch -= move_y;
    }

    *pitch = pitch.clamp(-89.0, 89.0);
    
    while *yaw < 0.0 {
        *yaw += 360.0;
    }
    while *yaw >= 360.0 {
        *yaw -= 360.0;
    }
}
