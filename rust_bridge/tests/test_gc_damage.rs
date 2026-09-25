use m2_client_bridge::net::gc::damage::GcDamageInfoPacket;

#[test]
fn test_gc_damage_info_valid() {
    // Konstruujemy bufor z prawidlowymi danymi.
    // header (1 bajt): 135
    // victim_vid (4 bajty): 1000 (0x03E8) w little-endian: 0xE8, 0x03, 0x00, 0x00
    // flags (1 bajt): 1
    // damage (4 bajty): 5000 (0x1388) w little-endian: 0x88, 0x13, 0x00, 0x00
    let buf: [u8; 10] = [135, 0xE8, 0x03, 0x00, 0x00, 1, 0x88, 0x13, 0x00, 0x00];

    let result = GcDamageInfoPacket::from_bytes(&buf);
    assert!(result.is_ok());

    let packet = result.unwrap();
    assert_eq!(packet.header, 135);
    assert_eq!(packet.victim_vid, 1000);
    assert_eq!(packet.flags, 1);
    assert_eq!(packet.damage, 5000);
}

#[test]
fn test_gc_damage_info_too_short() {
    // Bufor krotszy niz 10 bajtow (np. 9 bajtow).
    let buf: [u8; 9] = [135, 0xE8, 0x03, 0x00, 0x00, 1, 0x88, 0x13, 0x00];

    let result = GcDamageInfoPacket::from_bytes(&buf);
    assert!(result.is_err());
    
    // Upewniamy sie ze blad mowi o zbyt krotkim buforze.
    let err_msg = result.err().unwrap();
    assert!(err_msg.contains("za krotki"));
}

#[test]
fn test_gc_damage_info_longer_buf() {
    // Bufor z prawidlowymi danymi oraz dodatkowymi smieciami na koncu.
    let buf: [u8; 12] = [
        135, 0xE8, 0x03, 0x00, 0x00, // header + victim_vid
        1,                         // flags
        0x88, 0x13, 0x00, 0x00,    // damage
        0xFF, 0xFF,                // dodatkowe smieci
    ];

    let result = GcDamageInfoPacket::from_bytes(&buf);
    assert!(result.is_ok());

    let packet = result.unwrap();
    assert_eq!(packet.header, 135);
    assert_eq!(packet.victim_vid, 1000);
    assert_eq!(packet.flags, 1);
    assert_eq!(packet.damage, 5000);
}
