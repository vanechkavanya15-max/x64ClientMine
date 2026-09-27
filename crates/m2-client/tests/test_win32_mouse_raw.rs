use m2_client::win32_mouse_raw::*;

#[test]
fn test_register_device() {
    assert!(register_raw_mouse_device(), "Should return true (or succeed on Windows)");
}

#[test]
fn test_parse_raw_input_too_small() {
    let small_data = vec![0u8; 10];
    assert!(parse_raw_input_mouse(&small_data).is_none());
}

#[test]
fn test_parse_raw_input_valid() {
    let mut data = vec![0u8; 24];
    
    // Set usFlags
    data[0..2].copy_from_slice(&1u16.to_ne_bytes());
    // Set usButtonFlags (0x0400 = RI_MOUSE_WHEEL | 0x0004 = RI_MOUSE_RIGHT_BUTTON_DOWN)
    let flags: u16 = 0x0404;
    data[4..6].copy_from_slice(&flags.to_ne_bytes());
    // Set usButtonData (wheel delta = 120)
    data[6..8].copy_from_slice(&120i16.to_ne_bytes());
    // Set lLastX = 50
    data[12..16].copy_from_slice(&50i32.to_ne_bytes());
    // Set lLastY = -25
    data[16..20].copy_from_slice(&(-25i32).to_ne_bytes());

    let delta = parse_raw_input_mouse(&data).unwrap();
    assert_eq!(delta.flags, 1);
    assert_eq!(delta.button_flags, 0x0404);
    assert_eq!(delta.dx, 50);
    assert_eq!(delta.dy, -25);
    assert_eq!(delta.wheel_delta, 120);
}

#[test]
fn test_parse_raw_input_no_wheel() {
    let mut data = vec![0u8; 24];
    // Set usButtonFlags to left click down (0x0001)
    data[4..6].copy_from_slice(&0x0001u16.to_ne_bytes());
    // Set usButtonData to something, but shouldn't be read as wheel
    data[6..8].copy_from_slice(&120i16.to_ne_bytes());
    
    let delta = parse_raw_input_mouse(&data).unwrap();
    assert_eq!(delta.wheel_delta, 0); // Should be 0 since no wheel flag
    assert_eq!(delta.button_flags, 0x0001);
}

#[test]
fn test_apply_camera_rotation_normal() {
    let cfg = MouseSensitivityConfig {
        sensitivity: 0.5,
        invert_y: false,
        smooth_factor: 1.0,
    };
    let delta = RawMouseDelta { dx: 100, dy: 50, wheel_delta: 0, flags: 0, button_flags: 0 };
    
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    
    apply_camera_rotation_delta(delta, &cfg, &mut pitch, &mut yaw);
    
    assert_eq!(yaw, 50.0); // 100 * 0.5
    assert_eq!(pitch, -25.0); // 50 * 0.5, inverted by default logic (minus)
}

#[test]
fn test_apply_camera_rotation_invert_y() {
    let cfg = MouseSensitivityConfig {
        sensitivity: 0.5,
        invert_y: true,
        smooth_factor: 1.0,
    };
    let delta = RawMouseDelta { dx: 100, dy: 50, wheel_delta: 0, flags: 0, button_flags: 0 };
    
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    
    apply_camera_rotation_delta(delta, &cfg, &mut pitch, &mut yaw);
    
    assert_eq!(yaw, 50.0);
    assert_eq!(pitch, 25.0); // positive since invert_y is true
}

#[test]
fn test_apply_camera_rotation_clamping() {
    let cfg = MouseSensitivityConfig {
        sensitivity: 1.0,
        invert_y: false,
        smooth_factor: 1.0,
    };
    // Input massive deltas
    let delta = RawMouseDelta { dx: 5000, dy: -5000, wheel_delta: 0, flags: 0, button_flags: 0 };
    
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    
    apply_camera_rotation_delta(delta, &cfg, &mut pitch, &mut yaw);
    
    // dx clamping limits to 1000, so yaw is 1000
    assert_eq!(yaw, 1000.0 - 720.0); // 1000 normalized to [0, 360) -> 1000 - 360*2 = 280
    assert_eq!(yaw, 280.0);
    
    // dy limits to -1000. pitch delta is -(-1000) = 1000. Clamped to 89.0
    assert_eq!(pitch, 89.0);
}

#[test]
fn test_apply_camera_rotation_pitch_bottom_clamp() {
    let cfg = MouseSensitivityConfig {
        sensitivity: 1.0,
        invert_y: false,
        smooth_factor: 1.0,
    };
    
    // Look way down
    let delta = RawMouseDelta { dx: 0, dy: 1000, wheel_delta: 0, flags: 0, button_flags: 0 };
    let mut pitch = -50.0;
    let mut yaw = 0.0;
    
    apply_camera_rotation_delta(delta, &cfg, &mut pitch, &mut yaw);
    
    // Pitch goes down by 1000, should bottom out at -89.0
    assert_eq!(pitch, -89.0);
}

#[test]
fn test_apply_camera_yaw_normalization() {
    let cfg = MouseSensitivityConfig {
        sensitivity: 1.0,
        invert_y: false,
        smooth_factor: 1.0,
    };
    
    let mut pitch = 0.0;
    let mut yaw = 10.0;
    
    // dx negative, goes below 0
    let delta = RawMouseDelta { dx: -50, dy: 0, wheel_delta: 0, flags: 0, button_flags: 0 };
    apply_camera_rotation_delta(delta, &cfg, &mut pitch, &mut yaw);
    
    // 10 - 50 = -40 -> normalized to 320.0
    assert_eq!(yaw, 320.0);
}
