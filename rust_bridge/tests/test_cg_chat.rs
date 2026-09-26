use m2_client_bridge::net::cg::chat::{TPacketCGChat, m2_net_send_chat, HEADER_CG_CHAT};
use std::ffi::CString;
use std::ptr;

#[test]
fn test_cg_chat_packet_serialization() {
    let serialized = TPacketCGChat::build_with_message(1, "hello");

    // 5 (header + length + type) + 5 ("hello") + 1 (\0) = 11 bajtow
    assert_eq!(serialized.len(), 11);

    let header = u16::from_le_bytes([serialized[0], serialized[1]]);
    assert_eq!(header, HEADER_CG_CHAT);

    let length = u16::from_le_bytes([serialized[2], serialized[3]]);
    assert_eq!(length, 11);

    assert_eq!(serialized[4], 1); // chat_type
    assert_eq!(&serialized[5..10], b"hello");
    assert_eq!(serialized[10], 0); // null terminator
}

#[test]
fn test_cg_chat_packet_serialization_empty_string() {
    let serialized = TPacketCGChat::build_with_message(2, "");

    // 5 (header struct) + 0 ("") + 1 (\0) = 6 bajtow
    assert_eq!(serialized.len(), 6);

    let header = u16::from_le_bytes([serialized[0], serialized[1]]);
    assert_eq!(header, HEADER_CG_CHAT);

    let length = u16::from_le_bytes([serialized[2], serialized[3]]);
    assert_eq!(length, 6);

    assert_eq!(serialized[4], 2); // chat_type
    assert_eq!(serialized[5], 0); // null terminator
}

#[test]
fn test_cg_chat_packet_truncation() {
    let mut long_msg = String::new();
    for _ in 0..600 {
        long_msg.push('A');
    }
    
    let bytes = TPacketCGChat::build_with_message(1, &long_msg);
    // 5 (header) + 512 + 1 (null) = 518
    assert_eq!(bytes.len(), 518);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 518);
}

#[test]
fn test_cg_chat_packet_utf8_truncation() {
    // Znak \u{017C} zajmuje 2 bajty w UTF-8
    let mut utf8_msg = String::new();
    for _ in 0..256 {
        utf8_msg.push('\u{017C}'); // 512 bajtow
    }
    utf8_msg.push('\u{017C}'); // 514 bajtow totalnie

    let bytes = TPacketCGChat::build_with_message(1, &utf8_msg);
    // 5 (header) + 512 + 1 (null) = 518
    assert_eq!(bytes.len(), 518);
}


#[test]
fn test_m2_net_send_chat_null_ptr() {
    let result = unsafe { m2_net_send_chat(1, ptr::null()) };
    assert_eq!(result, false, "Oczekiwano false dla null pointera");
}

#[test]
fn test_m2_net_send_chat_valid_ptr() {
    let msg = CString::new("Testowa wiadomosc").unwrap();
    let result = unsafe { m2_net_send_chat(1, msg.as_ptr()) };
    assert_eq!(result, true, "Oczekiwano true dla poprawnego wskaznika na C-string");
}
