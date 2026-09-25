use m2_client::app_window::{AppWindow, SM_CXSCREEN, SM_CYSCREEN};

#[test]
fn test_integration_app_window() {
    let instance = AppWindow::new();
    assert!(instance.is_ready());
}

#[test]
fn test_app_window_creation_windowed() {
    let mut window = AppWindow::new();
    let success = window.create("Metin2", 800, 600, true);
    
    assert!(success);
    assert_eq!(window.title, "Metin2");
    assert!(window.is_windowed);
    assert!(!window.is_window_fullscreen_enable);
    
    // Test that the mock adjust_size worked for windowed
    assert_eq!(window.width, 816); // 800 + 16
    assert_eq!(window.height, 639); // 600 + 39
    
    // Position should be offset
    assert_eq!(window.x, -8);
    assert_eq!(window.y, 0);
}

#[test]
fn test_app_window_creation_fullscreen() {
    let mut window = AppWindow::new();
    // Simulate non-windowed creation (which triggers fullscreen by default based on IME mock logic)
    let success = window.create("Metin2 FS", 1024, 768, false);
    
    assert!(success);
    assert_eq!(window.title, "Metin2 FS");
    assert!(!window.is_windowed);
    assert!(window.is_window_fullscreen_enable);
}

#[test]
fn test_app_window_visibility_and_activity() {
    let mut window = AppWindow::new();
    window.create("Test", 800, 600, true);
    
    window.show();
    assert!(window.is_visible());
    assert!(window.is_active());
    
    window.hide();
    assert!(!window.is_visible());
    assert!(!window.is_active());
    
    window.set_visible_mode(true);
    assert!(window.is_visible());
}

#[test]
fn test_app_window_size_event() {
    let mut window = AppWindow::new();
    
    window.on_size(true); // Minimized
    assert!(!window.is_active());
    assert!(!window.is_visible());
    
    window.on_size(false); // Restored
    assert!(window.is_active());
    assert!(window.is_visible());
}

#[test]
fn test_app_window_set_center_position() {
    let mut window = AppWindow::new();
    // Use an exact size, avoiding adjust_size by setting it manually after create
    window.create("Center", 800, 600, true);
    window.set_size(800, 600); 
    
    window.set_center_position();
    
    let expected_x = (SM_CXSCREEN - 800) / 2;
    let expected_y = (SM_CYSCREEN - 600) / 2;
    
    assert_eq!(window.x, expected_x);
    assert_eq!(window.y, expected_y);
}

#[test]
fn test_app_window_destroy() {
    let mut window = AppWindow::new();
    window.create("Destroy", 800, 600, true);
    window.show();
    
    window.destroy();
    assert!(!window.is_visible());
    assert!(!window.is_active());
    assert!(!window.is_ready());
}
