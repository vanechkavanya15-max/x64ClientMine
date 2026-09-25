use m2_client_bridge::net::cg::item_move::{m2_net_send_item_move, TPacketCGItemMove};

#[test]
fn test_item_move_packet_to_bytes() {
    let pkt = TPacketCGItemMove::new(1, 45, 1, 50, 5);
    let bytes = pkt.to_bytes();
    assert_eq!(bytes.len(), 8);
    assert_eq!(bytes[0], 13); // HEADER_CG_ITEM_MOVE
    assert_eq!(bytes[1], 1);  // src window
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 45); // src cell
    assert_eq!(bytes[4], 1);  // dst window
    assert_eq!(u16::from_le_bytes([bytes[5], bytes[6]]), 50); // dst cell
    assert_eq!(bytes[7], 5);  // count
}

#[test]
fn test_item_move_ffi() {
    unsafe {
        let res = m2_net_send_item_move(1, 0, 1, 10, 1);
        assert!(res);
    }
}
