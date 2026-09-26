#[path = "../src/recv_chat.rs"]
mod recv_chat;

use recv_chat::{
    parse_chat, parse_whisper, sanitize_chat_text, ChatType, ParseError, WhisperType,
};

// Memory guideline: When testing #[repr(C, packed)] structs in Rust, attempting to read fields
// directly in macros like assert_eq! causes unaligned_references (E0793) errors. To safely assert,
// copy the field contents to a local variable first.
// The structures themselves are private to recv_chat, so we only test parse_chat and parse_whisper.

#[test]
fn test_parse_chat_talking() {
    let mut data = vec![0; 10]; // header
    
    // header = 0xAA (dummy)
    data[0] = 0xAA;
    data[1] = 0x00;
    
    // length = 10 (header) + 11 (text: "Hello World") = 21
    data[2] = 21;
    data[3] = 0;
    
    // type = 0 (Talking)
    data[4] = 0;
    
    // vid = 1234
    data[5] = 0xD2;
    data[6] = 0x04;
    data[7] = 0x00;
    data[8] = 0x00;
    
    // empire = 1
    data[9] = 1;
    
    // text
    data.extend_from_slice(b"Hello World");
    
    let result = parse_chat(&data).expect("Failed to parse valid chat packet");
    assert_eq!(result.0, 21);
    
    let msg = result.1;
    let chat_type = msg.chat_type;
    let vid = msg.vid;
    let empire = msg.empire;
    let text = msg.text;
    
    assert_eq!(chat_type, ChatType::Talking);
    assert_eq!(vid, 1234);
    assert_eq!(empire, 1);
    assert_eq!(text, "Hello World");
}

#[test]
fn test_parse_chat_notice() {
    let mut data = vec![0; 10];
    data[0] = 0xAA;
    data[2] = 25; // 10 + 15
    data[4] = 2; // Notice
    data[5] = 0; // vid=0
    data[9] = 0; // empire=0
    
    data.extend_from_slice(b"Server Restart!");
    
    let result = parse_chat(&data).expect("Failed to parse valid chat packet");
    let msg = result.1;
    
    assert_eq!(msg.chat_type, ChatType::Notice);
    assert_eq!(msg.text, "Server Restart!");
}

#[test]
fn test_parse_whisper() {
    let mut data = vec![0; 70]; // header=2, length=2, type=1, nameFrom=65
    
    data[0] = 0xBB;
    
    // len: 70 + 5 ("Hello") = 75
    data[2] = 75;
    
    // type = WhisperType::Gm (5)
    data[4] = 5;
    
    // sz_name_from starts at byte 5
    let name_bytes = b"GM_Jules";
    data[5..5 + name_bytes.len()].copy_from_slice(name_bytes);
    
    // text
    data.extend_from_slice(b"Hello");
    
    let result = parse_whisper(&data).expect("Failed to parse whisper packet");
    assert_eq!(result.0, 75);
    
    let msg = result.1;
    assert_eq!(msg.whisper_type, WhisperType::Gm);
    assert_eq!(msg.from, "GM_Jules");
    assert_eq!(msg.text, "Hello");
}

#[test]
fn test_sanitize_chat_text() {
    let raw = "Hello\x07World\n\x08";
    let clean = sanitize_chat_text(raw);
    assert_eq!(clean, "HelloWorld\n");
}

#[test]
fn test_parse_buffer_too_small() {
    let data = vec![0; 5]; // smaller than TPacketGCChat
    let result = parse_chat(&data);
    assert!(matches!(result, Err(ParseError::BufferTooSmall)));
}

#[test]
fn test_parse_chat_invalid_type() {
    let mut data = vec![0; 10];
    data[2] = 10; 
    data[4] = 200; // invalid chat type
    
    let result = parse_chat(&data);
    assert!(matches!(result, Err(ParseError::InvalidType)));
}

#[test]
fn test_parse_chat_buffer_smaller_than_length() {
    let mut data = vec![0; 10];
    data[2] = 20; 
    data[4] = 0; 
    
    let result = parse_chat(&data);
    assert!(matches!(result, Err(ParseError::BufferTooSmall)));
}

#[test]
fn test_filter_color_tags() {
    use recv_chat::filter_color_tags;
    let raw = "|cFF00FF00Hello|r |cffffffffWorld|r";
    let clean = filter_color_tags(raw);
    assert_eq!(clean, "Hello World");
}

#[test]
fn test_parse_chat_buffer_length_less_than_header() {
    let mut data = vec![0; 15];
    // Set length to 5, which is < header size (10)
    data[2] = 5;
    
    let result = parse_chat(&data);
    assert!(matches!(result, Err(ParseError::BufferTooSmall)));
}

#[test]
fn test_sanitize_is_used() {
    let mut data = vec![0; 10]; // header
    
    data[0] = 0xAA;
    
    let text = b"|cFF00FF00Hello|r \x07World\n\x08";
    let total_len = 10 + text.len() as u16;
    
    data[2] = (total_len & 0xFF) as u8;
    data[3] = (total_len >> 8) as u8;
    
    data[4] = 0; // Talking
    
    data.extend_from_slice(text);
    
    let result = parse_chat(&data).expect("Failed to parse valid chat packet");
    assert_eq!(result.1.text, "Hello World\n");
}
