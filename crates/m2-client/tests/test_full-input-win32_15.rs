use m2_client::full_input_win32_15::{Win32InputSystem, InputEvent};

#[test]
fn test_system_initialization() {
    let system = Win32InputSystem::new();
    assert!(system.is_focused);
    assert_eq!(system.event_queue.len(), 0);
    assert_eq!(system.pitch, 0.0);
    assert_eq!(system.yaw, 0.0);
}

#[test]
fn test_system_focus_handling() {
    let mut system = Win32InputSystem::new();
    
    // Test dispatch when focused
    system.dispatch_key_down(0x41, 100);
    assert_eq!(system.event_queue.len(), 1);
    
    system.clear_events();
    
    // Test dispatch when not focused
    system.set_focus(false);
    system.dispatch_key_down(0x42, 200);
    assert_eq!(system.event_queue.len(), 0); // Should be ignored
}

#[test]
fn test_mouse_button_dispatch() {
    let mut system = Win32InputSystem::new();
    
    // Simulate raw mouse payload for left button down (RI_MOUSE_LEFT_BUTTON_DOWN = 0x0001)
    let mut raw_data = [0u8; 24];
    raw_data[4..6].copy_from_slice(&0x0001u16.to_ne_bytes());
    
    system.dispatch_raw_mouse(&raw_data);
    
    let event = system.get_next_event().unwrap();
    if let InputEvent::MouseButton { button, is_down } = event {
        assert_eq!(button, 0);
        assert_eq!(is_down, true);
    } else {
        panic!("Expected MouseButton event");
    }
}

#[test]
fn test_mouse_wheel_dispatch() {
    let mut system = Win32InputSystem::new();
    
    // Simulate raw mouse payload for wheel delta (button_flags = 0x0400, button_data = 120)
    let mut raw_data = [0u8; 24];
    raw_data[4..6].copy_from_slice(&0x0400u16.to_ne_bytes());
    raw_data[6..8].copy_from_slice(&120i16.to_ne_bytes());
    
    system.dispatch_raw_mouse(&raw_data);
    
    let event = system.get_next_event().unwrap();
    if let InputEvent::MouseWheel { delta } = event {
        assert_eq!(delta, 120);
    } else {
        panic!("Expected MouseWheel event");
    }
}

#[test]
fn test_camera_rotation_update() {
    let mut system = Win32InputSystem::new();
    
    let initial_pitch = system.pitch;
    let initial_yaw = system.yaw;
    
    // Simulate raw mouse payload for dx=100, dy=50
    let mut raw_data = [0u8; 24];
    raw_data[12..16].copy_from_slice(&100i32.to_ne_bytes());
    raw_data[16..20].copy_from_slice(&50i32.to_ne_bytes());
    
    system.dispatch_raw_mouse(&raw_data);
    
    assert_ne!(system.pitch, initial_pitch);
    assert_ne!(system.yaw, initial_yaw);
}

#[test]
fn test_clear_events() {
    let mut system = Win32InputSystem::new();
    system.dispatch_key_down(0x41, 100);
    system.dispatch_key_up(0x41, 200);
    
    assert_eq!(system.event_queue.len(), 2);
    
    system.clear_events();
    assert_eq!(system.event_queue.len(), 0);
    assert_eq!(system.get_next_event(), None);
}
