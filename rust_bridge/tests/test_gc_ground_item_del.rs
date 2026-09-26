use m2_client_bridge::net::gc::ground_item_del::{
    GroundItemDelPacket, TPacketGCItemGroundDel, HEADER_GC_ITEM_GROUND_DEL,
};

#[test]
fn test_ground_item_del_from_bytes_success() {
    let mut buf = [0u8; 8];
    buf[0..2].copy_from_slice(&HEADER_GC_ITEM_GROUND_DEL.to_le_bytes()); // header
    buf[2..4].copy_from_slice(&8u16.to_le_bytes());                      // length
    buf[4..8].copy_from_slice(&12345u32.to_le_bytes());                  // vid

    let packet = TPacketGCItemGroundDel::from_bytes(&buf).unwrap();
    assert_eq!({ packet.header }, HEADER_GC_ITEM_GROUND_DEL);
    assert_eq!({ packet.length }, 8);
    assert_eq!(packet.get_vid(), 12345);
}

#[test]
fn test_ground_item_del_from_bytes_too_short() {
    let buf = [0u8; 7];
    let result = GroundItemDelPacket::from_bytes(&buf);
    assert!(result.is_err());
}
