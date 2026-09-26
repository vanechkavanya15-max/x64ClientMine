use m2_client::pvp_gvg_manager_v2::*;
use std::sync::RwLock;

#[test]
fn test_key_management_basic() {
    let mut manager = PvpGvgManagerV2::new();

    // PVP Keys
    manager.insert_pvp_key(100, 200);
    assert!(manager.has_pvp_key(100, 200));
    assert!(manager.has_pvp_key(200, 100)); // Should work regardless of order
    assert!(!manager.has_pvp_key(100, 300));
    manager.remove_pvp_key(200, 100);
    assert!(!manager.has_pvp_key(100, 200));

    // PVP Ready Keys
    manager.insert_pvp_ready_key(300, 400);
    assert!(manager.has_pvp_key(300, 400)); // InsertPVPReadyKey inserts into pvp_keys
    
    // GVG Keys
    manager.insert_gvg_key(10, 20);
    assert!(manager.has_gvg_key(10, 20));
    assert!(manager.has_gvg_key(20, 10));
    assert!(!manager.has_gvg_key(10, 30));
    manager.remove_gvg_key(20, 10);
    assert!(!manager.has_gvg_key(10, 20));

    // Duel Keys
    manager.insert_duel_key(500, 600);
    assert!(manager.has_duel_key(500, 600));
    assert!(manager.has_duel_key(600, 500));
    
    // Clear system
    manager.insert_pvp_key(1, 2);
    manager.insert_gvg_key(3, 4);
    manager.clear_pvp_key_system();
    assert!(!manager.has_pvp_key(1, 2));
    assert!(!manager.has_gvg_key(3, 4));
    assert!(!manager.has_duel_key(500, 600));
}

#[test]
fn test_m2_pvp_can_attack_edge_cases() {
    {
        let mut manager = get_pvp_gvg_manager().write().unwrap();
        manager.clear_pvp_key_system();
    }

    // GM attacking Observer
    assert!(!m2_pvp_can_attack(
        1, 2, true, true, false, true, false, false, 0, 0, 0, 0, 0,
        true, false, false, false, false, 0, 0, 0, 0, false, false
    ));

    // Self attack
    assert!(!m2_pvp_can_attack(
        1, 1, false, false, false, true, false, false, 0, 0, 0, 0, 0,
        true, false, false, false, false, 0, 0, 0, 0, false, false
    ));

    // Stone vs PC
    assert!(m2_pvp_can_attack(
        1, 2, false, false, true, false, false, false, 0, 0, 0, 0, 0,
        true, false, false, false, false, 0, 0, 0, 0, false, false
    ));

    // PC vs Stone
    assert!(m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, 0, 0, 0, 0, 0,
        false, true, false, false, false, 0, 0, 0, 0, false, false
    ));

    // PC vs PC (Duel CannotAttack)
    assert!(!m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, DUEL_CANNOTATTACK, 0, 0, 0, 0,
        true, false, false, false, false, 0, 0, 0, 0, false, false
    ));

    // PC vs PC (Duel Start without key)
    assert!(!m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, DUEL_START, 0, 0, 0, 0,
        true, false, false, false, false, 0, 0, 0, 0, false, false
    ));

    // PC vs PC (Duel Start with key)
    {
        let mut manager = get_pvp_gvg_manager().write().unwrap();
        manager.insert_duel_key(1, 2);
    }
    assert!(m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, DUEL_START, 0, 0, 0, 0,
        true, false, false, false, false, 0, 0, 0, 0, false, false
    ));
    {
        let mut manager = get_pvp_gvg_manager().write().unwrap();
        manager.clear_pvp_key_system(); // cleanup (no explicit remove duel key, just clear)
    }

    // PC vs PC (Guild mode, same guild)
    assert!(!m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, DUEL_NONE, 4, 100, 0, 0,
        true, false, false, false, false, 4, 100, 0, 0, false, false
    ));

    // PC vs PC (Guild mode, different guild)
    assert!(m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, DUEL_NONE, 4, 100, 0, 0,
        true, false, false, false, false, 4, 200, 0, 0, false, false
    ));

    // PC vs PC (Killer, not same party)
    assert!(m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, DUEL_NONE, 0, 0, 0, 0,
        true, false, false, false, false, 0, 0, 0, 0, true, false
    ));

    // PC vs PC (Free PK mode vs Protect)
    assert!(!m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, DUEL_NONE, 2, 0, 0, 0,
        true, false, false, false, false, 3, 0, 0, 0, false, false
    ));

    // PC vs PC (Free PK mode vs Normal)
    assert!(m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, DUEL_NONE, 2, 0, 0, 0,
        true, false, false, false, false, 0, 0, 0, 0, false, false
    ));

    // PC vs PC (Different Empire)
    assert!(m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, DUEL_NONE, 0, 0, 1, 0,
        true, false, false, false, false, 0, 0, 2, 0, false, false
    ));

    // PC vs PC (Revenge mode vs Dark alignment)
    assert!(m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, DUEL_NONE, 1, 0, 1, 1000,
        true, false, false, false, false, 1, 0, 1, -1000, false, false
    ));

    // Enemy vs PC
    assert!(m2_pvp_can_attack(
        1, 2, false, false, false, false, true, false, 0, 0, 0, 0, 0,
        true, false, false, false, false, 0, 0, 0, 0, false, false
    ));

    // PC vs Enemy
    assert!(m2_pvp_can_attack(
        1, 2, false, false, false, true, false, false, 0, 0, 0, 0, 0,
        false, false, false, true, false, 0, 0, 0, 0, false, false
    ));
}
