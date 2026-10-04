use m2_client::wgpu_magic_effect_mesh_pipeline::{D3DBLEND_INVSRCALPHA, D3DBLEND_ONE, D3DBLEND_SRCALPHA};
use m2_client::wgpu_screen_filter_postprocess::{FilterState, WgpuScreenFilter};

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
    assert_eq!(filter.state, FilterState::None);
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
async fn test_screen_filter_stateful_effects() {
    let (device, _queue) = get_test_device_and_queue().await;
    let mut filter = WgpuScreenFilter::new(&device, wgpu::TextureFormat::Rgba8Unorm);

    // Initial configuration for testing Flash effect revert logic
    filter.set_color(0.5, 0.5, 0.5, 0.5);

    // Test Flash
    filter.flash(1.0, 0.0, 0.0, 1.0);
    assert_eq!(filter.state, FilterState::Flash);
    assert_eq!(filter.enabled, true);
    
    // Halfway update
    filter.update(0.5);
    assert_eq!(filter.state, FilterState::Flash);
    // Interpolates from [1.0, 0.0, 0.0, 1.0] (start_color) to [0.5, 0.5, 0.5, 0.5] (flash_revert_color)
    // Over 0.5 out of 1.0 duration -> halfway interpolation
    assert_eq!(filter.color, [0.75, 0.25, 0.25, 0.75]);
    
    // Finish update
    filter.update(0.6);
    assert_eq!(filter.state, FilterState::None);
    assert_eq!(filter.color, [0.5, 0.5, 0.5, 0.5]); // Reverted to initial color

    // Reset initial config for fade tests
    filter.set_color(0.0, 0.0, 0.0, 0.0);

    // Test Fade In (Starts fully opaque, fades to transparent)
    filter.fade_in(0.0, 1.0, 0.0, 2.0);
    assert_eq!(filter.state, FilterState::FadeIn);
    assert_eq!(filter.color, [0.0, 1.0, 0.0, 1.0]);

    // Halfway update
    filter.update(1.0);
    assert_eq!(filter.color[3], 0.5);

    // Finish update
    filter.update(1.1);
    assert_eq!(filter.state, FilterState::None);
    assert_eq!(filter.color[3], 0.0);
    assert_eq!(filter.enabled, false); // Disabled when invisible

    // Test Fade Out (Starts transparent, fades to opaque)
    filter.fade_out(0.0, 0.0, 1.0, 1.0);
    assert_eq!(filter.state, FilterState::FadeOut);
    assert_eq!(filter.color, [0.0, 0.0, 1.0, 0.0]);

    // Halfway update
    filter.update(0.5);
    assert_eq!(filter.color[3], 0.5);
    
    // Finish update
    filter.update(0.6);
    assert_eq!(filter.state, FilterState::None);
    assert_eq!(filter.color[3], 1.0);
    assert_eq!(filter.enabled, true);

    // Test Tint
    filter.tint(1.0, 1.0, 1.0, 0.5);
    assert_eq!(filter.color, [1.0, 1.0, 1.0, 0.5]);
    assert_eq!(filter.enabled, true);

    filter.tint(1.0, 1.0, 1.0, 0.0);
    assert_eq!(filter.enabled, false);
}

#[tokio::test]
async fn test_screen_filter_blend_modes() {
    let (device, _queue) = get_test_device_and_queue().await;
    let mut filter = WgpuScreenFilter::new(&device, wgpu::TextureFormat::Rgba8Unorm);

    assert_eq!(filter.src_blend, D3DBLEND_SRCALPHA);
    assert_eq!(filter.dest_blend, D3DBLEND_INVSRCALPHA);

    // Test pipeline helper was invoked by checking state changes
    filter.set_blend_type(D3DBLEND_SRCALPHA, D3DBLEND_ONE, &device, wgpu::TextureFormat::Rgba8Unorm);
    
    assert_eq!(filter.src_blend, D3DBLEND_SRCALPHA);
    assert_eq!(filter.dest_blend, D3DBLEND_ONE);
}
