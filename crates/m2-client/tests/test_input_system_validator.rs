use m2_client::input_system_validator::InputValidator;
use m2_client::full_input_win32_15::{Win32InputSystem, InputEvent};

#[test]
fn test_validator_initial_state() {
    let validator = InputValidator::new();
    assert_eq!(validator.total_processed, 0);
    assert_eq!(validator.last_key_code, None);
    assert_eq!(validator.total_mouse_moves, 0);
    assert_eq!(validator.pitch_accum, 0.0);
    assert_eq!(validator.yaw_accum, 0.0);
}

#[test]
fn test_validator_key_processing() {
    let mut system = Win32InputSystem::new();
    system.dispatch_key_down(0x57, 1000); // W
    
    let mut validator = InputValidator::new();
    validator.validate_and_consume(&mut system);
    
    assert_eq!(validator.total_processed, 1);
    assert_eq!(validator.last_key_code, Some(0x57));
    
    system.dispatch_key_up(0x57, 1050);
    validator.validate_and_consume(&mut system);
    
    assert_eq!(validator.total_processed, 2);
    assert_eq!(validator.last_key_code, None);
}

#[test]
fn test_validator_mouse_processing() {
    let mut system = Win32InputSystem::new();
    
    // Simulate raw mouse payload for dx=10, dy=-5
    let mut raw_data = [0u8; 24];
    raw_data[12..16].copy_from_slice(&10i32.to_ne_bytes());
    raw_data[16..20].copy_from_slice(&(-5i32).to_ne_bytes());
    
    system.dispatch_raw_mouse(&raw_data);
    
    let mut validator = InputValidator::new();
    validator.validate_and_consume(&mut system);
    
    assert_eq!(validator.total_processed, 1);
    assert_eq!(validator.total_mouse_moves, 1);
    assert_eq!(validator.pitch_accum, -5.0);
    assert_eq!(validator.yaw_accum, 10.0);
}

#[test]
fn test_validator_mixed_input() {
    let mut system = Win32InputSystem::new();
    
    // Key down
    system.dispatch_key_down(0x20, 100); // Space
    
    // Mouse move
    let mut raw_data = [0u8; 24];
    raw_data[12..16].copy_from_slice(&50i32.to_ne_bytes());
    raw_data[16..20].copy_from_slice(&100i32.to_ne_bytes());
    system.dispatch_raw_mouse(&raw_data);
    
    // Key up
    system.dispatch_key_up(0x20, 200);
    
    let mut validator = InputValidator::new();
    validator.validate_and_consume(&mut system);
    
    assert_eq!(validator.total_processed, 3);
    assert_eq!(validator.last_key_code, None);
    assert_eq!(validator.total_mouse_moves, 1);
    assert_eq!(validator.pitch_accum, 100.0);
    assert_eq!(validator.yaw_accum, 50.0);
}
