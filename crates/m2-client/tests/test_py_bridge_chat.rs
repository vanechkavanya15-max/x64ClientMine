use std::ffi::CString;
use m2_client::py_bridge_chat::{
    m2_py_chat_append_chat, m2_py_chat_clear_chat, m2_py_chat_get_chat_color, m2_py_chat_get_chat_line_count,
    m2_py_chat_set_chat_color,
};
use m2_client::chat_buffer_v2::{get_chat_buffer, CHAT_LINE_MAX_NUM};

// Global mutex to prevent data races during parallel testing since chat buffer is a global singleton
static TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn test_py_chat_append_null() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let result = m2_py_chat_append_chat(0, std::ptr::null());
    assert!(!result);
}

#[test]
fn test_py_chat_append_text() {
    let _guard = TEST_MUTEX.lock().unwrap();
    m2_py_chat_clear_chat();
    assert_eq!(m2_py_chat_get_chat_line_count(), 0);

    let text = CString::new("Hello Metin2").unwrap();
    let result = m2_py_chat_append_chat(1, text.as_ptr());
    assert!(result);
    assert_eq!(m2_py_chat_get_chat_line_count(), 1);
    
    {
        let buffer = get_chat_buffer().lock().unwrap();
        let line = buffer.lines.back().unwrap();
        assert_eq!(line.chat_type, 1);
        assert_eq!(line.text, "Hello Metin2");
    }
}

#[test]
fn test_py_chat_clear_chat() {
    let _guard = TEST_MUTEX.lock().unwrap();
    m2_py_chat_clear_chat();
    
    // Add multiple lines, faking time because of anti flood
    {
        let mut buffer = get_chat_buffer().lock().unwrap();
        buffer.append_line(1, "Msg 1", 1000);
        buffer.append_line(1, "Msg 2", 2000);
        buffer.append_line(1, "Msg 3", 3000);
    }
    assert_eq!(m2_py_chat_get_chat_line_count(), 3);
    
    m2_py_chat_clear_chat();
    assert_eq!(m2_py_chat_get_chat_line_count(), 0);
}

#[test]
fn test_py_chat_colors() {
    let _guard = TEST_MUTEX.lock().unwrap();
    // Default color should be white
    assert_eq!(m2_py_chat_get_chat_color(10), 0xFFFFFFFF);
    
    // Set colors
    m2_py_chat_set_chat_color(10, 0xFFFF0000); // Red
    m2_py_chat_set_chat_color(11, 0xFF00FF00); // Green
    
    assert_eq!(m2_py_chat_get_chat_color(10), 0xFFFF0000);
    assert_eq!(m2_py_chat_get_chat_color(11), 0xFF00FF00);
}

#[test]
fn test_py_chat_max_buffer() {
    let _guard = TEST_MUTEX.lock().unwrap();
    m2_py_chat_clear_chat();
    
    {
        let mut buffer = get_chat_buffer().lock().unwrap();
        for i in 0..(CHAT_LINE_MAX_NUM + 10) {
            // we use manual append_line with time to bypass anti flood
            buffer.append_line(0, &format!("Line {}", i), (i * 1000) as u32);
        }
    }
    
    assert_eq!(m2_py_chat_get_chat_line_count(), CHAT_LINE_MAX_NUM);
    
    {
        let buffer = get_chat_buffer().lock().unwrap();
        assert_eq!(buffer.lines.front().unwrap().text, "Line 10");
        assert_eq!(buffer.lines.back().unwrap().text, format!("Line {}", CHAT_LINE_MAX_NUM + 9));
    }
}

#[test]
fn test_py_chat_anti_flood() {
    let _guard = TEST_MUTEX.lock().unwrap();
    m2_py_chat_clear_chat();
    
    let text = CString::new("Spam").unwrap();
    
    let result1 = m2_py_chat_append_chat(0, text.as_ptr());
    assert!(result1);
    
    // second append should fail due to anti-flood (timestamp < 500ms since last)
    let result2 = m2_py_chat_append_chat(0, text.as_ptr());
    assert!(!result2);
    
    assert_eq!(m2_py_chat_get_chat_line_count(), 1);
}
