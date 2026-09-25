use m2_client_bridge::net::gc::ground_item_del::GroundItemDelPacket;

#[test]
fn test_ground_item_del_from_bytes_success() {
    // Naglowek = 27 (0x1b)
    // VID = 123456789 (0x075BCD15 -> w pamieci Little Endian: 15 CD 5B 07)
    let buf = vec![27, 0x15, 0xCD, 0x5B, 0x07];
    
    let packet = GroundItemDelPacket::from_bytes(&buf).unwrap();
    
    assert_eq!(packet.header, 27);
    assert_eq!(packet.vid, 123456789);
}

#[test]
fn test_ground_item_del_from_bytes_buffer_too_short() {
    // Za krotki bufor (tylko naglowek i 3 bajty VID)
    let buf = vec![27, 0x15, 0xCD, 0x5B];
    
    let result = GroundItemDelPacket::from_bytes(&buf);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest zbyt krotki aby zdeserializowac GroundItemDelPacket");
}
