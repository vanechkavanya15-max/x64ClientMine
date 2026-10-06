#[path = "../src/camera_uniform_uploader.rs"]
mod camera_uniform_uploader;

use camera_uniform_uploader::{CameraUniforms, CameraUniformUploader};
use std::mem::size_of;

fn setup_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::default();
    
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: None,
        force_fallback_adapter: false,
        apply_limit_buckets: false,
    }))
    .expect("Failed to find wgpu adapter");
    
    pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            memory_hints: wgpu::MemoryHints::Performance,
            ..Default::default()
        }
    ))
    .expect("Failed to create wgpu device")
}

#[test]
fn test_camera_uniforms_memory_layout() {
    let size = size_of::<CameraUniforms>();
    // view_proj (64) + view (64) + proj (64) + eye_position (16) + clipping_planes (16) = 224 bytes
    assert_eq!(size, 224, "CameraUniforms size must be 224 bytes");
    
    let uniforms = CameraUniforms::default();
    assert_eq!(size_of::<[[f32; 4]; 4]>(), 64);
    assert_eq!(size_of::<[f32; 4]>(), 16);
    
    // Check that we can transmute safely (handled by bytemuck deriving)
    let bytes = bytemuck::cast_slice::<_, u8>(std::slice::from_ref(&uniforms));
    assert_eq!(bytes.len(), 224);
}

#[test]
fn test_camera_uniform_uploader_initialization() {
    let (device, _) = setup_device();
    
    let uploader = CameraUniformUploader::new(&device);
    
    // Assert that the buffer has the correct size
    assert_eq!(uploader.buffer.size(), size_of::<CameraUniforms>() as wgpu::BufferAddress);
}

#[test]
fn test_camera_uniforms_buffer_size_match() {
    let (device, _) = setup_device();
    
    let uploader = CameraUniformUploader::new(&device);
    assert_eq!(uploader.buffer.usage().contains(wgpu::BufferUsages::UNIFORM), true);
    assert_eq!(uploader.buffer.usage().contains(wgpu::BufferUsages::COPY_DST), true);
    
    let expected_size = size_of::<CameraUniforms>() as wgpu::BufferAddress;
    assert_eq!(uploader.buffer.size(), expected_size);
}

#[test]
fn test_camera_uniform_upload_logic() {
    let (device, queue) = setup_device();
    let uploader = CameraUniformUploader::new(&device);
    
    let mut uniforms = CameraUniforms::default();
    uniforms.eye_position = [1.0, 2.0, 3.0, 1.0];
    uniforms.clipping_planes = [0.1, 1000.0, 0.0, 0.0];
    
    // Set some dummy matrix data
    for i in 0..4 {
        for j in 0..4 {
            uniforms.view_proj[i][j] = (i * 4 + j) as f32;
        }
    }
    
    // Ensure no panic on upload
    uploader.upload(&queue, &uniforms);
    queue.submit(std::iter::empty());
    
    // Wait for queue
    device.poll(wgpu::Maintain::Wait);
}

#[test]
fn test_camera_uniform_upload_extreme_values() {
    let (device, queue) = setup_device();
    let uploader = CameraUniformUploader::new(&device);
    
    let mut uniforms = CameraUniforms::default();
    uniforms.eye_position = [f32::MAX, f32::MIN, f32::INFINITY, f32::NAN];
    
    // Fill view matrix with large values
    for i in 0..4 {
        for j in 0..4 {
            uniforms.view[i][j] = f32::MAX;
        }
    }
    
    // Upload extreme values
    uploader.upload(&queue, &uniforms);
    queue.submit(std::iter::empty());
    
    // Sync
    device.poll(wgpu::Maintain::Wait);
    
    // Test completes if it doesn't panic on extreme floats
}
