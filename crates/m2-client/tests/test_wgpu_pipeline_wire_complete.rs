#[path = "../src/wgpu_pipeline_wire_complete.rs"]
mod wgpu_pipeline_wire_complete;

use std::mem::size_of;
use wgpu_pipeline_wire_complete::{ViewProjectionMatrix, LightDir, FogParams, PipelineCompleteContext};

async fn create_test_device() -> wgpu::Device {
    let instance = wgpu::Instance::default();
    
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: true,
            ..Default::default()
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, _) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create device");

    device
}

#[test]
fn test_view_projection_matrix_size() {
    assert_eq!(size_of::<ViewProjectionMatrix>(), 64);
}

#[test]
fn test_light_dir_size() {
    assert_eq!(size_of::<LightDir>(), 48); // 3*4 + 4 + 4*4 + 4*4 = 48 bytes
}

#[test]
fn test_fog_params_size() {
    assert_eq!(size_of::<FogParams>(), 32); // 4*4 + 4 + 4 + 4 + 4 = 32 bytes
}

#[tokio::test]
async fn test_pipeline_complete_context_initialization() {
    let device = create_test_device().await;
    let context = PipelineCompleteContext::new(&device);

    assert_eq!(context.view_proj_buffer.size(), 64);
    assert_eq!(context.light_dir_buffer.size(), 48);
    assert_eq!(context.fog_params_buffer.size(), 32);
}

#[tokio::test]
async fn test_skybox_pipeline_creation() {
    let device = create_test_device().await;
    let context = PipelineCompleteContext::new(&device);
    let _pipeline = wgpu_pipeline_wire_complete::create_skybox_pipeline(&device, wgpu::TextureFormat::Bgra8UnormSrgb, &context.shared_bind_group_layout);
}

#[tokio::test]
async fn test_water_pipeline_creation() {
    let device = create_test_device().await;
    let context = PipelineCompleteContext::new(&device);
    let _pipeline = wgpu_pipeline_wire_complete::create_water_pipeline(&device, wgpu::TextureFormat::Bgra8UnormSrgb, &context.shared_bind_group_layout);
}

#[tokio::test]
async fn test_magic_effect_pipeline_creation() {
    let device = create_test_device().await;
    let context = PipelineCompleteContext::new(&device);
    let _pipeline = wgpu_pipeline_wire_complete::create_magic_effect_pipeline(&device, wgpu::TextureFormat::Bgra8UnormSrgb, &context.shared_bind_group_layout);
}
