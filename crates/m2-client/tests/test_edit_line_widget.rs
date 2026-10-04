#[path = "../src/ui_wndmgr_widget_edit_line.rs"]
mod ui_wndmgr_widget_edit_line;

use ui_wndmgr_widget_edit_line::{EditLineWidget, ClipboardProvider};

struct MockClipboard {
    text: Option<String>,
}

impl MockClipboard {
    fn new() -> Self {
        Self { text: None }
    }
}

impl ClipboardProvider for MockClipboard {
    fn set_text(&mut self, text: &str) {
        self.text = Some(text.to_string());
    }

    fn get_text(&mut self) -> Option<String> {
        self.text.clone()
    }
}

#[test]
fn test_basic_insertion_and_deletion() {
    let mut edit_line = EditLineWidget::new(10);
    
    // Insert text
    edit_line.insert_text("Hello");
    assert_eq!(edit_line.get_text(), "Hello");
    assert_eq!(edit_line.get_cursor_pos(), 5);

    // Delete left (backspace)
    edit_line.delete_left();
    assert_eq!(edit_line.get_text(), "Hell");
    assert_eq!(edit_line.get_cursor_pos(), 4);

    // Move left and delete right (delete)
    edit_line.move_cursor_left(false);
    edit_line.move_cursor_left(false);
    assert_eq!(edit_line.get_cursor_pos(), 2);
    edit_line.delete_right();
    assert_eq!(edit_line.get_text(), "Hel");
    assert_eq!(edit_line.get_cursor_pos(), 2);

    // Insert char
    edit_line.insert_char('a');
    assert_eq!(edit_line.get_text(), "Heal");
    assert_eq!(edit_line.get_cursor_pos(), 3);
}

#[test]
fn test_cursor_movement_and_selection() {
    let mut edit_line = EditLineWidget::new(20);
    edit_line.insert_text("OpenAI is great");

    // Move to home
    edit_line.move_cursor_home(false);
    assert_eq!(edit_line.get_cursor_pos(), 0);

    // Move right with shift
    for _ in 0..6 {
        edit_line.move_cursor_right(true);
    }
    assert_eq!(edit_line.get_cursor_pos(), 6);

    // Replace selected text
    edit_line.insert_text("Metin2");
    assert_eq!(edit_line.get_text(), "Metin2 is great");
    assert_eq!(edit_line.get_cursor_pos(), 6);

    // Select all and replace
    edit_line.select_all();
    edit_line.insert_text("Done");
    assert_eq!(edit_line.get_text(), "Done");
}

#[test]
fn test_clipboard_and_secret_mode() {
    let mut edit_line = EditLineWidget::new(50);
    let mut clipboard = MockClipboard::new();

    edit_line.insert_text("SecretPassword");
    edit_line.set_secret(true);

    assert_eq!(edit_line.get_text(), "SecretPassword");
    assert_eq!(edit_line.get_display_text(), "**************");

    edit_line.select_all();
    edit_line.copy(&mut clipboard);
    assert_eq!(clipboard.get_text(), None); // Shouldn't copy in secret mode

    edit_line.cut(&mut clipboard);
    assert_eq!(clipboard.get_text(), None); // Shouldn't cut in secret mode
    assert_eq!(edit_line.get_text(), "SecretPassword"); // Text shouldn't be deleted

    edit_line.set_secret(false);
    edit_line.select_all();
    edit_line.cut(&mut clipboard);
    assert_eq!(clipboard.get_text().unwrap(), "SecretPassword");
    assert_eq!(edit_line.get_text(), "");

    edit_line.paste(&mut clipboard);
    assert_eq!(edit_line.get_text(), "SecretPassword");
}

#[test]
fn test_max_length_and_blinking() {
    let mut edit_line = EditLineWidget::new(5);

    edit_line.insert_text("123456789");
    assert_eq!(edit_line.get_text(), "12345");
    assert_eq!(edit_line.get_cursor_pos(), 5);

    assert_eq!(edit_line.is_cursor_visible(), true);
    
    // Simulate time passing to toggle blink
    edit_line.update(0.51);
    assert_eq!(edit_line.is_cursor_visible(), false);

    edit_line.update(0.51);
    assert_eq!(edit_line.is_cursor_visible(), true);

    // Typing resets blink
    edit_line.update(0.51);
    assert_eq!(edit_line.is_cursor_visible(), false);
    edit_line.delete_left();
    assert_eq!(edit_line.is_cursor_visible(), true);
    
    edit_line.set_max_length(3);
    assert_eq!(edit_line.get_text(), "123");
    assert_eq!(edit_line.get_cursor_pos(), 3);
}
