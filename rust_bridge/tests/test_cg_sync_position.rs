use m2_client_bridge::net::cg::sync_position::{CGSyncPositionElement, CGSyncPositionPacket, HEADER_CG_SYNC_POSITION};

#[test]
fn test_sync_position_element_to_bytes() {
    let element = CGSyncPositionElement {
        vid: 0x12345678,
        x: -100, // 0xFFFFFF9C in 32-bit two's complement
        y: 200,  // 0x000000C8
    };

    let bytes = element.to_bytes();
    assert_eq!(bytes.len(), 12);
    
    // vid: 0x12345678 -> 78 56 34 12
    assert_eq!(&bytes[0..4], &[0x78, 0x56, 0x34, 0x12]);
    // x: -100 -> 9C FF FF FF
    assert_eq!(&bytes[4..8], &[0x9c, 0xff, 0xff, 0xff]);
    // y: 200 -> C8 00 00 00
    assert_eq!(&bytes[8..12], &[0xc8, 0x00, 0x00, 0x00]);
}

#[test]
fn test_sync_position_packet_to_bytes() {
    let packet = CGSyncPositionPacket {
        elements: vec![
            CGSyncPositionElement {
                vid: 1,
                x: 10,
                y: 20,
            },
            CGSyncPositionElement {
                vid: 2,
                x: 30,
                y: 40,
            },
        ],
    };

    let bytes = packet.to_bytes();
    
    // total size: 3 (header + wSize) + 2 * 12 = 27
    assert_eq!(bytes.len(), 27);
    
    // header
    assert_eq!(bytes[0], HEADER_CG_SYNC_POSITION);
    
    // wSize: 27 -> 1B 00
    assert_eq!(&bytes[1..3], &[0x1b, 0x00]);
    
    // element 1 vid
    assert_eq!(&bytes[3..7], &[0x01, 0x00, 0x00, 0x00]);
}
