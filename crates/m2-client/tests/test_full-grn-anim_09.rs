#[path = "../src/polymorph_scale_controller.rs"]
mod polymorph_scale_controller;

use glam::{Mat4, Vec3};
use polymorph_scale_controller::PolymorphScaleController;

#[test]
fn test_identity_scale_does_not_modify_matrix_or_radius() {
    let controller = PolymorphScaleController::new();
    assert_eq!(controller.get_current_scale(), 1.0);

    let mut matrix = Mat4::from_translation(Vec3::new(10.0, 20.0, 30.0));
    let original_matrix = matrix;

    controller.apply_scale_to_matrix(&mut matrix);
    assert_eq!(matrix, original_matrix);

    let radius = controller.get_scaled_collision_radius(50.0);
    assert_eq!(radius, 50.0);
}

#[test]
fn test_scaling_up_modifies_matrix_basis_and_radius() {
    let mut controller = PolymorphScaleController::new();
    controller.set_scale_instant(2.0);
    assert_eq!(controller.get_current_scale(), 2.0);

    let mut matrix = Mat4::from_translation(Vec3::new(10.0, 20.0, 30.0));
    controller.apply_scale_to_matrix(&mut matrix);

    // Translation should remain unaffected
    let translation = matrix.w_axis;
    assert_eq!(translation.x, 10.0);
    assert_eq!(translation.y, 20.0);
    assert_eq!(translation.z, 30.0);
    assert_eq!(translation.w, 1.0);

    // Basis vectors (x, y, z axes) should be scaled by 2.0
    assert_eq!(matrix.x_axis.x, 2.0);
    assert_eq!(matrix.y_axis.y, 2.0);
    assert_eq!(matrix.z_axis.z, 2.0);

    let radius = controller.get_scaled_collision_radius(50.0);
    assert_eq!(radius, 100.0);
}

#[test]
fn test_scaling_down_modifies_matrix_basis_and_radius() {
    let mut controller = PolymorphScaleController::new();
    controller.set_scale_instant(0.5);
    assert_eq!(controller.get_current_scale(), 0.5);

    let mut matrix = Mat4::from_translation(Vec3::new(10.0, 20.0, 30.0));
    controller.apply_scale_to_matrix(&mut matrix);

    let translation = matrix.w_axis;
    assert_eq!(translation.x, 10.0);

    assert_eq!(matrix.x_axis.x, 0.5);
    assert_eq!(matrix.y_axis.y, 0.5);
    assert_eq!(matrix.z_axis.z, 0.5);

    let radius = controller.get_scaled_collision_radius(50.0);
    assert_eq!(radius, 25.0);
}

#[test]
fn test_scale_clamping_prevents_zero_or_negative_scale() {
    let mut controller = PolymorphScaleController::new();
    
    controller.set_scale_instant(0.0);
    assert_eq!(controller.get_current_scale(), 0.01);
    
    controller.set_scale_instant(-5.0);
    assert_eq!(controller.get_current_scale(), 0.01);
}

#[test]
fn test_scale_interpolation_over_time() {
    let mut controller = PolymorphScaleController::new();
    // Starts at 1.0. Target 2.0, speed 0.5 per second.
    controller.set_target_scale(2.0, 0.5);
    
    // 1 second elapsed -> scale should increase by 0.5 -> 1.5
    let in_progress = controller.update(1.0);
    assert!(in_progress);
    assert!((controller.get_current_scale() - 1.5).abs() < f32::EPSILON);
    
    // 2 more seconds elapsed -> scale should reach target 2.0 (and clamp)
    let in_progress = controller.update(2.0);
    assert!(!in_progress);
    assert!((controller.get_current_scale() - 2.0).abs() < f32::EPSILON);

    // Scaling down
    controller.set_target_scale(0.5, 0.5);
    let in_progress = controller.update(1.0);
    assert!(in_progress);
    assert!((controller.get_current_scale() - 1.5).abs() < f32::EPSILON);
}
