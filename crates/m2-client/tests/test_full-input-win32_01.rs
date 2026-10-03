use m2_client::ui_window_core::{Window, WindowType};
use m2_client::ui_window_mgr::get_window_manager;
use m2_client::win32_message_dispatcher::input_mouse_lbutton_dispatcher;

#[test]
fn test_dispatch_lbutton_down_no_windows() {
    let mut wm = get_window_manager();
    wm.windows.clear();
    wm.root_windows.clear();
    wm.capture_window = None;
    wm.active_window = None;

    let hit = input_mouse_lbutton_dispatcher::dispatch_wm_lbutton_down(100, 100, 1024, 768);
    assert_eq!(hit, None);
    assert_eq!(wm.capture_window, None);
    assert_eq!(wm.active_window, None);
}

#[test]
fn test_dispatch_lbutton_down_hit_window() {
    let mut wm = get_window_manager();
    wm.windows.clear();
    wm.root_windows.clear();
    wm.capture_window = None;
    wm.active_window = None;

    let mut win = Window::new(1, WindowType::Normal);
    win.local_x = 50;
    win.local_y = 50;
    win.width = 100;
    win.height = 100;
    win.is_show = true;
    
    wm.windows.insert(1, win);
    wm.root_windows.push(1);

    let hit = input_mouse_lbutton_dispatcher::dispatch_wm_lbutton_down(100, 100, 1024, 768);
    assert_eq!(hit, Some(1));
    assert_eq!(wm.capture_window, Some(1));
    assert_eq!(wm.active_window, Some(1));
    
    let w = wm.windows.get(&1).unwrap();
    assert!(w.is_pressed);
}

#[test]
fn test_dispatch_lbutton_up_no_windows() {
    let mut wm = get_window_manager();
    wm.windows.clear();
    wm.root_windows.clear();
    wm.capture_window = None;
    wm.active_window = None;

    let (hit, name) = input_mouse_lbutton_dispatcher::dispatch_wm_lbutton_up(100, 100, 1024, 768);
    assert_eq!(hit, None);
    assert_eq!(name, "");
    assert_eq!(wm.capture_window, None);
}

#[test]
fn test_dispatch_lbutton_up_hit_window() {
    let mut wm = get_window_manager();
    wm.windows.clear();
    wm.root_windows.clear();
    
    let mut win = Window::new(1, WindowType::Normal);
    win.name = "TestWindow".to_string();
    win.local_x = 50;
    win.local_y = 50;
    win.width = 100;
    win.height = 100;
    win.is_show = true;
    win.is_pressed = true; // should be cleared
    
    wm.windows.insert(1, win);
    wm.root_windows.push(1);
    
    wm.capture_window = Some(1);
    wm.active_window = Some(1);

    let (hit, name) = input_mouse_lbutton_dispatcher::dispatch_wm_lbutton_up(100, 100, 1024, 768);
    
    assert_eq!(hit, Some(1));
    assert_eq!(name, "TestWindow");
    assert_eq!(wm.capture_window, None);
    
    let w = wm.windows.get(&1).unwrap();
    assert!(!w.is_pressed);
}

#[test]
fn test_dispatch_lbutton_up_miss_captured_window() {
    let mut wm = get_window_manager();
    wm.windows.clear();
    wm.root_windows.clear();
    
    let mut win = Window::new(1, WindowType::Normal);
    win.name = "TestWindow".to_string();
    win.local_x = 50;
    win.local_y = 50;
    win.width = 100;
    win.height = 100;
    win.is_show = true;
    win.is_pressed = true;
    
    wm.windows.insert(1, win);
    wm.root_windows.push(1);
    
    // Capture window 1, but mouse up is outside the window (mx=500, my=500)
    wm.capture_window = Some(1);
    
    let (hit, name) = input_mouse_lbutton_dispatcher::dispatch_wm_lbutton_up(500, 500, 1024, 768);
    
    // Hit is None because mouse up is outside any window
    assert_eq!(hit, None);
    assert_eq!(name, "");
    assert_eq!(wm.capture_window, None);
    
    let w = wm.windows.get(&1).unwrap();
    assert!(!w.is_pressed);
}
