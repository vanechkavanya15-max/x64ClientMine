// Direct import avoiding test inclusion errors
use m2_client::window_display_mode_switcher::DisplayModeSwitcher;
use m2_client::win32_window::{WindowConfig, create_game_window, _get_mock_window_style};
use m2_client::wgpu_engine_context::WgpuEngineContext;
use std::ptr;

// Helper to create a headless/mock WgpuEngineContext for testing
fn create_mock_wgpu_context() -> WgpuEngineContext {
    // 0 is handled as a headless indicator in WgpuEngineContext::new
    WgpuEngineContext::new(0, 800, 600).unwrap()
}

#[test]
fn test_display_mode_switcher_creation() {
    let config = WindowConfig {
        title: ptr::null(),
        width: 800,
        height: 600,
        is_fullscreen: false,
        is_borderless: false,
    };
    let hwnd = create_game_window(&config).unwrap();
    let switcher = DisplayModeSwitcher::new(hwnd, 800, 600, 1920, 1080);

    assert_eq!(switcher.is_fullscreen, false);
    assert_eq!(switcher.window_width, 800);
    assert_eq!(switcher.window_height, 600);
    assert_eq!(switcher.fullscreen_width, 1920);
    assert_eq!(switcher.fullscreen_height, 1080);
}

#[test]
fn test_switch_to_fullscreen() {
    let config = WindowConfig {
        title: ptr::null(),
        width: 800,
        height: 600,
        is_fullscreen: false,
        is_borderless: false,
    };
    let hwnd = create_game_window(&config).unwrap();
    let mut switcher = DisplayModeSwitcher::new(hwnd, 800, 600, 1920, 1080);
    let mut wgpu_ctx = create_mock_wgpu_context();

    switcher.set_fullscreen(&mut wgpu_ctx, true);

    assert_eq!(switcher.is_fullscreen, true);
    assert_eq!(wgpu_ctx.width, 1920);
    assert_eq!(wgpu_ctx.height, 1080);

    #[cfg(not(windows))]
    {
        let style = _get_mock_window_style(hwnd).unwrap();
        // WS_POPUP = 0x80000000, WS_VISIBLE = 0x10000000
        assert_eq!(style & 0x80000000, 0x80000000);
    }
}

#[test]
fn test_switch_to_windowed() {
    let config = WindowConfig {
        title: ptr::null(),
        width: 800,
        height: 600,
        is_fullscreen: false,
        is_borderless: false,
    };
    let hwnd = create_game_window(&config).unwrap();
    let mut switcher = DisplayModeSwitcher::new(hwnd, 800, 600, 1920, 1080);
    let mut wgpu_ctx = create_mock_wgpu_context();

    // First go to fullscreen
    switcher.set_fullscreen(&mut wgpu_ctx, true);
    assert_eq!(switcher.is_fullscreen, true);

    // Now go back to windowed
    switcher.set_fullscreen(&mut wgpu_ctx, false);

    assert_eq!(switcher.is_fullscreen, false);
    assert_eq!(wgpu_ctx.width, 800);
    assert_eq!(wgpu_ctx.height, 600);
}

#[test]
fn test_toggle_fullscreen() {
    let config = WindowConfig {
        title: ptr::null(),
        width: 800,
        height: 600,
        is_fullscreen: false,
        is_borderless: false,
    };
    let hwnd = create_game_window(&config).unwrap();
    let mut switcher = DisplayModeSwitcher::new(hwnd, 800, 600, 1920, 1080);
    let mut wgpu_ctx = create_mock_wgpu_context();

    // Toggle 1: windowed -> fullscreen
    switcher.toggle_fullscreen(&mut wgpu_ctx);
    assert_eq!(switcher.is_fullscreen, true);
    assert_eq!(wgpu_ctx.width, 1920);

    // Toggle 2: fullscreen -> windowed
    switcher.toggle_fullscreen(&mut wgpu_ctx);
    assert_eq!(switcher.is_fullscreen, false);
    assert_eq!(wgpu_ctx.width, 800);
}

#[test]
fn test_redundant_switch() {
    let config = WindowConfig {
        title: ptr::null(),
        width: 800,
        height: 600,
        is_fullscreen: false,
        is_borderless: false,
    };
    let hwnd = create_game_window(&config).unwrap();
    let mut switcher = DisplayModeSwitcher::new(hwnd, 800, 600, 1920, 1080);
    let mut wgpu_ctx = create_mock_wgpu_context();

    // Go to fullscreen
    switcher.set_fullscreen(&mut wgpu_ctx, true);
    let width_after_first = wgpu_ctx.width;

    // Redundant call
    switcher.set_fullscreen(&mut wgpu_ctx, true);

    assert_eq!(switcher.is_fullscreen, true);
    assert_eq!(wgpu_ctx.width, width_after_first); // Size shouldn't change
}

#[test]
fn test_update_resolutions() {
    let config = WindowConfig {
        title: ptr::null(),
        width: 800,
        height: 600,
        is_fullscreen: false,
        is_borderless: false,
    };
    let hwnd = create_game_window(&config).unwrap();
    let mut switcher = DisplayModeSwitcher::new(hwnd, 800, 600, 1920, 1080);

    switcher.update_resolutions(1024, 768, 2560, 1440);

    assert_eq!(switcher.window_width, 1024);
    assert_eq!(switcher.window_height, 768);
    assert_eq!(switcher.fullscreen_width, 2560);
    assert_eq!(switcher.fullscreen_height, 1440);
}
