use m2_client::win32_cursor_manager::{CursorController, CursorType};

#[test]
fn test_initial_state() {
    let controller = CursorController::new();
    assert_eq!(controller.get_current_cursor(), CursorType::Normal);
    assert_eq!(controller.is_cursor_visible(), true);
}

#[test]
fn test_set_cursor() {
    let mut controller = CursorController::new();
    
    controller.set_cursor(CursorType::Attack);
    assert_eq!(controller.get_current_cursor(), CursorType::Attack);

    controller.set_cursor(CursorType::Talk);
    assert_eq!(controller.get_current_cursor(), CursorType::Talk);
}

#[test]
fn test_visibility_toggle() {
    let mut controller = CursorController::new();
    assert_eq!(controller.is_cursor_visible(), true);

    controller.hide_cursor();
    assert_eq!(controller.is_cursor_visible(), false);

    controller.show_cursor();
    assert_eq!(controller.is_cursor_visible(), true);
}

#[test]
fn test_hide_already_hidden() {
    let mut controller = CursorController::new();
    
    controller.hide_cursor();
    assert_eq!(controller.is_cursor_visible(), false);
    
    // Calling hide again should keep it hidden and not panic
    controller.hide_cursor();
    assert_eq!(controller.is_cursor_visible(), false);
}

#[test]
fn test_camera_rotate_logic_simple() {
    let mut controller = CursorController::new();
    
    // Start with a specific cursor
    controller.set_cursor(CursorType::Target);
    assert_eq!(controller.get_current_cursor(), CursorType::Target);
    assert_eq!(controller.is_cursor_visible(), true);

    // Begin camera rotate
    controller.begin_camera_rotate();
    
    // Cursor should now be CameraRotate and hidden
    assert_eq!(controller.get_current_cursor(), CursorType::CameraRotate);
    assert_eq!(controller.is_cursor_visible(), false);

    // End camera rotate
    controller.end_camera_rotate();

    // Cursor should be restored to Target and visible
    assert_eq!(controller.get_current_cursor(), CursorType::Target);
    assert_eq!(controller.is_cursor_visible(), true);
}

#[test]
fn test_camera_rotate_logic_hidden_cursor() {
    let mut controller = CursorController::new();
    
    // Start with a specific cursor and hide it
    controller.set_cursor(CursorType::Pick);
    controller.hide_cursor();
    
    assert_eq!(controller.get_current_cursor(), CursorType::Pick);
    assert_eq!(controller.is_cursor_visible(), false);

    // Begin camera rotate
    controller.begin_camera_rotate();
    
    // Cursor should now be CameraRotate and still hidden
    assert_eq!(controller.get_current_cursor(), CursorType::CameraRotate);
    assert_eq!(controller.is_cursor_visible(), false);

    // End camera rotate
    controller.end_camera_rotate();

    // Cursor should be restored to Pick and stay hidden
    assert_eq!(controller.get_current_cursor(), CursorType::Pick);
    assert_eq!(controller.is_cursor_visible(), false);
}

#[test]
fn test_default_trait() {
    let controller = CursorController::default();
    assert_eq!(controller.get_current_cursor(), CursorType::Normal);
    assert_eq!(controller.is_cursor_visible(), true);
}

#[test]
fn test_cursor_type_equality() {
    assert_eq!(CursorType::Normal, CursorType::Normal);
    assert_ne!(CursorType::Normal, CursorType::Attack);
    
    // Testing copy/clone
    let cursor = CursorType::Magic;
    let cursor_copy = cursor;
    assert_eq!(cursor, cursor_copy);
}
