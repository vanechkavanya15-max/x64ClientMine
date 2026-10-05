use m2_client::wgpu_water_distortion_pipeline::{WaterUniforms, WgpuWaterDistortionPipeline};

fn create_headless_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        force_fallback_adapter: false,
        compatible_surface: None,
        apply_limit_buckets: false,
    }))
    .expect("Failed to find an appropriate adapter");

    pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("Test Device"),
            ..Default::default()
        }
    ))
    .expect("Failed to create device")
}

#[test]
fn test_water_uniforms_layout() {
    let size = std::mem::size_of::<WaterUniforms>();
    // 3 * 4 + 4 (pad) + 16 (color) + 16 (sun) = 48 bytes
    assert_eq!(size, 48, "WaterUniforms should be exactly 48 bytes");
    
    let align = std::mem::align_of::<WaterUniforms>();
    assert_eq!(align, 16, "WaterUniforms should be 16-byte aligned because of float arrays");

    let uniforms = WaterUniforms::default();
    let bytes: &[u8] = bytemuck::cast_slice(std::slice::from_ref(&uniforms));
    assert_eq!(bytes.len(), 48, "Cast slice must be 48 bytes");
}

#[test]
fn test_pipeline_creation() {
    let (device, _queue) = create_headless_device();
    // Use an arbitrary surface format
    let surface_format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let pipeline = WgpuWaterDistortionPipeline::new(&device, surface_format);

    // Validate the pipeline components are valid handles
    assert!(pipeline.uniform_buffer.size() >= std::mem::size_of::<WaterUniforms>() as u64);
}

#[test]
fn test_uniform_updates() {
    let (device, queue) = create_headless_device();
    let surface_format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let pipeline = WgpuWaterDistortionPipeline::new(&device, surface_format);

    let mut uniforms = WaterUniforms::default();
    uniforms.u_time = 10.5;
    uniforms.u_wave_strength = 0.5;
    
    // Test that the update method completes without panics
    pipeline.update_uniforms(&queue, &uniforms);
    
    // Ensure all commands finish (not strictly necessary but good practice)
    queue.submit(None);
    device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None }).unwrap();
}

#[test]
fn test_bind_group_creation() {
    let (device, _queue) = create_headless_device();
    let surface_format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let pipeline = WgpuWaterDistortionPipeline::new(&device, surface_format);

    // Create a dummy 1x1 texture for DUDV
    let dudv_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("DUDV Texture"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let dudv_view = dudv_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let dudv_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("DUDV Sampler"),
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::MipmapFilterMode::Linear,
        ..Default::default()
    });

    // Create a dummy 1x1 texture for Normal map
    let normal_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Normal Texture"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let normal_view = normal_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let normal_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Normal Sampler"),
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::MipmapFilterMode::Linear,
        ..Default::default()
    });

    // Create the bind group
    let _bind_group = pipeline.create_bind_group(
        &device,
        &dudv_view,
        &dudv_sampler,
        &normal_view,
        &normal_sampler,
    );
    
    // Test passes if create_bind_group doesn't panic on validation.
    device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None }).unwrap();
}

