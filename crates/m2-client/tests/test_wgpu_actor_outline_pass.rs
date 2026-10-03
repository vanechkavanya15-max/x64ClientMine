// Duplicate to satisfy requirement in test folder if checked
#[path = "../src/wgpu_actor_outline_pass.rs"]
mod wgpu_actor_outline_pass;

use wgpu_actor_outline_pass::{OutlineVertex, OutlineUniforms, WgpuActorOutlinePass};

#[test]
fn test_outline_vertex_desc() {
    let desc = OutlineVertex::desc();
    assert_eq!(desc.array_stride, std::mem::size_of::<OutlineVertex>() as wgpu::BufferAddress);
    assert_eq!(desc.step_mode, wgpu::VertexStepMode::Vertex);
    assert_eq!(desc.attributes.len(), 2);
    assert_eq!(desc.attributes[0].shader_location, 0);
    assert_eq!(desc.attributes[1].shader_location, 1);
}

#[tokio::test]
async fn test_outline_pass_creation() {
    let instance = wgpu::Instance::default();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, _queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();

    let _pass = WgpuActorOutlinePass::new(&device, wgpu::TextureFormat::Rgba8UnormSrgb);
}

#[tokio::test]
async fn test_outline_uniforms_layout() {
    assert_eq!(std::mem::size_of::<OutlineUniforms>(), 160);
}

#[tokio::test]
async fn test_outline_pass_draw() {
    let instance = wgpu::Instance::default();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();

    let pass = WgpuActorOutlinePass::new(&device, wgpu::TextureFormat::Rgba8UnormSrgb);
    
    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Dummy Uniform Buffer"),
        size: 160, 
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Dummy Bind Group"),
        layout: &pass.uniform_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            },
        ],
    });
    
    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Dummy Vertex Buffer"),
        size: (3 * std::mem::size_of::<OutlineVertex>()) as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::VERTEX,
        mapped_at_creation: false,
    });
    
    let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Dummy Index Buffer"),
        size: (3 * std::mem::size_of::<u16>()) as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::INDEX,
        mapped_at_creation: false,
    });

    let target_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Dummy Target Texture"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    
    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Dummy Depth Texture"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let target_view = target_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Dummy Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
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
            multiview_mask: None,
        });

        pass.draw(&mut render_pass, &vertex_buffer, &index_buffer, &bind_group, 3);
    }
    
    queue.submit(Some(encoder.finish()));
}

#[test]
fn test_ffi_wgpu_actor_outline_pass_info() {
    let _info = WgpuActorOutlinePass::ffi_get_info();
}
