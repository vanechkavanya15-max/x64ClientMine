use m2_client::motion_slerp_blend_v2::*;
use glam::{Vec3, Quat};
use std::f32::consts::PI;

#[test]
fn test_default_blend_time() {
    assert_eq!(DEFAULT_BLEND_TIME, 0.15);
}

#[test]
fn test_ffi_m2_motion_blend_poses_valid() {
    assert!(m2_motion_blend_poses(0.0));
    assert!(m2_motion_blend_poses(0.5));
    assert!(m2_motion_blend_poses(1.0));
}

#[test]
fn test_ffi_m2_motion_blend_poses_invalid_nan_inf() {
    assert!(!m2_motion_blend_poses(f32::NAN));
    assert!(!m2_motion_blend_poses(f32::INFINITY));
    assert!(!m2_motion_blend_poses(f32::NEG_INFINITY));
}

#[test]
fn test_lerp_position() {
    let pos_a = Vec3::new(0.0, 0.0, 0.0);
    let pos_b = Vec3::new(10.0, 20.0, 30.0);
    
    let mid = lerp_position(pos_a, pos_b, 0.5);
    assert_eq!(mid, Vec3::new(5.0, 10.0, 15.0));
    
    let end = lerp_position(pos_a, pos_b, 1.0);
    assert_eq!(end, pos_b);
}

#[test]
fn test_slerp_rotation_shortest_path_negative_dot() {
    // Two quaternions that represent the same rotation but are opposite on the hypersphere.
    let q1 = Quat::from_xyzw(0.0, 1.0, 0.0, 0.0);
    let q2 = Quat::from_xyzw(0.0, -1.0, 0.0, 0.0);
    
    assert!(q1.dot(q2) < 0.0);
    
    // SLERP should flip one to take the shortest path.
    // If it flips q2 to -q2, then it's interpolating between q1 and q1.
    // The result should be q1 (or very close to it).
    let mid = slerp_rotation(q1, q2, 0.5);
    assert!((mid.x - q1.x).abs() < 1e-5);
    assert!((mid.y - q1.y).abs() < 1e-5);
    assert!((mid.z - q1.z).abs() < 1e-5);
    assert!((mid.w - q1.w).abs() < 1e-5);
}

#[test]
fn test_slerp_rotation_shortest_path_positive_dot() {
    let q1 = Quat::from_axis_angle(Vec3::Y, 0.0);
    let q2 = Quat::from_axis_angle(Vec3::Y, PI / 2.0); // 90 degrees
    
    assert!(q1.dot(q2) >= 0.0);
    
    let mid = slerp_rotation(q1, q2, 0.5);
    let expected = Quat::from_axis_angle(Vec3::Y, PI / 4.0); // 45 degrees
    
    assert!((mid.x - expected.x).abs() < 1e-5);
    assert!((mid.y - expected.y).abs() < 1e-5);
    assert!((mid.z - expected.z).abs() < 1e-5);
    assert!((mid.w - expected.w).abs() < 1e-5);
}

#[test]
fn test_blend_poses_alpha_clamping() {
    let p_a = Pose::new(Vec3::ZERO, Quat::IDENTITY);
    let p_b = Pose::new(Vec3::new(10.0, 0.0, 0.0), Quat::IDENTITY);
    
    // Testing alpha < 0.0 clamping
    let res_below = blend_poses(&p_a, &p_b, -1.0);
    assert_eq!(res_below.position, Vec3::ZERO);
    
    // Testing alpha > 1.0 clamping
    let res_above = blend_poses(&p_a, &p_b, 2.0);
    assert_eq!(res_above.position, Vec3::new(10.0, 0.0, 0.0));
}

#[test]
fn test_blend_poses_full_struct() {
    let p_a = Pose::new(
        Vec3::new(10.0, 10.0, 10.0), 
        Quat::from_axis_angle(Vec3::Z, 0.0)
    );
    let p_b = Pose::new(
        Vec3::new(20.0, 20.0, 20.0), 
        Quat::from_axis_angle(Vec3::Z, PI / 2.0)
    );
    
    let mid = blend_poses(&p_a, &p_b, 0.5);
    assert_eq!(mid.position, Vec3::new(15.0, 15.0, 15.0));
    
    let expected_rot = Quat::from_axis_angle(Vec3::Z, PI / 4.0);
    assert!((mid.rotation.x - expected_rot.x).abs() < 1e-5);
    assert!((mid.rotation.y - expected_rot.y).abs() < 1e-5);
    assert!((mid.rotation.z - expected_rot.z).abs() < 1e-5);
    assert!((mid.rotation.w - expected_rot.w).abs() < 1e-5);
}
