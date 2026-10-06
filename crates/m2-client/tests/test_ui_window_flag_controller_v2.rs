#[path = "../src/ui_window_core.rs"]
mod ui_window_core;

#[path = "../src/ui_widget_slotwindow.rs"]
mod ui_widget_slotwindow;

#[path = "../src/ui_window_flag_controller_v2.rs"]
mod ui_window_flag_controller_v2;

use ui_window_core::{Window, WindowType};
use ui_window_flag_controller_v2::*;

#[test]
fn test_flag_constants() {
    assert_eq!(FLAG_MOVABLE, 1 << 0);
    assert_eq!(FLAG_LIMIT, 1 << 1);
    assert_eq!(FLAG_SNAP, 1 << 2);
    assert_eq!(FLAG_DRAGABLE, 1 << 3);
    assert_eq!(FLAG_ATTACH, 1 << 4);
    assert_eq!(FLAG_RESTRICT_X, 1 << 5);
    assert_eq!(FLAG_RESTRICT_Y, 1 << 6);
    assert_eq!(FLAG_NOT_CAPTURE, 1 << 7);
    assert_eq!(FLAG_FLOAT, 1 << 8);
    assert_eq!(FLAG_NOT_PICK, 1 << 9);
    assert_eq!(FLAG_IGNORE_SIZE, 1 << 10);
    assert_eq!(FLAG_RTL, 1 << 11);
}

#[test]
fn test_is_flag() {
    let mut win = Window::new(1, WindowType::Normal);
    assert_eq!(is_flag(&win, FLAG_MOVABLE), false);
    win.flags |= FLAG_MOVABLE;
    assert_eq!(is_flag(&win, FLAG_MOVABLE), true);
    assert_eq!(is_flag(&win, FLAG_NOT_PICK), false);
}

#[test]
fn test_add_flag() {
    let mut win = Window::new(1, WindowType::Normal);
    add_flag(&mut win, FLAG_NOT_PICK);
    assert_eq!(win.flags, FLAG_NOT_PICK);
    assert_eq!(is_flag(&win, FLAG_NOT_PICK), true);

    add_flag(&mut win, FLAG_DRAGABLE);
    assert_eq!(win.flags, FLAG_NOT_PICK | FLAG_DRAGABLE);
    assert_eq!(is_flag(&win, FLAG_DRAGABLE), true);
}

#[test]
fn test_remove_flag() {
    let mut win = Window::new(1, WindowType::Normal);
    win.flags = FLAG_NOT_PICK | FLAG_DRAGABLE | FLAG_MOVABLE;

    remove_flag(&mut win, FLAG_DRAGABLE);
    assert_eq!(win.flags, FLAG_NOT_PICK | FLAG_MOVABLE);
    assert_eq!(is_flag(&win, FLAG_DRAGABLE), false);
    assert_eq!(is_flag(&win, FLAG_NOT_PICK), true);
}

#[test]
fn test_parse_and_add_flag_string() {
    let mut win = Window::new(1, WindowType::Normal);
    
    parse_and_add_flag_string(&mut win, "movable");
    assert_eq!(is_flag(&win, FLAG_MOVABLE), true);

    parse_and_add_flag_string(&mut win, "NOT_PICK");
    assert_eq!(is_flag(&win, FLAG_NOT_PICK), true);

    parse_and_add_flag_string(&mut win, "Rtl");
    assert_eq!(is_flag(&win, FLAG_RTL), true);

    parse_and_add_flag_string(&mut win, "ltr");
    assert_eq!(is_flag(&win, FLAG_RTL), false);
    
    parse_and_add_flag_string(&mut win, "unknown_flag");
    // Should not panic, should just ignore
}
