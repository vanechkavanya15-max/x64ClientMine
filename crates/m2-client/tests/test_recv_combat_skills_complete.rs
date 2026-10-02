use std::mem::size_of;

#[path = "../src/recv_combat_skills_complete.rs"]
mod recv_combat_skills_complete;

use recv_combat_skills_complete::*;

#[test]
fn test_damage_info_layout_and_ffi() {
    assert_eq!(size_of::<TPacketGCDamageInfo>(), 13);
    
    let mut data = vec![0u8; size_of::<TPacketGCDamageInfo>()];
    
    // Simulate invalid size
    assert!(!m2_recv_damage_info(data.as_ptr(), 5));
    
    // Simulate valid parse
    assert!(m2_recv_damage_info(data.as_ptr(), data.len()));
}

#[test]
fn test_fly_targeting_layout_and_ffi() {
    assert_eq!(size_of::<TPacketGCFlyTargeting>(), 20);
    
    let mut data = vec![0u8; size_of::<TPacketGCFlyTargeting>()];
    
    // Simulate invalid size
    assert!(!m2_recv_fly_targeting(data.as_ptr(), 5));
    
    // Simulate valid parse
    assert!(m2_recv_fly_targeting(data.as_ptr(), data.len()));
}

#[test]
fn test_create_fly_layout_and_ffi() {
    assert_eq!(size_of::<TPacketGCCreateFly>(), 13);
    
    let mut data = vec![0u8; size_of::<TPacketGCCreateFly>()];
    
    // Simulate invalid size
    assert!(!m2_recv_create_fly(data.as_ptr(), 5));
    
    // Simulate valid parse
    assert!(m2_recv_create_fly(data.as_ptr(), data.len()));
}

#[test]
fn test_pvp_layout_and_ffi() {
    assert_eq!(size_of::<TPacketGCPVP>(), 13);
    
    let mut data = vec![0u8; size_of::<TPacketGCPVP>()];
    
    assert!(!m2_recv_pvp(data.as_ptr(), 5));
    assert!(m2_recv_pvp(data.as_ptr(), data.len()));
}

#[test]
fn test_duel_start_layout_and_ffi() {
    assert_eq!(size_of::<TPacketGCDuelStart>(), 4);
    
    let mut data = vec![0u8; size_of::<TPacketGCDuelStart>() + 8]; // Contains 2 extra VIDs (u32 each)
    let packet = unsafe { &mut *(data.as_mut_ptr() as *mut TPacketGCDuelStart) };
    packet.header = 0x0415;
    packet.length = 12; // 4 (base) + 8
    
    assert!(!m2_recv_duel_start(data.as_ptr(), 2));
    assert!(m2_recv_duel_start(data.as_ptr(), data.len()));
}

#[test]
fn test_skill_level_layout_and_ffi() {
    assert_eq!(size_of::<TPacketGCSkillLevel>(), 259);
    
    let mut data = vec![0u8; size_of::<TPacketGCSkillLevel>()];
    
    assert!(!m2_recv_skill_level(data.as_ptr(), 50));
    assert!(m2_recv_skill_level(data.as_ptr(), data.len()));
}

#[test]
fn test_skill_level_new_layout_and_ffi() {
    assert_eq!(size_of::<TPlayerSkill>(), 10);
    assert_eq!(size_of::<TPacketGCSkillLevelNew>(), 4 + 10 * SKILL_MAX_NUM);
    
    let mut data = vec![0u8; size_of::<TPacketGCSkillLevelNew>()];
    
    assert!(!m2_recv_skill_level_new(data.as_ptr(), 100));
    assert!(m2_recv_skill_level_new(data.as_ptr(), data.len()));
}

#[test]
fn test_skill_cooltime_end_layout_and_ffi() {
    assert_eq!(size_of::<TPacketGCSkillCoolTimeEnd>(), 5);
    
    let mut data = vec![0u8; size_of::<TPacketGCSkillCoolTimeEnd>()];
    
    assert!(!m2_recv_skill_cooltime_end(data.as_ptr(), 2));
    assert!(m2_recv_skill_cooltime_end(data.as_ptr(), data.len()));
}
