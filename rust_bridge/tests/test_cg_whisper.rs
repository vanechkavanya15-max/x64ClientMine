use m2_client_bridge::net::cg::whisper::{m2_net_send_whisper, TPacketCGWhisper};
use std::ffi::CString;

#[test]
fn test_cg_whisper_to_bytes() {
    let pkt = TPacketCGWhisper::new("PlayerOne", "Czesc!");
    let bytes = pkt.to_bytes();
    assert_eq!(bytes[0], 19); // HEADER_CG_WHISPER
    let sz = u16::from_le_bytes([bytes[1], bytes[2]]);
    assert_eq!(sz, 1 + 2 + 25 + 6);
    assert_eq!(&bytes[3..12], b"PlayerOne");
    assert_eq!(&bytes[28..], b"Czesc!");
}

#[test]
fn test_cg_whisper_ffi() {
    unsafe {
        let t = CString::new("PlayerTwo").unwrap();
        let m = CString::new("Witam").unwrap();
        assert!(m2_net_send_whisper(t.as_ptr(), m.as_ptr()));
        assert!(!m2_net_send_whisper(std::ptr::null(), m.as_ptr()));
    }
}
