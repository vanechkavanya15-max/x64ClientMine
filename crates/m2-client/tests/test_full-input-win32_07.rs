use m2_client::input_clipboard_handler::ClipboardHandler;
use std::sync::Mutex;
use std::sync::LazyLock;

// Mutex to serialize clipboard access in concurrent test runs
static CLIPBOARD_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[test]
#[cfg(windows)]
fn test_clipboard_basic_copy_paste() {
    let _guard = CLIPBOARD_LOCK.lock().unwrap();
    let handler = ClipboardHandler::new();
    let text = "Hello, Metin2!";
    assert!(handler.copy_text(text));
    assert_eq!(handler.paste_text(), Some(text.to_string()));
}

#[test]
#[cfg(windows)]
fn test_clipboard_empty_string() {
    let _guard = CLIPBOARD_LOCK.lock().unwrap();
    let handler = ClipboardHandler::new();
    let text = "";
    assert!(handler.copy_text(text));
    assert_eq!(handler.paste_text(), Some(text.to_string()));
}

#[test]
#[cfg(windows)]
fn test_clipboard_unicode_characters() {
    let _guard = CLIPBOARD_LOCK.lock().unwrap();
    let handler = ClipboardHandler::new();
    let text = "Zażółć gęślą jaźń 123! @# 🌟";
    assert!(handler.copy_text(text));
    assert_eq!(handler.paste_text(), Some(text.to_string()));
}

#[test]
#[cfg(windows)]
fn test_clipboard_multiline_text() {
    let _guard = CLIPBOARD_LOCK.lock().unwrap();
    let handler = ClipboardHandler::new();
    let text = "Line 1\nLine 2\r\nLine 3";
    assert!(handler.copy_text(text));
    assert_eq!(handler.paste_text(), Some(text.to_string()));
}

#[test]
#[cfg(windows)]
fn test_clipboard_long_text() {
    let _guard = CLIPBOARD_LOCK.lock().unwrap();
    let handler = ClipboardHandler::new();
    let text = "A".repeat(10000);
    assert!(handler.copy_text(&text));
    assert_eq!(handler.paste_text(), Some(text));
}

#[test]
#[cfg(windows)]
fn test_clipboard_overwrite() {
    let _guard = CLIPBOARD_LOCK.lock().unwrap();
    let handler = ClipboardHandler::new();
    
    let text1 = "First clipboard content";
    assert!(handler.copy_text(text1));
    assert_eq!(handler.paste_text(), Some(text1.to_string()));
    
    let text2 = "Second clipboard content";
    assert!(handler.copy_text(text2));
    assert_eq!(handler.paste_text(), Some(text2.to_string()));
}
