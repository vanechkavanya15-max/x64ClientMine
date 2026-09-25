use m2_client_bridge::net::cg::sync_position::{
    TPacketCGSyncPositionElement, TPacketCGSyncPosition, HEADER_CG_SYNC_POSITION, m2_net_send_sync_position
};

#[test]
fn test_sync_position_element_to_bytes() {
    let element = TPacketCGSyncPositionElement {
        dw_vid: 0x12345678,
        l_x: -100, // 0xFFFFFF9C in 32-bit two's complement
        l_y: 200,  // 0x000000C8
    };

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
    let packet = TPacketCGSyncPosition::new(vec![
        TPacketCGSyncPositionElement {
            dw_vid: 1,
            l_x: 10,
            l_y: 20,
        },
        TPacketCGSyncPositionElement {
            dw_vid: 2,
            l_x: 30,
            l_y: 40,
        },
    ]);

    let bytes = packet.to_bytes();
    
    // total size: 5 (header + wSize + count) + 2 * 12 = 29
    assert_eq!(bytes.len(), 29);
    assert_eq!(packet.w_size, 29);
    assert_eq!(packet.count, 2);
    
    // header
    assert_eq!(bytes[0], HEADER_CG_SYNC_POSITION);
    
    // wSize: 29 -> 1D 00
    assert_eq!(&bytes[1..3], &[0x1d, 0x00]);

    // count: 2 -> 02 00
    assert_eq!(&bytes[3..5], &[0x02, 0x00]);
    
    // element 1 dw_vid
    assert_eq!(&bytes[5..9], &[0x01, 0x00, 0x00, 0x00]);
}

#[test]
fn test_m2_net_send_sync_position() {
    let result = unsafe { m2_net_send_sync_position(1234, 100, 200) };
    assert!(result);
}
