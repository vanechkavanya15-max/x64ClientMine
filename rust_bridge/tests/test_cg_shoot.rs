use m2_client_bridge::net::cg::shoot::{m2_net_send_shoot, TPacketCGShoot};

#[test]
fn test_cg_shoot_to_bytes() {
    let pkt = TPacketCGShoot::new(1);
    let bytes = pkt.to_bytes();
    assert_eq!(bytes.len(), 5);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x0403);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 5);
    assert_eq!(bytes[4], 1);
}

#[test]
fn test_cg_shoot_ffi() {
    unsafe {
        assert!(m2_net_send_shoot(2));
    }
}
