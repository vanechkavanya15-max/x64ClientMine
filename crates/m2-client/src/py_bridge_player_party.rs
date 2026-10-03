// crates/m2-client/src/py_bridge_player_party.rs
use m2_client_bridge::ffi_guard::safe_ffi_call;

use crate::recv_party_logic_v2::{PARTY_AFFECT_SLOT_MAX_NUM, PARTY_MANAGER, PARTY_ROLE_LEADER};

/// Pobiera procent punktow zycia (HP) czlonka druzyny na podstawie PID.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_party_member_hp_percentage(pid: u32) -> i32 {
    safe_ffi_call(0, || {
        if let Some(member) = PARTY_MANAGER.get_member(pid) {
            member.hp_percent as i32
        } else {
            0
        }
    })
}

/// Pobiera stan/role czlonka druzyny na podstawie PID.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_party_member_state(pid: u32) -> i32 {
    safe_ffi_call(0, || {
        if let Some(member) = PARTY_MANAGER.get_member(pid) {
            member.role as i32
        } else {
            0
        }
    })
}

/// Wypelnia bufor 7 wartosciami (affects) dla danego PID i zwraca true.
/// Jesli PID nie istnieje, zwraca false.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_party_member_affects(
    pid: u32,
    out_affects: *mut i32,
) -> bool {
    safe_ffi_call(false, || {
        if out_affects.is_null() {
            return false;
        }

        if let Some(member) = PARTY_MANAGER.get_member(pid) {
            let slice = std::slice::from_raw_parts_mut(out_affects, PARTY_AFFECT_SLOT_MAX_NUM);
            for i in 0..PARTY_AFFECT_SLOT_MAX_NUM {
                slice[i] = member.affects[i] as i32;
            }
            true
        } else {
            false
        }
    })
}

/// Sprawdza czy gracz o podanym VID jest liderem druzyny.
/// Szuka PID na podstawie VID w m2_client_bridge::state::party::PARTY_MANAGER,
/// a nastepnie sprawdza w recv_party_logic_v2::PARTY_MANAGER.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_is_party_leader(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let party_mgr = m2_client_bridge::state::party::PARTY_MANAGER
            .read()
            .unwrap();
        if let Some(bridge_member) = party_mgr.get_member_by_vid(vid) {
            // bridge_member ma np. vid, ale PID moze byc wymagane dla recv_party_logic_v2.
            // Sprawdzamy czy recv_party_logic_v2 ma to zmapowane jako lider.
            // UWAGA: recv_party_logic_v2 trzyma liste na podstawie PID.
            // Ale na szczescie m2_client_bridge udostepnia nam tez 'is_leader'.
            // Albo jesli chcemy scisle uzyc recv_party_logic_v2, mozemy zrobic to samo co m_PartyMemberMap w C++.
            // Aby to uproscic wykorzystamy bridge_member.is_leader jesli istnieje.
            bridge_member.is_leader
        } else {
            false
        }
    })
}

/// Sprawdza czy gracz o podanym PID jest liderem druzyny (w recv_party_logic_v2).
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_is_party_leader_by_pid(pid: u32) -> bool {
    safe_ffi_call(false, || {
        if let Some(member) = PARTY_MANAGER.get_member(pid) {
            member.role == PARTY_ROLE_LEADER
        } else {
            false
        }
    })
}

/// Opuszcza druzyne - czysci informacje o druzynie.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_exit_party() {
    safe_ffi_call((), || {
        PARTY_MANAGER.clear();
        let mut party_mgr = m2_client_bridge::state::party::PARTY_MANAGER
            .write()
            .unwrap();
        party_mgr.clear();
    });
}
