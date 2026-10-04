#[path = "../src/wgpu_shadow_depth_pass_v2.rs"]
mod wgpu_shadow_depth_pass_v2;

use glam::{Mat4, Vec3};
use wgpu_shadow_depth_pass_v2::{
    WgpuShadowDepthPassV2, CHARACTER_SHADOW_SHADER, TERRAIN_SHADOW_PCF_SHADER_SNIPPET,
};

#[test]
fn test_calculate_light_matrices() {
    let mut pass = WgpuShadowDepthPassV2::new();
    let target = Vec3::new(100.0, 200.0, 50.0);
    pass.calculate_light_matrices(target);

    // Eye calculation reference:
    // x = 100.0 - 1.732 * 1250.0 = -2065.0
    // y = 200.0 - 1250.0 = -1050.0
    // z = 50.0 + 2.0 * 1.732 * 1250.0 = 50.0 + 4330.0 = 4380.0
    
    // We can verify camera look at behavior by applying it to target and eye
    let eye_expected = Vec3::new(-2065.0, -1050.0, 4380.0);
    
    let view = pass.light_view_matrix;
    let view_eye = view.transform_point3(eye_expected);
    
    // In view space, eye should be near origin
    assert!(view_eye.length() < 0.01);
}

#[test]
fn test_light_view_proj_matrix() {
    let mut pass = WgpuShadowDepthPassV2::new();
    let target = Vec3::new(100.0, 200.0, 50.0);
    pass.calculate_light_matrices(target);

    let vp = pass.get_light_view_proj_matrix();
    let target_clip = vp.transform_point3(target);
    
    // The target is strictly in view, so its clip x and y should be near 0
    assert!(target_clip.x.abs() < 0.01);
    assert!(target_clip.y.abs() < 0.01);
    // Depth is valid orthographic projection
    assert!(target_clip.z > 0.0 && target_clip.z <= 1.0);
}

#[test]
fn test_shadow_map_size() {
    let mut pass = WgpuShadowDepthPassV2::new();
    assert_eq!(pass.shadow_map_size, 2048);
    
    pass.set_shadow_map_size(1024);
    assert_eq!(pass.shadow_map_size, 1024);
}

#[test]
fn test_pcf_shader_snippet() {
    assert!(TERRAIN_SHADOW_PCF_SHADER_SNIPPET.contains("calculate_shadow_pcf"));
    assert!(TERRAIN_SHADOW_PCF_SHADER_SNIPPET.contains("textureSampleCompareLevel"));
    assert!(TERRAIN_SHADOW_PCF_SHADER_SNIPPET.contains("shadow_coord.xy + offset"));
    assert!(TERRAIN_SHADOW_PCF_SHADER_SNIPPET.contains("/ 9.0"));
    
    assert!(CHARACTER_SHADOW_SHADER.contains("light_view_proj"));
    assert!(CHARACTER_SHADOW_SHADER.contains("@builtin(position) clip_position: vec4<f32>"));
}
