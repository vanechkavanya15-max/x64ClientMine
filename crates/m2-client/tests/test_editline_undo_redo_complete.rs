#[path = "../src/editline_undo_redo_complete.rs"]
mod editline_undo_redo_complete;

use editline_undo_redo_complete::{ImeEditLineWidget, MAX_UNDO};

#[test]
fn test_undo_redo_basic() {
    let mut widget = ImeEditLineWidget::new(100, false);
    
    widget.insert_string("hello");
    assert_eq!(widget.get_visible_text(), "hello");
    
    // Typing the string pushed an undo state for the overall operation
    // Now trigger undo (Ctrl+Z -> 0x1A)
    let handled = widget.on_wm_char(0x1A, true, false);
    assert!(handled);
    assert_eq!(widget.get_visible_text(), "");
    
    // Redo (Ctrl+Y -> 0x19)
    let handled = widget.on_wm_char(0x19, true, false);
    assert!(handled);
    assert_eq!(widget.get_visible_text(), "hello");
}

#[test]
fn test_undo_redo_max_limit() {
    let mut widget = ImeEditLineWidget::new(100, false);
    
    // Do 70 edits (exceeds MAX_UNDO=64)
    for i in 0..70 {
        widget.insert_char('x');
    }
    
    assert_eq!(widget.text.len(), 70);
    assert_eq!(widget.undo_stack.len(), MAX_UNDO); // Stack is capped at 64
    
    // We can only undo MAX_UNDO times
    for _ in 0..MAX_UNDO {
        widget.undo();
    }
    
    // We should have 70 - 64 = 6 characters left
    assert_eq!(widget.text.len(), 6);
    assert_eq!(widget.text, "xxxxxx");
    
    // Attempting to undo more should safely do nothing
    widget.undo();
    assert_eq!(widget.text, "xxxxxx");
}

#[test]
fn test_navigation_and_deletion() {
    let mut widget = ImeEditLineWidget::new(100, false);
    
    widget.insert_string("12345");
    assert_eq!(widget.cursor_pos, 5);
    
    // Home key (0x24)
    widget.on_wm_char(0x24, false, false);
    assert_eq!(widget.cursor_pos, 0);
    
    // Delete key (0x2E) -> deletes character after cursor ('1')
    widget.on_wm_char(0x2E, false, false);
    assert_eq!(widget.text, "2345");
    assert_eq!(widget.cursor_pos, 0);
    
    // End key (0x23)
    widget.on_wm_char(0x23, false, false);
    assert_eq!(widget.cursor_pos, 4);
    
    // Backspace key (0x08) -> deletes character before cursor ('5')
    widget.on_wm_char(0x08, false, false);
    assert_eq!(widget.text, "234");
    assert_eq!(widget.cursor_pos, 3);
}

#[test]
fn test_ctrl_keys_non_destructive() {
    let mut widget = ImeEditLineWidget::new(100, false);
    widget.insert_string("test");
    
    // Return (0x0D)
    assert!(widget.on_wm_char(0x0D, false, false));
    
    // Escape (0x1B)
    assert!(widget.on_wm_char(0x1B, false, false));
    
    // Tab (0x09)
    assert!(widget.on_wm_char(0x09, false, false));
    
    // Text should remain unmodified
    assert_eq!(widget.text, "test");
}

#[test]
fn test_undo_clears_redo() {
    let mut widget = ImeEditLineWidget::new(100, false);
    
    widget.insert_string("a");
    widget.insert_string("b");
    assert_eq!(widget.get_visible_text(), "ab");
    
    widget.undo(); // back to 'a'
    assert_eq!(widget.get_visible_text(), "a");
    
    widget.insert_string("c"); // Should clear redo stack
    assert_eq!(widget.get_visible_text(), "ac");
    
    widget.redo(); // Should do nothing because redo was cleared
    assert_eq!(widget.get_visible_text(), "ac");
}
