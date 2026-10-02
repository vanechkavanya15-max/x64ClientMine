use m2_client::ui_widget_editline_ime::ImeEditLineWidget;

#[test]
fn test_editline_basic_typing() {
    let mut editline = ImeEditLineWidget::new(32, false);
    editline.insert_string("Hello");
    assert_eq!(editline.text, "Hello");
    assert_eq!(editline.cursor_pos, 5);
}

#[test]
fn test_editline_cursor_movement() {
    let mut editline = ImeEditLineWidget::new(32, false);
    editline.insert_string("Test");
    editline.move_cursor_left(false);
    assert_eq!(editline.cursor_pos, 3);
    editline.insert_char('a');
    assert_eq!(editline.text, "Tesat");
    editline.move_cursor_home(false);
    assert_eq!(editline.cursor_pos, 0);
    editline.move_cursor_end(false);
    assert_eq!(editline.cursor_pos, 5);
}

#[test]
fn test_editline_delete_backspace() {
    let mut editline = ImeEditLineWidget::new(32, false);
    editline.insert_string("World");
    editline.move_cursor_left(false);
    editline.backspace();
    assert_eq!(editline.text, "Word");
    assert_eq!(editline.cursor_pos, 3);

    editline.move_cursor_home(false);
    editline.delete_char();
    assert_eq!(editline.text, "ord");
    assert_eq!(editline.cursor_pos, 0);
}

#[test]
fn test_editline_selection() {
    let mut editline = ImeEditLineWidget::new(32, false);
    editline.insert_string("Selection");
    editline.select_all();
    assert!(editline.has_selection());
    assert_eq!(editline.selection_start, 0);
    assert_eq!(editline.selection_end, 9);

    editline.delete_selection();
    assert_eq!(editline.text, "");
    assert_eq!(editline.cursor_pos, 0);
    assert!(!editline.has_selection());
}

#[test]
fn test_editline_clipboard_mock() {
    let mut editline = ImeEditLineWidget::new(32, false);
    editline.insert_string("CopyPaste");
    editline.select_all();

    // Ctrl+C
    editline.on_wm_char(3, true, false);

    editline.move_cursor_end(false);

    // Ctrl+V
    editline.on_wm_char(22, true, false);
    assert_eq!(editline.text, "CopyPasteCopyPaste");
}

#[test]
fn test_editline_secret_mode() {
    let mut editline = ImeEditLineWidget::new(32, true);
    editline.insert_string("Secret");
    assert_eq!(editline.get_visible_text(), "******");

    // Attempt copy in secret mode
    editline.select_all();
    editline.on_wm_char(3, true, false);

    // Clear and paste to verify nothing was copied
    editline.clear();
    editline.on_wm_char(22, true, false);
    assert_eq!(editline.text, "");
}

#[test]
fn test_editline_number_only() {
    let mut editline = ImeEditLineWidget::new(32, false);
    editline.set_number_mode(true);
    editline.insert_string("123abc456");
    assert_eq!(editline.text, "123456");
}

#[test]
fn test_editline_max_len() {
    let mut editline = ImeEditLineWidget::new(3, false);
    editline.insert_string("1234");
    assert_eq!(editline.text, "123");
}
