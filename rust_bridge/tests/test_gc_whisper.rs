use m2_client_bridge::net::gc::whisper::{TPacketGCWhisper, C_TPacketGCWhisper, HEADER_GC_WHISPER, m2_net_parse_whisper, m2_net_free_whisper};
use std::ptr;
use std::ffi::CStr;

#[test]
fn test_from_bytes_success() {
    let mut buf = vec![0u8; 29 + 10]; // 29 MIN_PACKET_SIZE + 10 byte message
    buf[0] = HEADER_GC_WHISPER; // header
    
    let size = (29 + 10) as u16;
    buf[1..3].copy_from_slice(&size.to_le_bytes()); // size
    
    buf[3] = 1; // b_type
    
    // sz_name_from
    let name = "TestowyGracz";
    let name_bytes = name.as_bytes();
    buf[4..4+name_bytes.len()].copy_from_slice(name_bytes);
    
    // message
    let msg = "Hello 123";
    let msg_bytes = msg.as_bytes();
    buf[29..29+msg_bytes.len()].copy_from_slice(msg_bytes);
    
    let packet = TPacketGCWhisper::from_bytes(&buf).unwrap();
    assert_eq!(packet.header, HEADER_GC_WHISPER);
    assert_eq!(packet.size, size);
    assert_eq!(packet.b_type, 1);
    assert_eq!(packet.sz_name_from, name);
    assert_eq!(packet.message, msg);
}

#[test]
fn test_from_bytes_too_short() {
    let buf = vec![0u8; 28]; // Mniej niz MIN_PACKET_SIZE
    let result = TPacketGCWhisper::from_bytes(&buf);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_wrong_header() {
    let mut buf = vec![0u8; 35];
    buf[0] = 99; // Bledny naglowek
    let result = TPacketGCWhisper::from_bytes(&buf);
    assert!(result.is_err());
}

#[test]
fn test_ffi_parse_and_free() {
    let mut buf = vec![0u8; 29 + 4]; // 29 MIN_PACKET_SIZE + 4 byte message
    buf[0] = HEADER_GC_WHISPER;
    
    let size = (29 + 4) as u16;
    buf[1..3].copy_from_slice(&size.to_le_bytes());
    
    buf[3] = 2; // b_type
    
    let name = "Alice";
    buf[4..4+name.len()].copy_from_slice(name.as_bytes());
    
    let msg = "Hi!";
    buf[29..29+msg.len()].copy_from_slice(msg.as_bytes());
    
    let mut parsed_ptr: *mut C_TPacketGCWhisper = ptr::null_mut();
    
    unsafe {
        let success = m2_net_parse_whisper(buf.as_ptr(), buf.len(), &mut parsed_ptr);
        assert!(success);
        assert!(!parsed_ptr.is_null());
        
        let c_packet = &*parsed_ptr;
        assert_eq!(c_packet.header, HEADER_GC_WHISPER);
        assert_eq!(c_packet.size, size);
        assert_eq!(c_packet.b_type, 2);
        
        let c_name = CStr::from_ptr(c_packet.sz_name_from).to_str().unwrap();
        assert_eq!(c_name, name);
        
        let c_msg = CStr::from_ptr(c_packet.message).to_str().unwrap();
        assert_eq!(c_msg, msg);
        
        m2_net_free_whisper(parsed_ptr);
    }
}
