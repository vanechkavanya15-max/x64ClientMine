use m2_client_bridge::net::gc::char_status::{TPacketGCPoints, TPacketGCPointChange, POINT_MAX_NUM, HEADER_GC_PLAYER_POINTS, HEADER_GC_PLAYER_POINT_CHANGE};

#[path = "../src/recv_points.rs"] 
mod recv_points;
use recv_points::*;

#[test]
fn test_points_packet_deserialization_and_update() {
    let mut state = GlobalGameState::new();
    let mut pkt = TPacketGCPoints {
        header: HEADER_GC_PLAYER_POINTS,
        length: std::mem::size_of::<TPacketGCPoints>() as u16,
        points: [0; POINT_MAX_NUM],
    };
    
    pkt.points[POINT_LEVEL as usize] = 99;
    pkt.points[POINT_ST as usize] = 120;
    pkt.points[POINT_HT as usize] = 90;
    pkt.points[POINT_DX as usize] = 100;
    pkt.points[POINT_IQ as usize] = 10;
    pkt.points[POINT_GOLD as usize] = 1000000;
    pkt.points[POINT_HP as usize] = 5000;
    pkt.points[POINT_SP as usize] = 2000;
    pkt.points[POINT_EXP as usize] = 9999;
    
    assert!(state.handle_points_packet(&pkt));
    
    assert_eq!(state.level, 99);
    assert_eq!(state.st, 120);
    assert_eq!(state.ht, 90);
    assert_eq!(state.dx, 100);
    assert_eq!(state.iq, 10);
    assert_eq!(state.gold, 1000000);
    assert_eq!(state.hp, 5000);
    assert_eq!(state.sp, 2000);
    assert_eq!(state.exp, 9999);
    assert_eq!(state.points[POINT_LEVEL as usize], 99);
}

#[test]
fn test_point_change_packet() {
    let mut state = GlobalGameState::new();
    let my_vid = 12345;
    state.vid = my_vid;
    
    // Change HP
    let hp_change = TPacketGCPointChange {
        header: HEADER_GC_PLAYER_POINT_CHANGE,
        length: std::mem::size_of::<TPacketGCPointChange>() as u16,
        dwVID: my_vid,
        Type: POINT_HP,
        amount: -100,
        value: 500,
    };
    
    assert!(state.handle_point_change_packet(&hp_change, my_vid));
    assert_eq!(state.points[POINT_HP as usize], 500);
    assert_eq!(state.hp, 500);
    
    // Test gold update with amount
    let gold_change = TPacketGCPointChange {
        header: HEADER_GC_PLAYER_POINT_CHANGE,
        length: std::mem::size_of::<TPacketGCPointChange>() as u16,
        dwVID: my_vid,
        Type: POINT_GOLD,
        amount: 5000,
        value: 15000,
    };
    
    assert!(state.handle_point_change_packet(&gold_change, my_vid));
    assert_eq!(state.gold, 15000);
    assert_eq!(state.money_picked, 5000);
    
    // Test level change for different VID
    let other_vid = 999;
    let level_change = TPacketGCPointChange {
        header: HEADER_GC_PLAYER_POINT_CHANGE,
        length: std::mem::size_of::<TPacketGCPointChange>() as u16,
        dwVID: other_vid,
        Type: POINT_LEVEL,
        amount: 1,
        value: 50,
    };
    
    // Before: level is 0
    assert_eq!(state.level, 0);
    
    assert!(state.handle_point_change_packet(&level_change, my_vid));
    
    // Level shouldn't change for the main player state (but in C++ would update the instance target)
    assert_eq!(state.level, 0);
    
    // Test energy 0
    let energy_change = TPacketGCPointChange {
        header: HEADER_GC_PLAYER_POINT_CHANGE,
        length: std::mem::size_of::<TPacketGCPointChange>() as u16,
        dwVID: my_vid,
        Type: POINT_ENERGY,
        amount: -5,
        value: 0,
    };
    
    assert!(state.handle_point_change_packet(&energy_change, my_vid));
    assert_eq!(state.points[POINT_ENERGY as usize], 0);
    assert_eq!(state.points[POINT_ENERGY_END_TIME as usize], 0);

    // Test out of bounds point
    let invalid_change = TPacketGCPointChange {
        header: HEADER_GC_PLAYER_POINT_CHANGE,
        length: std::mem::size_of::<TPacketGCPointChange>() as u16,
        dwVID: my_vid,
        Type: 255, // POINT_MAX_NUM
        amount: 0,
        value: 0,
    };

    assert!(!state.handle_point_change_packet(&invalid_change, my_vid));
}
