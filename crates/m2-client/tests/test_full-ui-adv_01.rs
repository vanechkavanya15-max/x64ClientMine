use std::sync::Arc;
use m2_client::wgpu_engine_context::GpuTexture;
use m2_client::ui_rendertarget_texture_allocator::RenderTargetTextureAllocator;

fn setup_headless_device() -> Arc<wgpu::Device> {
    let instance = wgpu::Instance::default();
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: None,
        ..Default::default()
    }))
    .expect("Failed to find headless adapter");

    let (device, _) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("Test Headless Device"),
            ..Default::default()
        },
    ))
    .expect("Failed to create headless device");

    Arc::new(device)
}

#[test]
fn test_allocator_successful_creation() {
    let device = setup_headless_device();
    
    let result = RenderTargetTextureAllocator::allocate(&device, 256, 256, 1);
    
    assert!(result.is_ok(), "Texture allocation should succeed for valid dimensions");
    let gpu_texture = result.unwrap();
    assert_eq!(gpu_texture.width, 256);
    assert_eq!(gpu_texture.height, 256);
}

#[test]
fn test_allocator_zero_width_fails() {
    let device = setup_headless_device();
    
    let result = RenderTargetTextureAllocator::allocate(&device, 0, 256, 2);
    
    assert!(result.is_err(), "Texture allocation should fail for zero width");
    assert_eq!(result.err().unwrap(), "Width and height must be greater than zero");
}

#[test]
fn test_allocator_zero_height_fails() {
    let device = setup_headless_device();
    
    let result = RenderTargetTextureAllocator::allocate(&device, 256, 0, 3);
    
    assert!(result.is_err(), "Texture allocation should fail for zero height");
    assert_eq!(result.err().unwrap(), "Width and height must be greater than zero");
}

#[test]
fn test_allocator_format_and_usage() {
    let device = setup_headless_device();
    
    let result = RenderTargetTextureAllocator::allocate(&device, 128, 128, 4);
    assert!(result.is_ok());
    
    let gpu_texture = result.unwrap();
    
    assert_eq!(gpu_texture.texture.format(), wgpu::TextureFormat::Rgba8UnormSrgb);
    
    let usages = gpu_texture.texture.usage();
    assert!(usages.contains(wgpu::TextureUsages::RENDER_ATTACHMENT));
    assert!(usages.contains(wgpu::TextureUsages::TEXTURE_BINDING));
    assert!(usages.contains(wgpu::TextureUsages::COPY_SRC));
}

#[test]
fn test_allocator_multiple_allocations() {
    let device = setup_headless_device();
    
    let t1 = RenderTargetTextureAllocator::allocate(&device, 64, 64, 5);
    let t2 = RenderTargetTextureAllocator::allocate(&device, 512, 512, 6);
    
    assert!(t1.is_ok());
    assert!(t2.is_ok());
    
    let tex1 = t1.unwrap();
    let tex2 = t2.unwrap();
    
    assert_eq!(tex1.width, 64);
    assert_eq!(tex2.width, 512);
}
