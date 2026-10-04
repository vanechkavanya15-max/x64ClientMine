use m2_client::ui_window_mgr::WindowManager;
use m2_client::ui_window_core::{Window, WindowType};

#[test]
fn test_set_focus_kill_focus() {
    let mut wm = WindowManager::new();
    let win1 = Window::new(1, WindowType::Normal);
    let win2 = Window::new(2, WindowType::Normal);
    
    wm.windows.insert(1, win1);
    wm.windows.insert(2, win2);
    
    assert_eq!(wm.active_window, None);
    
    wm.set_focus(1);
    assert_eq!(wm.active_window, Some(1));
    assert!(wm.is_focus(1));
    assert!(!wm.is_focus(2));
    
    wm.set_focus(2);
    assert_eq!(wm.active_window, Some(2));
    assert!(!wm.is_focus(1));
    assert!(wm.is_focus(2));
    
    wm.kill_focus(1); // Should do nothing since 2 is active
    assert_eq!(wm.active_window, Some(2));
    
    wm.kill_focus(2);
    assert_eq!(wm.active_window, None);
}

#[test]
fn test_lock_unlock_window() {
    let mut wm = WindowManager::new();
    let win1 = Window::new(1, WindowType::Normal);
    let win2 = Window::new(2, WindowType::Normal);
    
    wm.windows.insert(1, win1);
    wm.windows.insert(2, win2);
    wm.root_windows.push(1);
    wm.root_windows.push(2);
    
    assert_eq!(wm.capture_window, None);
    
    wm.lock_window(1);
    assert_eq!(wm.capture_window, Some(1));
    
    // Test hitting outside modal
    // Since 1 is captured, hitting 2 should return 1
    // (We need to set up rects so they are hit)
    // Actually we can just pass an empty hit and it should return the captured window
    assert_eq!(wm.find_window_at(999, 999, 800, 600), Some(1));
    
    wm.unlock_window(2); // Should do nothing
    assert_eq!(wm.capture_window, Some(1));
    
    wm.unlock_window(1);
    assert_eq!(wm.capture_window, None);
}

#[test]
fn test_find_window_at() {
    let mut wm = WindowManager::new();
    
    let mut root_win = Window::new(10, WindowType::Normal);
    root_win.local_x = 0;
    root_win.local_y = 0;
    root_win.width = 100;
    root_win.height = 100;
    root_win.children.push(11);
    
    let mut child_win = Window::new(11, WindowType::Normal);
    child_win.local_x = 10;
    child_win.local_y = 10;
    child_win.width = 50;
    child_win.height = 50;
    
    wm.windows.insert(10, root_win);
    wm.windows.insert(11, child_win);
    wm.root_windows.push(10);
    
    // Test point inside child
    assert_eq!(wm.find_window_at(20, 20, 800, 600), Some(11));
    
    // Test point inside root but outside child
    assert_eq!(wm.find_window_at(80, 80, 800, 600), Some(10));
    
    // Test point outside both
    assert_eq!(wm.find_window_at(150, 150, 800, 600), None);
}

#[test]
fn test_attach_deattach_icon() {
    let mut wm = WindowManager::new();
    
    assert!(!wm.is_attaching);
    
    wm.attach_icon(1, 42, 5, 32, 32);
    assert!(wm.is_attaching);
    assert_eq!(wm.get_attaching_type(), 1); 
    assert_eq!(wm.get_attaching_index(), 42);
    assert_eq!(wm.get_attaching_slot_number(), 5);
    
    wm.deattach_icon();
    assert!(!wm.is_attaching);
    assert_eq!(wm.get_attaching_type(), 0);
    assert_eq!(wm.get_attaching_index(), 0);
}
