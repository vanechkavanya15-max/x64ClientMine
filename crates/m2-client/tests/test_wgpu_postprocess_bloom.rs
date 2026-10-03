#[path = "../src/wgpu_postprocess_bloom.rs"]
mod wgpu_postprocess_bloom;

use wgpu_postprocess_bloom::{PostprocessBloomPass, BloomUniforms};

#[tokio::test]
async fn test_postprocess_bloom_pass_creation() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find adapter");
    let (device, _queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("Failed to create device");

    let pass = PostprocessBloomPass::new(&device, wgpu::TextureFormat::Rgba8Unorm);
    assert_eq!(pass.uniform_buffer.size(), std::mem::size_of::<BloomUniforms>() as u64);
}

#[tokio::test]
async fn test_update_threshold() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("Failed to create device");

    let pass = PostprocessBloomPass::new(&device, wgpu::TextureFormat::Rgba8Unorm);
    pass.update_threshold(&queue, 0.5);

    // Wait for the queue to complete the write operation
    queue.submit(None);
    let _ = device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
}
