use m2_client_bridge::net::cg::target::{TPacketCGTarget, HEADER_CG_TARGET, m2_net_send_target};
use std::mem;

#[test]
fn test_cg_target_packet_size() {
    assert_eq!(
        mem::size_of::<TPacketCGTarget>(),
        5,
        "TPacketCGTarget powinien zajmowac dokladnie 5 bajtow"
    );
}

#[test]
fn test_cg_target_packet_layout() {
    let vid: u32 = 0x12345678;
    let packet = TPacketCGTarget::new(vid);

    assert_eq!(packet.header, HEADER_CG_TARGET);
    let packet_vid = { packet.dw_vid };
    assert_eq!(packet_vid, vid);

    let bytes = packet.to_bytes();
    assert_eq!(bytes[0], HEADER_CG_TARGET);
    
    // Sprawdzenie kodowania Little-Endian dla VID 0x12345678 to: 0x78, 0x56, 0x34, 0x12
    assert_eq!(bytes[1], 0x78);
    assert_eq!(bytes[2], 0x56);
    assert_eq!(bytes[3], 0x34);
    assert_eq!(bytes[4], 0x12);
}

#[test]
fn test_m2_net_send_target_ffi() {
    let result = m2_net_send_target(100);
    assert!(result, "m2_net_send_target powinno zwrocic true");
}
