use m2_client::wgpu_terrain_splatting_array::{TerrainSplattingArrayRenderer, TerrainUniformsArray};
use wgpu::util::DeviceExt;

async fn create_headless_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: None,
            ..Default::default()
        })
        .await
        .unwrap();
    adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .unwrap()
}

#[tokio::test]
async fn test_shader_compilation() {
    let (device, _queue) = create_headless_device().await;
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let renderer = TerrainSplattingArrayRenderer::new(&device, format);
    
    // Test if pipeline layout and WGSL compilation succeeds and creates an instance
    assert!(renderer.bind_group.is_none());
}

#[tokio::test]
async fn test_bind_group() {
    let (device, _queue) = create_headless_device().await;
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let mut renderer = TerrainSplattingArrayRenderer::new(&device, format);

    // Create dummy texture array and views
    let tex_array = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("dummy tex array"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 4 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    
    let tex_array_view = tex_array.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    });
    
    let splat_alpha = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("dummy splat alpha"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let splat_alpha_view = splat_alpha.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

    // Should load without errors
    renderer.load_bind_group(&device, &splat_alpha_view, &sampler, &tex_array_view, &sampler);
    
    assert!(renderer.bind_group.is_some());
}

#[tokio::test]
async fn test_uniform_update() {
    let (device, queue) = create_headless_device().await;
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let renderer = TerrainSplattingArrayRenderer::new(&device, format);

    let uniforms = TerrainUniformsArray {
        view_proj: [[1.0; 4]; 4],
        sun_dir: [0.0, -1.0, 0.0, 1.0],
        sun_color: [1.0, 1.0, 1.0, 1.0],
        ambient_color: [0.2, 0.2, 0.2, 1.0],
        fog_color: [0.5, 0.5, 0.5, 1.0],
        fog_params: [100.0, 500.0, 0.0, 0.0],
        camera_pos: [0.0, 0.0, 0.0, 1.0],
    };

    // Assuming this writes to the uniform buffer correctly without panicking
    renderer.update_uniforms(&queue, &uniforms);
}

#[tokio::test]
async fn test_draw_call() {
    let (device, queue) = create_headless_device().await;
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let mut renderer = TerrainSplattingArrayRenderer::new(&device, format);

    let tex_array = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("dummy tex array"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 4 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    
    let tex_array_view = tex_array.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    });
    
    let splat_alpha = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("dummy splat alpha"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let splat_alpha_view = splat_alpha.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

    renderer.load_bind_group(&device, &splat_alpha_view, &sampler, &tex_array_view, &sampler);

    // Dummy buffers
    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Dummy Vertex"),
        size: 256,
        usage: wgpu::BufferUsages::VERTEX,
        mapped_at_creation: false,
    });
    let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Dummy Index"),
        size: 256,
        usage: wgpu::BufferUsages::INDEX,
        mapped_at_creation: false,
    });

    let target_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("dummy target"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let target_view = target_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("dummy depth target"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());


    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("dummy pass"),
            multiview_mask: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        assert!(renderer.draw(&mut render_pass, &vertex_buffer, &index_buffer, 0).is_ok());
    }
}
