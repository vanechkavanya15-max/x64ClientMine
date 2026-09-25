use m2_client_bridge::net::gc::item_del::GcItemDel;

#[test]
fn test_gc_item_del_from_bytes_success() {
    let buf = [20, 5];
    let result = GcItemDel::from_bytes(&buf);
    
    assert!(result.is_ok());
    let packet = result.unwrap();
    assert_eq!(packet.header, 20);
    assert_eq!(packet.pos, 5);
}

#[test]
fn test_gc_item_del_from_bytes_truncated_buffer() {
    let buf = [20];
    let result = GcItemDel::from_bytes(&buf);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest za krotki aby zdeserializowac GcItemDel");
}

#[test]
fn test_gc_item_del_from_bytes_empty_buffer() {
    let buf: [u8; 0] = [];
    let result = GcItemDel::from_bytes(&buf);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest za krotki aby zdeserializowac GcItemDel");
}
