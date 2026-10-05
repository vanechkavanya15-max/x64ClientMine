use glam::Vec3;
use m2_client::wgpu_shadow_pcf_pipeline::{WgpuShadowPcfPipeline, SHADOW_MAP_SIZE, SHADOW_PCF_WGSL};

#[test]
fn test_wgpu_shadow_pcf_initialization() {
    let pipeline = WgpuShadowPcfPipeline::new();
    assert_eq!(pipeline.shadow_map_size, 2048);
    assert_eq!(pipeline.shadow_map_size, SHADOW_MAP_SIZE);
    
    // Check cascade defaults
    let splits = pipeline.uniforms.cascade_splits;
    assert_eq!(splits[0], 2500.0);
    assert_eq!(splits[1], 15000.0);
    assert_eq!(splits[2], 500.0);
}

#[test]
fn test_sun_light_matrix_calculation() {
    let target = Vec3::new(1000.0, 500.0, 200.0);
    let sun_dir = Vec3::new(1.0, -1.0, -1.0).normalize();
    
    let matrix = WgpuShadowPcfPipeline::calculate_sun_light_matrix(target, sun_dir, 1250.0);
    
    // Extract a test point to ensure projection works properly
    let test_pos = target; 
    let proj = matrix * test_pos.extend(1.0);
    
    // Near target center, X/Y should be close to 0 in light space
    let w = proj.w;
    let ndc_x = proj.x / w;
    let ndc_y = proj.y / w;
    
    assert!((ndc_x).abs() < 1e-4);
    assert!((ndc_y).abs() < 1e-4);
}

#[test]
fn test_wgpu_shadow_pcf_texture_creation() {
    // Basic headless test device initialization without real display setup
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    
    // Use block_on to run async wgpu init sequentially
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()));
    
    if let Ok(adapter) = adapter {
        let (device, _) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();
        
        let pipeline = WgpuShadowPcfPipeline::new();
        let (tex_near, _, tex_far, _, sampler) = pipeline.create_depth_textures(&device);
        
        assert_eq!(tex_near.width(), 2048);
        assert_eq!(tex_near.height(), 2048);
        assert_eq!(tex_near.format(), wgpu::TextureFormat::Depth32Float);
        
        assert_eq!(tex_far.width(), 2048);
        assert_eq!(tex_far.height(), 2048);
        assert_eq!(tex_far.format(), wgpu::TextureFormat::Depth32Float);
    }
}

#[test]
fn test_wgpu_shadow_pcf_shader_compilation() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()));
    
    if let Ok(adapter) = adapter {
        let (device, _) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();
        
        // This will panic if the WGSL has syntax errors
        let _shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shadow PCF Shader Test"),
            source: wgpu::ShaderSource::Wgsl(SHADOW_PCF_WGSL.into()),
        });
    }
}
