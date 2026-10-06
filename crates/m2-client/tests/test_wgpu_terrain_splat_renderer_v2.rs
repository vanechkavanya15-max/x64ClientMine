#[path = "../src/wgpu_terrain_splat_renderer_v2.rs"]
mod wgpu_terrain_splat_renderer_v2;

use m2_client::wgpu_engine_context::WgpuEngineContext;
use m2_client::wgpu_terrain_renderer::TerrainVertex;
use wgpu_terrain_splat_renderer_v2::{TerrainSplattingRendererV2, TerrainUniforms};

fn create_dummy_texture(device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32, format: wgpu::TextureFormat) -> wgpu::TextureView {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Dummy Texture"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let block_size = format.block_copy_size(None).unwrap();
    let data_len = width * height * block_size;
    let data = vec![255u8; data_len as usize];

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(width * block_size),
            rows_per_image: Some(height),
        },
        wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
    );

    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

#[test]
fn test_terrain_splatting_renderer_creation() {
    let ctx = WgpuEngineContext::new(0, 800, 600).unwrap();
    let format = wgpu::TextureFormat::Bgra8UnormSrgb;
    let renderer = TerrainSplattingRendererV2::new(&ctx.device, format);
    assert!(renderer.is_ok(), "Renderer should be created successfully");
}

#[test]
fn test_terrain_splatting_uniform_update() {
    let ctx = WgpuEngineContext::new(0, 800, 600).unwrap();
    let format = wgpu::TextureFormat::Bgra8UnormSrgb;
    let renderer = TerrainSplattingRendererV2::new(&ctx.device, format).unwrap();

    let uniforms = TerrainUniforms {
        view_proj: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
        sun_dir: [0.0, -1.0, 0.0, 0.0],
        ambient_color: [0.2, 0.2, 0.2, 1.0],
        sun_color: [0.8, 0.8, 0.8, 1.0],
    };

    renderer.update_uniforms(&ctx.queue, uniforms);
}

#[test]
fn test_terrain_splatting_bind_group_loading() {
    let ctx = WgpuEngineContext::new(0, 800, 600).unwrap();
    let format = wgpu::TextureFormat::Bgra8UnormSrgb;
    let mut renderer = TerrainSplattingRendererV2::new(&ctx.device, format).unwrap();

    let splat_alpha_view = create_dummy_texture(&ctx.device, &ctx.queue, 128, 128, wgpu::TextureFormat::Rgba8Unorm);
    let shadow_map_view = create_dummy_texture(&ctx.device, &ctx.queue, 128, 128, wgpu::TextureFormat::Rgba8Unorm);
    
    let tex_views = [
        create_dummy_texture(&ctx.device, &ctx.queue, 128, 128, wgpu::TextureFormat::Rgba8UnormSrgb),
        create_dummy_texture(&ctx.device, &ctx.queue, 128, 128, wgpu::TextureFormat::Rgba8UnormSrgb),
        create_dummy_texture(&ctx.device, &ctx.queue, 128, 128, wgpu::TextureFormat::Rgba8UnormSrgb),
        create_dummy_texture(&ctx.device, &ctx.queue, 128, 128, wgpu::TextureFormat::Rgba8UnormSrgb),
    ];

    let splat_sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Splat Sampler"),
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });

    let tex_sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Tex Sampler"),
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::MipmapFilterMode::Linear,
        ..Default::default()
    });

    let view_refs = [&tex_views[0], &tex_views[1], &tex_views[2], &tex_views[3]];
    
    renderer.load_bind_group(
        &ctx.device,
        &splat_alpha_view,
        &splat_sampler,
        &shadow_map_view,
        view_refs,
        &tex_sampler,
    );

    assert!(renderer.bind_group.is_some(), "Bind group should be populated after loading");
}

#[test]
fn test_terrain_splatting_draw_call_uninitialized() {
    let ctx = WgpuEngineContext::new(0, 800, 600).unwrap();
    let format = wgpu::TextureFormat::Bgra8UnormSrgb;
    let renderer = TerrainSplattingRendererV2::new(&ctx.device, format).unwrap();

    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    
    let target_texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Target Texture"),
        size: wgpu::Extent3d { width: 800, height: 600, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let target_view = target_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let depth_texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Depth Texture"),
        size: wgpu::Extent3d { width: 800, height: 600, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    use wgpu::util::DeviceExt;
    
    let dummy_vertices = [TerrainVertex { position: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0] }];
    let vertex_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&dummy_vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    
    let dummy_indices: [u16; 3] = [0, 0, 0];
    let index_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&dummy_indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target_view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        let result = renderer.draw(&mut render_pass, &vertex_buffer, &index_buffer, 3);
        assert!(result.is_err(), "Draw should fail if bind group is not loaded");
    }
}
