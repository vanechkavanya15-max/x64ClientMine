#[path = "../src/horse_rider_animation_sync.rs"]
mod horse_rider_animation_sync;

use horse_rider_animation_sync::{
    calculate_rider_transform, HorseBoneInfo, Quaternion, RideState, Vector3,
};

#[test]
fn test_dismounted_state_returns_base_transform() {
    let horse_bone = HorseBoneInfo {
        position: Vector3::new(10.0, 20.0, 30.0),
        rotation: Quaternion::identity(),
    };
    let base_pos = Vector3::new(5.0, 5.0, 0.0);
    let base_rot = Quaternion::new(0.0, 1.0, 0.0, 0.0);

    let result = calculate_rider_transform(&horse_bone, RideState::Dismounted, &base_pos, &base_rot);

    assert_eq!(result.position, base_pos);
    assert_eq!(result.rotation, base_rot);
}

#[test]
fn test_riding_state_calculates_correct_position_offset() {
    let horse_bone = HorseBoneInfo {
        position: Vector3::new(10.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    };
    let base_pos = Vector3::new(0.0, 0.0, 0.0);
    let base_rot = Quaternion::identity();

    let result = calculate_rider_transform(&horse_bone, RideState::Riding, &base_pos, &base_rot);

    // Default riding offset is (0, 0, 20) with identity rotation
    assert_eq!(result.position, Vector3::new(10.0, 0.0, 20.0));
    assert_eq!(result.rotation, Quaternion::identity());
}

#[test]
fn test_combat_state_calculates_correct_position_and_rotation() {
    let horse_bone = HorseBoneInfo {
        position: Vector3::new(0.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    };
    let base_pos = Vector3::new(0.0, 0.0, 0.0);
    let base_rot = Quaternion::identity();

    let result = calculate_rider_transform(&horse_bone, RideState::Combat, &base_pos, &base_rot);

    // Combat offset is (0, 10, 25)
    assert_eq!(result.position, Vector3::new(0.0, 10.0, 25.0));
    
    // Check if rotation has been modified (pitch rotation applied)
    let half_angle: f32 = (15.0 * std::f32::consts::PI / 180.0) / 2.0;
    let expected_rot = Quaternion::new(half_angle.sin(), 0.0, 0.0, half_angle.cos());
    
    // Quaternion::multiply with identity doesn't exactly preserve floating precision,
    // so we use approximate comparison.
    assert!((result.rotation.x - expected_rot.x).abs() < 1e-5);
    assert!((result.rotation.y - expected_rot.y).abs() < 1e-5);
    assert!((result.rotation.z - expected_rot.z).abs() < 1e-5);
    assert!((result.rotation.w - expected_rot.w).abs() < 1e-5);
}

#[test]
fn test_rotated_horse_spine_applies_rotation_to_offset() {
    // Rotate horse 90 degrees around Y axis
    let angle: f32 = std::f32::consts::PI / 2.0;
    let half_angle = angle / 2.0;
    let q_rot = Quaternion::new(0.0, half_angle.sin(), 0.0, half_angle.cos());

    let horse_bone = HorseBoneInfo {
        position: Vector3::new(0.0, 0.0, 0.0),
        rotation: q_rot,
    };
    let base_pos = Vector3::new(0.0, 0.0, 0.0);
    let base_rot = Quaternion::identity();

    let result = calculate_rider_transform(&horse_bone, RideState::Riding, &base_pos, &base_rot);

    // Riding offset is (0, 0, 20).
    // A 90 deg rotation around Y axis should change Z=20 to X=20 (or X=-20 depending on coordinate system right handness, wait.
    // X axis points right, Y points up, Z points forward? Let's check rotate_vector output:
    // v = (0, 0, 20)
    // u = (0, sin(45), 0), w = cos(45)
    // Actually, rotate_vector logic applies the quaternion exactly.
    let expected_rotated_offset = q_rot.rotate_vector(&Vector3::new(0.0, 0.0, 20.0));
    
    let expected_pos = horse_bone.position.add(&expected_rotated_offset);

    assert_eq!(result.position.x, expected_pos.x);
    assert_eq!(result.position.y, expected_pos.y);
    assert_eq!(result.position.z, expected_pos.z);
    
    assert_eq!(result.rotation, q_rot.multiply(&Quaternion::identity()));
}

#[test]
fn test_quaternion_multiplication() {
    let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let q2 = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    
    // Standard quaternion multiplication rules:
    // w = w1w2 - x1x2 - y1y2 - z1z2 = 4*8 - 1*5 - 2*6 - 3*7 = 32 - 5 - 12 - 21 = -6
    // x = w1x2 + x1w2 + y1z2 - z1y2 = 4*5 + 1*8 + 2*7 - 3*6 = 20 + 8 + 14 - 18 = 24
    // y = w1y2 - x1z2 + y1w2 + z1x2 = 4*6 - 1*7 + 2*8 + 3*5 = 24 - 7 + 16 + 15 = 48
    // z = w1z2 + x1y2 - y1x2 + z1w2 = 4*7 + 1*6 - 2*5 + 3*8 = 28 + 6 - 10 + 24 = 48

    let result = q1.multiply(&q2);
    
    assert_eq!(result.w, -6.0);
    assert_eq!(result.x, 24.0);
    assert_eq!(result.y, 48.0);
    assert_eq!(result.z, 48.0);
}
