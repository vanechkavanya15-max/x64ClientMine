use m2_client_bridge::net::gc::chat::GcChatPacket;

#[test]
fn test_gc_chat_packet_valid_no_message() {
    let buf = vec![
        4,          // header: HEADER_GC_CHAT
        9, 0,       // size: 9
        1,          // chat_type: CHAT_TYPE_INFO
        10, 0, 0, 0,// vid: 10
        2,          // empire: 2
    ];
    
    let result = GcChatPacket::from_bytes(&buf);
    assert!(result.is_ok());
    let packet = result.unwrap();
    assert_eq!(packet.header, 4);
    assert_eq!(packet.size, 9);
    assert_eq!(packet.chat_type, 1);
    assert_eq!(packet.vid, 10);
    assert_eq!(packet.empire, 2);
    assert_eq!(packet.message, "");
}

#[test]
fn test_gc_chat_packet_valid_with_message() {
    let msg = b"Witaj swiecie";
    let size = 9 + msg.len() as u16;
    
    let mut buf = vec![
        4,                      // header: HEADER_GC_CHAT
        (size & 0xFF) as u8,    // size: lower byte
        (size >> 8) as u8,      // size: upper byte
        0,                      // chat_type: CHAT_TYPE_TALKING
        0xFF, 0, 0, 0,          // vid: 255
        1,                      // empire: 1
    ];
    buf.extend_from_slice(msg);
    
    let result = GcChatPacket::from_bytes(&buf);
    assert!(result.is_ok());
    let packet = result.unwrap();
    assert_eq!(packet.header, 4);
    assert_eq!(packet.size, size);
    assert_eq!(packet.chat_type, 0);
    assert_eq!(packet.vid, 255);
    assert_eq!(packet.empire, 1);
    assert_eq!(packet.message, "Witaj swiecie");
}

#[test]
fn test_gc_chat_packet_buffer_too_short_for_header() {
    // Bufor krotszy niz 9 bajtow (tylko 8 bajtow)
    let buf = vec![4, 9, 0, 1, 10, 0, 0, 0];
    let result = GcChatPacket::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest za krotki aby zawierac naglowek pakietu czatu");
}

#[test]
fn test_gc_chat_packet_buffer_shorter_than_size() {
    // Rozmiar to 15, ale w buforze jest tylko 10 bajtow
    let buf = vec![
        4,          // header: HEADER_GC_CHAT
        15, 0,      // size: 15
        1,          // chat_type: CHAT_TYPE_INFO
        10, 0, 0, 0,// vid: 10
        2,          // empire: 2
        b'X',       // message: 1 byte
    ];
    let result = GcChatPacket::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest krotszy niz deklarowany rozmiar pakietu");
}

#[test]
fn test_gc_chat_packet_invalid_header() {
    // Nieprawidlowy naglowek (np. 5)
    let buf = vec![
        5,          // header: INVALID
        9, 0,       // size: 9
        1,          // chat_type
        10, 0, 0, 0,// vid
        2,          // empire
    ];
    let result = GcChatPacket::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Nieprawidlowy naglowek pakiety czatu");
}
