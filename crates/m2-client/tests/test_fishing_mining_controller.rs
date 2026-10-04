use m2_client::fishing_mining_controller::*;
use std::sync::Mutex;
use std::sync::OnceLock;

#[test]
fn test_fishing_state_machine() {
    let mut ctrl = FishingMiningController::new();
    assert_eq!(ctrl.fishing_state, FishingState::Idle);

    // Casting
    ctrl.send_cast_fishing(1);
    assert_eq!(ctrl.fishing_state, FishingState::Casting);

    // Waiting
    let mut packet = TPacketGCFishing {
        header: 0,
        subheader: FISHING_SUB_HEADER_START,
        info: 0,
        dir: 0,
    };
    ctrl.handle_gc_fishing_packet(&packet);
    assert_eq!(ctrl.fishing_state, FishingState::Waiting);

    // Pull during waiting (not a hit yet)
    assert!(ctrl.pull_fishing_rod());
    assert_eq!(ctrl.fishing_state, FishingState::Idle);

    // Casting again
    ctrl.send_cast_fishing(2);
    assert_eq!(ctrl.fishing_state, FishingState::Casting);

    // Hit
    packet.subheader = FISHING_SUB_HEADER_HIT;
    packet.info = 5000;
    ctrl.handle_gc_fishing_packet(&packet);
    assert_eq!(ctrl.fishing_state, FishingState::Hit);
    assert_eq!(ctrl.hit_window_ms, 5000);

    // Pulling during hit
    assert!(ctrl.pull_fishing_rod());
    assert_eq!(ctrl.fishing_state, FishingState::Idle);

    // Success
    packet.subheader = FISHING_SUB_HEADER_SUCCESS;
    packet.info = 0;
    ctrl.handle_gc_fishing_packet(&packet);
    assert_eq!(ctrl.fishing_state, FishingState::Done);
    assert_eq!(ctrl.hit_window_ms, 0);

    // Fail
    packet.subheader = FISHING_SUB_HEADER_FAIL;
    ctrl.handle_gc_fishing_packet(&packet);
    assert_eq!(ctrl.fishing_state, FishingState::Done);
    assert_eq!(ctrl.hit_window_ms, 0);
}

#[test]
fn test_mining_state_machine() {
    let mut ctrl = FishingMiningController::new();
    assert_eq!(ctrl.mining_state, MiningState::Idle);
    assert_eq!(ctrl.pickaxe_points, 0);

    // Start mining
    assert!(ctrl.start_mining(1001, 1000, 5000));
    assert!(matches!(ctrl.mining_state, MiningState::Mining { .. }));

    // Cannot start mining while already mining
    assert!(!ctrl.start_mining(1002, 2000, 5000));

    // Update before duration ends (should return false)
    assert!(!ctrl.update_mining(3000, true));
    assert!(matches!(ctrl.mining_state, MiningState::Mining { .. }));

    // Update after duration ends with success
    assert!(ctrl.update_mining(7000, true));
    assert_eq!(ctrl.mining_state, MiningState::Success);
    assert_eq!(ctrl.pickaxe_points, 1);

    // Stop mining
    ctrl.stop_mining();
    assert_eq!(ctrl.mining_state, MiningState::Idle);

    // Start mining again and fail
    assert!(ctrl.start_mining(1003, 10000, 5000));
    assert!(ctrl.update_mining(16000, false));
    assert_eq!(ctrl.mining_state, MiningState::Fail);
    assert_eq!(ctrl.pickaxe_points, 1); // no points gained on fail
}

#[test]
fn test_fishing_packet_struct() {
    let pkt = TPacketCGFishing::new(5);
    assert_eq!({ pkt.header }, HEADER_CG_FISHING);
    assert_eq!({ pkt.dir }, 5);
    
    let bytes = pkt.to_bytes();
    assert_eq!(bytes, vec![HEADER_CG_FISHING, 5]);
}
