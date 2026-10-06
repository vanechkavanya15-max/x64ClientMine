#[path = "../src/mining_vein_drill_state_v2.rs"]
mod mining_vein_drill_state_v2;

use mining_vein_drill_state_v2::*;

#[test]
fn test_is_pickaxe() {
    assert!(MiningVeinDrillManager::is_pickaxe(PICKAXE_VNUM_MIN));
    assert!(MiningVeinDrillManager::is_pickaxe(PICKAXE_VNUM_MAX));
    assert!(MiningVeinDrillManager::is_pickaxe(29105));
    assert!(!MiningVeinDrillManager::is_pickaxe(10));
    assert!(!MiningVeinDrillManager::is_pickaxe(29100));
}

#[test]
fn test_is_vein() {
    assert!(MiningVeinDrillManager::is_vein(VEIN_VNUM_MIN));
    assert!(MiningVeinDrillManager::is_vein(VEIN_VNUM_MAX));
    assert!(MiningVeinDrillManager::is_vein(20050));
    assert!(!MiningVeinDrillManager::is_vein(101)); // dog
}

#[test]
fn test_can_dig_conditions() {
    let mgr = MiningVeinDrillManager::new();
    
    // Success case
    assert!(mgr.can_dig(false, false, false, 29101, 20050).is_ok());

    // Fail cases
    assert_eq!(mgr.can_dig(true, false, false, 29101, 20050).unwrap_err(), "Cannot mine while mounted");
    assert_eq!(mgr.can_dig(false, true, false, 29101, 20050).unwrap_err(), "Cannot mine while dead or stunned");
    assert_eq!(mgr.can_dig(false, false, true, 29101, 20050).unwrap_err(), "Cannot mine while dead or stunned");
    assert_eq!(mgr.can_dig(false, false, false, 10, 20050).unwrap_err(), "Not holding a pickaxe");
    assert_eq!(mgr.can_dig(false, false, false, 29101, 101).unwrap_err(), "Target is not a resource vein");
}

#[test]
fn test_mining_flow_success() {
    let mut mgr = MiningVeinDrillManager::new();
    
    assert_eq!(mgr.state, MiningDrillState::Idle);

    // Start dig
    let start_time = 1000;
    assert!(mgr.start_dig(1234, 20050, start_time));
    assert!(matches!(mgr.state, MiningDrillState::Digging { .. }));

    // Cannot start again while digging
    assert!(!mgr.start_dig(1234, 20050, start_time + 100));

    // Update before completion
    assert!(mgr.update(start_time + 5000, 1).is_none());

    // Update after completion with max skill level to ensure 90% chance (still non-deterministic so we can't assert success strictly without mocking RNG)
    let res = mgr.update(start_time + MINING_DEFAULT_DURATION_MS, 40);
    assert!(res.is_some());
    let res = res.unwrap();
    // we cannot assert res.success because it's random now, but we check valid target data
    assert_eq!(res.target_vid, 1234);
    assert_eq!(res.vein_vnum, 20050);

    // State should be success or fail
    assert!(mgr.state == MiningDrillState::Success || mgr.state == MiningDrillState::Failed);
}

#[test]
fn test_mining_cancellation() {
    let mut mgr = MiningVeinDrillManager::new();
    
    assert!(mgr.start_dig(1234, 20050, 1000));
    assert!(matches!(mgr.state, MiningDrillState::Digging { .. }));

    mgr.cancel_dig();
    assert_eq!(mgr.state, MiningDrillState::Idle);

    // After cancel, update should do nothing
    assert!(mgr.update(10000, 1).is_none());
}

#[test]
fn test_ffi_exports() {
    unsafe {
        // Reset state
        m2_mining_drill_cancel();

        assert!(m2_mining_drill_is_pickaxe(29101));
        assert!(m2_mining_drill_is_vein(20050));
        
        let now = 1000;
        assert!(m2_mining_drill_start(555, 20050, now));
        
        // Cannot start twice
        assert!(!m2_mining_drill_start(555, 20050, now + 100));

        // Update before time -> false
        assert!(!m2_mining_drill_update(now + 5000, 10));

        // Update at completion -> it'll run the random code and return bool (can be either due to rand)
        let _ = m2_mining_drill_update(now + MINING_DEFAULT_DURATION_MS + 10, 10);

        m2_mining_drill_cancel();
    }
}
