use m2_client::send_combat_skill_packets::*;
use std::mem::size_of;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_packet_attack_layout_and_size() {
    assert_eq!(size_of::<TPacketCGAttack>(), 13);
    
    let packet = TPacketCGAttack {
        header: HEADER_CG_ATTACK,
        length: 13,
        b_type: 5,
        dw_victim_vid: 12345,
        dw_crc_hash: 0xDEADBEEF,
    };
    
    // Convert to bytes manually to verify memory layout (little endian)
    let bytes = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketCGAttack>()) 
    };
    
    assert_eq!(bytes[0..2], HEADER_CG_ATTACK.to_le_bytes());
    assert_eq!(bytes[2..4], 13u16.to_le_bytes());
    assert_eq!(bytes[4], 5u8);
    assert_eq!(bytes[5..9], 12345u32.to_le_bytes());
    assert_eq!(bytes[9..13], 0xDEADBEEFu32.to_le_bytes());
}

#[test]
fn test_packet_use_skill_layout_and_size() {
    assert_eq!(size_of::<TPacketCGUseSkill>(), 12);
    
    let packet = TPacketCGUseSkill {
        header: HEADER_CG_USE_SKILL,
        length: 12,
        dw_vnum: 42,
        dw_target_vid: 98765,
    };
    
    let bytes = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketCGUseSkill>()) 
    };
    
    assert_eq!(bytes[0..2], HEADER_CG_USE_SKILL.to_le_bytes());
    assert_eq!(bytes[2..4], 12u16.to_le_bytes());
    assert_eq!(bytes[4..8], 42u32.to_le_bytes());
    assert_eq!(bytes[8..12], 98765u32.to_le_bytes());
}

#[test]
fn test_packet_shoot_layout_and_size() {
    assert_eq!(size_of::<TPacketCGShoot>(), 6);
    
    let packet = TPacketCGShoot {
        header: HEADER_CG_SHOOT,
        length: 6,
        b_type: 10,
        u_skill: 2,
    };
    
    let bytes = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketCGShoot>()) 
    };
    
    assert_eq!(bytes[0..2], HEADER_CG_SHOOT.to_le_bytes());
    assert_eq!(bytes[2..4], 6u16.to_le_bytes());
    assert_eq!(bytes[4], 10u8);
    assert_eq!(bytes[5], 2u8);
}

#[test]
fn test_ffi_send_attack() {
    let mut buf = [0u8; 128];
    let size = m2_combat_skill_send_attack(7, 4444, 0xABCDEF01, buf.as_mut_ptr());
    
    assert_eq!(size, 13);
    assert_eq!(u16::from_le_bytes([buf[0], buf[1]]), HEADER_CG_ATTACK);
    assert_eq!(u16::from_le_bytes([buf[2], buf[3]]), 13);
    assert_eq!(buf[4], 7);
    assert_eq!(u32::from_le_bytes([buf[5], buf[6], buf[7], buf[8]]), 4444);
    assert_eq!(u32::from_le_bytes([buf[9], buf[10], buf[11], buf[12]]), 0xABCDEF01);
}

#[test]
fn test_ffi_send_shoot() {
    let mut buf = [0u8; 128];
    let size = m2_combat_skill_send_shoot(9, 3, buf.as_mut_ptr());
    
    assert_eq!(size, 6);
    assert_eq!(u16::from_le_bytes([buf[0], buf[1]]), HEADER_CG_SHOOT);
    assert_eq!(u16::from_le_bytes([buf[2], buf[3]]), 6);
    assert_eq!(buf[4], 9);
    assert_eq!(buf[5], 3);
}

#[test]
fn test_ffi_send_use_skill_with_cooldown() {
    let mut buf = [0u8; 128];
    let skill_vnum = 100;
    let target_vid = 5555;
    let cooldown_ms = 50;
    
    // 1st use - should succeed
    let size1 = m2_combat_skill_send_use_skill(skill_vnum, target_vid, cooldown_ms, buf.as_mut_ptr());
    assert_eq!(size1, 12);
    
    assert_eq!(u16::from_le_bytes([buf[0], buf[1]]), HEADER_CG_USE_SKILL);
    assert_eq!(u16::from_le_bytes([buf[2], buf[3]]), 12);
    assert_eq!(u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]), skill_vnum);
    assert_eq!(u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]), target_vid);
    
    // 2nd use immediately - should be rejected (cooldown active)
    let size2 = m2_combat_skill_send_use_skill(skill_vnum, target_vid, cooldown_ms, buf.as_mut_ptr());
    assert_eq!(size2, 0); // Blocked by cooldown
    
    // Wait for cooldown to expire
    sleep(Duration::from_millis(60));
    
    // 3rd use - should succeed again
    let size3 = m2_combat_skill_send_use_skill(skill_vnum, target_vid, cooldown_ms, buf.as_mut_ptr());
    assert_eq!(size3, 12);
}

#[test]
fn test_ffi_send_use_skill_multiple_skills_cooldown() {
    let mut buf = [0u8; 128];
    let cooldown_ms = 100;
    
    // Use Skill A
    let size_a1 = m2_combat_skill_send_use_skill(1, 1000, cooldown_ms, buf.as_mut_ptr());
    assert_eq!(size_a1, 12);
    
    // Use Skill B immediately - should succeed because it's a different skill
    let size_b1 = m2_combat_skill_send_use_skill(2, 1000, cooldown_ms, buf.as_mut_ptr());
    assert_eq!(size_b1, 12);
    
    // Use Skill A again - should fail
    let size_a2 = m2_combat_skill_send_use_skill(1, 1000, cooldown_ms, buf.as_mut_ptr());
    assert_eq!(size_a2, 0);
}

#[test]
fn test_ffi_null_pointers() {
    assert_eq!(m2_combat_skill_send_attack(1, 1, 1, std::ptr::null_mut()), 0);
    assert_eq!(m2_combat_skill_send_use_skill(1, 1, 1, std::ptr::null_mut()), 0);
    assert_eq!(m2_combat_skill_send_shoot(1, 1, std::ptr::null_mut()), 0);
}