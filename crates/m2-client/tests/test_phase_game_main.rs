use m2_client::phase_game_main::{
    PhaseGameMain, TPacketGCPointChange, TPacketGCPoints, POINT_ENERGY, POINT_ENERGY_END_TIME,
    POINT_GOLD, POINT_LEVEL, POINT_MAX_NUM,
};
 
 #[test]
 fn test_integration_phase_game_main() {
     let instance = PhaseGameMain::new();
     assert!(instance.is_ready());
 }

#[test]
fn test_recv_player_points() {
    let mut instance = PhaseGameMain::new();
    let mut packet = TPacketGCPoints {
        header: 0x10,
        points: [0; POINT_MAX_NUM],
    };

    packet.points[POINT_LEVEL as usize] = 99;
    packet.points[POINT_ENERGY as usize] = 100;

    let result = instance.recv_player_points(&packet);
    
    assert!(result);
    assert!(instance.status_refreshed);
    assert_eq!(instance.player_points[POINT_LEVEL as usize], 99);
    assert_eq!(instance.player_points[POINT_ENERGY as usize], 100);
}

#[test]
fn test_recv_point_change_main_character() {
    let mut instance = PhaseGameMain::new();
    instance.main_character_vid = 1337;

    let packet = TPacketGCPointChange {
        header: 0x11,
        dwVID: 1337,
        type_: POINT_LEVEL,
        amount: 1,
        value: 100,
    };

    let result = instance.recv_point_change(&packet);
    assert!(result);
    assert_eq!(instance.player_points[POINT_LEVEL as usize], 100);
    assert!(instance.status_refreshed);
    assert!(instance.skill_window_refreshed);
}

#[test]
fn test_recv_point_change_energy_reset() {
    let mut instance = PhaseGameMain::new();
    instance.main_character_vid = 1337;
    instance.player_points[POINT_ENERGY_END_TIME as usize] = 5000;

    let packet = TPacketGCPointChange {
        header: 0x11,
        dwVID: 1337,
        type_: POINT_ENERGY,
        amount: 0,
        value: 0, // Energy reached 0
    };

    instance.recv_point_change(&packet);
    assert_eq!(instance.player_points[POINT_ENERGY as usize], 0);
    assert_eq!(instance.player_points[POINT_ENERGY_END_TIME as usize], 0); // Should reset end time
}

#[test]
fn test_recv_point_change_gold() {
    let mut instance = PhaseGameMain::new();
    instance.main_character_vid = 1337;
    instance.gold_picked_up = 0;

    let packet = TPacketGCPointChange {
        header: 0x11,
        dwVID: 1337,
        type_: POINT_GOLD,
        amount: 500,
        value: 1500,
    };

    instance.recv_point_change(&packet);
    assert_eq!(instance.player_points[POINT_GOLD as usize], 1500);
    assert_eq!(instance.gold_picked_up, 500); // Amount should be added to gold_picked_up
}

#[test]
fn test_recv_point_change_other_character() {
    let mut instance = PhaseGameMain::new();
    instance.main_character_vid = 1337;

    let packet = TPacketGCPointChange {
        header: 0x11,
        dwVID: 9999, // Other character
        type_: POINT_LEVEL,
        amount: 1,
        value: 100,
    };

    instance.recv_point_change(&packet);
    
    // Points should not be applied to player_points
    assert_eq!(instance.player_points[POINT_LEVEL as usize], 0);
    // UI should not refresh
    assert!(!instance.status_refreshed);
    assert!(!instance.skill_window_refreshed);
}

#[test]
fn test_recv_ping_packet() {
    use m2_client::phase_game_main::{TPacketGCPing, TPacketCGPong};

    let mut instance = PhaseGameMain::new();
    let packet = TPacketGCPing { header: 0x44 }; // Dummy header value

    let response = instance.recv_ping_packet(&packet);
    
    assert!(response.is_some());
    assert_eq!(response.unwrap(), TPacketCGPong { header: 0xfe });
    assert_eq!(instance.last_ping_time, 12345);
}

#[test]
fn test_recv_stun_packet_main_character() {
    use m2_client::phase_game_main::TPacketGCStun;

    let mut instance = PhaseGameMain::new();
    instance.main_character_vid = 1337;
    let packet = TPacketGCStun {
        header: 0x22,
        vid: 1337,
    };

    instance.recv_stun_packet(&packet);
    
    assert!(instance.is_game_over); // Simulacja `pkInstSel->Die();` z C++
    assert!(!instance.is_stunned);
}

#[test]
fn test_recv_stun_packet_other_character() {
    use m2_client::phase_game_main::TPacketGCStun;

    let mut instance = PhaseGameMain::new();
    instance.main_character_vid = 1337;
    let packet = TPacketGCStun {
        header: 0x22,
        vid: 9999, // Other
    };

    instance.recv_stun_packet(&packet);
    
    assert!(!instance.is_game_over);
    assert!(instance.is_stunned); // Simulacja `pkInstSel->Stun();`
}

#[test]
fn test_recv_dead_packet() {
    use m2_client::phase_game_main::TPacketGCDead;

    let mut instance = PhaseGameMain::new();
    instance.main_character_vid = 1337;
    
    let packet = TPacketGCDead {
        header: 0x33,
        vid: 1337,
    };

    instance.recv_dead_packet(&packet);
    assert!(instance.is_game_over);
}
