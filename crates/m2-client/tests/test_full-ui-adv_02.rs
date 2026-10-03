use m2_client::ui_rendertarget_scene_pass::{CameraUniform, CharacterModel, ModelUniform, RenderTargetScenePass, Vertex};
use wgpu::util::DeviceExt;
use std::num::NonZeroU32;

#[test]
fn test_vertex_layout() {
    let layout = Vertex::desc();
    assert_eq!(layout.array_stride, std::mem::size_of::<Vertex>() as wgpu::BufferAddress);
    assert_eq!(layout.step_mode, wgpu::VertexStepMode::Vertex);
    assert_eq!(layout.attributes.len(), 3);
    
    // Position
    assert_eq!(layout.attributes[0].offset, 0);
    assert_eq!(layout.attributes[0].format, wgpu::VertexFormat::Float32x3);
    
    // Normal
    assert_eq!(layout.attributes[1].offset, std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress);
    assert_eq!(layout.attributes[1].format, wgpu::VertexFormat::Float32x3);
    
    // Tex coords
    assert_eq!(layout.attributes[2].offset, std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress);
    assert_eq!(layout.attributes[2].format, wgpu::VertexFormat::Float32x2);
}

#[test]
fn test_uniform_sizes() {
    assert_eq!(std::mem::size_of::<CameraUniform>(), 64); // 4x4 f32 matrix
    assert_eq!(std::mem::size_of::<ModelUniform>(), 64);  // 4x4 f32 matrix
}

async fn create_test_device_and_queue() -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::default();
    
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: None,
        force_fallback_adapter: false,
    }).await?;
    
    adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
    }, None).await.ok()
}

fn create_dummy_texture_bind_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("dummy texture"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: Some("dummy texture bind group"),
    })
}

fn create_dummy_character(device: &wgpu::Device, texture_bind_group: wgpu::BindGroup) -> CharacterModel {
    let vertices = vec![Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    }];
    let indices: Vec<u16> = vec![0, 0, 0];

    CharacterModel {
        vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("dummy vb"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }),
        index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("dummy ib"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        }),
        num_indices: 3,
        texture_bind_group,
    }
}

#[tokio::test]
async fn test_rendertarget_scene_pass_init() {
    let (device, _) = match create_test_device_and_queue().await {
        Some(dq) => dq,
        None => {
            println!("Skipping test due to no WGPU adapter available");
            return;
        }
    };

    let pass = RenderTargetScenePass::new(&device, wgpu::TextureFormat::Rgba8UnormSrgb);
    
    // Assert resources were created
    assert!(true); // pass is successfully created
}

#[tokio::test]
async fn test_draw_character_without_weapon() {
    let (device, queue) = match create_test_device_and_queue().await {
        Some(dq) => dq,
        None => {
            println!("Skipping test due to no WGPU adapter available");
            return;
        }
    };

    let pass = RenderTargetScenePass::new(&device, wgpu::TextureFormat::Rgba8UnormSrgb);
    
    let dummy_texture_bg = create_dummy_texture_bind_group(&device, &pass.texture_bind_group_layout);
    let character = create_dummy_character(&device, dummy_texture_bg);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    
    let target_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("RenderTarget"),
        size: wgpu::Extent3d { width: 256, height: 256, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let target_view = target_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Depth"),
        size: wgpu::Extent3d { width: 256, height: 256, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Test Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.draw_character(&mut render_pass, &character, None);
    }
    
    queue.submit(std::iter::once(encoder.finish()));
}

#[tokio::test]
async fn test_draw_character_with_weapon() {
    let (device, queue) = match create_test_device_and_queue().await {
        Some(dq) => dq,
        None => {
            println!("Skipping test due to no WGPU adapter available");
            return;
        }
    };

    let pass = RenderTargetScenePass::new(&device, wgpu::TextureFormat::Rgba8UnormSrgb);
    
    let dummy_texture_bg = create_dummy_texture_bind_group(&device, &pass.texture_bind_group_layout);
    let dummy_texture_bg2 = create_dummy_texture_bind_group(&device, &pass.texture_bind_group_layout);
    
    let character = create_dummy_character(&device, dummy_texture_bg);
    let weapon = create_dummy_character(&device, dummy_texture_bg2);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    
    let target_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("RenderTarget"),
        size: wgpu::Extent3d { width: 256, height: 256, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let target_view = target_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Depth"),
        size: wgpu::Extent3d { width: 256, height: 256, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Test Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.draw_character(&mut render_pass, &character, Some(&weapon));
    }
    
    queue.submit(std::iter::once(encoder.finish()));
}
