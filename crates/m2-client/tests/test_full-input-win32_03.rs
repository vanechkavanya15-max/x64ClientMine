use m2_client::ui_window_mgr::WindowManager;
use m2_client::wgpu_camera_orbit_controller::CameraOrbitController;
use m2_client::ui_window_core::{Window, WindowType};
use m2_client::py_event_dispatcher;

#[path = "../src/input_mouse_wheel_dispatcher.rs"]
mod input_mouse_wheel_dispatcher;

#[test]
fn test_mouse_wheel_no_ui_zoom_in() {
    let mut ui_mgr = WindowManager::new();
    let mut camera = CameraOrbitController::new();
    
    // Zoom initially to see effect
    camera.set_distance(1000.0);
    
    // delta = 120 (positive, zoom in)
    // 120 << 16 = 0x00780000
    let wparam: usize = 120 << 16;
    
    let handled = input_mouse_wheel_dispatcher::dispatch_mouse_wheel(&mut ui_mgr, &mut camera, wparam);
    
    assert!(handled);
    // Should zoom in (decrease distance by 10%)
    // 1000.0 * 0.9 = 900.0
    assert!((camera.distance - 900.0).abs() < 0.1, "Camera distance should be 900.0, was {}", camera.distance);
}

#[test]
fn test_mouse_wheel_no_ui_zoom_out() {
    let mut ui_mgr = WindowManager::new();
    let mut camera = CameraOrbitController::new();
    
    camera.set_distance(1000.0);
    
    // delta = -120 (negative, zoom out)
    // -120 is 0xFF88 in i16
    let wparam: usize = 0xFF880000;
    
    let handled = input_mouse_wheel_dispatcher::dispatch_mouse_wheel(&mut ui_mgr, &mut camera, wparam);
    
    assert!(handled);
    // Should zoom out (increase distance by ~11%)
    // 1000.0 * 1.11111 = 1111.11
    assert!((camera.distance - 1111.11).abs() < 1.0, "Camera distance should be ~1111.1, was {}", camera.distance);
}

#[test]
fn test_mouse_wheel_ui_intercepts_zoom_in() {
    py_event_dispatcher::clear_call_log();
    
    let mut ui_mgr = WindowManager::new();
    let mut camera = CameraOrbitController::new();
    camera.set_distance(1000.0);
    
    let mut win = Window::new(101, WindowType::Normal);
    win.py_handler = 12345;
    ui_mgr.windows.insert(101, win);
    ui_mgr.point_window = Some(101);
    
    let wparam: usize = 120 << 16;
    
    let handled = input_mouse_wheel_dispatcher::dispatch_mouse_wheel(&mut ui_mgr, &mut camera, wparam);
    
    assert!(handled);
    
    // Since UI intercepted, distance should remain 1000.0
    assert!((camera.distance - 1000.0).abs() < 0.001);
    
    // Check if python handler was called via py_event_dispatcher
    let log = py_event_dispatcher::get_call_log();
    // In mock mode, this will record the call
    let found = log.iter().any(|msg| msg.contains("obj_12345.OnRunMouseWheel(120)"));
    assert!(found, "Should have dispatched to OnRunMouseWheel with delta 120");
}

#[test]
fn test_mouse_wheel_ui_intercepts_zoom_out() {
    py_event_dispatcher::clear_call_log();
    
    let mut ui_mgr = WindowManager::new();
    let mut camera = CameraOrbitController::new();
    camera.set_distance(1000.0);
    
    let mut win = Window::new(102, WindowType::ListBox);
    win.py_handler = 67890;
    ui_mgr.windows.insert(102, win);
    ui_mgr.point_window = Some(102);
    
    let wparam: usize = 0xFF880000; // -120
    
    let handled = input_mouse_wheel_dispatcher::dispatch_mouse_wheel(&mut ui_mgr, &mut camera, wparam);
    
    assert!(handled);
    
    // Since UI intercepted, distance should remain 1000.0
    assert!((camera.distance - 1000.0).abs() < 0.001);
    
    // Check if python handler was called
    let log = py_event_dispatcher::get_call_log();
    let found = log.iter().any(|msg| msg.contains("obj_67890.OnRunMouseWheel(-120)"));
    assert!(found, "Should have dispatched to OnRunMouseWheel with delta -120");
}

#[test]
fn test_mouse_wheel_ui_hovered_but_no_handler() {
    py_event_dispatcher::clear_call_log();
    
    let mut ui_mgr = WindowManager::new();
    let mut camera = CameraOrbitController::new();
    camera.set_distance(1000.0);
    
    let mut win = Window::new(103, WindowType::Normal);
    win.py_handler = 0; // No handler
    ui_mgr.windows.insert(103, win);
    ui_mgr.point_window = Some(103);
    
    let wparam: usize = 120 << 16;
    
    let handled = input_mouse_wheel_dispatcher::dispatch_mouse_wheel(&mut ui_mgr, &mut camera, wparam);
    
    assert!(handled);
    
    // Since UI has no py_handler, it shouldn't intercept and camera should zoom
    assert!((camera.distance - 900.0).abs() < 0.1);
}
