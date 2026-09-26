use m2_client_bridge::net::cg::whisper::{m2_net_send_whisper, TPacketCGWhisper};
use std::ffi::CString;

#[test]
fn test_cg_whisper_to_bytes() {
    let bytes = TPacketCGWhisper::build_with_message("PlayerOne", "Czesc!");
    // base header struct: 2 + 2 + 65 = 69; msg: 6 bytes -> 75
    assert_eq!(bytes.len(), 75);
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0602); // CG::WHISPER
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 75);
    assert_eq!(&bytes[4..13], b"PlayerOne");
    assert_eq!(&bytes[69..], b"Czesc!");
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
