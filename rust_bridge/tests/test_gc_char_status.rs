use m2_client_bridge::net::gc::char_status::GcCharStatusPacket;

#[test]
fn test_parse_valid_buffer() {
    let mut buf = Vec::new();
    // vid: 1234
    buf.extend_from_slice(&1234u32.to_le_bytes());
    // hp: 500
    buf.extend_from_slice(&500u32.to_le_bytes());
    // max_hp: 1000
    buf.extend_from_slice(&1000u32.to_le_bytes());

    let packet = GcCharStatusPacket::from_bytes(&buf).expect("Parsowanie poprawnego bufora powinno sie udac");

    assert_eq!(packet.vid, 1234);
    assert_eq!(packet.hp, 500);
    assert_eq!(packet.max_hp, 1000);
}

#[test]
fn test_parse_truncated_buffer() {
    let mut buf = Vec::new();
    // vid: 1234
    buf.extend_from_slice(&1234u32.to_le_bytes());
    // hp: 500
    buf.extend_from_slice(&500u32.to_le_bytes());
    // max_hp (brakuje jednego bajtu)
    buf.extend_from_slice(&[0, 0, 0]);

    assert_eq!(buf.len(), 11);

    let result = GcCharStatusPacket::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest za krotki");
}
