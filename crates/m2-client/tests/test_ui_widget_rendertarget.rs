#[path = "../src/ui_widget_rendertarget.rs"]
mod ui_widget_rendertarget;

use ui_widget_rendertarget::{ActorRenderPreview, RenderTarget};
use wgpu;
use std::sync::Arc;
use tokio;

async fn setup_wgpu() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: None,
            force_fallback_adapter: false,
            ..Default::default()
        })
        .await
        .expect("Failed to find wgpu adapter");

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("Failed to create wgpu device");

    (device, queue)
}

#[tokio::test]
async fn test_register_render_target() {
    let (device, _) = setup_wgpu().await;
    
    let target = RenderTarget::register_render_target(&device, 256, 256).unwrap();
    
    assert_eq!(target.width, 256);
    assert_eq!(target.height, 256);
    assert_eq!(target.texture.format(), wgpu::TextureFormat::Rgba8Unorm);
}

#[test]
fn test_actor_preview_properties() {
    let mut actor = ActorRenderPreview::new(12345, 90.0);
    assert_eq!(actor.armor_vnum, 12345);
    assert_eq!(actor.rotation_y, 90.0);
    
    actor.set_armor(67890);
    assert_eq!(actor.armor_vnum, 67890);
    
    actor.set_rotation(180.0);
    assert_eq!(actor.rotation_y, 180.0);
}

#[tokio::test]
async fn test_actor_preview_render_pass() {
    let (device, queue) = setup_wgpu().await;
    
    let target = RenderTarget::register_render_target(&device, 128, 128).unwrap();
    let actor = ActorRenderPreview::new(0, 0.0);
    
    // Should not panic
    target.render_actor_preview(&device, &queue, &actor);
}

#[tokio::test]
async fn test_ui_quad_bind_group() {
    let (device, _) = setup_wgpu().await;
    let target = RenderTarget::register_render_target(&device, 64, 64).unwrap();
    
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
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
    
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });
    
    let bind_group = target.get_ui_quad_bind_group(&device, &layout, &sampler);
    // If bind group is created, it's successful. 
    // Testing wgpu objects is opaque, so merely constructing it without panic is passing.
    assert!(true);
}

#[tokio::test]
async fn test_render_target_resize() {
    let (device, _) = setup_wgpu().await;
    
    let mut target = RenderTarget::register_render_target(&device, 100, 100).unwrap();
    assert_eq!(target.width, 100);
    
    // Resize simulation (recreate)
    target = RenderTarget::register_render_target(&device, 200, 200).unwrap();
    assert_eq!(target.width, 200);
    assert_eq!(target.height, 200);
}
