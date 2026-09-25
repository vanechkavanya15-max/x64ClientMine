use m2_client_bridge::net::cg::chat::{TPacketCGChat, m2_net_send_chat};
use std::ffi::CString;
use std::ptr;

#[test]
fn test_cg_chat_packet_serialization() {
    let packet = TPacketCGChat::new(1, "hello");
    let serialized = packet.to_bytes();

    // 4 (header + size + type) + 5 ("hello") + 1 (\0) = 10 bajtow
    let expected: Vec<u8> = vec![
        3,          // HEADER_CG_CHAT
        10, 0,      // length (little-endian u16)
        1,          // chat_type
        104, 101, 108, 108, 111, // "hello" w ASCII
        0           // null terminator
    ];

    assert_eq!(serialized, expected, "Nieprawidlowy uklad bajtow dla TPacketCGChat");
    assert_eq!(packet.size, 10);
}

#[test]
fn test_cg_chat_packet_serialization_empty_string() {
    let packet = TPacketCGChat::new(2, "");
    let serialized = packet.to_bytes();

    // 4 (header) + 0 ("") + 1 (\0) = 5 bajtow
    let expected: Vec<u8> = vec![
        3,          // HEADER_CG_CHAT
        5, 0,       // length (little-endian u16)
        2,          // chat_type
        0           // null terminator
    ];

    assert_eq!(serialized, expected, "Nieprawidlowy uklad bajtow dla TPacketCGChat (pusty string)");
}

#[test]
fn test_cg_chat_packet_truncation() {
    let mut long_msg = String::new();
    for _ in 0..600 {
        long_msg.push('A');
    }
    
    let packet = TPacketCGChat::new(1, &long_msg);
    assert_eq!(packet.message.len(), 513); // 512 + null byte
    assert_eq!(packet.size, 517); // 4 + 513
}

#[test]
fn test_cg_chat_packet_utf8_truncation() {
    // Znak "ż" zajmuje 2 bajty w utf8
    let mut utf8_msg = String::new();
    for _ in 0..256 {
        utf8_msg.push('ż'); // 512 bajtow
    }
    utf8_msg.push('ż'); // extra z, co daje 514 bajtow totalnie

    let packet = TPacketCGChat::new(1, &utf8_msg);
    // Skoro makso to 512 bajtow, i ma "ż" rowne (256 * 2), to bedzie rowno 512
    assert_eq!(packet.message.len(), 513); 
    assert_eq!(packet.size, 517);

    let decoded = std::str::from_utf8(&packet.message[..packet.message.len()-1]).unwrap();
    assert_eq!(decoded.chars().count(), 256);
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
