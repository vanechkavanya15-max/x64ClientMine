// Testy jednostkowe dla modulu wysylania quickslotow i questow
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use m2_client::send_quickslot_quest_v2::{
    SendQuickSlotAddPacket, m2_send_script_answer, TPacketCGQuickSlotAdd, TPacketCGScriptAnswer,
    TQuickSlot, HEADER_CG_QUICKSLOT_ADD, HEADER_CG_SCRIPT_ANSWER,
};
use m2_client_bridge::net::send_queue::{
    clear_outgoing_packets, get_last_outgoing_packet, outgoing_packet_count,
};
use std::ptr::addr_of;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_tquickslot_creation() {
    let slot = TQuickSlot::new(5, 12);
    assert_eq!(slot.type_, 5);
    assert_eq!(slot.position, 12);
}

#[test]
fn test_packet_cg_quickslot_add_serialization() {
    let packet = TPacketCGQuickSlotAdd::new(1, 2, 3);
    assert_eq!(unsafe { addr_of!(packet.header).read_unaligned() }, HEADER_CG_QUICKSLOT_ADD);
    assert_eq!(unsafe { addr_of!(packet.length).read_unaligned() }, std::mem::size_of::<TPacketCGQuickSlotAdd>() as u16);
    assert_eq!(packet.pos, 1);
    assert_eq!(packet.slot.type_, 2);
    assert_eq!(packet.slot.position, 3);

    let bytes = packet.to_bytes();
    assert_eq!(bytes.len(), 7); // 2 (header) + 2 (length) + 1 (pos) + 1 (type) + 1 (position)
    
    // Check specific bytes
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_QUICKSLOT_ADD);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 7);
    assert_eq!(bytes[4], 1); // pos
    assert_eq!(bytes[5], 2); // slot type
    assert_eq!(bytes[6], 3); // slot position
}

#[test]
fn test_packet_cg_script_answer_serialization() {
    let packet = TPacketCGScriptAnswer::new(42);
    assert_eq!(unsafe { addr_of!(packet.header).read_unaligned() }, HEADER_CG_SCRIPT_ANSWER);
    assert_eq!(unsafe { addr_of!(packet.length).read_unaligned() }, std::mem::size_of::<TPacketCGScriptAnswer>() as u16);
    assert_eq!(packet.answer, 42);

    let bytes = packet.to_bytes();
    assert_eq!(bytes.len(), 5); // 2 (header) + 2 (length) + 1 (answer)
    
    // Check specific bytes
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_SCRIPT_ANSWER);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 5);
    assert_eq!(bytes[4], 42); // answer
}

#[test]
fn test_send_quickslot_add_packet_ffi() {
    let _lock = TEST_LOCK.lock().unwrap();
    clear_outgoing_packets();
    
    let result = SendQuickSlotAddPacket(10, 3, 5);
    assert!(result);
    assert_eq!(outgoing_packet_count(), 1);
    
    let queued_bytes = get_last_outgoing_packet().unwrap();
    assert_eq!(queued_bytes.len(), 7);
    assert_eq!(u16::from_le_bytes([queued_bytes[0], queued_bytes[1]]), HEADER_CG_QUICKSLOT_ADD);
    assert_eq!(u16::from_le_bytes([queued_bytes[2], queued_bytes[3]]), 7);
    assert_eq!(queued_bytes[4], 10);
    assert_eq!(queued_bytes[5], 3);
    assert_eq!(queued_bytes[6], 5);
}

#[test]
fn test_m2_send_script_answer_ffi_valid_buffer() {
    let mut out_buf = vec![0u8; 10];
    let written = unsafe { m2_send_script_answer(99, out_buf.as_mut_ptr()) };
    
    assert_eq!(written, 5);
    assert_eq!(u16::from_le_bytes([out_buf[0], out_buf[1]]), HEADER_CG_SCRIPT_ANSWER);
    assert_eq!(u16::from_le_bytes([out_buf[2], out_buf[3]]), 5);
    assert_eq!(out_buf[4], 99);
}

#[test]
fn test_m2_send_script_answer_ffi_null_pointer() {
    let written = unsafe { m2_send_script_answer(99, std::ptr::null_mut()) };
    assert_eq!(written, 0); // Sprawdzamy bezpieczenstwo dla null-pointera
}

#[test]
fn test_send_quickslot_add_packet_zero_values() {
    let _lock = TEST_LOCK.lock().unwrap();
    clear_outgoing_packets();
    let result = SendQuickSlotAddPacket(0, 0, 0);
    assert!(result);
    
    let queued_bytes = get_last_outgoing_packet().unwrap();
    assert_eq!(queued_bytes[4], 0);
    assert_eq!(queued_bytes[5], 0);
    assert_eq!(queued_bytes[6], 0);
}

#[test]
fn test_m2_send_script_answer_max_value() {
    let mut out_buf = vec![0u8; 5];
    let written = unsafe { m2_send_script_answer(255, out_buf.as_mut_ptr()) };
    
    assert_eq!(written, 5);
    assert_eq!(out_buf[4], 255);
}
