use m2_client::send_movement_packets_v2::*;
use std::mem::size_of;

#[test]
fn test_packet_size_and_header() {
    assert_eq!(size_of::<TPacketCGMove>(), 19);
    assert_eq!(TPacketCGMove::SIZE, 19);
    
    let packet = TPacketCGMove::new(1, 0, 10, 100, 200, 12345);
    assert_eq!({packet.header}, 0x0301);
    assert_eq!({packet.length}, 19);
}

#[test]
fn test_serialization() {
    let packet = TPacketCGMove::new(2, 0, 15, 1024, -512, 55555);
    let bytes = packet.to_bytes();
    
    assert_eq!(bytes.len(), 19);
    
    // Check header and length
    assert_eq!(bytes[0..2], 0x0301u16.to_le_bytes());
    assert_eq!(bytes[2..4], 19u16.to_le_bytes());
    // Check b_func, b_arg, b_rot
    assert_eq!(bytes[4], 2);
    assert_eq!(bytes[5], 0);
    assert_eq!(bytes[6], 15);
    
    // Check coordinates
    assert_eq!(&bytes[7..11], &1024i32.to_le_bytes());
    assert_eq!(&bytes[11..15], &(-512i32).to_le_bytes());
    
    // Check time
    assert_eq!(&bytes[15..19], &55555u32.to_le_bytes());
}

#[test]
fn test_m2_send_move_normal() {
    let mut buf = [0u8; 19];
    let written = m2_send_move(3, 1000, 2000, 45.0, 123456, buf.as_mut_ptr());
    
    assert_eq!(written, 19);
    assert_eq!(buf[4], 3); // func
    assert_eq!(buf[6], 9); // bRot = 45.0 / 5.0
    assert_eq!(&buf[7..11], &1000i32.to_le_bytes());
}

#[test]
fn test_m2_send_move_negative_rot() {
    let mut buf = [0u8; 19];
    // -45.0 -> +360.0 = 315.0. 315.0 / 5.0 = 63
    let written = m2_send_move(1, 0, 0, -45.0, 0, buf.as_mut_ptr());
    assert_eq!(written, 19);
    assert_eq!(buf[6], 63);
}

#[test]
fn test_m2_send_move_overflow_rot() {
    let mut buf = [0u8; 19];
    // 380.0 % 360.0 = 20.0. 20.0 / 5.0 = 4
    let written = m2_send_move(1, 0, 0, 380.0, 0, buf.as_mut_ptr());
    assert_eq!(written, 19);
    assert_eq!(buf[6], 4);
}

#[test]
fn test_m2_send_move_null_buffer() {
    let written = m2_send_move(1, 0, 0, 0.0, 0, std::ptr::null_mut());
    assert_eq!(written, 0);
}

#[test]
fn test_m2_send_move_negative_coordinates() {
    let mut buf = [0u8; 19];
    let written = m2_send_move(1, -9999, -8888, 0.0, 0, buf.as_mut_ptr());
    
    assert_eq!(written, 19);
    assert_eq!(&buf[7..11], &(-9999i32).to_le_bytes());
    assert_eq!(&buf[11..15], &(-8888i32).to_le_bytes());
}

#[test]
fn test_m2_send_move_zero_values() {
    let mut buf = [0u8; 19];
    let written = m2_send_move(0, 0, 0, 0.0, 0, buf.as_mut_ptr());
    
    assert_eq!(written, 19);
    assert_eq!(buf[4], 0); // func
    assert_eq!(buf[6], 0); // rot
    assert_eq!(&buf[7..11], &0i32.to_le_bytes());
    assert_eq!(&buf[11..15], &0i32.to_le_bytes());
    assert_eq!(&buf[15..19], &0u32.to_le_bytes());
}
