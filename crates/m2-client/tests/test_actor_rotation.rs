#[path = "../src/actor_rotation.rs"]
mod actor_rotation;

use actor_rotation::ActorRotation;

#[test]
fn test_smooth_rotation_shortest_path() {
    let mut rot = ActorRotation::new(0.0);
    rot.set_target(90.0);
    
    // speed is 720 deg/s. delta = 0.1s -> max step = 72.0 deg
    rot.update(0.1);
    assert_eq!(rot.get_current_rotation(), 72.0);

    rot.update(0.1);
    // target is 90, so diff is 18.0 deg, which is less than 72.0
    assert_eq!(rot.get_current_rotation(), 90.0);
}

#[test]
fn test_smooth_rotation_shortest_path_negative() {
    let mut rot = ActorRotation::new(0.0);
    rot.set_target(270.0); // Equivalent to -90
    
    // should rotate negatively: shortest path from 0 to 270 is -90.
    // diff = (270 - 0 + 540) % 360 - 180 = (810) % 360 - 180 = 90 - 180 = -90.
    // speed is 720. delta = 0.1s -> max step = 72.0.
    rot.update(0.1);
    // step is -72.0. cur = (0 - 72 + 360) % 360 = 288.0.
    assert_eq!(rot.get_current_rotation(), 288.0);

    rot.update(0.1);
    // diff = (270 - 288 + 540) % 360 - 180 = 522 % 360 - 180 = 162 - 180 = -18
    // step is -18. cur = (288 - 18 + 360) % 360 = 270.0.
    assert_eq!(rot.get_current_rotation(), 270.0);
}

#[test]
fn test_rotation_cross_0_360_boundary() {
    let mut rot = ActorRotation::new(350.0);
    rot.set_target(10.0);
    
    // Shortest path from 350 to 10 is 20 deg positive.
    // diff = (10 - 350 + 540) % 360 - 180 = 200 % 360 - 180 = 200 - 180 = 20.
    // 20 is smaller than 72.0, so it will reach target in one step.
    rot.update(0.1);
    assert_eq!(rot.get_current_rotation(), 10.0);
}

#[test]
fn test_rotation_cross_0_360_boundary_negative() {
    let mut rot = ActorRotation::new(10.0);
    rot.set_target(350.0);
    
    // Shortest path from 10 to 350 is 20 deg negative (-20).
    // diff = (350 - 10 + 540) % 360 - 180 = 880 % 360 - 180 = 160 - 180 = -20.
    // -20 is within limit.
    rot.update(0.1);
    assert_eq!(rot.get_current_rotation(), 350.0);
}

#[test]
fn test_immediate_target_acquisition() {
    let mut rot = ActorRotation::new(45.0);
    rot.set_target(45.5);
    
    // small difference
    rot.update(0.1);
    assert_eq!(rot.get_current_rotation(), 45.5);
}

#[test]
fn test_half_circle_rotation() {
    let mut rot = ActorRotation::new(0.0);
    rot.set_target(180.0);
    
    // diff = (180 - 0 + 540) % 360 - 180 = 720 % 360 - 180 = 0 - 180 = -180 (or 180 depending on float math).
    // Let's trace it exactly. rem_euclid of 720 by 360 is 0. So diff is -180.
    // step is clamped to -72.0 (for 0.1s).
    rot.update(0.1);
    assert_eq!(rot.get_current_rotation(), 288.0);
}
