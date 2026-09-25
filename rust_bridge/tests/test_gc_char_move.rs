use m2_client_bridge::net::gc::char_move::PacketGcCharacterMove;

#[test]
fn test_parse_valid_buffer() {
    // Naglowek: 3 (HEADER_GC_CHARACTER_MOVE)
    // Func: 1, Arg: 2, Rot: 3
    // VID: 0x01020304
    // X: 0x05060708 (84281096)
    // Y: 0x090A0B0C (151659276)
    // Time: 0x0D0E0F10
    // Duration: 0x11121314
    
    let buf: [u8; 24] = [
        0x03, 0x01, 0x02, 0x03,       // Header, Func, Arg, Rot
        0x04, 0x03, 0x02, 0x01,       // VID (Little Endian)
        0x08, 0x07, 0x06, 0x05,       // X (Little Endian)
        0x0C, 0x0B, 0x0A, 0x09,       // Y (Little Endian)
        0x10, 0x0F, 0x0E, 0x0D,       // Time (Little Endian)
        0x14, 0x13, 0x12, 0x11,       // Duration (Little Endian)
    ];

    let result = PacketGcCharacterMove::from_bytes(&buf);
    assert!(result.is_ok(), "Oczekiwano poprawnego parsowania, otrzymano: {:?}", result);
    
    let packet = result.unwrap();
    assert_eq!(packet.header, 3);
    assert_eq!(packet.func, 1);
    assert_eq!(packet.arg, 2);
    assert_eq!(packet.rot, 3);
    assert_eq!(packet.vid, 0x01020304);
    assert_eq!(packet.x, 0x05060708);
    assert_eq!(packet.y, 0x090A0B0C);
    assert_eq!(packet.time, 0x0D0E0F10);
    assert_eq!(packet.duration, 0x11121314);
}

#[test]
fn test_parse_buffer_too_short() {
    let buf: [u8; 23] = [0; 23];
    let result = PacketGcCharacterMove::from_bytes(&buf);
    
    assert!(result.is_err(), "Oczekiwano bledu parsowania dla zbyt krotkiego bufora");
    
    let err = result.unwrap_err();
    assert!(err.contains("Bufor jest za krotki"), "Oczekiwano komunikatu o krotkim buforze, otrzymano: {}", err);
}
