#[path = "../src/chat_system_network.rs"]
mod chat_system_network;

use chat_system_network::*;
use std::ffi::CString;
use std::ptr;

#[test]
fn test_send_chat_packet_serialization() {
    let msg = CString::new("Hello, world!").unwrap();
    let mut out_buf = [0u8; 1024];

    let written = unsafe {
        m2_net_send_chat_v2(
            CHAT_TYPE_TALKING,
            msg.as_ptr(),
            out_buf.as_mut_ptr(),
            out_buf.len(),
        )
    };

    assert!(written > 0);
    
    let packet_size = std::mem::size_of::<TPacketCGChat>();
    let packet = unsafe { &*(out_buf.as_ptr() as *const TPacketCGChat) };

    assert_eq!(u16::from_le(packet.header), CG_CHAT);
    assert_eq!(u16::from_le(packet.length) as usize, packet_size + 13 + 1); // 13 chars + null
    assert_eq!(packet.type_, CHAT_TYPE_TALKING);

    let text_slice = &out_buf[packet_size..packet_size + 13];
    assert_eq!(text_slice, b"Hello, world!");
    assert_eq!(out_buf[packet_size + 13], 0);
}

#[test]
fn test_send_whisper_packet_serialization() {
    let target = CString::new("TargetUser").unwrap();
    let msg = CString::new("Secret message").unwrap();
    let mut out_buf = [0u8; 1024];

    let written = unsafe {
        m2_net_send_whisper_v2(
            target.as_ptr(),
            msg.as_ptr(),
            out_buf.as_mut_ptr(),
            out_buf.len(),
        )
    };

    assert!(written > 0);

    let packet_size = std::mem::size_of::<TPacketCGWhisper>();
    let packet = unsafe { &*(out_buf.as_ptr() as *const TPacketCGWhisper) };

    assert_eq!(u16::from_le(packet.header), CG_WHISPER);
    assert_eq!(u16::from_le(packet.length) as usize, packet_size + 14 + 1); // 14 chars + null

    let target_str = CString::new("TargetUser").unwrap();
    let target_bytes = target_str.to_bytes();
    assert_eq!(&packet.sz_name_to[..target_bytes.len()], target_bytes);
    assert_eq!(packet.sz_name_to[target_bytes.len()], 0);

    let text_slice = &out_buf[packet_size..packet_size + 14];
    assert_eq!(text_slice, b"Secret message");
    assert_eq!(out_buf[packet_size + 14], 0);
}

#[test]
fn test_send_whisper_target_truncation() {
    let long_target = "A".repeat(50);
    let target = CString::new(long_target).unwrap();
    let msg = CString::new("Hi").unwrap();
    let mut out_buf = [0u8; 1024];

    let written = unsafe {
        m2_net_send_whisper_v2(
            target.as_ptr(),
            msg.as_ptr(),
            out_buf.as_mut_ptr(),
            out_buf.len(),
        )
    };

    assert!(written > 0);
    let packet = unsafe { &*(out_buf.as_ptr() as *const TPacketCGWhisper) };

    // Assert that the name was truncated to CHARACTER_NAME_MAX_LEN (24)
    assert_eq!(packet.sz_name_to[CHARACTER_NAME_MAX_LEN - 1], b'A');
    assert_eq!(packet.sz_name_to[CHARACTER_NAME_MAX_LEN], 0); // null termination
}

#[test]
fn test_send_chat_exceed_length() {
    let long_msg = "A".repeat(600);
    let msg = CString::new(long_msg).unwrap();
    let mut out_buf = [0u8; 1024];

    let written = unsafe {
        m2_net_send_chat_v2(
            CHAT_TYPE_TALKING,
            msg.as_ptr(),
            out_buf.as_mut_ptr(),
            out_buf.len(),
        )
    };

    assert_eq!(written, 0, "Messages >= 512 bytes should be rejected");
}

#[test]
fn test_send_whisper_exceed_length() {
    let long_msg = "A".repeat(600);
    let target = CString::new("Target").unwrap();
    let msg = CString::new(long_msg).unwrap();
    let mut out_buf = [0u8; 1024];

    let written = unsafe {
        m2_net_send_whisper_v2(
            target.as_ptr(),
            msg.as_ptr(),
            out_buf.as_mut_ptr(),
            out_buf.len(),
        )
    };

    assert_eq!(written, 0, "Whisper messages >= 512 bytes should be rejected");
}

