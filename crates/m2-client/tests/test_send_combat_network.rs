use m2_client::send_combat_network::*;
use m2_client::character_manager_core::{get_character_manager, CharacterManagerCore};
use std::mem::size_of;

fn setup_main_actor(is_dead: bool, is_stunned: bool) {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    let main_vid = 1000;
    mgr.create_instance_full(main_vid, 0, 0.0, 0.0, 0.0, "Main".to_string(), is_dead);
    let inst = mgr.get_instance_mut(main_vid).unwrap();
    inst.is_stunned = is_stunned;
    mgr.main_actor_vid = Some(main_vid);
}

fn clear_main_actor() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
}

#[test]
fn test_send_attack_packet_success() {
    setup_main_actor(false, false);
    let attack_type = 1;
    let target_vid = 5000;
    
    let result = send_attack_packet(attack_type, target_vid);
    assert!(result.is_ok());
    let bytes = result.unwrap();
    
    assert_eq!(bytes.len(), size_of::<TPacketCGAttack>());
    
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    let b_type = bytes[4];
    let victim_vid = u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
    
    assert_eq!(header, HEADER_CG_ATTACK);
    assert_eq!(length, size_of::<TPacketCGAttack>() as u16);
    assert_eq!(b_type, attack_type);
    assert_eq!(victim_vid, target_vid);
}

#[test]
fn test_send_use_skill_packet_success() {
    setup_main_actor(false, false);
    let skill_vnum = 20;
    let target_vid = 5000;
    
    let result = send_use_skill_packet(skill_vnum, target_vid);
    assert!(result.is_ok());
    let bytes = result.unwrap();
    
    assert_eq!(bytes.len(), size_of::<TPacketCGUseSkill>());
    
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    let parsed_skill_vnum = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let parsed_target_vid = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    
    assert_eq!(header, HEADER_CG_USE_SKILL);
    assert_eq!(length, size_of::<TPacketCGUseSkill>() as u16);
    assert_eq!(parsed_skill_vnum, skill_vnum);
    assert_eq!(parsed_target_vid, target_vid);
}

#[test]
fn test_send_shoot_packet_success() {
    setup_main_actor(false, false);
    let b_type = 5;
    
    let result = send_shoot_packet(b_type);
    assert!(result.is_ok());
    let bytes = result.unwrap();
    
    assert_eq!(bytes.len(), size_of::<TPacketCGShoot>());
    
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    let parsed_b_type = bytes[4];
    
    assert_eq!(header, HEADER_CG_SHOOT);
    assert_eq!(length, size_of::<TPacketCGShoot>() as u16);
    assert_eq!(parsed_b_type, b_type);
}

#[test]
fn test_invalid_target_vid() {
    setup_main_actor(false, false);
    let result = send_attack_packet(1, 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid target VID");
    
    let result_skill = send_use_skill_packet(20, 0);
    assert!(result_skill.is_err());
    assert_eq!(result_skill.unwrap_err(), "Invalid target VID");
}

#[test]
fn test_actor_is_dead_cannot_act() {
    setup_main_actor(true, false);
    
    let result = send_attack_packet(1, 5000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Main actor cannot act (stunned or dead)");
    
    let result_skill = send_use_skill_packet(20, 5000);
    assert!(result_skill.is_err());
    assert_eq!(result_skill.unwrap_err(), "Main actor cannot act (stunned or dead)");
    
    let result_shoot = send_shoot_packet(5);
    assert!(result_shoot.is_err());
    assert_eq!(result_shoot.unwrap_err(), "Main actor cannot act (stunned or dead)");
}

#[test]
fn test_actor_is_stunned_cannot_act() {
    setup_main_actor(false, true);
    
    let result = send_attack_packet(1, 5000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Main actor cannot act (stunned or dead)");
}

#[test]
fn test_actor_is_dead_and_stunned_cannot_act() {
    setup_main_actor(true, true);
    
    let result_skill = send_use_skill_packet(20, 5000);
    assert!(result_skill.is_err());
    assert_eq!(result_skill.unwrap_err(), "Main actor cannot act (stunned or dead)");
}

#[test]
fn test_no_main_actor_cannot_act() {
    clear_main_actor();
    
    let result = send_attack_packet(1, 5000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Main actor cannot act (stunned or dead)");
}
