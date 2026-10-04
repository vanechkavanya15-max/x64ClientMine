use m2_client::actor_splash_aoe_attacks::SSplashArea;
use glam::Vec3;

#[test]
fn test_splash_hit_inside_cone() {
    let mut area = SSplashArea::new();
    
    // Setup area with 10.0 radius, 90 degree cone, facing 0.0 rotation (points to -Y in our math), 5.0 seconds duration
    // The rotation 0.0 -> -sin(0)=0, cos(0)=1 -> +Y axis.
    // Wait, in our implementation: `Vec3::new(-rot_rad.sin(), rot_rad.cos(), 0.0)`
    // If rotation is 0, forward is (0, 1, 0).
    area.setup(
        Vec3::new(0.0, 0.0, 0.0), // Center
        10.0,                     // Radius
        90.0,                     // Cone angle
        0.0,                      // Rotation (0 deg)
        5.0,                      // Duration
        3,                        // Hit limit
        0.0,                      // Current time
    );

    // Target inside cone, +Y direction, distance 5.0
    let target_pos = Vec3::new(0.0, 5.0, 0.0);
    
    // Check hit at time 1.0 (inside duration)
    let is_hit = area.check_hit(1, target_pos, 1.0);
    assert!(is_hit, "Target should be hit inside the cone.");
}

#[test]
fn test_splash_hit_outside_cone_and_radius() {
    let mut area = SSplashArea::new();
    
    area.setup(
        Vec3::new(0.0, 0.0, 0.0), // Center
        10.0,                     // Radius
        90.0,                     // Cone angle
        0.0,                      // Rotation (0 deg) -> +Y direction
        5.0,                      // Duration
        3,                        // Hit limit
        0.0,                      // Current time
    );

    // Target outside cone, -Y direction, distance 5.0
    let target_pos_behind = Vec3::new(0.0, -5.0, 0.0);
    let is_hit_behind = area.check_hit(2, target_pos_behind, 1.0);
    assert!(!is_hit_behind, "Target behind the cone should not be hit.");

    // Target outside radius, +Y direction, distance 15.0
    let target_pos_far = Vec3::new(0.0, 15.0, 0.0);
    let is_hit_far = area.check_hit(3, target_pos_far, 1.0);
    assert!(!is_hit_far, "Target outside radius should not be hit.");
}

#[test]
fn test_splash_hit_limit() {
    let mut area = SSplashArea::new();
    
    area.setup(
        Vec3::new(0.0, 0.0, 0.0),
        10.0,
        90.0,
        0.0,
        5.0,
        2, // Hit limit of 2
        0.0,
    );

    let target_pos = Vec3::new(0.0, 5.0, 0.0);

    assert!(area.check_hit(1, target_pos, 1.0), "First target should be hit.");
    assert!(area.check_hit(2, target_pos, 1.0), "Second target should be hit.");
    assert!(!area.check_hit(3, target_pos, 1.0), "Third target should NOT be hit due to limit.");
}

#[test]
fn test_splash_duration_expired() {
    let mut area = SSplashArea::new();
    
    area.setup(
        Vec3::new(0.0, 0.0, 0.0),
        10.0,
        90.0,
        0.0,
        5.0, // Duration 5.0
        3,
        0.0, // Current time 0.0 -> Disappearing time 5.0
    );

    let target_pos = Vec3::new(0.0, 5.0, 0.0);

    // Check hit at time 6.0 (outside duration)
    let is_hit = area.check_hit(1, target_pos, 6.0);
    assert!(!is_hit, "Target should not be hit when duration expired.");
}

#[test]
fn test_splash_already_hit() {
    let mut area = SSplashArea::new();
    
    area.setup(
        Vec3::new(0.0, 0.0, 0.0),
        10.0,
        90.0,
        0.0,
        5.0,
        3,
        0.0,
    );

    let target_pos = Vec3::new(0.0, 5.0, 0.0);

    let hit1 = area.check_hit(1, target_pos, 1.0);
    assert!(hit1, "Target should be hit the first time.");
    
    let hit2 = area.check_hit(1, target_pos, 1.1); // Same VID
    assert!(!hit2, "Target should not be hit again since it was already hit.");
}
