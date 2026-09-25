use m2_client::app_procedure::*;

#[test]
fn test_activate_app() {
    let mut app = AppProcedure::new();
    app.is_window_full_screen_enable = true;
    app.dw_width = 1920;
    app.dw_height = 1080;
    app.bpp = 32;

    // WA_ACTIVE
    app.window_procedure(1234, WM_ACTIVATEAPP, WA_ACTIVE, 0);
    assert!(app.is_activate_wnd);
    assert_eq!(app.events[0], "m_SoundManager.RestoreVolume");
    assert_eq!(app.events[1], "__SetFullScreenWindow(1234, 1920, 1080, 32)");

    app.events.clear();

    // INACTIVE
    app.window_procedure(1234, WM_ACTIVATEAPP, 0, 0);
    assert!(!app.is_activate_wnd);
    assert_eq!(app.events[0], "m_SoundManager.SaveVolume");
    assert_eq!(app.events[1], "__MinimizeFullScreenWindow(1234, 1920, 1080)");
}

#[test]
fn test_mouse_capture_double_click() {
    let mut app = AppProcedure::new();
    
    // First click
    app.current_time_ms = 1000;
    let lparam_down = ((200 << 16) | 100) as isize; // X=100, Y=200
    app.window_procedure(4321, WM_LBUTTONDOWN, 0, lparam_down);
    
    assert_eq!(app.events[0], "SetCapture(4321)");
    assert_eq!(app.events[1], "OnMouseLeftButtonDown(100, 200)");
    assert_eq!(app.l_button_down_time, 1000);
    assert_eq!(app.mouse_capture_ref, 1);
    
    app.events.clear();

    // Second click (Double click)
    app.current_time_ms = 1100; // Within 300ms
    let lparam_down_2 = ((202 << 16) | 102) as isize; // X=102, Y=202 (Within box of 5)
    app.window_procedure(4321, WM_LBUTTONDOWN, 0, lparam_down_2);

    assert_eq!(app.events[0], "SetCapture(4321)");
    assert_eq!(app.events[1], "OnMouseLeftButtonDoubleClick(102, 202)");
    assert_eq!(app.l_button_down_time, 0);
    assert_eq!(app.mouse_capture_ref, 2);

    app.events.clear();

    // Third click (Too late, outside time bounds)
    app.current_time_ms = 1500;
    app.window_procedure(4321, WM_LBUTTONDOWN, 0, lparam_down_2);
    assert_eq!(app.events[0], "SetCapture(4321)");
    assert_eq!(app.events[1], "OnMouseLeftButtonDown(102, 202)");
    assert_eq!(app.l_button_down_time, 1500);
    assert_eq!(app.mouse_capture_ref, 3);
}

#[test]
fn test_mouse_up_release_capture() {
    let mut app = AppProcedure::new();
    app.safe_set_capture(4321);
    app.events.clear();

    let lparam = ((200 << 16) | 100) as isize; 
    app.window_procedure(4321, WM_LBUTTONUP, 0, lparam);

    assert_eq!(app.events[0], "ReleaseCapture");
    assert_eq!(app.events[1], "OnMouseLeftButtonUp(100, 200)");
    assert_eq!(app.mouse_capture_ref, 0);
}

#[test]
fn test_resize_events() {
    let mut app = AppProcedure::new();
    let lparam = ((1080 << 16) | 1920) as isize; 
    
    app.window_procedure(0, WM_SIZE, SIZE_RESTORED, lparam);
    assert_eq!(app.events[0], "ResizeBackBuffer");
    assert_eq!(app.events[1], "OnSizeChange(1920, 1080)");
    assert_eq!(app.events[2], format!("CMSApplication::WindowProcedure(0, {}, {}, {})", WM_SIZE, SIZE_RESTORED, lparam));
    assert!(!app.is_minimized_wnd);

    app.events.clear();

    app.window_procedure(0, WM_SIZE, SIZE_MINIMIZED, lparam);
    assert_eq!(app.events[0], "OnSizeChange(1920, 1080)");
    assert!(app.is_minimized_wnd);
}

#[test]
fn test_ime_passthrough() {
    let mut app = AppProcedure::new();
    let ret = app.window_procedure(0, WM_CHAR, 65, 0);
    assert_eq!(ret, 1);
    assert_eq!(app.events[0], "CPythonIME::WMChar(65)");

    app.events.clear();
    let ret = app.window_procedure(0, WM_IME_SETCONTEXT, 0, 0xFFFFFFFFu32 as isize);
    let expected_lparam = 0xFFFFFFFFu32 as isize & !(ISC_SHOWUICOMPOSITIONWINDOW | ISC_SHOWUIALLCANDIDATEWINDOW);
    assert_eq!(ret, 0);
    assert_eq!(app.events[0], format!("WM_IME_SETCONTEXT(lParam={})", expected_lparam));
}

#[test]
fn test_setcursor() {
    let mut app = AppProcedure::new();
    app.is_active = true;
    app.cursor_visible = true;
    app.cursor_mode = CURSOR_MODE_HARDWARE;
    app.current_cursor = 999;

    let ret = app.window_procedure(0, WM_SETCURSOR, 0, 0);
    assert_eq!(ret, 0);
    assert_eq!(app.events[0], "SetCursor(999)");

    app.events.clear();
    app.cursor_visible = false;
    app.window_procedure(0, WM_SETCURSOR, 0, 0);
    assert_eq!(app.events[0], "SetCursor(NULL)");
}

#[test]
fn test_unhandled_msg() {
    let mut app = AppProcedure::new();
    let ret = app.window_procedure(777, 0x1234, 1, 2);
    assert_eq!(ret, 0);
    assert_eq!(app.events[0], "CMSApplication::WindowProcedure(777, 4660, 1, 2)");
}

#[test]
fn test_syskeys() {
    let mut app = AppProcedure::new();
    
    app.window_procedure(0, WM_SYSKEYDOWN, VK_F10, 0);
    assert_eq!(app.events[0], "VK_F10_DOWN");
    app.events.clear();

    let ret = app.window_procedure(0, WM_SYSKEYUP, 18, 0);
    assert_eq!(ret, 0);
    assert!(app.events.is_empty());
}
