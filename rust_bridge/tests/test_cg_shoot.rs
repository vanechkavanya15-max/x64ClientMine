use m2_client_bridge::net::cg::shoot::{m2_net_send_shoot, TPacketCGShoot};

#[test]
fn test_cg_shoot_to_bytes() {
    let pkt = TPacketCGShoot::new(1);
    let bytes = pkt.to_bytes();
    assert_eq!(bytes, [54, 1]);
}

#[test]
fn test_cg_shoot_ffi() {
    unsafe {
        assert!(m2_net_send_shoot(2));
    }
}
