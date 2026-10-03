use m2_client::ui_widget_editline::EditLineWidget;
use m2_client::input_char_composition_handler::InputCharCompositionHandler;

#[test]
fn test_insert_printable_char() {
    let mut edit_line = EditLineWidget::new(10, false);
    let handled = InputCharCompositionHandler::handle_wm_char(&mut edit_line, 'a' as u32);
    assert!(handled);
    assert_eq!(edit_line.text, "a");
    assert_eq!(edit_line.cursor_pos, 1);
}

#[test]
fn test_insert_max_length() {
    let mut edit_line = EditLineWidget::new(3, false);
    InputCharCompositionHandler::handle_wm_char(&mut edit_line, 'a' as u32);
    InputCharCompositionHandler::handle_wm_char(&mut edit_line, 'b' as u32);
    InputCharCompositionHandler::handle_wm_char(&mut edit_line, 'c' as u32);
    
    // Attempt to insert past max length
    let handled = InputCharCompositionHandler::handle_wm_char(&mut edit_line, 'd' as u32);
    assert!(handled); // Still returns true as it handles the character, even if it doesn't insert
    assert_eq!(edit_line.text, "abc");
    assert_eq!(edit_line.cursor_pos, 3);
}

#[test]
fn test_backspace() {
    let mut edit_line = EditLineWidget::new(10, false);
    InputCharCompositionHandler::handle_wm_char(&mut edit_line, 'a' as u32);
    InputCharCompositionHandler::handle_wm_char(&mut edit_line, 'b' as u32);
    assert_eq!(edit_line.text, "ab");
    
    let handled = InputCharCompositionHandler::handle_wm_char(&mut edit_line, '\x08' as u32);
    assert!(handled);
    assert_eq!(edit_line.text, "a");
    assert_eq!(edit_line.cursor_pos, 1);
}

#[test]
fn test_backspace_on_empty() {
    let mut edit_line = EditLineWidget::new(10, false);
    let handled = InputCharCompositionHandler::handle_wm_char(&mut edit_line, '\x08' as u32);
    assert!(handled);
    assert_eq!(edit_line.text, "");
    assert_eq!(edit_line.cursor_pos, 0);
}

#[test]
fn test_ignore_control_chars() {
    let mut edit_line = EditLineWidget::new(10, false);
    // Enter key
    let handled = InputCharCompositionHandler::handle_wm_char(&mut edit_line, '\r' as u32);
    assert!(!handled);
    assert_eq!(edit_line.text, "");
    
    // Escape key
    let handled = InputCharCompositionHandler::handle_wm_char(&mut edit_line, '\x1B' as u32);
    assert!(!handled);
    assert_eq!(edit_line.text, "");
}

#[test]
fn test_multibyte_chars() {
    let mut edit_line = EditLineWidget::new(10, false);
    // Polish character 'ą'
    let handled = InputCharCompositionHandler::handle_wm_char(&mut edit_line, 'ą' as u32);
    assert!(handled);
    assert_eq!(edit_line.text, "ą");
    assert_eq!(edit_line.cursor_pos, 1);
    
    // Japanese character 'あ'
    let handled = InputCharCompositionHandler::handle_wm_char(&mut edit_line, 'あ' as u32);
    assert!(handled);
    assert_eq!(edit_line.text, "ąあ");
    assert_eq!(edit_line.cursor_pos, 2);
}
