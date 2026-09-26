use m2_client::window_event_loop::{InputState, MouseButton, WindowEvent};

#[test]
fn test_key_press_and_release() {
    let mut state = InputState::new();

    // Test a valid key
    state.process_event(WindowEvent::KeyDown(65)); // 'A' key
    assert!(state.is_key_down(65));
    assert!(!state.is_key_down(66));

    state.process_event(WindowEvent::KeyUp(65));
    assert!(!state.is_key_down(65));

    // Test out of bounds key (should not panic and should return false)
    state.process_event(WindowEvent::KeyDown(300));
    assert!(!state.is_key_down(300));

    state.process_event(WindowEvent::KeyUp(300));
    assert!(!state.is_key_down(300));
}

#[test]
fn test_mouse_position_and_delta() {
    let mut state = InputState::new();

    // Initial state
    let (dx, dy) = state.consume_mouse_delta();
    assert_eq!((dx, dy), (0, 0));

    // Move to (100, 150)
    state.process_event(WindowEvent::MouseMove(100, 150));
    assert_eq!(state.mouse_x, 100);
    assert_eq!(state.mouse_y, 150);

    let (dx, dy) = state.consume_mouse_delta();
    assert_eq!((dx, dy), (100, 150));

    // Move further to (120, 130)
    state.process_event(WindowEvent::MouseMove(120, 130));
    assert_eq!(state.mouse_x, 120);
    assert_eq!(state.mouse_y, 130);

    let (dx, dy) = state.consume_mouse_delta();
    assert_eq!((dx, dy), (20, -20));

    // Delta should be reset
    let (dx, dy) = state.consume_mouse_delta();
    assert_eq!((dx, dy), (0, 0));
}

#[test]
fn test_mouse_buttons() {
    let mut state = InputState::new();

    state.process_event(WindowEvent::MouseDown(MouseButton::Left, 10, 10));
    assert!(state.mouse_left_down);
    assert!(!state.mouse_right_down);
    assert!(!state.mouse_middle_down);
    // Mouse position should update too
    assert_eq!(state.mouse_x, 10);
    assert_eq!(state.mouse_y, 10);
    let (dx, dy) = state.consume_mouse_delta();
    assert_eq!((dx, dy), (10, 10));

    state.process_event(WindowEvent::MouseDown(MouseButton::Right, 20, 20));
    assert!(state.mouse_left_down);
    assert!(state.mouse_right_down);
    let (dx, dy) = state.consume_mouse_delta();
    assert_eq!((dx, dy), (10, 10));

    state.process_event(WindowEvent::MouseUp(MouseButton::Left, 20, 20));
    assert!(!state.mouse_left_down);
    assert!(state.mouse_right_down);
    let (dx, dy) = state.consume_mouse_delta();
    assert_eq!((dx, dy), (0, 0)); // No movement
}

#[test]
fn test_mouse_wheel() {
    let mut state = InputState::new();

    assert_eq!(state.consume_scroll_delta(), 0);

    state.process_event(WindowEvent::MouseWheel(120));
    state.process_event(WindowEvent::MouseWheel(-60));
    
    assert_eq!(state.consume_scroll_delta(), 60);
    assert_eq!(state.consume_scroll_delta(), 0);
}

#[test]
fn test_resize() {
    let mut state = InputState::new();
    assert_eq!(state.window_width, 800);
    assert_eq!(state.window_height, 600);

    state.process_event(WindowEvent::Resize(1920, 1080));
    assert_eq!(state.window_width, 1920);
    assert_eq!(state.window_height, 1080);
}

#[test]
fn test_focus_changed() {
    let mut state = InputState::new();
    
    // Set some keys and buttons
    state.process_event(WindowEvent::KeyDown(32)); // Space
    state.process_event(WindowEvent::MouseDown(MouseButton::Left, 50, 50));
    
    assert!(state.is_key_down(32));
    assert!(state.mouse_left_down);
    assert!(state.is_focused);

    // Lose focus
    state.process_event(WindowEvent::FocusChanged(false));
    
    assert!(!state.is_focused);
    assert!(!state.is_key_down(32));
    assert!(!state.mouse_left_down);
    
    // Make sure we can gain focus again
    state.process_event(WindowEvent::FocusChanged(true));
    assert!(state.is_focused);
    assert!(!state.is_key_down(32)); // Key should still be clear
}

#[test]
fn test_close_requested() {
    let mut state = InputState::new();
    assert!(!state.close_requested);
    
    state.process_event(WindowEvent::CloseRequested);
    assert!(state.close_requested);
}
