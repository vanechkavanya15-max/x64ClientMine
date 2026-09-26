#[path = "../src/effect_mesh.rs"]
mod effect_mesh;

use effect_mesh::{EffectMesh, MeshType};
use glam::{Mat4, Quat, Vec3};

#[test]
fn test_mesh_scale_interpolation() {
    let mut mesh = EffectMesh::new(
        MeshType::Ring,
        (0.0, 0.0, 1.0),
        0.0,
        1.0,
        3.0,
        1.0,
        1.0,
        2.0,
    );

    // Initial state
    assert_eq!(mesh.current_scale, 1.0);

    // Mid-point
    mesh.update(1.0);
    assert_eq!(mesh.current_scale, 2.0);

    // End-point
    mesh.update(1.0);
    assert_eq!(mesh.current_scale, 3.0);
    
    // Past end-point (clamped)
    mesh.update(1.0);
    assert_eq!(mesh.current_scale, 3.0);
}

#[test]
fn test_smooth_fading_alpha() {
    let mut mesh = EffectMesh::new(
        MeshType::Cylinder,
        (0.0, 0.0, 1.0),
        0.0,
        1.0,
        1.0,
        1.0,
        0.0,
        1.0,
    );

    assert_eq!(mesh.current_alpha, 1.0);

    mesh.update(0.5);
    assert_eq!(mesh.current_alpha, 0.5);

    mesh.update(0.5);
    assert_eq!(mesh.current_alpha, 0.0);
    
    mesh.update(0.5);
    assert_eq!(mesh.current_alpha, 0.0);
}

#[test]
fn test_rotation_angle_accumulation_z_axis() {
    let speed = std::f32::consts::PI; // PI radians per second (half circle)
    let mut mesh = EffectMesh::new(
        MeshType::Plane,
        (0.0, 0.0, 1.0),
        speed,
        1.0,
        1.0,
        1.0,
        1.0,
        10.0,
    );

    assert_eq!(mesh.current_rotation, 0.0);

    mesh.update(1.0);
    assert_eq!(mesh.current_rotation, speed);
    
    mesh.update(1.0);
    assert_eq!(mesh.current_rotation, speed * 2.0);
}

#[test]
fn test_expiration_after_reaching_duration() {
    let mut mesh = EffectMesh::new(
        MeshType::Ring,
        (0.0, 0.0, 1.0),
        0.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.5,
    );

    assert!(!mesh.is_finished());

    mesh.update(1.0);
    assert!(!mesh.is_finished());

    mesh.update(0.5);
    assert!(mesh.is_finished());
    
    mesh.update(0.5);
    assert!(mesh.is_finished());
}

#[test]
fn test_edge_cases_negative_dt_and_zero_duration() {
    // Zero duration should be clamped to a very small positive value
    let mut mesh = EffectMesh::new(
        MeshType::Ring,
        (0.0, 0.0, 1.0),
        0.0,
        1.0,
        5.0,
        1.0,
        0.0,
        0.0,
    );

    // Initial state
    assert_eq!(mesh.current_scale, 1.0);
    assert_eq!(mesh.current_alpha, 1.0);

    // Negative dt should be ignored
    mesh.update(-1.0);
    assert_eq!(mesh.current_scale, 1.0);
    assert_eq!(mesh.current_alpha, 1.0);
    assert_eq!(mesh.cur_time, 0.0);

    // Small positive dt should immediately finish the effect due to clamp
    mesh.update(0.1);
    assert_eq!(mesh.current_scale, 5.0);
    assert_eq!(mesh.current_alpha, 0.0);
    assert!(mesh.is_finished());
}

#[test]
fn test_zero_rotation_axis_defaults_to_z() {
    let mesh = EffectMesh::new(
        MeshType::Plane,
        (0.0, 0.0, 0.0), // Zero vector
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
    );

    // Should default to Z axis (0, 0, 1)
    assert_eq!(mesh.rotation_axis, Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn test_get_transform_matrix() {
    let mut mesh = EffectMesh::new(
        MeshType::Ring,
        (0.0, 0.0, 1.0), // Z-axis rotation
        std::f32::consts::PI / 2.0, // 90 degrees per second
        1.0,
        2.0,
        1.0,
        1.0,
        1.0,
    );

    // Test matrix at t = 0
    let mat_start = mesh.get_transform_matrix((10.0, 20.0, 30.0));
    
    // Scale is 1, Rotation is 0, Translation is (10, 20, 30)
    let expected_start = Mat4::from_translation(Vec3::new(10.0, 20.0, 30.0))
        * Mat4::from_quat(Quat::from_axis_angle(Vec3::Z, 0.0))
        * Mat4::from_scale(Vec3::splat(1.0));

    assert_eq!(mat_start, expected_start);

    // Test matrix at t = 1
    mesh.update(1.0);
    let mat_end = mesh.get_transform_matrix((10.0, 20.0, 30.0));
    
    // Scale is 2, Rotation is PI/2, Translation is (10, 20, 30)
    let expected_end = Mat4::from_translation(Vec3::new(10.0, 20.0, 30.0))
        * Mat4::from_quat(Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI / 2.0))
        * Mat4::from_scale(Vec3::splat(2.0));

    assert_eq!(mat_end, expected_end);
}
