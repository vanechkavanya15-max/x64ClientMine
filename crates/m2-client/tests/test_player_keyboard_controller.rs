use m2_client::player_keyboard_controller::PlayerKeyboardController;
use m2_client::win32_keyboard_hooks::{ActionType, ActionBinding, KeyStateTable};

#[test]
fn test_get_degree_from_direction() {
    // Forward (W)
    assert_eq!(PlayerKeyboardController::get_degree_from_direction(true, false, false, false), 0.0);
    // Backward (S)
    assert_eq!(PlayerKeyboardController::get_degree_from_direction(false, true, false, false), 180.0);
    // Left (A)
    assert_eq!(PlayerKeyboardController::get_degree_from_direction(false, false, true, false), 90.0);
    // Right (D)
    assert_eq!(PlayerKeyboardController::get_degree_from_direction(false, false, false, true), -90.0);
    
    // W + A
    assert_eq!(PlayerKeyboardController::get_degree_from_direction(true, false, true, false), 45.0);
    // W + D
    assert_eq!(PlayerKeyboardController::get_degree_from_direction(true, false, false, true), -45.0);
    // S + A
    assert_eq!(PlayerKeyboardController::get_degree_from_direction(false, true, true, false), 135.0);
    // S + D
    assert_eq!(PlayerKeyboardController::get_degree_from_direction(false, true, false, true), -135.0);
}

#[test]
fn test_update_keys_and_movement() {
    let mut ctrl = PlayerKeyboardController::new();
    let mut key_state = KeyStateTable::new();
    
    let bindings = vec![
        ActionBinding { action_type: ActionType::MoveForward, primary_key: 0x57, modifier: None },
        ActionBinding { action_type: ActionType::MoveRight, primary_key: 0x44, modifier: None },
    ];
    
    // Simulate W and D pressed
    key_state.on_key_down(0x57, 0); // W
    key_state.on_key_down(0x44, 0); // D
    
    ctrl.update_keys(&key_state, &bindings);
    assert!(ctrl.is_up);
    assert!(!ctrl.is_down);
    assert!(!ctrl.is_left);
    assert!(ctrl.is_right);
    
    // Target yaw = 90.0 (camera) - 45.0 (local) = 45.0
    ctrl.update_movement(90.0);
    assert!(ctrl.is_moving);
    assert_eq!(ctrl.target_rotation, 45.0);
}

#[test]
fn test_update_keys_stop_moving() {
    let mut ctrl = PlayerKeyboardController::new();
    let key_state = KeyStateTable::new();
    let bindings = vec![];
    
    ctrl.update_keys(&key_state, &bindings);
    ctrl.update_movement(0.0);
    
    assert!(!ctrl.is_moving);
}

#[test]
fn test_rotation_interpolation() {
    let mut ctrl = PlayerKeyboardController::new();
    ctrl.turn_speed = 90.0; // 90 degrees per second
    ctrl.current_rotation = 0.0;
    ctrl.target_rotation = 180.0;
    
    // Turn 1 second
    ctrl.update_interpolation(1.0);
    assert_eq!(ctrl.current_rotation, 90.0);
    
    // Turn another 1 second
    ctrl.update_interpolation(1.0);
    assert_eq!(ctrl.current_rotation, 180.0);
}

#[test]
fn test_attack_interval() {
    let mut ctrl = PlayerKeyboardController::new();
    ctrl.is_attacking = true;
    ctrl.attack_interval_ms = 300.0;
    ctrl.time_since_last_attack_ms = 0.0;
    
    let mut buf = [0u8; 256];
    
    // Advance time by 200ms -> should not trigger attack
    let written = ctrl.update_attack(0.2, Some(&mut buf));
    assert_eq!(written, 0);
    assert_eq!(ctrl.time_since_last_attack_ms, 200.0);
    
    // Advance time by another 100ms -> should trigger attack (300ms reached)
    let written = ctrl.update_attack(0.1, Some(&mut buf));
    // Size of TPacketCGAttack is normally > 0 (it's 7 bytes usually)
    assert!(written > 0);
    assert_eq!(ctrl.time_since_last_attack_ms, 0.0);
}

#[test]
fn test_packet_dispatch_start_stop() {
    let mut ctrl = PlayerKeyboardController::new();
    ctrl.is_moving = true;
    ctrl.was_moving = false;
    ctrl.current_rotation = 45.0;
    
    let mut buf = [0u8; 256];
    
    // Start moving
    let written = ctrl.update_packet_dispatch(0.1, 100, 200, 1000, Some(&mut buf));
    assert!(written > 0);
    assert!(ctrl.was_moving);
    
    // Stop moving
    ctrl.is_moving = false;
    let written_stop = ctrl.update_packet_dispatch(0.1, 100, 200, 1100, Some(&mut buf));
    assert!(written_stop > 0);
    assert!(!ctrl.was_moving);
}

#[test]
fn test_packet_dispatch_sync() {
    let mut ctrl = PlayerKeyboardController::new();
    ctrl.is_moving = true;
    ctrl.was_moving = true;
    ctrl.move_sync_interval_ms = 500.0;
    ctrl.time_since_last_move_sync_ms = 400.0;
    
    let mut buf = [0u8; 256];
    
    // Advance 50ms, no sync yet
    let written = ctrl.update_packet_dispatch(0.05, 100, 200, 1000, Some(&mut buf));
    assert_eq!(written, 0);
    
    // Advance 60ms, should trigger sync
    let written2 = ctrl.update_packet_dispatch(0.06, 100, 200, 1050, Some(&mut buf));
    assert!(written2 > 0);
    assert!(ctrl.time_since_last_move_sync_ms < 100.0);
}
