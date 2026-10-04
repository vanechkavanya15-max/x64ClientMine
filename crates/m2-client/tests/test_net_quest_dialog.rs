#[path = "../src/net_quest_dialog_packets.rs"]
mod net_quest_dialog_packets;

use net_quest_dialog_packets::*;
use std::ffi::CString;

#[test]
fn test_packet_cg_quest_confirm_creation() {
    let packet = TPacketCGQuestConfirm::new(1, 12345);
    
    assert_eq!({packet.header}, 0x0905);
    assert_eq!({packet.length} as usize, std::mem::size_of::<TPacketCGQuestConfirm>());
    assert_eq!({packet.answer}, 1);
    assert_eq!({packet.request_pid}, 12345);
    
    let bytes = packet.to_bytes();
    assert_eq!(bytes.len(), std::mem::size_of::<TPacketCGQuestConfirm>());
}

#[test]
fn test_packet_cg_quest_input_string_creation() {
    let input = "Hello, Metin2!";
    let packet = TPacketCGQuestInputString::new(input);
    
    assert_eq!({packet.header}, 0x0904);
    assert_eq!({packet.length} as usize, std::mem::size_of::<TPacketCGQuestInputString>());
    
    // Check if the string is correctly copied
    let sz_string = packet.sz_string; // Copy the array
    assert_eq!(&sz_string[..input.len()], input.as_bytes());
    // Check null termination
    assert_eq!(sz_string[input.len()], 0);
    
    let bytes = packet.to_bytes();
    assert_eq!(bytes.len(), std::mem::size_of::<TPacketCGQuestInputString>());
}

#[test]
fn test_packet_cg_quest_input_string_overflow() {
    let long_input = "A".repeat(100);
    let packet = TPacketCGQuestInputString::new(&long_input);
    
    // Only max 64 bytes should be copied
    let sz_string = packet.sz_string; // Copy the array
    assert_eq!(&sz_string[..64], "A".repeat(64).as_bytes());
    // The 65th byte should be null terminator (or at least 0 from initialization and no overwrite)
    assert_eq!(sz_string[64], 0);
}

#[test]
fn test_ffi_m2_net_send_quest_confirm_packet() {
    let result = m2_net_send_quest_confirm_packet(9999, 1);
    assert!(result);
    
    let sent = get_last_sent_packet().expect("Packet should be sent");
    assert_eq!(sent.len(), std::mem::size_of::<TPacketCGQuestConfirm>());
    
    // Check if header is correctly serialized
    let header = u16::from_ne_bytes([sent[0], sent[1]]);
    assert_eq!(header, 0x0905);
}

#[test]
fn test_ffi_m2_net_send_quest_input_string_packet() {
    let valid_string = CString::new("Test Quest Input").unwrap();
    let result = m2_net_send_quest_input_string_packet(valid_string.as_ptr());
    assert!(result);
    
    let sent = get_last_sent_packet().expect("Packet should be sent");
    assert_eq!(sent.len(), std::mem::size_of::<TPacketCGQuestInputString>());
    
    let header = u16::from_ne_bytes([sent[0], sent[1]]);
    assert_eq!(header, 0x0904);
    
    let null_result = m2_net_send_quest_input_string_packet(std::ptr::null());
    assert!(!null_result);
}
