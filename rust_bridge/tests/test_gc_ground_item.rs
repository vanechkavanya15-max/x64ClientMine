use m2_client_bridge::net::gc::ground_item::{
    GcItemGroundAddPacket, TPacketGCItemGroundAdd, HEADER_GC_ITEM_GROUND_ADD,
};

#[test]
fn test_ground_item_parse_valid() {
    let mut buf = [0u8; 24];
    buf[0..2].copy_from_slice(&HEADER_GC_ITEM_GROUND_ADD.to_le_bytes()); // header
    buf[2..4].copy_from_slice(&24u16.to_le_bytes());                     // length
    buf[4..8].copy_from_slice(&1000i32.to_le_bytes());                   // lX
    buf[8..12].copy_from_slice(&2000i32.to_le_bytes());                  // lY
    buf[12..16].copy_from_slice(&3000i32.to_le_bytes());                 // lZ
    buf[16..20].copy_from_slice(&50001u32.to_le_bytes());                // dwVID
    buf[20..24].copy_from_slice(&10u32.to_le_bytes());                   // dwVnum

    let packet = TPacketGCItemGroundAdd::from_bytes(&buf).expect("Powinno sparsowac poprawny bufor");

    assert_eq!({ packet.header }, HEADER_GC_ITEM_GROUND_ADD);
    assert_eq!({ packet.length }, 24);
    assert_eq!(packet.x(), 1000);
    assert_eq!(packet.y(), 2000);
    assert_eq!(packet.z(), 3000);
    assert_eq!(packet.vid(), 50001);
    assert_eq!(packet.vnum(), 10);
}

#[test]
fn test_ground_item_parse_too_short() {
    let buf = [0u8; 23];
    let result = GcItemGroundAddPacket::from_bytes(&buf);
    assert!(result.is_err());
}
