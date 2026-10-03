// crates/m2-client/tests/test_py_bridge_player_party.rs

#[path = "../src/py_bridge_player_party.rs"]
mod py_bridge_player_party;

#[path = "../src/recv_party_logic_v2.rs"]
mod recv_party_logic_v2;

use py_bridge_player_party::{
    m2_py_player_exit_party, m2_py_player_get_party_member_affects,
    m2_py_player_get_party_member_hp_percentage, m2_py_player_get_party_member_state,
    m2_py_player_is_party_leader, m2_py_player_is_party_leader_by_pid,
};

use recv_party_logic_v2::{
    TPacketGCPartyAdd, TPacketGCPartyUpdate, PARTY_AFFECT_SLOT_MAX_NUM, PARTY_MANAGER,
    PARTY_ROLE_LEADER, PARTY_ROLE_NORMAL,
};

fn setup_party_member(pid: u32, vid: u32, name: &str, hp: u8, role: u8, affects: [i16; 7]) {
    // Utworz testowy pakiet dodania
    let mut name_buf = [0u8; 25];
    let name_bytes = name.as_bytes();
    let len = std::cmp::min(name_bytes.len(), 24);
    name_buf[..len].copy_from_slice(&name_bytes[..len]);

    let add_pkt = TPacketGCPartyAdd {
        header: 0x0711,
        length: std::mem::size_of::<TPacketGCPartyAdd>() as u16,
        pid,
        name: name_buf,
    };
    PARTY_MANAGER.handle_party_add(&add_pkt);

    // Utworz testowy pakiet aktualizacji
    let update_pkt = TPacketGCPartyUpdate {
        header: 0x0712,
        length: std::mem::size_of::<TPacketGCPartyUpdate>() as u16,
        pid,
        state: role,
        percent_hp: hp,
        affects,
    };
    PARTY_MANAGER.handle_party_update(&update_pkt);

    // Jesli ustawiamy lidera to mozemy tez do bridge'a (ze wzgledu na is_party_leader)
    if vid > 0 {
        let mut bridge_mgr = m2_client_bridge::state::party::PARTY_MANAGER
            .write()
            .unwrap();
        bridge_mgr.add_member(m2_client_bridge::state::party::PartyMember {
            vid,
            name: name.to_string(),
            hp_percent: hp,
            is_leader: role == PARTY_ROLE_LEADER,
        });
    }
}

fn clear_party_state() {
    PARTY_MANAGER.clear();
    m2_client_bridge::state::party::PARTY_MANAGER
        .write()
        .unwrap()
        .clear();
}

#[test]
fn test_get_party_member_hp_percentage() {
    clear_party_state();
    setup_party_member(100, 10, "TestPlayer", 85, PARTY_ROLE_NORMAL, [0; 7]);

    let hp = unsafe { m2_py_player_get_party_member_hp_percentage(100) };
    assert_eq!(hp, 85);

    let hp_invalid = unsafe { m2_py_player_get_party_member_hp_percentage(999) };
    assert_eq!(hp_invalid, 0);
}

#[test]
fn test_get_party_member_state() {
    clear_party_state();
    setup_party_member(101, 11, "TestLeader", 100, PARTY_ROLE_LEADER, [0; 7]);

    let state = unsafe { m2_py_player_get_party_member_state(101) };
    assert_eq!(state, PARTY_ROLE_LEADER as i32);

    let state_invalid = unsafe { m2_py_player_get_party_member_state(999) };
    assert_eq!(state_invalid, 0);
}

#[test]
fn test_get_party_member_affects() {
    clear_party_state();
    let affects = [1, 2, 3, 4, 5, 6, 7];
    setup_party_member(102, 12, "TestAffects", 50, PARTY_ROLE_NORMAL, affects);

    let mut out_affects = [0i32; 7];
    let result = unsafe { m2_py_player_get_party_member_affects(102, out_affects.as_mut_ptr()) };
    assert_eq!(result, true);
    for i in 0..7 {
        assert_eq!(out_affects[i], affects[i] as i32);
    }

    let result_invalid =
        unsafe { m2_py_player_get_party_member_affects(999, out_affects.as_mut_ptr()) };
    assert_eq!(result_invalid, false);

    let result_null = unsafe { m2_py_player_get_party_member_affects(102, std::ptr::null_mut()) };
    assert_eq!(result_null, false);
}

#[test]
fn test_is_party_leader() {
    clear_party_state();
    setup_party_member(103, 13, "RealLeader", 100, PARTY_ROLE_LEADER, [0; 7]);
    setup_party_member(104, 14, "NotLeader", 100, PARTY_ROLE_NORMAL, [0; 7]);

    let is_leader = unsafe { m2_py_player_is_party_leader(13) };
    assert_eq!(is_leader, true);

    let is_leader_false = unsafe { m2_py_player_is_party_leader(14) };
    assert_eq!(is_leader_false, false);

    let is_leader_invalid = unsafe { m2_py_player_is_party_leader(999) };
    assert_eq!(is_leader_invalid, false);
}

#[test]
fn test_is_party_leader_by_pid() {
    clear_party_state();
    setup_party_member(105, 15, "LeaderByPid", 100, PARTY_ROLE_LEADER, [0; 7]);
    setup_party_member(106, 16, "NotLeaderByPid", 100, PARTY_ROLE_NORMAL, [0; 7]);

    let is_leader = unsafe { m2_py_player_is_party_leader_by_pid(105) };
    assert_eq!(is_leader, true);

    let is_leader_false = unsafe { m2_py_player_is_party_leader_by_pid(106) };
    assert_eq!(is_leader_false, false);

    let is_leader_invalid = unsafe { m2_py_player_is_party_leader_by_pid(999) };
    assert_eq!(is_leader_invalid, false);
}

#[test]
fn test_exit_party() {
    clear_party_state();
    setup_party_member(107, 17, "ExitTest", 100, PARTY_ROLE_NORMAL, [0; 7]);

    // Upewnijmy sie ze jest dodany
    assert!(PARTY_MANAGER.get_member(107).is_some());
    assert!(m2_client_bridge::state::party::PARTY_MANAGER
        .read()
        .unwrap()
        .get_member_by_vid(17)
        .is_some());

    unsafe { m2_py_player_exit_party() };

    // Sprawdzamy czy zniknal
    assert!(PARTY_MANAGER.get_member(107).is_none());
    assert!(m2_client_bridge::state::party::PARTY_MANAGER
        .read()
        .unwrap()
        .get_member_by_vid(17)
        .is_none());
}
