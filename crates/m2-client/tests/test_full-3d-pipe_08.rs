#[path = "../src/wgpu_skybox_cycle.rs"]
pub mod wgpu_skybox_cycle;

use wgpu_skybox_cycle::{
    calculate_skybox_cycle_params, update_camera_for_skybox,
    SkyboxCycleUniforms, SkyboxCycleRenderer,
};
use glam::{Mat4, Vec4, Quat};

#[test]
fn test_calculate_skybox_cycle_params_midnight() {
    let (blend, _) = calculate_skybox_cycle_params(0.0, 1.0);
    assert_eq!(blend, 0.0, "Midnight should have a blend factor of 0.0 (night only)");
}

#[test]
fn test_calculate_skybox_cycle_params_noon() {
    let (blend, _) = calculate_skybox_cycle_params(0.5, 1.0);
    assert_eq!(blend, 1.0, "Noon should have a blend factor of 1.0 (day only)");
}

#[test]
fn test_calculate_skybox_cycle_params_rotation() {
    let (_, rotation) = calculate_skybox_cycle_params(0.25, 1.0);
    let expected = Mat4::from_rotation_y(0.25 * std::f32::consts::PI * 2.0);
    
    // Check elements
    for i in 0..4 {
        for j in 0..4 {
            assert!((rotation.col(i)[j] - expected.col(i)[j]).abs() < 1e-5, "Rotation matrix mismatch");
        }
    }
}

#[test]
fn test_update_camera_for_skybox() {
    // A view matrix with translation
    let mut view = Mat4::IDENTITY;
    view.w_axis = Vec4::new(10.0, 20.0, 30.0, 1.0);
    
    let proj = Mat4::perspective_rh(1.0, 1.0, 0.1, 100.0);
    
    let view_proj = update_camera_for_skybox(view, proj);
    
    // The resulting matrix shouldn't be affected by the view translation.
    // If it was, the bottom row of the view-proj matrix would be very different.
    let expected_view = Mat4::IDENTITY; // Because we strip w_axis
    let expected_vp = proj * expected_view;
    
    for i in 0..4 {
        for j in 0..4 {
            assert!((view_proj.col(i)[j] - expected_vp.col(i)[j]).abs() < 1e-5, "Camera update failed to strip translation");
        }
    }
}

#[test]
fn test_skybox_cycle_uniforms_default() {
    let uniforms = SkyboxCycleUniforms::default();
    assert_eq!(uniforms.blend_factor, [0.0; 4]);
    
    let identity = Mat4::IDENTITY.to_cols_array_2d();
    assert_eq!(uniforms.view_proj, identity);
    assert_eq!(uniforms.rotation, identity);
}
