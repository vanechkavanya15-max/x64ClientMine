use m2_client::send_chat_packets_v2::*;
use std::mem::size_of;
use std::ptr;
use std::slice;

#[test]
fn test_m2_send_chat_normal() {
    let msg = b"Hello, world!";
    let chat_type = 1;
    let mut out_buf = vec![0u8; 1024];

    unsafe {
        let size = m2_send_chat(chat_type, msg.as_ptr(), msg.len(), out_buf.as_mut_ptr());
        
        let expected_size = size_of::<TPacketCGChat>() + msg.len() + 1;
        assert_eq!(size, expected_size);

        let packet_header = std::ptr::read_unaligned((out_buf.as_ptr() as *const TPacketCGChat).cast::<u16>());
        assert_eq!(packet_header, CG_CHAT);
        let packet_length = std::ptr::read_unaligned(out_buf.as_ptr().add(2) as *const u16);
        assert_eq!(packet_length, expected_size as u16);
        let packet_type = std::ptr::read_unaligned(out_buf.as_ptr().add(4) as *const u8);
        assert_eq!(packet_type, chat_type);

        let msg_in_buf = slice::from_raw_parts(
            out_buf.as_ptr().add(size_of::<TPacketCGChat>()),
            msg.len() + 1,
        );
        assert_eq!(&msg_in_buf[..msg.len()], msg);
        assert_eq!(msg_in_buf[msg.len()], 0); // null terminator
    }
}

#[test]
fn test_m2_send_chat_empty() {
    let msg = b"";
    let chat_type = 1;
    let mut out_buf = vec![0u8; 1024];

    unsafe {
        let size = m2_send_chat(chat_type, msg.as_ptr(), msg.len(), out_buf.as_mut_ptr());
        assert_eq!(size, 0);
    }
}

#[test]
fn test_m2_send_chat_too_long() {
    let msg = vec![b'A'; 512];
    let chat_type = 1;
    let mut out_buf = vec![0u8; 1024];

    unsafe {
        let size = m2_send_chat(chat_type, msg.as_ptr(), msg.len(), out_buf.as_mut_ptr());
        assert_eq!(size, 0); // returns 0 for lengths >= 512
    }
}

#[test]
fn test_m2_send_chat_null_ptrs() {
    let msg = b"test";
    let mut out_buf = vec![0u8; 1024];

    unsafe {
        let size1 = m2_send_chat(1, ptr::null(), msg.len(), out_buf.as_mut_ptr());
        assert_eq!(size1, 0);

        let size2 = m2_send_chat(1, msg.as_ptr(), msg.len(), ptr::null_mut());
        assert_eq!(size2, 0);
    }
}

#[test]
fn test_m2_send_whisper_normal() {
    let name = b"TargetPlayer";
    let msg = b"Hello from Rust!";
    let mut out_buf = vec![0u8; 1024];

    unsafe {
        let size = m2_send_whisper(
            name.as_ptr(),
            name.len(),
            msg.as_ptr(),
            msg.len(),
            out_buf.as_mut_ptr(),
        );

        let expected_size = size_of::<TPacketCGWhisper>() + msg.len() + 1;
        assert_eq!(size, expected_size);

        let packet_header = std::ptr::read_unaligned((out_buf.as_ptr() as *const TPacketCGWhisper).cast::<u16>());
        assert_eq!(packet_header, CG_WHISPER);
        let packet_length = std::ptr::read_unaligned(out_buf.as_ptr().add(2) as *const u16);
        assert_eq!(packet_length, expected_size as u16);

        // Check name
        let mut expected_name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
        expected_name[..name.len()].copy_from_slice(name);
        
        let packet = std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGWhisper);
        assert_eq!(packet.sz_name_to, expected_name);

        let msg_in_buf = slice::from_raw_parts(
            out_buf.as_ptr().add(size_of::<TPacketCGWhisper>()),
            msg.len() + 1,
        );
        assert_eq!(&msg_in_buf[..msg.len()], msg);
        assert_eq!(msg_in_buf[msg.len()], 0); // null terminator
    }
}

#[test]
fn test_m2_send_whisper_name_too_long() {
    let name = vec![b'A'; 30]; // longer than CHARACTER_NAME_MAX_LEN (24)
    let msg = b"Hi!";
    let mut out_buf = vec![0u8; 1024];

    unsafe {
        let size = m2_send_whisper(
            name.as_ptr(),
            name.len(),
            msg.as_ptr(),
            msg.len(),
            out_buf.as_mut_ptr(),
        );

        let expected_size = size_of::<TPacketCGWhisper>() + msg.len() + 1;
        assert_eq!(size, expected_size);

        let packet = std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGWhisper);
        
        let mut expected_name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
        expected_name[..CHARACTER_NAME_MAX_LEN].copy_from_slice(&name[..CHARACTER_NAME_MAX_LEN]);
        assert_eq!(packet.sz_name_to, expected_name);
    }
}

#[test]
fn test_m2_send_whisper_msg_too_long() {
    let name = b"Target";
    let msg = vec![b'B'; 255]; // Whisper msg limit is 255
    let mut out_buf = vec![0u8; 1024];

    unsafe {
        let size = m2_send_whisper(
            name.as_ptr(),
            name.len(),
            msg.as_ptr(),
            msg.len(),
            out_buf.as_mut_ptr(),
        );
        
        assert_eq!(size, 0); // fails validation
    }
}

#[test]
fn test_m2_send_whisper_null_ptrs() {
    let name = b"Target";
    let msg = b"Msg";
    let mut out_buf = vec![0u8; 1024];

    unsafe {
        let size1 = m2_send_whisper(ptr::null(), name.len(), msg.as_ptr(), msg.len(), out_buf.as_mut_ptr());
        assert_eq!(size1, 0);

        let size2 = m2_send_whisper(name.as_ptr(), name.len(), ptr::null(), msg.len(), out_buf.as_mut_ptr());
        assert_eq!(size2, 0);

        let size3 = m2_send_whisper(name.as_ptr(), name.len(), msg.as_ptr(), msg.len(), ptr::null_mut());
        assert_eq!(size3, 0);
    }
}
