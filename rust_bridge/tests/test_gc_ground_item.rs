use m2_client_bridge::net::gc::ground_item::GcItemGroundAddPacket;

#[test]
fn test_gc_ground_item_add_valid() {
    let mut buf = Vec::new();
    
    // header: 26
    buf.push(26);
    
    // x: 1000
    buf.extend_from_slice(&1000i32.to_le_bytes());
    
    // y: 2000
    buf.extend_from_slice(&2000i32.to_le_bytes());
    
    // z: 3000
    buf.extend_from_slice(&3000i32.to_le_bytes());
    
    // vid: 4000
    buf.extend_from_slice(&4000u32.to_le_bytes());
    
    // vnum: 5000
    buf.extend_from_slice(&5000u32.to_le_bytes());
    
    assert_eq!(buf.len(), 21);
    
    let packet = GcItemGroundAddPacket::from_bytes(&buf).unwrap();
    
    assert_eq!(packet.header, 26);
    assert_eq!(packet.x, 1000);
    assert_eq!(packet.y, 2000);
    assert_eq!(packet.z, 3000);
    assert_eq!(packet.vid, 4000);
    assert_eq!(packet.vnum, 5000);
}

#[test]
fn test_gc_ground_item_add_short_buffer() {
    let mut buf = Vec::new();
    
    // header: 26
    buf.push(26);
    
    // x: 1000
    buf.extend_from_slice(&1000i32.to_le_bytes());
    
    // Zbyt krotki bufor (tylko 5 bajtow)
    assert_eq!(buf.len(), 5);
    
    let result = GcItemGroundAddPacket::from_bytes(&buf);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Zbyt krotki bufor. Oczekiwano 21, otrzymano 5");
}
