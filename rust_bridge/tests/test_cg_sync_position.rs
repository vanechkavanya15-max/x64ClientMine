use m2_client_bridge::net::cg::sync_position::{
    TPacketCGSyncPositionElement, TPacketCGSyncPosition, HEADER_CG_SYNC_POSITION, m2_net_send_sync_position
};

#[test]
fn test_sync_position_element_to_bytes() {
    let element = TPacketCGSyncPositionElement::new(
        0x12345678,
        -100, // 0xFFFFFF9C in 32-bit two's complement
        200,  // 0x000000C8
    );

    let bytes = element.to_bytes();
    assert_eq!(bytes.len(), 12);
    
    // dw_vid: 0x12345678 -> 78 56 34 12
    assert_eq!(&bytes[0..4], &[0x78, 0x56, 0x34, 0x12]);
    // l_x: -100 -> 9C FF FF FF
    assert_eq!(&bytes[4..8], &[0x9c, 0xff, 0xff, 0xff]);
    // l_y: 200 -> C8 00 00 00
    assert_eq!(&bytes[8..12], &[0xc8, 0x00, 0x00, 0x00]);
}

#[test]
fn test_sync_position_packet_to_bytes() {
    let elements = vec![
        TPacketCGSyncPositionElement::new(1, 10, 20),
        TPacketCGSyncPositionElement::new(2, 30, 40),
    ];

    let bytes = TPacketCGSyncPosition::serialize_with_elements(&elements);
    
    // total size: 4 (header + length) + 2 * 12 = 28
    assert_eq!(bytes.len(), 28);
    
    // header
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, HEADER_CG_SYNC_POSITION);
    
    // length: 28 -> 1C 00
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 28);
    
    // element 1 dw_vid
    assert_eq!(&bytes[4..8], &[0x01, 0x00, 0x00, 0x00]);
}

#[test]
fn test_m2_net_send_sync_position() {
    let result = unsafe { m2_net_send_sync_position(1234, 100, 200) };
    assert!(result);
}
