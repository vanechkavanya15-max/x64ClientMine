use m2_client::send_combat_packets_v2::*;
use std::mem::size_of;

#[test]
fn test_send_attack_packet_size() {
    assert_eq!(size_of::<TPacketCGAttack>(), 7);
}

#[test]
fn test_send_shoot_packet_size() {
    assert_eq!(size_of::<TPacketCGShoot>(), 3);
}

#[test]
fn test_send_use_skill_packet_size() {
    assert_eq!(size_of::<TPacketCGUseSkill>(), 10);
}

#[test]
fn test_m2_send_attack() {
    let mut buf = [0u8; 64];
    let b_type = 5u8;
    let vid = 12345u32;
    
    let size = m2_send_attack(b_type, vid, buf.as_mut_ptr());
    assert_eq!(size, 7);
    
    let header = u16::from_le_bytes([buf[0], buf[1]]);
    let written_b_type = buf[2];
    let written_vid = u32::from_le_bytes([buf[3], buf[4], buf[5], buf[6]]);
    
    assert_eq!(header, HEADER_CG_ATTACK);
    assert_eq!(written_b_type, b_type);
    assert_eq!(written_vid, vid);
}

#[test]
fn test_m2_send_shoot() {
    let mut buf = [0u8; 64];
    let b_type = 2u8;
    
    let size = m2_send_shoot(b_type, buf.as_mut_ptr());
    assert_eq!(size, 3);
    
    let header = u16::from_le_bytes([buf[0], buf[1]]);
    let written_b_type = buf[2];
    
    assert_eq!(header, HEADER_CG_SHOOT);
    assert_eq!(written_b_type, b_type);
}

#[test]
fn test_m2_send_use_skill() {
    let mut buf = [0u8; 64];
    let skill_index = 32u32;
    let target_vid = 98765u32;
    
    let size = m2_send_use_skill(skill_index, target_vid, buf.as_mut_ptr());
    assert_eq!(size, 10);
    
    let header = u16::from_le_bytes([buf[0], buf[1]]);
    let written_skill_index = u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]);
    let written_target_vid = u32::from_le_bytes([buf[6], buf[7], buf[8], buf[9]]);
    
    assert_eq!(header, HEADER_CG_USE_SKILL);
    assert_eq!(written_skill_index, skill_index);
    assert_eq!(written_target_vid, target_vid);
}

#[test]
fn test_null_pointers() {
    let size = m2_send_attack(1, 1, std::ptr::null_mut());
    assert_eq!(size, 0);
    
    let size = m2_send_shoot(1, std::ptr::null_mut());
    assert_eq!(size, 0);
    
    let size = m2_send_use_skill(1, 1, std::ptr::null_mut());
    assert_eq!(size, 0);
}

#[test]
fn test_m2_send_attack_edge_case_max_values() {
    let mut buf = [0u8; 64];
    let b_type = 255u8;
    let vid = 4294967295u32;
    
    let size = m2_send_attack(b_type, vid, buf.as_mut_ptr());
    assert_eq!(size, 7);
    
    let written_b_type = buf[2];
    let written_vid = u32::from_le_bytes([buf[3], buf[4], buf[5], buf[6]]);
    
    assert_eq!(written_b_type, 255);
    assert_eq!(written_vid, 4294967295);
}

#[test]
fn test_m2_send_use_skill_edge_case_max_values() {
    let mut buf = [0u8; 64];
    let skill_index = 4294967295u32;
    let target_vid = 4294967295u32;
    
    let size = m2_send_use_skill(skill_index, target_vid, buf.as_mut_ptr());
    assert_eq!(size, 10);
    
    let written_skill_index = u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]);
    let written_target_vid = u32::from_le_bytes([buf[6], buf[7], buf[8], buf[9]]);
    
    assert_eq!(written_skill_index, 4294967295);
    assert_eq!(written_target_vid, 4294967295);
}
