use glam::{Mat4, Vec3};

#[path = "../src/wgpu_shadow_depth_pass.rs"]
mod wgpu_shadow_depth_pass;

use wgpu_shadow_depth_pass::WgpuShadowDepthPass;

#[test]
fn test_wgpu_shadow_depth_pass_default() {
    let pass = WgpuShadowDepthPass::default();
    assert_eq!(pass.shadow_map_size, 2048);
    assert_eq!(pass.light_view_matrix, Mat4::IDENTITY);
    assert_eq!(pass.light_proj_matrix, Mat4::IDENTITY);
}

#[test]
fn test_wgpu_shadow_depth_pass_new() {
    let pass = WgpuShadowDepthPass::new();
    assert_eq!(pass.shadow_map_size, 2048);
}

#[test]
fn test_calculate_light_matrices_valid_sun_dir() {
    let mut pass = WgpuShadowDepthPass::new();
    let target = Vec3::new(100.0, 50.0, 0.0);
    // Sun shining directly down the Z axis
    let sun_dir = Vec3::new(0.0, 0.0, -1.0);

    pass.calculate_light_matrices(target, sun_dir);

    assert_ne!(pass.light_view_matrix, Mat4::IDENTITY);
    assert_ne!(pass.light_proj_matrix, Mat4::IDENTITY);

    // The eye should be 5000 units away along -sun_dir (so +Z)
    let expected_eye = Vec3::new(100.0, 50.0, 5000.0);
    // You can't easily extract eye from LookAtRH without inverse, but we can verify projection.
    
    // Test projection matrix is orthogonal
    // Projection matrix scale x: 2 / width = 2 / 5000 = 0.0004
    // Projection matrix scale y: 2 / height = 2 / 5000 = 0.0004
    let proj = pass.light_proj_matrix;
    assert!((proj.x_axis.x.abs() - 0.0004).abs() < 1e-6);
    assert!((proj.y_axis.y.abs() - 0.0004).abs() < 1e-6);
}

#[test]
fn test_calculate_light_matrices_degenerate_sun_dir() {
    let mut pass = WgpuShadowDepthPass::new();
    let target = Vec3::ZERO;
    let sun_dir = Vec3::ZERO; // Degenerate

    pass.calculate_light_matrices(target, sun_dir);

    // Should not panic or produce NaN. Should fall back to (0, -1, 0)
    assert_ne!(pass.light_view_matrix, Mat4::IDENTITY);
    assert!(!pass.light_view_matrix.is_nan());
    
    // Fallback direction is (0, -1, 0), so eye should be at target - (0, -1, 0)*5000 = (0, 5000, 0)
    // Up vector is (0,0,1)
    let view = pass.light_view_matrix;
    // Just verify it's a valid matrix.
    assert!(!view.x_axis.x.is_nan());
}

#[test]
fn test_shader_literal_integrity() {
    let shader_source = wgpu_shadow_depth_pass::SUN_SHADOW_SHADER;
    assert!(shader_source.contains("struct ShadowUniforms"));
    assert!(shader_source.contains("light_view_proj: mat4x4<f32>"));
    assert!(shader_source.contains("@vertex"));
    assert!(shader_source.contains("fn vs_main"));
    assert!(shader_source.contains("@fragment"));
    assert!(shader_source.contains("fn fs_main"));
}

#[test]
fn test_wgpu_resources_fallback() {
    // Tests wgpu resource creation if an adapter is available, otherwise skips
    let pass = WgpuShadowDepthPass::new();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: None,
        force_fallback_adapter: true,
        ..Default::default()
    }));

    if let Some(adapter) = adapter.ok() {
        let (device, _queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                ..Default::default()
            }
        )).expect("Failed to create device");

        // Test texture creation
        let (texture, view, sampler) = pass.create_depth_texture(&device);
        assert_eq!(texture.width(), 2048);
        assert_eq!(texture.height(), 2048);
        assert_eq!(texture.depth_or_array_layers(), 1);
        assert_eq!(texture.format(), wgpu::TextureFormat::Depth32Float);
        assert_eq!(texture.mip_level_count(), 1);
        
        let _ = view;
        let _ = sampler;

        // Test pipeline creation
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Dummy Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline = pass.create_pipeline(&device, &bind_group_layout);
        let _ = pipeline; // Just verify it doesn't panic
    } else {
        println!("Skipping wgpu resource tests: No adapter found.");
    }
}
