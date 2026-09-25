use m2_client_render::gpu_context::{ColorFormat, DepthFormat, GpuContext, Viewport};

#[test]
fn test_integration_gpu_context_initialization() {
    let instance = GpuContext::new();
    assert!(instance.is_ready());
    assert_eq!(instance.resolution(), (800, 600));
    assert_eq!(instance.z_buffer_enabled(), true);
    assert_eq!(instance.vsync_enabled(), false);
    assert_eq!(instance.color_format(), ColorFormat::Rgba8Unorm);
    assert_eq!(instance.depth_format(), DepthFormat::Depth32Float);
    let vp = instance.viewport();
    assert_eq!(vp.width, 800.0);
    assert_eq!(vp.height, 600.0);
}

#[test]
fn test_integration_gpu_context_resolution_change() {
    let mut instance = GpuContext::new();
    instance.set_resolution(1920, 1080);
    assert_eq!(instance.resolution(), (1920, 1080));
    let vp = instance.viewport();
    assert_eq!(vp.width, 1920.0);
    assert_eq!(vp.height, 1080.0);
}

#[test]
fn test_integration_gpu_context_toggles() {
    let mut instance = GpuContext::new();

    instance.set_z_buffer_enabled(false);
    assert_eq!(instance.z_buffer_enabled(), false);

    instance.set_vsync(true);
    assert_eq!(instance.vsync_enabled(), true);
}

#[test]
fn test_integration_gpu_context_formats() {
    let mut instance = GpuContext::new();

    instance.set_color_format(ColorFormat::Bgra8Unorm);
    assert_eq!(instance.color_format(), ColorFormat::Bgra8Unorm);

    instance.set_depth_format(DepthFormat::Depth24Plus);
    assert_eq!(instance.depth_format(), DepthFormat::Depth24Plus);
}

#[test]
fn test_integration_gpu_context_viewport() {
    let mut instance = GpuContext::new();

    let vp = Viewport::new(10.0, 20.0, 400.0, 300.0, 0.1, 0.9);
    instance.set_viewport(vp.clone());

    let current_vp = instance.viewport();
    assert_eq!(current_vp.x, 10.0);
    assert_eq!(current_vp.y, 20.0);
    assert_eq!(current_vp.width, 400.0);
    assert_eq!(current_vp.height, 300.0);
    assert_eq!(current_vp.min_depth, 0.1);
    assert_eq!(current_vp.max_depth, 0.9);
}
