use m2_client::chat_buffer_v2::*;
use std::ffi::CString;

#[test]
fn test_item_hyperlink_parsing() {
    // Normal item
    let link = "|cffffc700|Hitem:1234:0:1:2:3:1:64:2:C8|h[MySword]|h|r";
    let parsed = parse_item_hyperlink(link).unwrap();
    assert_eq!(parsed.vnum, 0x1234);
    assert_eq!(parsed.flags, 0);
    assert_eq!(parsed.sockets, [1, 2, 3]);
    assert_eq!(parsed.attrs.len(), 2);
    assert_eq!(parsed.attrs[0], (1, 100)); // 0x64
    assert_eq!(parsed.attrs[1], (2, 200)); // 0xC8
    assert_eq!(parsed.name, "MySword");

    // Negative socket values (C++ %x formatting)
    let link2 = "|cffffc700|Hitem:4321:F:ffffffff:fffffffe:fffffffd|h[BrokenSword]|h|r";
    let parsed2 = parse_item_hyperlink(link2).unwrap();
    assert_eq!(parsed2.vnum, 0x4321);
    assert_eq!(parsed2.flags, 0xF);
    assert_eq!(parsed2.sockets, [-1, -2, -3]);
    assert_eq!(parsed2.attrs.len(), 0);
    assert_eq!(parsed2.name, "BrokenSword");
    
    // Invalid
    assert!(parse_item_hyperlink("not a link").is_none());
    assert!(parse_item_hyperlink("|cffffc700|Hnotitem:1|h[A]|h|r").is_none());
}

#[test]
fn test_chat_buffer_append_and_flood_limit() {
    let mut buffer = ChatBuffer::new();
    
    // Append first line
    assert!(buffer.append_line(CHAT_TYPE_TALKING, "Hello World", 1000));
    assert_eq!(buffer.lines.len(), 1);
    
    // Should be rejected by anti-flood
    assert!(!buffer.append_line(CHAT_TYPE_TALKING, "Spam", 1100));
    assert_eq!(buffer.lines.len(), 1);
    
    // Should be accepted
    assert!(buffer.append_line(CHAT_TYPE_TALKING, "Hello Again", 1500));
    assert_eq!(buffer.lines.len(), 2);
    
    let last = buffer.lines.back().unwrap();
    assert_eq!(last.text, "Hello Again");
    assert_eq!(last.sender_name, "");
}

#[test]
fn test_chat_buffer_sender_split() {
    let mut buffer = ChatBuffer::new();
    assert!(buffer.append_line(CHAT_TYPE_TALKING, "Player1 : Hi guys", 1000));
    
    let line = buffer.lines.back().unwrap();
    assert_eq!(line.sender_name, "Player1");
    assert_eq!(line.text, "Hi guys");
}

#[test]
fn test_chat_buffer_forbidden_words() {
    let mut buffer = ChatBuffer::new();
    buffer.set_forbidden_words(vec!["fuck".to_string(), "kurwa".to_string()]);
    
    assert!(buffer.append_line(CHAT_TYPE_TALKING, "what the fuck is this", 1000));
    let line = buffer.lines.back().unwrap();
    assert_eq!(line.text, "what the **** is this");
    
    assert!(buffer.append_line(CHAT_TYPE_TALKING, "Player : kurwa mac", 2000));
    let line2 = buffer.lines.back().unwrap();
    assert_eq!(line2.sender_name, "Player");
    assert_eq!(line2.text, "***** mac");
}

#[test]
fn test_chat_buffer_ring_limit() {
    let mut buffer = ChatBuffer::new();
    
    for i in 0..1500 {
        // Need to add >500ms to bypass flood filter
        buffer.append_line(CHAT_TYPE_TALKING, &format!("Message {}", i), i * 1000);
    }
    
    assert_eq!(buffer.lines.len(), CHAT_LINE_MAX_NUM);
    assert_eq!(buffer.lines.front().unwrap().text, "Message 500");
    assert_eq!(buffer.lines.back().unwrap().text, "Message 1499");
}

#[test]
fn test_ffi_append() {
    let text = CString::new("FFI Test").unwrap();
    
    // First message should work
    let res = m2_chat_append_line(CHAT_TYPE_TALKING, text.as_ptr());
    assert!(res);
    
    // Second message immediately should be blocked by flood filter
    let res2 = m2_chat_append_line(CHAT_TYPE_TALKING, text.as_ptr());
    assert!(!res2);
    
    // Check if it's in the buffer
    let buffer = get_chat_buffer().lock().unwrap();
    assert!(buffer.lines.iter().any(|l| l.text == "FFI Test"));
}
