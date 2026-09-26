use m2_client_bridge::net::gc::chat::{
    GcChatPacket, TPacketGCChat, HEADER_GC_CHAT, HEADER_GC_CHAT_SIZE,
};

#[test]
fn test_gc_chat_packet_valid_no_message() {
    let mut buf = vec![0u8; 10];
    buf[0..2].copy_from_slice(&HEADER_GC_CHAT.to_le_bytes()); // header
    buf[2..4].copy_from_slice(&10u16.to_le_bytes());          // length: 10
    buf[4] = 1;                                              // chat_type: CHAT_TYPE_INFO
    buf[5..9].copy_from_slice(&10u32.to_le_bytes());         // vid: 10
    buf[9] = 2;                                              // empire: 2
    
    let result = GcChatPacket::from_bytes(&buf);
    assert!(result.is_ok());
    let packet = result.unwrap();
    assert_eq!(packet.header, HEADER_GC_CHAT);
    assert_eq!(packet.length, 10);
    assert_eq!(packet.chat_type, 1);
    assert_eq!(packet.vid, 10);
    assert_eq!(packet.empire, 2);
    assert_eq!(packet.message, "");
}

#[test]
fn test_gc_chat_packet_valid_with_message() {
    let msg = b"Witaj swiecie";
    let length = (HEADER_GC_CHAT_SIZE + msg.len()) as u16;
    
    let mut buf = vec![0u8; 10];
    buf[0..2].copy_from_slice(&HEADER_GC_CHAT.to_le_bytes());
    buf[2..4].copy_from_slice(&length.to_le_bytes());
    buf[4] = 0; // chat_type: CHAT_TYPE_TALKING
    buf[5..9].copy_from_slice(&255u32.to_le_bytes()); // vid
    buf[9] = 1; // empire: 1
    buf.extend_from_slice(msg);
    
    let result = GcChatPacket::from_bytes(&buf);
    assert!(result.is_ok());
    let packet = result.unwrap();
    assert_eq!(packet.header, HEADER_GC_CHAT);
    assert_eq!(packet.length, length);
    assert_eq!(packet.chat_type, 0);
    assert_eq!(packet.vid, 255);
    assert_eq!(packet.empire, 1);
    assert_eq!(packet.message, "Witaj swiecie");
}

#[test]
fn test_gc_chat_packet_buffer_too_short_for_header() {
    let buf = vec![0u8; 9]; // mniej niz 10 bajtow
    let result = GcChatPacket::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest za krotki aby zawierac naglowek pakietu czatu");
}

#[test]
fn test_gc_chat_packet_buffer_shorter_than_size() {
    let mut buf = vec![0u8; 10];
    buf[0..2].copy_from_slice(&HEADER_GC_CHAT.to_le_bytes());
    buf[2..4].copy_from_slice(&15u16.to_le_bytes()); // deklaracja 15 bajtow
    buf[4] = 1;
    buf[5..9].copy_from_slice(&10u32.to_le_bytes());
    buf[9] = 2;
    buf.push(b'X'); // razem 11 bajtow, mniej niz 15

    let result = GcChatPacket::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest krotszy niz deklarowany rozmiar pakietu");
}

#[test]
fn test_gc_chat_header_struct() {
    let mut buf = [0u8; 10];
    buf[0..2].copy_from_slice(&HEADER_GC_CHAT.to_le_bytes());
    buf[2..4].copy_from_slice(&10u16.to_le_bytes());
    buf[4] = 3;
    buf[5..9].copy_from_slice(&999u32.to_le_bytes());
    buf[9] = 1;

    let packet = TPacketGCChat::from_bytes(&buf).unwrap();
    assert_eq!({ packet.header }, HEADER_GC_CHAT);
    assert_eq!({ packet.length }, 10);
    assert_eq!(packet.type_, 3);
    assert_eq!({ packet.dwVID }, 999);
    assert_eq!(packet.bEmpire, 1);
}
