use m2_client::flying_instance::{FlyingInstance, FlyingStatus, FlyingTarget};

#[test]
fn test_flying_straight() {
    let mut instance = FlyingInstance::new(
        (0.0, 0.0, 0.0),
        (10.0, 0.0, 0.0),
        Some(FlyingTarget::Position(100.0, 0.0, 0.0)),
        10.0,
        1.0,
        1.0,
        200.0,
    );

    let status = instance.update(1.0, |_| None);
    assert_eq!(status, FlyingStatus::Flying);
    assert_eq!(instance.pos, (10.0, 0.0, 0.0));
    assert_eq!(instance.velocity, (10.0, 0.0, 0.0));
    assert_eq!(instance.flown_distance, 10.0);
}

#[test]
fn test_homing_trajectory_correction() {
    let mut instance = FlyingInstance::new(
        (0.0, 0.0, 0.0),
        (10.0, 0.0, 0.0),
        Some(FlyingTarget::Position(10.0, 10.0, 0.0)),
        10.0, // speed
        1.0,  // turn_rate
        1.0,  // hit_radius
        200.0, // max_range
    );

    // Initial velocity is (10, 0, 0)
    // Target is (10, 10, 0). Dir is (10, 10, 0), normalized is (~0.707, ~0.707, 0)
    // V normalized is (1, 0, 0).
    // lerp(v, dir, turn_rate*dt)
    // t = 0.5. new_v = (1 + (0.707 - 1)*0.5, 0 + (0.707 - 0)*0.5, 0)
    // new_v = (1 - 0.1465, 0.3535, 0) = (0.8535, 0.3535, 0)
    // Normalize new_v, multiply by speed.
    // It should start curving towards y axis.

    let status = instance.update(0.5, |_| None);
    assert_eq!(status, FlyingStatus::Flying);
    
    // y velocity should increase
    assert!(instance.velocity.1 > 0.0);
    // x velocity should decrease
    assert!(instance.velocity.0 < 10.0);
    
    // Total speed should still be 10.0
    let vel_len = (instance.velocity.0 * instance.velocity.0 + instance.velocity.1 * instance.velocity.1 + instance.velocity.2 * instance.velocity.2).sqrt();
    assert!((vel_len - 10.0).abs() < 0.001);
}

#[test]
fn test_hit_detection() {
    let mut instance = FlyingInstance::new(
        (0.0, 0.0, 0.0),
        (10.0, 0.0, 0.0),
        Some(FlyingTarget::Position(5.0, 0.0, 0.0)),
        10.0,
        1.0,
        10.0, // hit_radius is large enough to hit immediately
        200.0,
    );

    let status = instance.update(0.1, |_| None);
    assert_eq!(status, FlyingStatus::Hit);
}

#[test]
fn test_max_range_reached() {
    let mut instance = FlyingInstance::new(
        (0.0, 0.0, 0.0),
        (10.0, 0.0, 0.0),
        Some(FlyingTarget::Position(200.0, 0.0, 0.0)),
        10.0,
        1.0,
        1.0,
        5.0, // small max range
    );

    let status = instance.update(1.0, |_| None);
    assert_eq!(status, FlyingStatus::MaxRangeReached);
    assert_eq!(instance.flown_distance, 10.0);
}

#[test]
fn test_zero_velocity_edge_case() {
    let mut instance = FlyingInstance::new(
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0), // zero velocity
        Some(FlyingTarget::Position(0.0, 10.0, 0.0)),
        10.0,
        1.0,
        1.0,
        200.0,
    );

    let status = instance.update(1.0, |_| None);
    assert_eq!(status, FlyingStatus::Flying);
    // Even if initial velocity was zero, it should head towards the target.
    // Dir is (0, 1, 0).
    assert!((instance.velocity.1 - 10.0).abs() < 0.001);
    assert_eq!(instance.velocity.0, 0.0);
    assert_eq!(instance.velocity.2, 0.0);
}

#[test]
fn test_homing_with_target_vid() {
    let mut instance = FlyingInstance::new(
        (0.0, 0.0, 0.0),
        (10.0, 0.0, 0.0),
        Some(FlyingTarget::Vid(42)),
        10.0,
        1.0,
        1.0,
        200.0,
    );

    // Target moves away
    let status = instance.update(1.0, |vid| {
        if vid == 42 {
            Some((100.0, 100.0, 0.0))
        } else {
            None
        }
    });

    assert_eq!(status, FlyingStatus::Flying);
    assert!(instance.velocity.1 > 0.0); // Should curve towards y
}

#[test]
fn test_missing_target_vid() {
    let mut instance = FlyingInstance::new(
        (0.0, 0.0, 0.0),
        (10.0, 0.0, 0.0),
        Some(FlyingTarget::Vid(42)),
        10.0,
        1.0,
        1.0,
        200.0,
    );

    let status = instance.update(1.0, |_| None); // Target not found
    assert_eq!(status, FlyingStatus::Flying);
    assert_eq!(instance.velocity, (10.0, 0.0, 0.0)); // Keeps going straight
    assert_eq!(instance.pos, (10.0, 0.0, 0.0));
}
