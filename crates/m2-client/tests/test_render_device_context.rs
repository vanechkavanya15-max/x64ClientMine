use std::time::{Duration, Instant};

#[path = "../src/render_device_context.rs"]
mod render_device_context;

use render_device_context::RenderDeviceContext;

#[test]
fn test_new_context() {
    let context = RenderDeviceContext::new(1920, 1080);
    assert_eq!(context.viewport_x, 0);
    assert_eq!(context.viewport_y, 0);
    assert_eq!(context.viewport_width, 1920);
    assert_eq!(context.viewport_height, 1080);
    assert_eq!(context.clear_color, [0.5, 0.6, 0.7, 1.0]);
    assert_eq!(context.clear_depth, 1.0);
    assert!(context.vsync_enabled);
    assert!(!context.is_in_frame());
    assert_eq!(context.get_fps(), 0);
    assert_eq!(context.get_cpu_overhead_ms(), 0.0);
    assert_eq!(context.get_gpu_overhead_ms(), 0.0);
}

#[test]
fn test_set_viewport() {
    let mut context = RenderDeviceContext::new(800, 600);
    context.set_viewport(10, 20, 1024, 768);
    assert_eq!(context.viewport_x, 10);
    assert_eq!(context.viewport_y, 20);
    assert_eq!(context.viewport_width, 1024);
    assert_eq!(context.viewport_height, 768);
}

#[test]
fn test_frame_cycle() {
    let mut context = RenderDeviceContext::new(800, 600);

    // Frame should start successfully
    assert!(context.begin_frame().is_ok());
    assert!(context.is_in_frame());

    // Starting again while in frame should fail
    assert_eq!(context.begin_frame(), Err("Frame already started"));

    // Simulating work
    std::thread::sleep(Duration::from_millis(10));

    // End frame
    context.end_frame();
    assert!(!context.is_in_frame());
    
    // Check CPU overhead is > 0 since we slept
    assert!(context.get_cpu_overhead_ms() > 0.0);
}

#[test]
fn test_frame_cycle_zero_dimensions() {
    let mut context = RenderDeviceContext::new(0, 0);

    // Frame should fail due to zero dimensions
    assert_eq!(context.begin_frame(), Err("Invalid viewport dimensions"));
    assert!(!context.is_in_frame());

    context.set_viewport(0, 0, 100, 0);
    assert_eq!(context.begin_frame(), Err("Invalid viewport dimensions"));

    context.set_viewport(0, 0, 0, 100);
    assert_eq!(context.begin_frame(), Err("Invalid viewport dimensions"));
}

#[test]
fn test_fps_calculation() {
    let mut context = RenderDeviceContext::new(800, 600);
    
    // Push updates backward by 1.1 seconds so FPS will update on next end_frame
    let fake_start = Instant::now() - Duration::from_millis(1100);
    context.force_last_fps_update(fake_start);

    // Run a frame
    assert!(context.begin_frame().is_ok());
    context.end_frame();

    // Since more than 1 second has passed between our fake_start and now, 
    // the fps update condition triggered and recorded our 1 frame over ~1.1s.
    // 1 frame / 1.1 sec is about 0.9, rounded to 1 fps.
    assert!(context.get_fps() > 0);
}

#[test]
fn test_gpu_overhead() {
    let mut context = RenderDeviceContext::new(800, 600);
    context.set_gpu_overhead(15.5);
    assert_eq!(context.get_gpu_overhead_ms(), 15.5);
}

#[test]
fn test_end_frame_when_not_started() {
    let mut context = RenderDeviceContext::new(800, 600);
    assert!(!context.is_in_frame());
    
    // Ending a frame when not in frame shouldn't panic or alter states maliciously
    context.end_frame();
    assert!(!context.is_in_frame());
    assert_eq!(context.get_cpu_overhead_ms(), 0.0);
}
