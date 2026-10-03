#[path = "../src/wgpu_shadow_map_pipeline.rs"]
mod wgpu_shadow_map_pipeline;

use wgpu_shadow_map_pipeline::{WgpuShadowMapPipeline, CHARACTER_SHADOW_SHADER};
use glam::{Vec3, Mat4};

#[test]
fn test_shadow_map_pipeline_initialization() {
    let pipeline = WgpuShadowMapPipeline::new();
    assert_eq!(pipeline.shadow_map_size, 512);
    assert_eq!(pipeline.recreate.load(std::sync::atomic::Ordering::SeqCst), false);
    assert_eq!(pipeline.light_view_matrix, Mat4::IDENTITY);
    assert_eq!(pipeline.light_proj_matrix, Mat4::IDENTITY);
}

#[test]
fn test_shadow_map_pipeline_set_size_triggers_recreate() {
    let mut pipeline = WgpuShadowMapPipeline::new();
    
    // Setting to same size should not trigger recreate
    pipeline.set_shadow_texture_size(512);
    assert_eq!(pipeline.recreate.load(std::sync::atomic::Ordering::SeqCst), false);
    
    // Setting to different size should trigger recreate
    pipeline.set_shadow_texture_size(1024);
    assert_eq!(pipeline.recreate.load(std::sync::atomic::Ordering::SeqCst), true);
    assert_eq!(pipeline.shadow_map_size, 1024);
}

#[test]
fn test_calculate_light_matrices_math() {
    let mut pipeline = WgpuShadowMapPipeline::new();
    let target = Vec3::new(1000.0, 2000.0, 3000.0);
    
    pipeline.calculate_light_matrices(target);
    
    let expected_eye = Vec3::new(
        1000.0 - 1.732 * 1250.0,
        2000.0 - 1250.0,
        3000.0 + 2.0 * 1.732 * 1250.0,
    );
    let expected_up = Vec3::new(0.0, 0.0, 1.0);
    
    let expected_view = Mat4::look_at_rh(expected_eye, target, expected_up);
    let expected_proj = Mat4::orthographic_rh(
        -1275.0, 1275.0,
        -1275.0, 1275.0,
        1.0, 15000.0,
    );
    
    assert_eq!(pipeline.light_view_matrix, expected_view);
    assert_eq!(pipeline.light_proj_matrix, expected_proj);
}

#[test]
fn test_shader_presence() {
    assert!(CHARACTER_SHADOW_SHADER.contains("sampler_comparison"));
    assert!(CHARACTER_SHADOW_SHADER.contains("texture_depth_2d"));
    assert!(CHARACTER_SHADOW_SHADER.contains("mat4x4<f32>"));
}

#[test]
fn test_struct_layout_and_defaults() {
    let pipeline = WgpuShadowMapPipeline::default();
    assert_eq!(pipeline.shadow_map_size, 512);
}
