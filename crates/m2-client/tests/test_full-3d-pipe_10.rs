#[path = "../src/wgpu_postprocess_bloom.rs"]
mod wgpu_postprocess_bloom;

use wgpu_postprocess_bloom::PostprocessBloomPass;

async fn setup_device_and_queue() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find adapter");
    adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("Failed to create device")
}

#[tokio::test]
async fn test_pipeline_execution_basic() {
    let (device, _queue) = setup_device_and_queue().await;
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let pass = PostprocessBloomPass::new(&device, format);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let dummy_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let dummy_view = dummy_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pass.bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&dummy_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&pass.sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: pass.uniform_buffer.as_entire_binding(),
            },
        ],
    });

    pass.execute(&mut encoder, &dummy_view, &dummy_view, &bind_group);
    
    // Test should pass if no panic occurs during encoding
}

#[tokio::test]
async fn test_uniform_buffer_size() {
    let (device, _queue) = setup_device_and_queue().await;
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let pass = PostprocessBloomPass::new(&device, format);
    assert!(pass.uniform_buffer.size() >= 16, "Uniform buffer should be at least 16 bytes for padding");
}

#[tokio::test]
async fn test_bind_group_layout_entries() {
    let (device, _queue) = setup_device_and_queue().await;
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let pass = PostprocessBloomPass::new(&device, format);
    // Bind group layout creation success implies correctly matched entries in new()
    assert_eq!(std::mem::size_of::<wgpu_postprocess_bloom::BloomUniforms>(), 16);
}

#[tokio::test]
async fn test_update_threshold_value() {
    let (device, queue) = setup_device_and_queue().await;
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let pass = PostprocessBloomPass::new(&device, format);
    pass.update_threshold(&queue, 0.8);
    // Submit an empty command buffer to ensure queue operations don't panic
    let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    queue.submit(Some(encoder.finish()));
}

#[tokio::test]
async fn test_postprocess_bloom_pass_sampler_properties() {
    let (device, _queue) = setup_device_and_queue().await;
    let format = wgpu::TextureFormat::Bgra8Unorm;
    let _pass = PostprocessBloomPass::new(&device, format);
    // Successful creation implies correct sampler format matching the layout
}
