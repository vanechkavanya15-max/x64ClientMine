use m2_client_bridge::net::gc::auth_success::AuthSuccessPacket;

#[test]
fn test_auth_success_from_bytes_valid() {
    let mut buf = vec![150, 0x78, 0x56, 0x34, 0x12];
    buf.extend_from_slice(&1u32.to_le_bytes()); // key 1
    buf.extend_from_slice(&2u32.to_le_bytes()); // key 2
    buf.extend_from_slice(&3u32.to_le_bytes()); // key 3
    buf.extend_from_slice(&4u32.to_le_bytes()); // key 4
    
    let result = AuthSuccessPacket::from_bytes(&buf);
    assert!(result.is_ok(), "Deserializacja powinna zakonczyc sie sukcesem");
    
    let packet = result.unwrap();
    assert_eq!(packet.header, 150);
    assert_eq!(packet.login_key, 0x12345678);
    assert_eq!(packet.encryption_keys, [1, 2, 3, 4]);
}

#[test]
fn test_auth_success_from_bytes_too_short() {
    let mut buf = vec![150, 0x78, 0x56, 0x34, 0x12];
    buf.extend_from_slice(&1u32.to_le_bytes()); // key 1
    buf.extend_from_slice(&2u32.to_le_bytes()); // key 2
    // Brakuje keys 3 i 4
    
    let result = AuthSuccessPacket::from_bytes(&buf);
    assert!(result.is_err(), "Deserializacja powinna zwrocic blad dla za krotkiego bufora");
    
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("Bufor jest za krotki"));
}

#[test]
fn test_auth_success_from_bytes_longer_buffer() {
    let mut buf = vec![150, 0x78, 0x56, 0x34, 0x12];
    buf.extend_from_slice(&1u32.to_le_bytes()); // key 1
    buf.extend_from_slice(&2u32.to_le_bytes()); // key 2
    buf.extend_from_slice(&3u32.to_le_bytes()); // key 3
    buf.extend_from_slice(&4u32.to_le_bytes()); // key 4
    buf.extend_from_slice(&[0xFF, 0xFF]); // Dodatkowe bajty
    
    let result = AuthSuccessPacket::from_bytes(&buf);
    assert!(result.is_ok(), "Deserializacja powinna zakonczyc sie sukcesem ignorujac dodatkowe bajty");
    
    let packet = result.unwrap();
    assert_eq!(packet.header, 150);
    assert_eq!(packet.login_key, 0x12345678);
    assert_eq!(packet.encryption_keys, [1, 2, 3, 4]);
}
