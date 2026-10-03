use m2_client::full_3d_pipe_01::*;
use m2_client::wgpu_render_pipeline_mgr::WgpuRenderPipelineManager;
use m2_client::wgpu_engine_context::{WgpuEngineContext, Quad2D, GpuTexture};

fn setup_headless_context() -> WgpuEngineContext {
    let mut ctx = WgpuEngineContext::new(0, 800, 600).unwrap();
    ctx.is_headless = true;
    ctx
}

fn create_dummy_texture(device: &wgpu::Device) -> GpuTexture {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("dummy"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
        ],
    });
    GpuTexture {
        texture,
        view,
        bind_group,
        width: 1,
        height: 1,
    }
}

#[test]
fn test_skybox_pass_execution() {
    let ctx = setup_headless_context();
    let pipeline_mgr = WgpuRenderPipelineManager::new(&ctx.device, 800, 600);
    
    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let view = pipeline_mgr.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        
        let mut flags = 0;
        SkyboxPass::execute(&mut render_pass, &pipeline_mgr, &mut flags, 1);
        assert_eq!(flags, 1);
    }
}

#[test]
fn test_terrain_splatting_pass_execution() {
    let ctx = setup_headless_context();
    let pipeline_mgr = WgpuRenderPipelineManager::new(&ctx.device, 800, 600);
    
    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let view = pipeline_mgr.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        
        let mut flags = 0;
        TerrainSplattingPass::execute(&mut render_pass, &pipeline_mgr, &mut flags, 2);
        assert_eq!(flags, 2);
    }
}

#[test]
fn test_water_plane_pass_execution() {
    let ctx = setup_headless_context();
    let pipeline_mgr = WgpuRenderPipelineManager::new(&ctx.device, 800, 600);
    
    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let view = pipeline_mgr.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        
        let mut flags = 0;
        WaterPlanePass::execute(&mut render_pass, &pipeline_mgr, &mut flags, 4);
        assert_eq!(flags, 4);
    }
}

#[test]
fn test_skeletal_actor_pass_execution() {
    let ctx = setup_headless_context();
    let pipeline_mgr = WgpuRenderPipelineManager::new(&ctx.device, 800, 600);
    
    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let view = pipeline_mgr.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        
        let mut flags = 0;
        SkeletalActorPass::execute(&mut render_pass, &pipeline_mgr, &mut flags, 8);
        assert_eq!(flags, 8);
    }
}

#[test]
fn test_particle_billboard_pass_execution() {
    let ctx = setup_headless_context();
    let pipeline_mgr = WgpuRenderPipelineManager::new(&ctx.device, 800, 600);
    
    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let view = pipeline_mgr.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        
        let mut flags = 0;
        ParticleBillboardPass::execute(&mut render_pass, &pipeline_mgr, &mut flags, 16);
        assert_eq!(flags, 16);
    }
}

#[test]
fn test_texttail_pass_execution() {
    let ctx = setup_headless_context();
    let pipeline_mgr = WgpuRenderPipelineManager::new(&ctx.device, 800, 600);
    
    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let view = pipeline_mgr.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        
        let mut flags = 0;
        TextTailPass::execute(&mut render_pass, &pipeline_mgr, &mut flags, 32);
        assert_eq!(flags, 32);
    }
}

#[test]
fn test_ui2d_pass_execution_with_quads() {
    let ctx = setup_headless_context();
    let dummy_tex = create_dummy_texture(&ctx.device);
    
    let quad = Quad2D {
        rect: [0.0, 0.0, 100.0, 100.0],
        uv: [0.0, 0.0, 1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    };
    let quads = vec![quad];
    let ui_batches = vec![(&dummy_tex, quads.as_slice())];
    
    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let color_view = create_dummy_texture(&ctx.device).view;
    
    let mut flags = 0;
    UI2DPass::execute(&mut encoder, &color_view, &ctx, &ui_batches, &mut flags, 64);
    
    assert_eq!(flags, 64);
}
