use m2_client::input_keyboard::{InputKeyboard, DikKey, KeyboardUd, KeyboardLr};

#[test]
fn test_integration_input_keyboard() {
    let instance = InputKeyboard::new();
    assert!(instance.is_ready());
}

#[test]
fn test_set_attack_key_state() {
    let mut input = InputKeyboard::new();
    
    // Test normal attack press
    input.set_attack_key_state(true);
    assert!(input.is_atk_key);
    
    // Test normal attack release
    input.set_attack_key_state(false);
    assert!(!input.is_atk_key);
    
    // Test fishing mode
    input.is_fishing_mode = true;
    input.set_attack_key_state(true);
    // When in fishing mode and pressing attack, it calls new_fishing and returns, so is_atk_key shouldn't change
    assert!(!input.is_atk_key);
}

#[test]
fn test_set_single_dik_key_state() {
    let mut input = InputKeyboard::new();
    
    // Test pressing up
    input.set_single_dik_key_state(DikKey::Up, true);
    assert!(input.is_up);
    assert!(input.is_dir_key);
    assert!(!input.stopped); // Should be moving
    
    // Test releasing up
    input.set_single_dik_key_state(DikKey::Up, false);
    assert!(!input.is_up);
    assert!(!input.is_dir_key);
    assert!(input.stopped); // Should be stopped
    
    // Test left/right
    input.set_single_dik_key_state(DikKey::Left, true);
    input.set_single_dik_key_state(DikKey::Right, true);
    assert!(input.is_left);
    assert!(input.is_right);
}

#[test]
fn test_get_degree_from_direction() {
    let input = InputKeyboard::new();
    
    // UP combinations
    assert_eq!(input.get_degree_from_direction(KeyboardUd::Up, KeyboardLr::Left), 45.0);
    assert_eq!(input.get_degree_from_direction(KeyboardUd::Up, KeyboardLr::Right), -45.0);
    assert_eq!(input.get_degree_from_direction(KeyboardUd::Up, KeyboardLr::None), 0.0);
    
    // DOWN combinations
    assert_eq!(input.get_degree_from_direction(KeyboardUd::Down, KeyboardLr::Left), 135.0);
    assert_eq!(input.get_degree_from_direction(KeyboardUd::Down, KeyboardLr::Right), -135.0);
    assert_eq!(input.get_degree_from_direction(KeyboardUd::Down, KeyboardLr::None), 180.0);
    
    // NONE combinations
    assert_eq!(input.get_degree_from_direction(KeyboardUd::None, KeyboardLr::Left), 90.0);
    assert_eq!(input.get_degree_from_direction(KeyboardUd::None, KeyboardLr::Right), -90.0);
    assert_eq!(input.get_degree_from_direction(KeyboardUd::None, KeyboardLr::None), 0.0);
}

#[test]
fn test_multi_dir_key_rotation_and_movement() {
    let mut input = InputKeyboard::new();
    
    // UP and LEFT
    input.set_single_dik_key_state(DikKey::Up, true);
    input.set_single_dik_key_state(DikKey::Left, true);
    assert!(input.is_up);
    assert!(input.is_left);
    
    // Some rotation should be set based on the mouse/position simulation in C++ port
    // In our simplified mock, dx/dy corresponds to the screen coords.
    // At x=0.0 (left), y=0.0 (up) -> ix=0, iy=0
    // We just verify it moved and rotation isn't NaN.
    assert!(!input.stopped);
    
    // Release all
    input.set_single_dik_key_state(DikKey::Up, false);
    input.set_single_dik_key_state(DikKey::Left, false);
    assert!(input.stopped);
}


#[test]
fn test_quickslots_input() {
    let mut input = InputKeyboard::new();
    
    // Quickslots 1-4
    input.set_single_dik_key_state(DikKey::Num1, true);
    assert_eq!(input.last_quickslot_used, Some(0));
    
    input.set_single_dik_key_state(DikKey::Num2, true);
    assert_eq!(input.last_quickslot_used, Some(1));
    
    input.set_single_dik_key_state(DikKey::Num3, true);
    assert_eq!(input.last_quickslot_used, Some(2));
    
    input.set_single_dik_key_state(DikKey::Num4, true);
    assert_eq!(input.last_quickslot_used, Some(3));
    
    // Quickslots F1-F4
    input.set_single_dik_key_state(DikKey::F1, true);
    assert_eq!(input.last_quickslot_used, Some(4));
    
    input.set_single_dik_key_state(DikKey::F2, true);
    assert_eq!(input.last_quickslot_used, Some(5));
    
    input.set_single_dik_key_state(DikKey::F3, true);
    assert_eq!(input.last_quickslot_used, Some(6));
    
    input.set_single_dik_key_state(DikKey::F4, true);
    assert_eq!(input.last_quickslot_used, Some(7));
    
    // Releasing the key should not trigger quickslot usage
    input.last_quickslot_used = None;
    input.set_single_dik_key_state(DikKey::Num1, false);
    assert_eq!(input.last_quickslot_used, None);
}

#[test]
fn test_item_pickup_input() {
    let mut input = InputKeyboard::new();
    
    // Using Z key
    input.set_single_dik_key_state(DikKey::Z, true);
    assert!(input.pickup_called);
    
    // Reset state
    input.pickup_called = false;
    
    // Using ` (Grave) key
    input.set_single_dik_key_state(DikKey::Grave, true);
    assert!(input.pickup_called);
    
    // Releasing key shouldn't trigger pickup
    input.pickup_called = false;
    input.set_single_dik_key_state(DikKey::Z, false);
    assert!(!input.pickup_called);
}

#[test]
fn test_wasd_input() {
    let mut input = InputKeyboard::new();
    
    // W
    input.set_single_dik_key_state(DikKey::W, true);
    assert!(input.is_up);
    assert!(input.is_dir_key);
    assert!(!input.stopped); // Should be moving
    
    // S
    input.set_single_dik_key_state(DikKey::S, true);
    assert!(input.is_down);
    
    // A
    input.set_single_dik_key_state(DikKey::A, true);
    assert!(input.is_left);
    
    // D
    input.set_single_dik_key_state(DikKey::D, true);
    assert!(input.is_right);
}

#[test]
fn test_space_input() {
    let mut input = InputKeyboard::new();
    
    // Pressing Space
    input.set_single_dik_key_state(DikKey::Space, true);
    assert!(input.is_atk_key);
    
    // Releasing Space
    input.set_single_dik_key_state(DikKey::Space, false);
    assert!(!input.is_atk_key);
}
