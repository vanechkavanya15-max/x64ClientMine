use m2_client_bridge::net::gc::whisper::{
    C_TPacketGCWhisper, GcWhisperPacket, TPacketGCWhisper,
    HEADER_GC_WHISPER, m2_net_parse_whisper, m2_net_free_whisper,
};
use std::ptr;
use std::ffi::CStr;

#[test]
fn test_from_bytes_success() {
    let mut buf = vec![0u8; 70 + 10]; // 70 MIN_PACKET_SIZE + 10 byte message
    buf[0..2].copy_from_slice(&HEADER_GC_WHISPER.to_le_bytes()); // header: u16
    
    let length = (70 + 10) as u16;
    buf[2..4].copy_from_slice(&length.to_le_bytes()); // length: u16
    
    buf[4] = 1; // bType
    
    // szNameFrom
    let name = "TestowyGracz";
    let name_bytes = name.as_bytes();
    buf[5..5 + name_bytes.len()].copy_from_slice(name_bytes);
    
    // message
    let msg = "Hello 123";
    let msg_bytes = msg.as_bytes();
    buf[70..70 + msg_bytes.len()].copy_from_slice(msg_bytes);
    
    let packet = GcWhisperPacket::from_bytes(&buf).unwrap();
    assert_eq!(packet.header, HEADER_GC_WHISPER);
    assert_eq!(packet.length, length);
    assert_eq!(packet.b_type, 1);
    assert_eq!(packet.sz_name_from, name);
    assert_eq!(packet.message, msg);
}

#[test]
fn test_from_bytes_too_short() {
    let buf = vec![0u8; 69]; // Mniej niz MIN_PACKET_SIZE
    let result = GcWhisperPacket::from_bytes(&buf);
    assert!(result.is_err());
}

#[test]
fn test_whisper_raw_struct() {
    let mut buf = [0u8; 70];
    buf[0..2].copy_from_slice(&HEADER_GC_WHISPER.to_le_bytes());
    buf[2..4].copy_from_slice(&70u16.to_le_bytes());
    buf[4] = 2;
    let name = b"Sender";
    buf[5..5 + name.len()].copy_from_slice(name);

    let raw = TPacketGCWhisper::from_bytes(&buf).unwrap();
    assert_eq!({ raw.header }, HEADER_GC_WHISPER);
    assert_eq!({ raw.length }, 70);
    assert_eq!(raw.bType, 2);
}

#[test]
fn test_ffi_parse_and_free() {
    let mut buf = vec![0u8; 70 + 4]; // 70 MIN_PACKET_SIZE + 4 byte message
    buf[0..2].copy_from_slice(&HEADER_GC_WHISPER.to_le_bytes());
    
    let length = (70 + 4) as u16;
    buf[2..4].copy_from_slice(&length.to_le_bytes());
    
    buf[4] = 2; // bType
    
    let name = "Alice";
    buf[5..5 + name.len()].copy_from_slice(name.as_bytes());
    
    let msg = "Hi!";
    buf[70..70 + msg.len()].copy_from_slice(msg.as_bytes());
    
    let mut parsed_ptr: *mut C_TPacketGCWhisper = ptr::null_mut();
    
    unsafe {
        let success = m2_net_parse_whisper(buf.as_ptr(), buf.len(), &mut parsed_ptr);
        assert!(success);
        assert!(!parsed_ptr.is_null());
        
        let c_packet = &*parsed_ptr;
        assert_eq!(c_packet.header, HEADER_GC_WHISPER);
        assert_eq!(c_packet.length, length);
        assert_eq!(c_packet.b_type, 2);
        
        let c_name = CStr::from_ptr(c_packet.sz_name_from).to_str().unwrap();
        assert_eq!(c_name, name);
        
        let c_msg = CStr::from_ptr(c_packet.message).to_str().unwrap();
        assert_eq!(c_msg, msg);
        
        m2_net_free_whisper(parsed_ptr);
    }
}
