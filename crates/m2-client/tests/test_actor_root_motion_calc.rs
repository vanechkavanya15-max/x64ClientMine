#[path = "../src/actor_root_motion_calc.rs"]
mod actor_root_motion_calc;

use actor_root_motion_calc::{calculate_accumulated_movement, ActorRootMotionState};
use glam::Vec3;

#[test]
fn test_calculate_accumulated_movement_no_rotation() {
    let base_movement = Vec3::new(10.0, 0.0, 0.0);
    let result = calculate_accumulated_movement(base_movement, 0.0);
    
    assert!((result.x - 10.0).abs() < 1e-5);
    assert!((result.y - 0.0).abs() < 1e-5);
    assert!((result.z - 0.0).abs() < 1e-5);
}

#[test]
fn test_calculate_accumulated_movement_90_deg_rotation() {
    let base_movement = Vec3::new(10.0, 0.0, 0.0);
    // Metin2 coordinates usually Z is up, rotate by 90 degrees around Z
    // X -> Y
    let result = calculate_accumulated_movement(base_movement, 90.0);
    
    assert!((result.x - 0.0).abs() < 1e-5);
    assert!((result.y - 10.0).abs() < 1e-5);
    assert!((result.z - 0.0).abs() < 1e-5);
}

#[test]
fn test_calculate_accumulated_movement_180_deg_rotation() {
    let base_movement = Vec3::new(10.0, 0.0, 0.0);
    let result = calculate_accumulated_movement(base_movement, 180.0);
    
    assert!((result.x - (-10.0)).abs() < 1e-5);
    assert!((result.y - 0.0).abs() < 1e-5);
    assert!((result.z - 0.0).abs() < 1e-5);
}

#[test]
fn test_actor_root_motion_state() {
    let mut state = ActorRootMotionState::new(Vec3::new(100.0, 200.0, 300.0));
    
    let base_movement = Vec3::new(50.0, 0.0, 0.0);
    let rot_movement = calculate_accumulated_movement(base_movement, 90.0); // -> (0.0, 50.0, 0.0)
    
    state.accumulate_movement(rot_movement);
    
    assert!((state.position.x - 100.0).abs() < 1e-5);
    assert!((state.position.y - 250.0).abs() < 1e-5);
    assert!((state.position.z - 300.0).abs() < 1e-5);
}
