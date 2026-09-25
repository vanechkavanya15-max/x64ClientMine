use m2_client_bridge::net::gc::handshake::HandshakePacket;

#[test]
fn test_gc_handshake_integration_success() {
    // Poprawny bufor
    let mut buf = [0u8; 13];
    buf[0] = 0xff;
    buf[1..5].copy_from_slice(&54321u32.to_le_bytes()); // dw_handshake
    buf[5..9].copy_from_slice(&12345u32.to_le_bytes()); // dw_time
    buf[9..13].copy_from_slice(&500i32.to_le_bytes()); // l_delta

    let packet = HandshakePacket::from_bytes(&buf).expect("Oczekiwano poprawnego parsowania pakietu");
    assert_eq!(packet.header, 0xff);
    assert_eq!(packet.dw_handshake, 54321);
    assert_eq!(packet.dw_time, 12345);
    assert_eq!(packet.l_delta, 500);
}

#[test]
fn test_gc_handshake_integration_too_short() {
    // Bufor za krotki
    let buf = [0u8; 10]; // tylko 10 bajtow
    
    let result = HandshakePacket::from_bytes(&buf);
    assert!(result.is_err(), "Oczekiwano bledu z powodu zbyt krotkiego bufora");
    
    let err_msg = result.unwrap_err();
    assert_eq!(err_msg, "Za krotki bufor. Oczekiwano 13, otrzymano 10");
}
