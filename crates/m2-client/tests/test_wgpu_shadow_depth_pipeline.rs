use glam::{Mat4, Vec3};
use m2_client::wgpu_shadow_depth_pipeline::{ShadowDepthPipeline, ShadowUniforms};

#[test]
fn test_shadow_uniforms_defaults() {
    let uniforms = ShadowUniforms::default();
    assert_eq!(uniforms.shadow_map_size, 2048.0);
    assert_eq!(uniforms.shadow_bias, 0.005);
    assert_eq!(uniforms.pcf_samples, 9);
    assert_eq!(uniforms.split_distances, [0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_calculate_cascades() {
    let mut pipeline = ShadowDepthPipeline::new();
    
    let camera_view = Mat4::look_at_rh(
        Vec3::new(0.0, 5.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::Y,
    );
    let camera_proj = Mat4::perspective_rh_gl(f32::to_radians(45.0), 16.0 / 9.0, 0.1, 1000.0);
    let sun_direction = Vec3::new(0.5, -1.0, 0.5);
    
    let near_split = 0.1;
    let mid_split = 0.5;
    let far_split = 1.0;
    
    pipeline.calculate_cascades(
        camera_view,
        camera_proj,
        sun_direction,
        near_split,
        mid_split,
        far_split,
    );
    
    assert_eq!(pipeline.uniforms.split_distances[0], near_split);
    assert_eq!(pipeline.uniforms.split_distances[1], mid_split);
    assert_eq!(pipeline.uniforms.split_distances[2], far_split);
    
    // Ensure matrices are not identity after calculation
    let near_matrix = Mat4::from_cols_array(&pipeline.uniforms.light_view_proj_near);
    let mid_matrix = Mat4::from_cols_array(&pipeline.uniforms.light_view_proj_mid);
    let far_matrix = Mat4::from_cols_array(&pipeline.uniforms.light_view_proj_far);
    
    assert_ne!(near_matrix, Mat4::IDENTITY);
    assert_ne!(mid_matrix, Mat4::IDENTITY);
    assert_ne!(far_matrix, Mat4::IDENTITY);
}
