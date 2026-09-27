use m2_client::wgpu_engine_context::{WgpuEngineContext, WgpuWindowTarget};

#[test]
fn test_wgpu_headless_context_creation() {
    let ctx = WgpuEngineContext::new(0, 800, 600).expect("Headless wgpu context should succeed");
    assert!(ctx.is_headless);
    assert_eq!(ctx.width, 800);
    assert_eq!(ctx.height, 600);
    assert!(ctx.get_adapter_info().contains("Adapter:"));
}

#[test]
fn test_wgpu_headless_render_clear_frame() {
    let mut ctx = WgpuEngineContext::new(0, 1024, 768).expect("Headless wgpu context should succeed");
    let dark_bg = wgpu::Color {
        r: 0.04,
        g: 0.05,
        b: 0.07,
        a: 1.0,
    };
    let render_res = ctx.render_clear_frame(dark_bg);
    assert!(render_res.is_ok(), "Clear frame in headless mode must succeed");
}

#[test]
fn test_wgpu_headless_resize() {
    let mut ctx = WgpuEngineContext::new(0, 800, 600).expect("Headless wgpu context should succeed");
    ctx.resize(1920, 1080);
    assert_eq!(ctx.width, 1920);
    assert_eq!(ctx.height, 1080);
}

#[test]
fn test_wgpu_window_target_handles() {
    use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

    let target = WgpuWindowTarget {
        hwnd: 0x1234,
        hinstance: Some(0x5678),
    };

    let win_handle = target.window_handle().expect("Window handle must be valid");
    if let RawWindowHandle::Win32(h) = win_handle.as_raw() {
        assert_eq!(h.hwnd.get(), 0x1234);
        assert_eq!(h.hinstance.unwrap().get(), 0x5678);
    } else {
        panic!("Expected Win32 window handle");
    }

    let disp_handle = target.display_handle().expect("Display handle must be valid");
    if let RawDisplayHandle::Windows(_) = disp_handle.as_raw() {
        // ok
    } else {
        panic!("Expected Windows display handle");
    }
}
