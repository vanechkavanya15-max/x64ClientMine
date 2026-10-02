use glam::{Mat4, Vec3};
use m2_client::wgpu_shadow_mapping::*;

#[test]
fn test_shadow_config_default() {
    let config = ShadowConfig::default();
    assert_eq!(config.texture_size, 2048);
    assert_eq!(config.format, wgpu::TextureFormat::Depth32Float);
}

#[test]
fn test_shadow_uniforms_layout() {
    assert_eq!(std::mem::size_of::<ShadowUniforms>(), 96);
    assert_eq!(std::mem::align_of::<ShadowUniforms>(), 4);
    
    let uniforms = ShadowUniforms {
        light_view_proj: [0.0; 16],
        light_dir: [1.0, -1.0, 1.0, 0.0],
        bias: 0.005,
        pcf_enabled: 1,
        _padding: [0; 2],
    };
    
    let bytes = bytemuck::bytes_of(&uniforms);
    assert_eq!(bytes.len(), 96);
}

#[test]
fn test_shadow_light_vp_matrix() {
    let view = Mat4::look_at_rh(Vec3::new(10.0, 10.0, 10.0), Vec3::ZERO, Vec3::Y);
    let proj = Mat4::orthographic_rh(-50.0, 50.0, -50.0, 50.0, 0.1, 100.0);
    let vp = proj * view;
    
    // Convert to row-major or column-major float array as expected by the uniform struct
    let uniforms = ShadowUniforms {
        light_view_proj: vp.to_cols_array(),
        light_dir: [1.0, -1.0, 1.0, 0.0],
        bias: 0.002,
        pcf_enabled: 1,
        _padding: [0; 2],
    };
    
    assert_eq!(uniforms.light_view_proj.len(), 16);
    assert_eq!(uniforms.bias, 0.002);
}

#[test]
fn test_shadow_depth_texture_struct_fields() {
    // A dummy test just to enforce that the structs compile and contain expected fields.
    // Full wgpu texture allocation isn't trivial without an instance, but we can verify our types.
    let _conf = ShadowConfig {
        texture_size: 1024,
        format: wgpu::TextureFormat::Depth32Float,
    };
    
    // Test calculate logic
    let light_dir = Vec3::new(-1.0, -1.0, -1.0);
    let center = Vec3::ZERO;
    let radius = 50.0;
    
    let light_vp = calculate_light_view_proj(light_dir, center, radius);
    let uniforms = ShadowUniforms {
        light_view_proj: light_vp.to_cols_array(),
        light_dir: [light_dir.x, light_dir.y, light_dir.z, 0.0],
        bias: 0.005,
        pcf_enabled: 1,
        _padding: [0; 2],
    };
    
    assert!(uniforms.light_dir[0] < 0.0);
    assert_eq!(uniforms.pcf_enabled, 1);
}
#[test]
fn test_shadow_light_vp_normalization() {
    let light_dir = Vec3::new(10.0, 10.0, 10.0);
    let center = Vec3::ZERO;
    let radius = 100.0;
    
    // Test that the method handles unnormalized directions automatically
    let mat = calculate_light_view_proj(light_dir, center, radius);
    
    // Make sure matrix has reasonable values, not NaNs
    for col in mat.to_cols_array_2d() {
        for val in col {
            assert!(!val.is_nan());
            assert!(!val.is_infinite());
        }
    }
}
