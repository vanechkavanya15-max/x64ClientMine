use m2_client::ui_window_mgr::{UiWindowMgr, WindowFlag};

#[test]
fn test_integration_ui_window_mgr_init() {
    let instance = UiWindowMgr::new();
    assert!(instance.is_ready());
    assert_eq!(instance.layer_window_list.len(), 5);
}

#[test]
fn test_register_and_pick_window() {
    let mut mgr = UiWindowMgr::new();
    mgr.set_resolution(800, 600);
    mgr.set_screen_size(800, 600);
    
    let win_id = mgr.register_window("UI").unwrap();
    if let Some(wnd) = mgr.windows.get_mut(&win_id) {
        wnd.x = 100;
        wnd.y = 100;
        wnd.width = 50;
        wnd.height = 50;
        wnd.is_show = true;
        wnd.is_rendering = true;
    }
    
    mgr.run_mouse_move(125, 125);
    assert_eq!(mgr.get_point_window(), Some(win_id));
}

#[test]
fn test_drag_window() {
    let mut mgr = UiWindowMgr::new();
    mgr.set_resolution(800, 600);
    mgr.set_screen_size(800, 600);
    
    let win_id = mgr.register_window("UI").unwrap();
    if let Some(wnd) = mgr.windows.get_mut(&win_id) {
        wnd.x = 100;
        wnd.y = 100;
        wnd.width = 50;
        wnd.height = 50;
        wnd.is_show = true;
        wnd.is_rendering = true;
        wnd.set_flag(WindowFlag::Movable, true);
    }
    
    mgr.run_mouse_move(125, 125); // Set point window
    
    // Simulate left button down on window
    mgr.run_mouse_left_button_down(125, 125);
    assert!(mgr.is_dragging());
    assert_eq!(mgr.left_capture_window, Some(win_id));
    assert_eq!(mgr.drag_x, 25);
    assert_eq!(mgr.drag_y, 25);
    
    // Move window
    mgr.run_mouse_move(200, 200);
    
    // The window should now be at global (175, 175) because the drag offset is (25, 25)
    if let Some(wnd) = mgr.windows.get(&win_id) {
        assert_eq!(wnd.x, 175);
        assert_eq!(wnd.y, 175);
    }
    
    // Release
    mgr.run_mouse_left_button_up(200, 200);
    assert!(!mgr.is_dragging());
}

#[test]
fn test_focus_and_lock() {
    let mut mgr = UiWindowMgr::new();
    
    let win_id = mgr.register_window("UI").unwrap();
    mgr.activate_window(win_id);
    assert!(mgr.is_focus());
    assert!(mgr.is_focus_window(win_id));
    
    mgr.deactivate_window();
    assert!(!mgr.is_focus());
    
    mgr.lock_window(win_id);
    assert!(mgr.is_focus());
    assert_eq!(mgr.get_lock_window(), Some(win_id));
    
    mgr.unlock_window();
    assert!(!mgr.is_focus());
}
