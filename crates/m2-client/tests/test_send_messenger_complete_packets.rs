#[path = "../src/send_messenger_complete_packets.rs"]
mod send_messenger_complete_packets;

use send_messenger_complete_packets::*;
use std::mem::size_of;
use std::ptr;

#[test]
fn test_packet_layout_and_size() {
    assert_eq!(size_of::<TPacketCGMessenger>(), 5, "TPacketCGMessenger size must be exactly 5 bytes (16+16+8 bits)");
}

#[test]
fn test_send_messenger_add_by_vid() {
    let mut out_buf = [0u8; 128];
    let vid: u32 = 0x12345678;

    let len = unsafe { m2_send_messenger_add_by_vid(vid, out_buf.as_mut_ptr()) };

    assert_eq!(len, 9, "Total length should be 5 + 4 = 9 bytes");

    let header = u16::from_ne_bytes([out_buf[0], out_buf[1]]);
    let packet_length = u16::from_ne_bytes([out_buf[2], out_buf[3]]);
    let subheader = out_buf[4];
    let packet_vid = u32::from_ne_bytes([out_buf[5], out_buf[6], out_buf[7], out_buf[8]]);

    assert_eq!(header, CG_MESSENGER);
    assert_eq!(packet_length, 9);
    assert_eq!(subheader, messenger_sub::cg::ADD_BY_VID);
    assert_eq!(packet_vid, vid);
}

#[test]
fn test_send_messenger_add_by_name() {
    let mut out_buf = [0u8; 128];
    let name = b"TestName";
    
    let len = unsafe { m2_send_messenger_add_by_name(name.as_ptr(), name.len(), out_buf.as_mut_ptr()) };

    assert_eq!(len, 29, "Total length should be 5 + 24 = 29 bytes");

    let header = u16::from_ne_bytes([out_buf[0], out_buf[1]]);
    let packet_length = u16::from_ne_bytes([out_buf[2], out_buf[3]]);
    let subheader = out_buf[4];

    assert_eq!(header, CG_MESSENGER);
    assert_eq!(packet_length, 29);
    assert_eq!(subheader, messenger_sub::cg::ADD_BY_NAME);

    let mut expected_name = [0u8; CHARACTER_NAME_MAX_LEN];
    expected_name[..name.len()].copy_from_slice(name);

    assert_eq!(&out_buf[5..29], &expected_name);
}

#[test]
fn test_send_messenger_add_by_name_overflow() {
    let mut out_buf = [0u8; 128];
    let name = b"VeryLongNameThatExceeds24Characters";
    
    let len = unsafe { m2_send_messenger_add_by_name(name.as_ptr(), name.len(), out_buf.as_mut_ptr()) };

    assert_eq!(len, 29, "Total length should be 5 + 24 = 29 bytes");

    let mut expected_name = [0u8; CHARACTER_NAME_MAX_LEN];
    expected_name[..23].copy_from_slice(&name[..23]); // Max 23 chars + 1 null terminator
    expected_name[23] = 0; // Null terminator

    assert_eq!(&out_buf[5..29], &expected_name);
}

#[test]
fn test_send_messenger_remove() {
    let mut out_buf = [0u8; 128];
    let key = b"FriendKey";
    
    let len = unsafe { m2_send_messenger_remove(key.as_ptr(), key.len(), out_buf.as_mut_ptr()) };

    assert_eq!(len, 29, "Total length should be 5 + 24 = 29 bytes");

    let header = u16::from_ne_bytes([out_buf[0], out_buf[1]]);
    let packet_length = u16::from_ne_bytes([out_buf[2], out_buf[3]]);
    let subheader = out_buf[4];

    assert_eq!(header, CG_MESSENGER);
    assert_eq!(packet_length, 29);
    assert_eq!(subheader, messenger_sub::cg::REMOVE);

    let mut expected_key = [0u8; CHARACTER_NAME_MAX_LEN];
    expected_key[..key.len()].copy_from_slice(key);

    assert_eq!(&out_buf[5..29], &expected_key);
}

#[test]
fn test_send_messenger_null_pointers() {
    let mut out_buf = [0u8; 128];
    let name = b"TestName";

    let len1 = unsafe { m2_send_messenger_add_by_vid(123, ptr::null_mut()) };
    assert_eq!(len1, 0, "Should return 0 for null out_buf");

    let len2 = unsafe { m2_send_messenger_add_by_name(ptr::null(), name.len(), out_buf.as_mut_ptr()) };
    assert_eq!(len2, 0, "Should return 0 for null name ptr");

    let len3 = unsafe { m2_send_messenger_remove(ptr::null(), name.len(), out_buf.as_mut_ptr()) };
    assert_eq!(len3, 0, "Should return 0 for null key ptr");
}
