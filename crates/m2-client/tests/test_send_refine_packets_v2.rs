use m2_client::send_refine_packets_v2::{m2_send_refine, RefineType, TPacketCGRefine, HEADER_CG_REFINE};
use std::mem::size_of;

#[test]
fn test_refine_packet_layout_and_size() {
    assert_eq!(size_of::<TPacketCGRefine>(), 6, "TPacketCGRefine should be exactly 6 bytes according to C++ Packet.h layout");

    let packet = TPacketCGRefine::new(12, RefineType::MagicMetal as u8);
    
    // Copy the fields to local variables to avoid unaligned references in assert_eq! macro
    let header = packet.header;
    let length = packet.length;
    let pos = packet.pos;
    let type_ = packet.type_;

    assert_eq!(header, HEADER_CG_REFINE);
    assert_eq!(length, 6);
    assert_eq!(pos, 12);
    assert_eq!(type_, 2);
}

#[test]
fn test_refine_type_enum_values() {
    assert_eq!(RefineType::Normal as u8, 0);
    assert_eq!(RefineType::BlessingScroll as u8, 1);
    assert_eq!(RefineType::MagicMetal as u8, 2);
}

#[test]
fn test_m2_send_refine_null_pointer() {
    let result = unsafe { m2_send_refine(5, RefineType::Normal as u8, std::ptr::null_mut()) };
    assert_eq!(result, 0, "m2_send_refine should return 0 if out_buf is null");
}

#[test]
fn test_m2_send_refine_success() {
    let mut buf = [0u8; 10];
    let expected_pos: u16 = 250;
    let expected_type = RefineType::BlessingScroll as u8;
    
    let written = unsafe { m2_send_refine(expected_pos, expected_type, buf.as_mut_ptr()) };
    
    assert_eq!(written, 6, "m2_send_refine should return the number of bytes written (6)");
    
    // Manual byte layout verification
    // bytes 0-1: header (0x050C, little endian)
    assert_eq!(buf[0], 0x0C);
    assert_eq!(buf[1], 0x05);
    
    // bytes 2-3: length (6, little endian)
    assert_eq!(buf[2], 0x06);
    assert_eq!(buf[3], 0x00);
    
    // byte 4: pos
    assert_eq!(buf[4], expected_pos as u8);
    
    // byte 5: type_
    assert_eq!(buf[5], expected_type);
}

#[test]
fn test_m2_send_refine_truncation_of_pos() {
    let mut buf = [0u8; 6];
    let large_pos: u16 = 256 + 42; // Should truncate to 42 when cast to u8
    let expected_type = RefineType::MagicMetal as u8;
    
    let written = unsafe { m2_send_refine(large_pos, expected_type, buf.as_mut_ptr()) };
    
    assert_eq!(written, 6);
    assert_eq!(buf[4], 42, "pos parameter should correctly truncate via standard u16 as u8 cast");
}

#[test]
fn test_tpacketcgrefine_derives() {
    // Tests Clone, Copy, PartialEq, Eq on RefineType
    let t1 = RefineType::Normal;
    let t2 = t1.clone();
    assert_eq!(t1, t2);
}

#[test]
fn test_m2_send_refine_normal_type() {
    let mut buf = [0u8; 6];
    let result = unsafe { m2_send_refine(10, RefineType::Normal as u8, buf.as_mut_ptr()) };
    
    assert_eq!(result, 6);
    assert_eq!(buf[5], 0, "Normal refine type should write 0 to buffer");
}
