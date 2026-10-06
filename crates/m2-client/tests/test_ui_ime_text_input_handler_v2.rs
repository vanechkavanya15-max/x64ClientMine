#[path = "../src/ui_ime_text_input_handler_v2.rs"]
mod ui_ime_text_input_handler_v2;

use ui_ime_text_input_handler_v2::*;

#[test]
fn test_basic_insertion() {
    let mut handler = ImeTextInputHandlerV2::new();
    handler.on_char('H');
    handler.on_char('e');
    handler.on_char('l');
    handler.on_char('l');
    handler.on_char('o');

    assert_eq!(handler.get_text(), "Hello");
    assert_eq!(handler.get_curpos(), 5);
    assert_eq!(handler.get_lastpos(), 5);
}

#[test]
fn test_cursor_movement() {
    let mut handler = ImeTextInputHandlerV2::new();
    handler.paste_string("Hello");

    handler.move_left();
    assert_eq!(handler.get_curpos(), 4);
    
    handler.move_home();
    assert_eq!(handler.get_curpos(), 0);

    handler.move_right();
    assert_eq!(handler.get_curpos(), 1);

    handler.move_end();
    assert_eq!(handler.get_curpos(), 5);
}

#[test]
fn test_backspace_and_delete() {
    let mut handler = ImeTextInputHandlerV2::new();
    handler.paste_string("Hello");

    handler.on_backspace();
    assert_eq!(handler.get_text(), "Hell");
    assert_eq!(handler.get_curpos(), 4);

    handler.move_left(); // curpos = 3 ('l')
    handler.move_left(); // curpos = 2 ('l')
    
    handler.on_delete(); // Deletes 'l' at index 2
    assert_eq!(handler.get_text(), "Hel");
    assert_eq!(handler.get_curpos(), 2);

    handler.on_backspace(); // Deletes 'e' at index 1
    assert_eq!(handler.get_text(), "Hl");
    assert_eq!(handler.get_curpos(), 1);
}

#[test]
fn test_selection_and_deletion() {
    let mut handler = ImeTextInputHandlerV2::new();
    handler.paste_string("Hello World");
    
    handler.set_selection(0, 5);
    assert_eq!(handler.has_selection(), true);
    
    handler.delete_selection();
    assert_eq!(handler.get_text(), " World");
    assert_eq!(handler.get_curpos(), 0);
    assert_eq!(handler.has_selection(), false);
}

#[test]
fn test_secret_mode() {
    let mut handler = ImeTextInputHandlerV2::new();
    handler.paste_string("password");

    assert_eq!(handler.get_render_text(), "password");

    handler.set_secret_mode(true);
    assert_eq!(handler.get_render_text(), "********");
}

#[test]
fn test_undo_redo() {
    let mut handler = ImeTextInputHandlerV2::new();
    handler.paste_string("Hello");
    handler.paste_string(" World");

    assert_eq!(handler.get_text(), "Hello World");

    handler.undo();
    assert_eq!(handler.get_text(), "Hello");

    handler.redo();
    assert_eq!(handler.get_text(), "Hello World");
}

#[test]
fn test_clipboard_copy_paste() {
    let mut handler = ImeTextInputHandlerV2::new();
    handler.paste_string("Test Copy");
    
    handler.select_all();
    handler.copy_selection_to_clipboard();
    
    handler.clear();
    
    handler.paste_text_from_clipboard();
    assert_eq!(handler.get_text(), "Test Copy");
}

#[test]
fn test_blinking_cursor() {
    let handler = ImeTextInputHandlerV2::new();
    assert_eq!(handler.is_cursor_visible(0), true);
    assert_eq!(handler.is_cursor_visible(499), true);
    assert_eq!(handler.is_cursor_visible(500), false);
    assert_eq!(handler.is_cursor_visible(999), false);
    assert_eq!(handler.is_cursor_visible(1000), true);
}
