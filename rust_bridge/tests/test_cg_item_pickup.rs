use m2_client_bridge::net::cg::item_pickup::TPacketCGItemPickUp;
use std::mem::size_of;

#[test]
fn test_cg_item_pickup_serialization() {
    assert_eq!(size_of::<TPacketCGItemPickUp>(), 8);

    let packet = TPacketCGItemPickUp {
        header: 0x0505,
        length: 8,
        vid: 0x12345678,
    };

    let bytes = packet.to_bytes();

    assert_eq!(bytes.len(), 8);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x0505);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 8);
    assert_eq!(bytes[4], 0x78);
    assert_eq!(bytes[5], 0x56);
    assert_eq!(bytes[6], 0x34);
    assert_eq!(bytes[7], 0x12);
}

#[test]
fn test_cg_item_pickup_new() {
    let packet = TPacketCGItemPickUp::new(0xAABBCCDD);
    
    let header = { packet.header };
    let length = { packet.length };
    let vid = { packet.vid };

    assert_eq!(header, 0x0505);
    assert_eq!(length, 8);
    assert_eq!(vid, 0xAABBCCDD);
    
    let bytes = packet.to_bytes();
    assert_eq!(bytes, [0x05, 0x05, 8, 0, 0xDD, 0xCC, 0xBB, 0xAA]);
}
