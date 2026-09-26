// Testy weryfikujace fizyczna serializacje i kolejkowanie pakietow CG przez funkcje api::net_send_*
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use m2_client_bridge::api::net_send_move::m2_net_send_move;
use m2_client_bridge::api::net_send_attack::m2_net_send_attack;
use m2_client_bridge::api::net_send_chat::m2_net_send_chat;
use m2_client_bridge::api::net_send_pickup::m2_net_send_pickup;
use m2_client_bridge::api::net_send_use_item::m2_net_send_use_item;
use m2_client_bridge::api::net_send_use_skill::m2_net_send_use_skill;
use m2_client_bridge::net::send_queue::{get_last_outgoing_packet, clear_outgoing_packets, outgoing_packet_count};
use std::ffi::CString;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_full_pipeline_net_send_move() {
    let _lock = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();
    assert_eq!(outgoing_packet_count(), 0);

    let res = m2_net_send_move(1234.0, 5678.0, 180.0, 1);
    assert!(res);
    assert_eq!(outgoing_packet_count(), 1);

    let bytes = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    // TPacketCGMove size = 19
    assert_eq!(bytes.len(), 19);

    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0301); // CG::MOVE

    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 19);

    assert_eq!(bytes[4], 1); // b_func (move_type)
    assert_eq!(bytes[5], 0); // b_arg
    assert_eq!(bytes[6], (180.0 / 5.0) as u8); // b_rot = 36

    let l_x = i32::from_le_bytes([bytes[7], bytes[8], bytes[9], bytes[10]]);
    assert_eq!(l_x, 1234);

    let l_y = i32::from_le_bytes([bytes[11], bytes[12], bytes[13], bytes[14]]);
    assert_eq!(l_y, 5678);
}

#[test]
fn test_full_pipeline_net_send_attack() {
    let _lock = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();

    let res = m2_net_send_attack(9999, 2);
    assert!(res);
    assert_eq!(outgoing_packet_count(), 1);

    let bytes = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    // TPacketCGAttack size = 11
    assert_eq!(bytes.len(), 11);

    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0401); // CG::ATTACK

    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 11);

    assert_eq!(bytes[4], 2); // b_type
    let target_vid = u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
    assert_eq!(target_vid, 9999);
}

#[test]
fn test_full_pipeline_net_send_chat() {
    let _lock = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();

    let msg = CString::new("Witaj swiecie!").unwrap();
    let res = unsafe { m2_net_send_chat(0, msg.as_ptr()) };
    assert!(res);
    assert_eq!(outgoing_packet_count(), 1);

    let bytes = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0601); // CG::CHAT

    // 5 bajtow naglowka + 14 bajtow tresci + 1 bajt null
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 5 + 14 + 1);
    assert_eq!(bytes.len(), length as usize);

    assert_eq!(bytes[4], 0); // chat_type
    assert_eq!(&bytes[5..19], b"Witaj swiecie!");
    assert_eq!(bytes[19], 0); // null terminator
}

#[test]
fn test_full_pipeline_net_send_pickup() {
    let _lock = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();

    let res = unsafe { m2_net_send_pickup(55555) };
    assert!(res);
    assert_eq!(outgoing_packet_count(), 1);

    let bytes = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    // TPacketCGItemPickUp size = 8
    assert_eq!(bytes.len(), 8);

    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0505); // CG::ITEM_PICKUP

    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 8);

    let vid = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    assert_eq!(vid, 55555);
}

#[test]
fn test_full_pipeline_net_send_use_item() {
    let _lock = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();

    let res = m2_net_send_use_item(1, 42);
    assert!(res);
    assert_eq!(outgoing_packet_count(), 1);

    let bytes = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    // TPacketCGItemUse size = 7
    assert_eq!(bytes.len(), 7);

    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0501); // CG::ITEM_USE

    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 7);

    assert_eq!(bytes[4], 1); // window
    let cell = u16::from_le_bytes([bytes[5], bytes[6]]);
    assert_eq!(cell, 42); // cell
}

#[test]
fn test_full_pipeline_net_send_use_skill() {
    let _lock = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();

    let res = unsafe { m2_net_send_use_skill(121, 7777) };
    assert!(res);
    assert_eq!(outgoing_packet_count(), 1);

    let bytes = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    // TPacketCGUseSkill size = 12
    assert_eq!(bytes.len(), 12);

    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0402); // CG::USE_SKILL

    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 12);

    let vnum = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    assert_eq!(vnum, 121);

    let vid = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    assert_eq!(vid, 7777);
}
