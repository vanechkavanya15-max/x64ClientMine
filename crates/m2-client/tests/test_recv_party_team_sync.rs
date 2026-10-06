#[path = "../src/recv_party_team_sync.rs"]
mod recv_party_team_sync;

use recv_party_team_sync::*;
use std::mem::size_of;

#[test]
fn test_packet_sizes_and_alignment() {
    assert_eq!(size_of::<TPacketGCPartyInvite>(), 8);
    assert_eq!(size_of::<TPacketGCPartyAdd>(), 2 + 2 + 4 + CHARACTER_NAME_MAX_LEN + 1);
    assert_eq!(size_of::<TPacketGCPartyUpdate>(), 2 + 2 + 4 + 1 + 1 + 2 * PARTY_AFFECT_SLOT_MAX_NUM);
    assert_eq!(size_of::<TPacketGCPartyRemove>(), 8);
    assert_eq!(size_of::<TPacketGCPartyLink>(), 12);
    assert_eq!(size_of::<TPacketGCPartyUnlink>(), 12);
    assert_eq!(size_of::<TPacketGCPartyParameter>(), 5);
    
    assert_eq!(size_of::<TPacketGCParty>(), 2 + 2 + 1 + 4 + 1);
}

#[test]
fn test_recv_party_packet_wrapper() {
    PARTY_MEMBERS.lock().unwrap().clear();

    let mut packet = TPacketGCParty {
        header: 0x07,
        length: 10,
        subheader: PARTY_SUBHEADER_GC_INVITE,
        pid: 12345,
        role_or_state: 0,
    };
    
    // Test INVITE
    let mut res = CPythonNetworkStreamPhaseGame::recv_party_packet(&packet);
    assert!(res.is_ok());
    
    // Test ADD
    packet.subheader = PARTY_SUBHEADER_GC_ADD;
    packet.pid = 111;
    res = CPythonNetworkStreamPhaseGame::recv_party_packet(&packet);
    assert!(res.is_ok());
    assert!(PARTY_MEMBERS.lock().unwrap().contains_key(&111));

    // Test UPDATE
    packet.subheader = PARTY_SUBHEADER_GC_UPDATE;
    packet.role_or_state = 2; // state = 2
    res = CPythonNetworkStreamPhaseGame::recv_party_packet(&packet);
    assert!(res.is_ok());
    assert_eq!(PARTY_MEMBERS.lock().unwrap().get(&111).unwrap().state, 2);

    // Test CHANGE LEADER
    packet.subheader = PARTY_SUBHEADER_GC_CHANGE_LEADER;
    res = CPythonNetworkStreamPhaseGame::recv_party_packet(&packet);
    assert!(res.is_ok());
    assert_eq!(PARTY_MEMBERS.lock().unwrap().get(&111).unwrap().role, PARTY_SUBHEADER_GC_CHANGE_LEADER);
    
    // Test ROLES
    packet.subheader = PARTY_SUBHEADER_GC_ATTACKER;
    res = CPythonNetworkStreamPhaseGame::recv_party_packet(&packet);
    assert!(res.is_ok());
    assert_eq!(PARTY_MEMBERS.lock().unwrap().get(&111).unwrap().role, 5);
}

#[test]
fn test_original_handlers() {
    PARTY_MEMBERS.lock().unwrap().clear();

    // Add
    let mut name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    name[0] = b'A'; name[1] = b'B'; name[2] = b'C';
    let add_pkt = TPacketGCPartyAdd { header: 0, length: 0, pid: 222, name };
    assert!(CPythonNetworkStreamPhaseGame::recv_party_add(&add_pkt).is_ok());
    assert_eq!(PARTY_MEMBERS.lock().unwrap().get(&222).unwrap().name, "ABC");

    // Update
    let update_pkt = TPacketGCPartyUpdate { header: 0, length: 0, pid: 222, state: 1, percent_hp: 50, affects: [0; 7] };
    assert!(CPythonNetworkStreamPhaseGame::recv_party_update(&update_pkt).is_ok());
    assert_eq!(PARTY_MEMBERS.lock().unwrap().get(&222).unwrap().state, 1);

    // Link
    let link_pkt = TPacketGCPartyLink { header: 0, length: 0, pid: 222, vid: 333 };
    assert!(CPythonNetworkStreamPhaseGame::recv_party_link(&link_pkt).is_ok());
    assert_eq!(PARTY_MEMBERS.lock().unwrap().get(&222).unwrap().vid, 333);

    // Unlink
    let unlink_pkt = TPacketGCPartyUnlink { header: 0, length: 0, pid: 222, vid: 333 };
    assert!(CPythonNetworkStreamPhaseGame::recv_party_unlink(&unlink_pkt).is_ok());
    assert_eq!(PARTY_MEMBERS.lock().unwrap().get(&222).unwrap().vid, 0);

    // Remove
    let remove_pkt = TPacketGCPartyRemove { header: 0, length: 0, pid: 222 };
    assert!(CPythonNetworkStreamPhaseGame::recv_party_remove(&remove_pkt).is_ok());
    assert!(PARTY_MEMBERS.lock().unwrap().get(&222).is_none());
}

