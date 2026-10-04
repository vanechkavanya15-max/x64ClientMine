use m2_client::wgpu_magic_effect_mesh_pipeline::{D3DBLEND_INVSRCALPHA, D3DBLEND_ONE, D3DBLEND_SRCALPHA};
use m2_client::wgpu_screen_filter_postprocess::WgpuScreenFilter;

async fn get_test_device_and_queue() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: None,
            apply_limit_buckets: false,
        })
        .await
        .unwrap();

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Test Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                experimental_features: Default::default(),
                memory_hints: Default::default(),
                trace: Default::default(),
            },
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn test_screen_filter_creation() {
    let (device, _queue) = get_test_device_and_queue().await;
    let format = wgpu::TextureFormat::Rgba8Unorm;
    
    let filter = WgpuScreenFilter::new(&device, format);

    assert_eq!(filter.enabled, false);
    assert_eq!(filter.src_blend, D3DBLEND_SRCALPHA);
    assert_eq!(filter.dest_blend, D3DBLEND_INVSRCALPHA);
    assert_eq!(filter.color, [0.0, 0.0, 0.0, 0.0]);
}

#[tokio::test]
async fn test_screen_filter_enable_disable() {
    let (device, _queue) = get_test_device_and_queue().await;
    let mut filter = WgpuScreenFilter::new(&device, wgpu::TextureFormat::Rgba8Unorm);

    filter.set_enable(true);
    assert_eq!(filter.enabled, true);

    filter.set_enable(false);
    assert_eq!(filter.enabled, false);
}

#[tokio::test]
async fn test_screen_filter_effects() {
    let (device, _queue) = get_test_device_and_queue().await;
    let mut filter = WgpuScreenFilter::new(&device, wgpu::TextureFormat::Rgba8Unorm);

    // Flash effect
    filter.flash(1.0, 1.0, 1.0);
    assert_eq!(filter.color, [1.0, 1.0, 1.0, 1.0]);

    // Fade effect (simulate fade out over 2 steps)
    filter.fade(1.0, 1.0, 1.0, 0.0, 0.5);
    assert_eq!(filter.color, [1.0, 1.0, 1.0, 0.5]);

    filter.fade(1.0, 1.0, 1.0, 0.0, 0.5);
    assert_eq!(filter.color, [1.0, 1.0, 1.0, 0.25]); // current alpha is 0.5, target 0.0, diff is -0.5, progress 0.5 -> new alpha = 0.5 - 0.25 = 0.25

    // Tint effect
    filter.tint(0.2, 0.3, 0.4, 0.8);
    assert_eq!(filter.color, [0.2, 0.3, 0.4, 0.8]);
}

#[tokio::test]
async fn test_screen_filter_blend_modes() {
    let (device, _queue) = get_test_device_and_queue().await;
    let mut filter = WgpuScreenFilter::new(&device, wgpu::TextureFormat::Rgba8Unorm);

    assert_eq!(filter.src_blend, D3DBLEND_SRCALPHA);
    assert_eq!(filter.dest_blend, D3DBLEND_INVSRCALPHA);

    filter.set_blend_type(D3DBLEND_SRCALPHA, D3DBLEND_ONE, &device, wgpu::TextureFormat::Rgba8Unorm);
    
    assert_eq!(filter.src_blend, D3DBLEND_SRCALPHA);
    assert_eq!(filter.dest_blend, D3DBLEND_ONE);
}
