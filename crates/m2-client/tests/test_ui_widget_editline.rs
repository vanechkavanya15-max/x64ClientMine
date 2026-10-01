use m2_client::ui_widget_editline::EditLineWidget;

#[test]
fn test_editline_insert_char() {
    let mut widget = EditLineWidget::new(10, false);
    widget.insert_char('A');
    widget.insert_char('B');
    widget.insert_char('C');

    assert_eq!(widget.text, "ABC");
    assert_eq!(widget.cursor_pos, 3);
}

#[test]
fn test_editline_backspace() {
    let mut widget = EditLineWidget::new(10, false);
    widget.insert_char('A');
    widget.insert_char('B');
    widget.insert_char('C');

    widget.backspace();
    assert_eq!(widget.text, "AB");
    assert_eq!(widget.cursor_pos, 2);

    widget.move_cursor_left();
    widget.backspace();
    assert_eq!(widget.text, "B");
    assert_eq!(widget.cursor_pos, 0);
}

#[test]
fn test_editline_delete_char() {
    let mut widget = EditLineWidget::new(10, false);
    widget.insert_char('A');
    widget.insert_char('B');
    widget.insert_char('C');

    widget.move_cursor_home();
    widget.delete_char();
    assert_eq!(widget.text, "BC");
    assert_eq!(widget.cursor_pos, 0);

    widget.move_cursor_right();
    widget.delete_char();
    assert_eq!(widget.text, "B");
    assert_eq!(widget.cursor_pos, 1);
}

#[test]
fn test_editline_cursor_movement() {
    let mut widget = EditLineWidget::new(10, false);
    widget.insert_char('1');
    widget.insert_char('2');
    widget.insert_char('3');

    widget.move_cursor_left();
    assert_eq!(widget.cursor_pos, 2);

    widget.move_cursor_home();
    assert_eq!(widget.cursor_pos, 0);

    widget.move_cursor_right();
    assert_eq!(widget.cursor_pos, 1);

    widget.move_cursor_end();
    assert_eq!(widget.cursor_pos, 3);
}

#[test]
fn test_editline_max_length() {
    let mut widget = EditLineWidget::new(3, false);
    widget.insert_char('1');
    widget.insert_char('2');
    widget.insert_char('3');
    widget.insert_char('4'); // Should not insert

    assert_eq!(widget.text, "123");
    assert_eq!(widget.cursor_pos, 3);
}

#[test]
fn test_editline_secret_masking() {
    let mut widget = EditLineWidget::new(10, true);
    widget.insert_char('P');
    widget.insert_char('A');
    widget.insert_char('S');
    widget.insert_char('S');

    assert_eq!(widget.text, "PASS");
    assert_eq!(widget.get_visible_text(), "****");
}

#[test]
fn test_editline_clear() {
    let mut widget = EditLineWidget::new(10, false);
    widget.insert_char('A');
    widget.insert_char('B');
    
    widget.clear();
    assert_eq!(widget.text, "");
    assert_eq!(widget.cursor_pos, 0);
}

#[test]
fn test_editline_insert_middle() {
    let mut widget = EditLineWidget::new(10, false);
    widget.insert_char('A');
    widget.insert_char('C');
    
    widget.move_cursor_left();
    widget.insert_char('B');

    assert_eq!(widget.text, "ABC");
    assert_eq!(widget.cursor_pos, 2);
}
