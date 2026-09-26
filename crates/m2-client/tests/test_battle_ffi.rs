use m2_client::battle_ffi::{
    m2_battle_can_attack, m2_battle_combo_attack, m2_battle_get_combo_index,
    m2_battle_reset_combo,
};
use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, POINT_HP, POINT_MAX_HP, Actor};
use std::sync::{Mutex, MutexGuard};

static TEST_MUTEX: Mutex<()> = Mutex::new(());

fn setup_state() -> MutexGuard<'static, ()> {
    let guard = TEST_MUTEX.lock().unwrap();
    let mut state = GLOBAL_GAME_STATE.write().unwrap();
    state.reset();

    // Player
    state.set_player_vid(1);
    state.player.x = 100.0;
    state.player.y = 100.0;
    state.set_point(POINT_HP, 1000);
    state.set_point(POINT_MAX_HP, 1000);
    state.player.is_mounted = false;

    // Actor 1 (Enemy 1) - Close
    let mut actor1 = Actor::default();
    actor1.vid = 2;
    actor1.x = 110.0;
    actor1.y = 110.0;
    actor1.hp = 500;
    state.add_or_update_actor(actor1);

    // Actor 2 (Enemy 2) - Far away
    let mut actor2 = Actor::default();
    actor2.vid = 3;
    actor2.x = 5000.0;
    actor2.y = 5000.0;
    actor2.hp = 500;
    state.add_or_update_actor(actor2);

    // Actor 3 (Dead)
    let mut actor3 = Actor::default();
    actor3.vid = 4;
    actor3.x = 105.0;
    actor3.y = 105.0;
    actor3.hp = 0;
    state.add_or_update_actor(actor3);

    guard
}

#[test]
fn test_m2_battle_can_attack_valid() {
    let _guard = setup_state();
    // Gracz atakuje bliskiego aktora
    assert!(m2_battle_can_attack(1, 2));
}

#[test]
fn test_m2_battle_can_attack_invalid_vids() {
    let _guard = setup_state();
    // Same VID
    assert!(!m2_battle_can_attack(1, 1));
    // 0 VID
    assert!(!m2_battle_can_attack(0, 2));
    assert!(!m2_battle_can_attack(1, 0));
    // Nieistniejace VID
    assert!(!m2_battle_can_attack(999, 2));
    assert!(!m2_battle_can_attack(1, 999));
}

#[test]
fn test_m2_battle_can_attack_dead() {
    let _guard = setup_state();
    // Ofiara martwa
    assert!(!m2_battle_can_attack(1, 4));

    // Gracz martwy
    {
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        state.set_point(POINT_HP, 0);
    }
    assert!(!m2_battle_can_attack(1, 2));
}

#[test]
fn test_m2_battle_can_attack_far_away() {
    let _guard = setup_state();
    // Ofiara jest za daleko (> 500 dystansu)
    assert!(!m2_battle_can_attack(1, 3));
}

#[test]
fn test_m2_battle_combo_attack_and_index() {
    let _guard = setup_state();
    m2_battle_reset_combo();

    assert_eq!(m2_battle_get_combo_index(), 0);

    // Gracz moze wyprowadzic 4 ataki w combo
    assert!(m2_battle_combo_attack(0.0));
    assert_eq!(m2_battle_get_combo_index(), 1);

    assert!(m2_battle_combo_attack(0.0));
    assert_eq!(m2_battle_get_combo_index(), 2);

    assert!(m2_battle_combo_attack(0.0));
    assert_eq!(m2_battle_get_combo_index(), 3);

    assert!(m2_battle_combo_attack(0.0));
    assert_eq!(m2_battle_get_combo_index(), 4);

    // Przekroczenie limitu (dla nie-wsiadanego max to 4)
    assert!(!m2_battle_combo_attack(0.0));
    assert_eq!(m2_battle_get_combo_index(), 4);

    // Reset
    m2_battle_reset_combo();
    assert_eq!(m2_battle_get_combo_index(), 0);
}

#[test]
fn test_m2_battle_combo_attack_dead() {
    let _guard = setup_state();
    m2_battle_reset_combo();

    {
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        state.set_point(POINT_HP, 0);
    }

    assert!(!m2_battle_combo_attack(0.0));
    assert_eq!(m2_battle_get_combo_index(), 0);
}

#[test]
fn test_m2_battle_combo_attack_mounted() {
    let _guard = setup_state();
    m2_battle_reset_combo();

    {
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        state.player.is_mounted = true;
    }

    // Mounted allow 5 attacks
    assert!(m2_battle_combo_attack(0.0));
    assert!(m2_battle_combo_attack(0.0));
    assert!(m2_battle_combo_attack(0.0));
    assert!(m2_battle_combo_attack(0.0));
    assert!(m2_battle_combo_attack(0.0)); // 5th attack
    assert_eq!(m2_battle_get_combo_index(), 5);

    // 6th attack false
    assert!(!m2_battle_combo_attack(0.0));
}
