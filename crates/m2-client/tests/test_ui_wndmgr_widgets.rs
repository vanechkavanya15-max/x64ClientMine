#[path = "../src/ui_wndmgr_widgets.rs"]
mod ui_wndmgr_widgets;

use ui_wndmgr_widgets::{EditLine, ScrollBar, ScrollBarOrientation, Button, ButtonState};

// ==========================================
// 1. EDITLINE TESTS
// ==========================================

#[test]
fn test_editline_insertion_and_deletion() {
    let mut el = EditLine::new(10, false);
    
    el.insert_char('H');
    el.insert_char('e');
    el.insert_char('l');
    el.insert_string("lo");
    
    assert_eq!(el.text, "Hello");
    assert_eq!(el.cursor_pos, 5);
    
    el.backspace();
    assert_eq!(el.text, "Hell");
    assert_eq!(el.cursor_pos, 4);
    
    el.move_cursor_left(false);
    el.move_cursor_left(false);
    assert_eq!(el.cursor_pos, 2);
    
    el.delete_char(); // deletes 'l' at index 2
    assert_eq!(el.text, "Hel");
    assert_eq!(el.cursor_pos, 2);
}

#[test]
fn test_editline_utf8() {
    let mut el = EditLine::new(10, false);
    el.insert_string("Zażółć");
    assert_eq!(el.text, "Zażółć");
    assert_eq!(el.cursor_pos, 6);
    
    el.backspace();
    assert_eq!(el.text, "Zażół");
    assert_eq!(el.cursor_pos, 5);
}

#[test]
fn test_editline_selection() {
    let mut el = EditLine::new(20, false);
    el.insert_string("Metin2");
    assert_eq!(el.cursor_pos, 6);
    
    // Select "in2"
    el.move_cursor_left(true);
    el.move_cursor_left(true);
    el.move_cursor_left(true);
    
    assert!(el.has_selection());
    assert_eq!(el.selection_start, Some(6));
    assert_eq!(el.cursor_pos, 3);
    
    el.delete_selection();
    assert_eq!(el.text, "Met");
    assert_eq!(el.cursor_pos, 3);
    assert!(!el.has_selection());
}

#[test]
fn test_editline_secret_mode() {
    let mut el = EditLine::new(20, true);
    el.insert_string("password");
    assert_eq!(el.get_visible_text(), "********");
}


// ==========================================
// 2. SCROLLBAR TESTS
// ==========================================

#[test]
fn test_scrollbar_basic_movement() {
    let mut sb = ScrollBar::new(ScrollBarOrientation::Vertical);
    sb.step = 0.1;
    
    assert_eq!(sb.get_pos(), 0.0);
    
    sb.scroll_down();
    assert_eq!(sb.get_pos(), 0.1);
    
    sb.handle_mouse_wheel(-1); // down
    assert!((sb.get_pos() - 0.2).abs() < f32::EPSILON);
    
    sb.handle_mouse_wheel(1); // up
    assert_eq!(sb.get_pos(), 0.1);
}

#[test]
fn test_scrollbar_clamping() {
    let mut sb = ScrollBar::new(ScrollBarOrientation::Vertical);
    sb.set_pos(-1.0);
    assert_eq!(sb.get_pos(), 0.0);
    
    sb.set_pos(2.0);
    assert_eq!(sb.get_pos(), 1.0);
}

#[test]
fn test_scrollbar_track_click() {
    let mut sb = ScrollBar::new(ScrollBarOrientation::Vertical);
    sb.track_length = 100.0;
    sb.thumb_length = 20.0;
    sb.page_step = 0.2;
    sb.set_pos(0.5); // movable = 80. thumb is at 40 (pixels 40-60)
    
    // Click above thumb
    sb.handle_track_click(20.0);
    assert!((sb.get_pos() - 0.3).abs() < f32::EPSILON);
    
    // Click below thumb
    sb.set_pos(0.5);
    sb.handle_track_click(70.0);
    assert!((sb.get_pos() - 0.7).abs() < f32::EPSILON);
    
    // Click on thumb (should do nothing)
    sb.set_pos(0.5);
    sb.handle_track_click(50.0);
    assert_eq!(sb.get_pos(), 0.5);
}

#[test]
fn test_scrollbar_dragging() {
    let mut sb = ScrollBar::new(ScrollBarOrientation::Vertical);
    sb.track_length = 100.0;
    sb.thumb_length = 20.0;
    sb.set_pos(0.0);
    
    sb.begin_drag(0.0);
    assert!(sb.is_dragging);
    
    // Move mouse 40 pixels down. Movable track is 80.
    // Pos delta should be 40/80 = 0.5
    sb.update_drag(40.0);
    assert!((sb.get_pos() - 0.5).abs() < f32::EPSILON);
    
    sb.end_drag();
    assert!(!sb.is_dragging);
}


// ==========================================
// 3. BUTTON TESTS
// ==========================================

#[test]
fn test_button_state_transitions() {
    let mut btn = Button::new();
    assert_eq!(btn.state, ButtonState::Normal);
    
    btn.handle_mouse_enter();
    assert_eq!(btn.state, ButtonState::Over);
    
    btn.handle_mouse_down();
    assert_eq!(btn.state, ButtonState::Down);
    
    // Mouse up over the button (click)
    let clicked = btn.handle_mouse_up(true);
    assert!(clicked);
    assert_eq!(btn.state, ButtonState::Over);
    
    btn.handle_mouse_leave();
    assert_eq!(btn.state, ButtonState::Normal);
}

#[test]
fn test_button_mouse_up_outside() {
    let mut btn = Button::new();
    btn.handle_mouse_enter();
    btn.handle_mouse_down();
    
    // Mouse up outside the button (no click)
    let clicked = btn.handle_mouse_up(false);
    assert!(!clicked);
    assert_eq!(btn.state, ButtonState::Normal);
}

#[test]
fn test_button_disable() {
    let mut btn = Button::new();
    btn.disable();
    assert!(btn.is_disabled());
    assert_eq!(btn.state, ButtonState::Disable);
    
    // Should not respond to events
    btn.handle_mouse_enter();
    assert_eq!(btn.state, ButtonState::Disable);
    
    // Must be explicitly enabled
    btn.enable();
    assert_eq!(btn.state, ButtonState::Normal);
}