#[test]
fn test_send_chat_command_filtering() {
    let msg = CString::new("/dance").unwrap();
    let mut out_buf = [0u8; 1024];

    let written = unsafe {
        m2_net_send_chat_v2(
            CHAT_TYPE_TALKING,
            msg.as_ptr(),
            out_buf.as_mut_ptr(),
            out_buf.len(),
        )
    };

    assert!(written > 0);
    let packet_size = std::mem::size_of::<TPacketCGChat>();
    let packet = unsafe { &*(out_buf.as_ptr() as *const TPacketCGChat) };

    // Should change type to CHAT_TYPE_COMMAND and truncate the slash
    assert_eq!(packet.type_, CHAT_TYPE_COMMAND);
    
    let text_slice = &out_buf[packet_size..packet_size + 5];
    assert_eq!(text_slice, b"dance");
}

static TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn test_chat_history_buffer_limit() {
    let _guard = TEST_MUTEX.lock().unwrap();
    clear_chat_history();
    
    for i in 0..350 {
        push_chat_message(CHAT_TYPE_TALKING, format!("Message {}", i), None);
    }
    
    let history = get_chat_history_clone();
    assert_eq!(history.len(), CHAT_HISTORY_MAX_LEN.min(300));
    
    // The oldest message should be "Message 50"
    assert_eq!(history[0].message, "Message 50");
    // The newest message should be "Message 349"
    assert_eq!(history[299].message, "Message 349");
}

#[test]
fn test_recv_chat_packet() {
    let _guard = TEST_MUTEX.lock().unwrap();
    clear_chat_history();

    let mut buf = vec![0u8; std::mem::size_of::<TPacketGCChat>() + 4]; // 3 chars + null
    let packet = unsafe { &mut *(buf.as_mut_ptr() as *mut TPacketGCChat) };
    packet.header = GC_CHAT;
    packet.length = buf.len() as u16;
    packet.type_ = CHAT_TYPE_GUILD;
    packet.dw_vid = 1234;
    packet.b_empire = 1;
    
    let text_ptr = unsafe { buf.as_mut_ptr().add(std::mem::size_of::<TPacketGCChat>()) };
    unsafe {
        ptr::copy_nonoverlapping(b"Hey\0".as_ptr(), text_ptr, 4);
    }

    let result = unsafe { m2_net_recv_chat(buf.as_ptr(), buf.len()) };
    assert!(result);

    let history = get_chat_history_clone();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].chat_type, CHAT_TYPE_GUILD);
    assert_eq!(history[0].message, "Hey");
    assert!(history[0].sender.is_none());
}

#[test]
fn test_recv_whisper_packet() {
    let _guard = TEST_MUTEX.lock().unwrap();
    clear_chat_history();

    let mut buf = vec![0u8; std::mem::size_of::<TPacketGCWhisper>() + 5]; // 4 chars + null
    let packet = unsafe { &mut *(buf.as_mut_ptr() as *mut TPacketGCWhisper) };
    packet.header = GC_WHISPER;
    packet.length = buf.len() as u16;
    packet.type_ = CHAT_TYPE_WHISPER;
    
    // Set sender name
    let sender = b"Tester\0";
    unsafe {
        ptr::copy_nonoverlapping(sender.as_ptr(), packet.sz_name_from.as_mut_ptr(), sender.len());
    }

    let text_ptr = unsafe { buf.as_mut_ptr().add(std::mem::size_of::<TPacketGCWhisper>()) };
    unsafe {
        ptr::copy_nonoverlapping(b"Psst\0".as_ptr(), text_ptr, 5);
    }

    let result = unsafe { m2_net_recv_whisper(buf.as_ptr(), buf.len()) };
    assert!(result);

    let history = get_chat_history_clone();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].chat_type, CHAT_TYPE_WHISPER);
    assert_eq!(history[0].message, "Psst");
    assert_eq!(history[0].sender.as_deref(), Some("Tester"));
}
