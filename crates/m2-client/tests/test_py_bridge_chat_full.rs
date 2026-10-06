use std::ffi::CString;
use std::os::raw::c_char;

#[path = "../src/py_bridge_chat_full.rs"]
mod py_bridge_chat_full;

use py_bridge_chat_full::{
    m2_py_chat_full_append_chat, m2_py_chat_full_append_whisper,
    m2_py_chat_full_clear_chat, m2_py_chat_full_is_open_whisper,
    m2_py_chat_full_set_whisper_position,
};

#[test]
fn test_m2_py_chat_full_append_and_clear_chat() {
    // Clear first to ensure a clean state
    m2_py_chat_full_clear_chat();

    let text = CString::new("Test message").unwrap();
    let chat_type = 1;

    let result = m2_py_chat_full_append_chat(chat_type, text.as_ptr());
    assert!(result, "Expected to successfully append chat");

    // Clear chat
    m2_py_chat_full_clear_chat();
}

#[test]
fn test_m2_py_chat_full_whisper_operations() {
    let name = CString::new("TestUser").unwrap();
    let text = CString::new("Hello from whisper").unwrap();
    let whisper_type = 2;

    // Check it's not open yet
    let is_open_initially = m2_py_chat_full_is_open_whisper(name.as_ptr());
    assert!(!is_open_initially, "Whisper should not be open initially");

    // Append whisper (should create conversation)
    let append_result = m2_py_chat_full_append_whisper(whisper_type, name.as_ptr(), text.as_ptr());
    assert!(append_result, "Expected to successfully append whisper");

    // Check it's open now
    let is_open_after = m2_py_chat_full_is_open_whisper(name.as_ptr());
    assert!(is_open_after, "Whisper should be open after appending");

    // Set position
    let set_pos_result = m2_py_chat_full_set_whisper_position(name.as_ptr(), 100.0, 200.0);
    assert!(set_pos_result, "Expected to successfully set whisper position");
}

#[test]
fn test_m2_py_chat_full_whisper_set_position_nonexistent() {
    let name = CString::new("NonexistentUser").unwrap();

    // Trying to set position of a whisper that doesn't exist should fail gracefully
    let set_pos_result = m2_py_chat_full_set_whisper_position(name.as_ptr(), 50.0, 50.0);
    assert!(!set_pos_result, "Expected false when setting position of nonexistent whisper");
}

#[test]
fn test_m2_py_chat_full_null_pointers() {
    let name = CString::new("SomeUser").unwrap();
    let text = CString::new("Some text").unwrap();

    let null_ptr: *const c_char = std::ptr::null();

    // Test append chat with null
    assert!(!m2_py_chat_full_append_chat(1, null_ptr));

    // Test append whisper with nulls
    assert!(!m2_py_chat_full_append_whisper(1, null_ptr, text.as_ptr()));
    assert!(!m2_py_chat_full_append_whisper(1, name.as_ptr(), null_ptr));
    assert!(!m2_py_chat_full_append_whisper(1, null_ptr, null_ptr));

    // Test set whisper position with null
    assert!(!m2_py_chat_full_set_whisper_position(null_ptr, 10.0, 10.0));

    // Test is open whisper with null
    assert!(!m2_py_chat_full_is_open_whisper(null_ptr));
}
