use m2_client::recv_party_logic_v2::*;

#[test]
fn test_party_add() {
    let manager = PartyManager::new();
    let mut packet = TPacketGCPartyAdd {
        header: 0,
        length: 0,
        pid: 1234,
        name: [0; CHARACTER_NAME_MAX_LEN + 1],
    };
    
    let name_str = b"TestUser";
    packet.name[..name_str.len()].copy_from_slice(name_str);

    assert!(manager.handle_party_add(&packet));
    
    let member = manager.get_member(1234).unwrap();
    assert_eq!(member.pid, 1234);
    assert_eq!(member.name, "TestUser");
    assert_eq!(member.hp_percent, 100);
    assert_eq!(member.role, PARTY_ROLE_NORMAL);
    assert_eq!(member.leader, false);
}

#[test]
fn test_party_update() {
    let manager = PartyManager::new();
    let add_packet = TPacketGCPartyAdd {
        header: 0,
        length: 0,
        pid: 1234,
        name: [0; CHARACTER_NAME_MAX_LEN + 1],
    };
    assert!(manager.handle_party_add(&add_packet));
    
    let update_packet = TPacketGCPartyUpdate {
        header: 0,
        length: 0,
        pid: 1234,
        state: PARTY_ROLE_LEADER,
        percent_hp: 50,
        affects: [1, 2, 3, 4, 5, 6, 7],
    };

    assert!(manager.handle_party_update(&update_packet));

    let member = manager.get_member(1234).unwrap();
    assert_eq!(member.role, PARTY_ROLE_LEADER);
    assert_eq!(member.leader, true);
    assert_eq!(member.hp_percent, 50);
    assert_eq!(member.affects, [1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_party_update_non_leader() {
    let manager = PartyManager::new();
    let add_packet = TPacketGCPartyAdd {
        header: 0,
        length: 0,
        pid: 1234,
        name: [0; CHARACTER_NAME_MAX_LEN + 1],
    };
    assert!(manager.handle_party_add(&add_packet));
    
    let update_packet = TPacketGCPartyUpdate {
        header: 0,
        length: 0,
        pid: 1234,
        state: PARTY_ROLE_ATTACKER,
        percent_hp: 80,
        affects: [0; 7],
    };

    assert!(manager.handle_party_update(&update_packet));

    let member = manager.get_member(1234).unwrap();
    assert_eq!(member.role, PARTY_ROLE_ATTACKER);
    assert_eq!(member.leader, false);
}

#[test]
fn test_party_remove() {
    let manager = PartyManager::new();
    let add_packet = TPacketGCPartyAdd {
        header: 0,
        length: 0,
        pid: 5555,
        name: [0; CHARACTER_NAME_MAX_LEN + 1],
    };
    assert!(manager.handle_party_add(&add_packet));
    
    assert!(manager.get_member(5555).is_some());
    
    let remove_packet = TPacketGCPartyRemove {
        header: 0,
        length: 0,
        pid: 5555,
    };
    
    assert!(manager.handle_party_remove(&remove_packet));
    assert!(manager.get_member(5555).is_none());
    
    // Removing non-existing member
    assert!(!manager.handle_party_remove(&remove_packet));
}

#[test]
fn test_m2_party_get_member_ffi() {
    PARTY_MANAGER.clear();
    
    assert!(!m2_party_get_member(9999));
    
    let add_packet = TPacketGCPartyAdd {
        header: 0,
        length: 0,
        pid: 9999,
        name: [0; CHARACTER_NAME_MAX_LEN + 1],
    };
    
    assert!(PARTY_MANAGER.handle_party_add(&add_packet));
    assert!(m2_party_get_member(9999));
}

#[test]
fn test_invalid_packet_sizes() {
    let manager = PartyManager::new();
    let packet = TPacketGCPartyAdd {
        header: 0,
        length: 0,
        pid: 1,
        name: [b'A'; CHARACTER_NAME_MAX_LEN + 1],
    };
    
    assert!(manager.handle_party_add(&packet));
    let member = manager.get_member(1).unwrap();
    assert_eq!(member.name, "AAAAAAAAAAAAAAAAAAAAAAAA"); // 24 characters
}

#[test]
fn test_update_non_existing() {
    let manager = PartyManager::new();
    let update_packet = TPacketGCPartyUpdate {
        header: 0,
        length: 0,
        pid: 999,
        state: PARTY_ROLE_LEADER,
        percent_hp: 50,
        affects: [0; 7],
    };

    assert!(!manager.handle_party_update(&update_packet));
}
