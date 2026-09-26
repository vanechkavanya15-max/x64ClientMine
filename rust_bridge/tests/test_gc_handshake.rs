use m2_client_bridge::net::gc::handshake::{HandshakePacket, TPacketGCHandShake, HEADER_GC_HANDSHAKE};

#[test]
fn test_gc_handshake_integration_success() {
    let mut buf = [0u8; 16];
    buf[0..2].copy_from_slice(&HEADER_GC_HANDSHAKE.to_le_bytes()); // header
    buf[2..4].copy_from_slice(&16u16.to_le_bytes());               // length
    buf[4..8].copy_from_slice(&54321u32.to_le_bytes());           // dwHandshake
    buf[8..12].copy_from_slice(&12345u32.to_le_bytes());          // dwTime
    buf[12..16].copy_from_slice(&500i32.to_le_bytes());           // lDelta

    let packet = TPacketGCHandShake::from_bytes(&buf).expect("Oczekiwano poprawnego parsowania pakietu");
    assert_eq!({ packet.header }, HEADER_GC_HANDSHAKE);
    assert_eq!({ packet.length }, 16);
    assert_eq!(packet.dw_handshake(), 54321);
    assert_eq!(packet.dw_time(), 12345);
    assert_eq!(packet.l_delta(), 500);
}

#[test]
fn test_gc_handshake_integration_too_short() {
    let buf = [0u8; 15]; // tylko 15 bajtow
    
    let result = HandshakePacket::from_bytes(&buf);
    assert!(result.is_err(), "Oczekiwano bledu z powodu zbyt krotkiego bufora");
    
    let err_msg = result.unwrap_err();
    assert_eq!(err_msg, "Za krotki bufor. Oczekiwano 16, otrzymano 15");
}
