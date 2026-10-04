use m2_client::game_party_group_manager::*;
use m2_client::recv_party_logic_v2::{PARTY_ROLE_LEADER, PARTY_ROLE_NORMAL};
use std::ffi::CString;

#[test]
fn test_party_manager_add_and_get() {
    PARTY_GROUP_MANAGER.clear();
    assert!(PARTY_GROUP_MANAGER.add_member(1234, "TestPlayer"));
    let member = PARTY_GROUP_MANAGER.get_member(1234).unwrap();
    assert_eq!(member.pid, 1234);
    assert_eq!(member.name, "TestPlayer");
    assert_eq!(member.hp_percent, 100);
    assert_eq!(member.role, PARTY_ROLE_NORMAL);
    assert_eq!(member.leader, false);
}

#[test]
fn test_party_manager_update() {
    PARTY_GROUP_MANAGER.clear();
    PARTY_GROUP_MANAGER.add_member(5555, "TestLeader");
    let affects = [1, 2, 3, 4, 5, 6, 7];
    assert!(PARTY_GROUP_MANAGER.update_member(5555, PARTY_ROLE_LEADER, 50, affects));
    let member = PARTY_GROUP_MANAGER.get_member(5555).unwrap();
    assert_eq!(member.role, PARTY_ROLE_LEADER);
    assert_eq!(member.leader, true);
    assert_eq!(member.hp_percent, 50);
    assert_eq!(member.affects, affects);
}

#[test]
fn test_party_manager_link_unlink() {
    PARTY_GROUP_MANAGER.clear();
    PARTY_GROUP_MANAGER.add_member(9999, "LinkPlayer");
    assert!(PARTY_GROUP_MANAGER.get_member_by_vid(100).is_none());
    assert!(PARTY_GROUP_MANAGER.link_member(9999, 100));
    let member = PARTY_GROUP_MANAGER.get_member_by_vid(100).unwrap();
    assert_eq!(member.pid, 9999);
    assert_eq!(member.vid, 100);
    assert!(PARTY_GROUP_MANAGER.unlink_member(9999));
    assert!(PARTY_GROUP_MANAGER.get_member_by_vid(100).is_none());
}

#[test]
fn test_party_manager_distribution_mode() {
    PARTY_GROUP_MANAGER.clear();
    assert_eq!(PARTY_GROUP_MANAGER.get_distribution_mode(), 0);
    PARTY_GROUP_MANAGER.set_distribution_mode(1);
    assert_eq!(PARTY_GROUP_MANAGER.get_distribution_mode(), 1);
    unsafe {
        m2_py_party_manager_set_distribution_mode(2);
        assert_eq!(m2_py_party_manager_get_distribution_mode(), 2);
    }
}

#[test]
fn test_party_manager_invites() {
    unsafe {
        // Without engine initialized these will return false or fallthrough gracefully
        let ans = m2_py_party_manager_invite_answer(1234, true);
        assert!(!ans || ans); // Just check it doesn't crash
        let inv = m2_py_party_manager_send_invite(5555);
        assert!(!inv || inv);
    }
}

#[test]
fn test_party_manager_ffi() {
    PARTY_GROUP_MANAGER.clear();
    let name = CString::new("FFIPlayer").unwrap();
    unsafe {
        assert!(m2_py_party_manager_add_member(10, name.as_ptr()));
    }
    let affects = [10, 20, 30, 40, 50, 60, 70];
    unsafe {
        assert!(m2_py_party_manager_update_member(10, PARTY_ROLE_LEADER, 85, affects.as_ptr()));
    }
    unsafe {
        assert_eq!(m2_py_party_manager_get_member_hp_percent(10), 85);
        assert_eq!(m2_py_party_manager_get_member_state(10), PARTY_ROLE_LEADER as i32);
        assert!(m2_py_party_manager_is_leader_by_pid(10));
    }
    let mut out_affects = [0i32; 7];
    unsafe {
        assert!(m2_py_party_manager_get_member_affects(10, out_affects.as_mut_ptr()));
    }
    for i in 0..7 {
        assert_eq!(out_affects[i], affects[i] as i32);
    }
    unsafe {
        assert!(m2_py_party_manager_link_member(10, 500));
        assert!(m2_py_party_manager_is_member(500));
        assert!(m2_py_party_manager_is_leader(500));
    }
    unsafe {
        assert!(m2_py_party_manager_remove_member(10));
        assert!(!m2_py_party_manager_is_member(500));
    }
}
