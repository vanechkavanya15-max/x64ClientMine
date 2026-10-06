#[path = "../src/send_player_combat_actions.rs"]
mod send_player_combat_actions;

use std::mem::size_of;
use send_player_combat_actions::{
    TPacketCGAttack, TPacketCGUseSkill, TPacketCGShoot,
    HEADER_CG_ATTACK, HEADER_CG_USE_SKILL, HEADER_CG_SHOOT,
    m2_net_send_player_attack, m2_net_send_player_use_skill, m2_net_send_player_shoot,
};
use m2_client_bridge::net::send_queue::{get_last_outgoing_packet, clear_outgoing_packets};

#[test]
fn test_packet_cg_attack_size() {
    let expected_size = 2 + 2 + 1 + 4 + 1 + 1;
    assert_eq!(size_of::<TPacketCGAttack>(), expected_size);
}

#[test]
fn test_packet_cg_use_skill_size() {
    let expected_size = 2 + 2 + 4 + 4;
    assert_eq!(size_of::<TPacketCGUseSkill>(), expected_size);
}

#[test]
fn test_packet_cg_shoot_size() {
    let expected_size = 2 + 2 + 1;
    assert_eq!(size_of::<TPacketCGShoot>(), expected_size);
}

#[test]
fn test_m2_net_send_player_attack() {
    clear_outgoing_packets();
    
    let dw_vid_victim = 123456;
    let u_mot_attack = 5;
    
    let result = unsafe { m2_net_send_player_attack(dw_vid_victim, u_mot_attack) };
    assert!(result);
    
    let packet_data = get_last_outgoing_packet().expect("Brak pakietu w kolejce");
    assert_eq!(packet_data.len(), size_of::<TPacketCGAttack>());
    
    let packet_ptr = packet_data.as_ptr() as *const TPacketCGAttack;
    let packet = unsafe { &*packet_ptr };
    
    assert_eq!({ packet.header }, HEADER_CG_ATTACK);
    assert_eq!({ packet.length }, size_of::<TPacketCGAttack>() as u16);
    assert_eq!(packet.b_type, u_mot_attack);
    assert_eq!({ packet.dw_victim_vid }, dw_vid_victim);
    assert_eq!(packet.b_crc_magic_cube_proc_piece, 0);
    assert_eq!(packet.b_crc_magic_cube_file_piece, 0);
}

#[test]
fn test_m2_net_send_player_use_skill() {
    clear_outgoing_packets();
    
    let dw_skill_index = 10;
    let dw_target_vid = 654321;
    
    let result = unsafe { m2_net_send_player_use_skill(dw_skill_index, dw_target_vid) };
    assert!(result);
    
    let packet_data = get_last_outgoing_packet().expect("Brak pakietu w kolejce");
    assert_eq!(packet_data.len(), size_of::<TPacketCGUseSkill>());
    
    let packet_ptr = packet_data.as_ptr() as *const TPacketCGUseSkill;
    let packet = unsafe { &*packet_ptr };
    
    assert_eq!({ packet.header }, HEADER_CG_USE_SKILL);
    assert_eq!({ packet.length }, size_of::<TPacketCGUseSkill>() as u16);
    assert_eq!({ packet.dw_vnum }, dw_skill_index);
    assert_eq!({ packet.dw_target_vid }, dw_target_vid);
}

#[test]
fn test_m2_net_send_player_shoot() {
    clear_outgoing_packets();
    
    let u_skill = 3;
    
    let result = unsafe { m2_net_send_player_shoot(u_skill) };
    assert!(result);
    
    let packet_data = get_last_outgoing_packet().expect("Brak pakietu w kolejce");
    assert_eq!(packet_data.len(), size_of::<TPacketCGShoot>());
    
    let packet_ptr = packet_data.as_ptr() as *const TPacketCGShoot;
    let packet = unsafe { &*packet_ptr };
    
    assert_eq!({ packet.header }, HEADER_CG_SHOOT);
    assert_eq!({ packet.length }, size_of::<TPacketCGShoot>() as u16);
    assert_eq!(packet.b_type, u_skill);
}
