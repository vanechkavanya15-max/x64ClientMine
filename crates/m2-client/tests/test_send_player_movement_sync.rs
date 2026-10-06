#[path = "../src/send_player_movement_sync.rs"]
mod send_player_movement_sync;

use send_player_movement_sync::{create_character_state_packet, PacketCgMove, HEADER_CG_MOVE, MOVE_TYPE_WAIT, MOVE_TYPE_WALK, MOVE_TYPE_RUN};

#[test]
fn test_packet_cg_move_size() {
    assert_eq!(std::mem::size_of::<PacketCgMove>(), 19);
}

#[test]
fn test_create_character_state_packet_normal() {
    let packet = create_character_state_packet(100.0, 200.0, 90.0, MOVE_TYPE_WALK, 0, 123456);
    
    // Read fields safely from packed struct
    let header = { packet.header };
    let length = { packet.length };
    let b_func = { packet.b_func };
    let b_arg = { packet.b_arg };
    let b_rot = { packet.b_rot };
    let l_x = { packet.l_x };
    let l_y = { packet.l_y };
    let dw_time = { packet.dw_time };
    
    assert_eq!(header, HEADER_CG_MOVE);
    assert_eq!(length, 19);
    assert_eq!(b_func, MOVE_TYPE_WALK);
    assert_eq!(b_arg, 0);
    assert_eq!(b_rot, 18); // 90 / 5 = 18
    assert_eq!(l_x, 100);
    assert_eq!(l_y, 200);
    assert_eq!(dw_time, 123456);
}

#[test]
fn test_create_character_state_packet_negative_rotation() {
    let packet = create_character_state_packet(0.0, 0.0, -90.0, MOVE_TYPE_WAIT, 0, 0);
    let b_rot = { packet.b_rot };
    // -90 + 360 = 270. 270 / 5 = 54
    assert_eq!(b_rot, 54);
}

#[test]
fn test_create_character_state_packet_overflow_rotation() {
    let packet = create_character_state_packet(0.0, 0.0, 450.0, MOVE_TYPE_RUN, 0, 0);
    let b_rot = { packet.b_rot };
    // 450 % 360 = 90. 90 / 5 = 18
    assert_eq!(b_rot, 18);
    let b_func = { packet.b_func };
    assert_eq!(b_func, MOVE_TYPE_RUN);
}
