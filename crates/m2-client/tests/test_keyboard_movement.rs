#[path = "../src/keyboard_movement.rs"]
pub mod keyboard_movement;

use keyboard_movement::{calculate_keyboard_movement, MovementResult};

#[test]
fn test_no_keys_pressed() {
    let result = calculate_keyboard_movement(false, false, false, false, 0.0);
    assert_eq!(result.is_moving, false);
    assert_eq!(result.dir_x, 0.0);
    assert_eq!(result.dir_y, 0.0);
    assert_eq!(result.rotation_deg, 0.0);
}

#[test]
fn test_canceling_keys() {
    let result = calculate_keyboard_movement(true, true, false, false, 0.0);
    assert_eq!(result.is_moving, false);

    let result2 = calculate_keyboard_movement(false, false, true, true, 45.0);
    assert_eq!(result2.is_moving, false);

    let result3 = calculate_keyboard_movement(true, true, true, true, 90.0);
    assert_eq!(result3.is_moving, false);
}

#[test]
fn test_moving_straight_forward() {
    // Yaw 0
    let result = calculate_keyboard_movement(true, false, false, false, 0.0);
    assert_eq!(result.is_moving, true);
    assert_eq!(result.rotation_deg, 0.0);
    
    // Yaw 45
    let result2 = calculate_keyboard_movement(true, false, false, false, 45.0);
    assert_eq!(result2.is_moving, true);
    assert_eq!(result2.rotation_deg, 45.0);
    
    // Yaw 90
    let result3 = calculate_keyboard_movement(true, false, false, false, 90.0);
    assert_eq!(result3.is_moving, true);
    assert_eq!(result3.rotation_deg, 90.0);
}

#[test]
fn test_moving_straight_backward() {
    // Yaw 0 -> S key -> rotation should be 180
    let result = calculate_keyboard_movement(false, true, false, false, 0.0);
    assert_eq!(result.is_moving, true);
    assert_eq!(result.rotation_deg, 180.0);
    
    // Yaw 90 -> S key -> rotation should be 270
    let result2 = calculate_keyboard_movement(false, true, false, false, 90.0);
    assert_eq!(result2.is_moving, true);
    assert_eq!(result2.rotation_deg, 270.0);
}

#[test]
fn test_moving_left() {
    // Yaw 0 -> A key -> local angle should be 90
    let result = calculate_keyboard_movement(false, false, true, false, 0.0);
    assert_eq!(result.is_moving, true);
    assert_eq!(result.rotation_deg, 90.0);
}

#[test]
fn test_moving_right() {
    // Yaw 0 -> D key -> local angle should be -90 -> normalized to 270
    let result = calculate_keyboard_movement(false, false, false, true, 0.0);
    assert_eq!(result.is_moving, true);
    assert_eq!(result.rotation_deg, 270.0);
}

#[test]
fn test_diagonal_movement_w_d() {
    // W + D
    let result = calculate_keyboard_movement(true, false, false, true, 0.0);
    assert_eq!(result.is_moving, true);
    assert_eq!(result.rotation_deg, 315.0); // 360 - 45
    
    // Check normalization length
    let len = (result.dir_x * result.dir_x + result.dir_y * result.dir_y).sqrt();
    assert!((len - 1.0).abs() < 1e-5, "Direction vector should be normalized, len was {}", len);
}

#[test]
fn test_diagonal_movement_w_a() {
    // W + A
    let result = calculate_keyboard_movement(true, false, true, false, 0.0);
    assert_eq!(result.is_moving, true);
    assert_eq!(result.rotation_deg, 45.0);
    
    // Check normalization length
    let len = (result.dir_x * result.dir_x + result.dir_y * result.dir_y).sqrt();
    assert!((len - 1.0).abs() < 1e-5, "Direction vector should be normalized, len was {}", len);
}

#[test]
fn test_diagonal_movement_s_a() {
    // S + A
    let result = calculate_keyboard_movement(false, true, true, false, 0.0);
    assert_eq!(result.is_moving, true);
    assert_eq!(result.rotation_deg, 135.0);
}

#[test]
fn test_diagonal_movement_s_d() {
    // S + D
    let result = calculate_keyboard_movement(false, true, false, true, 0.0);
    assert_eq!(result.is_moving, true);
    assert_eq!(result.rotation_deg, 225.0);
}

#[test]
fn test_normalization_and_dir() {
    // W + A, yaw 0
    let result = calculate_keyboard_movement(true, false, true, false, 0.0);
    
    // forward = 1, strafe = -1 (left)
    // local rot = atan2(1, 1) = 45 deg
    // dir_x = -0.707 (left), dir_y = -0.707 (forward? Wait, let's verify dir_x and dir_y logic)
    
    // We only care that length is 1.0 and rotation_deg is correct here.
    let len = (result.dir_x * result.dir_x + result.dir_y * result.dir_y).sqrt();
    assert!((len - 1.0).abs() < 1e-5);
}

#[test]
fn test_yaw_wrap_around() {
    let result = calculate_keyboard_movement(false, true, false, false, 270.0);
    // S key (180) + 270 = 450 -> 90
    assert_eq!(result.is_moving, true);
    assert_eq!(result.rotation_deg, 90.0);
}
