#![allow(unused)]

#[path = "../src/recv_skill_level_sync.rs"]
mod recv_skill_level_sync;

use recv_skill_level_sync::*;

#[test]
fn test_skill_level_layout() {
    assert_eq!(std::mem::size_of::<TPacketGCSkillLevel>(), 259);
    assert_eq!(std::mem::size_of::<TPlayerSkill>(), 10);
    assert_eq!(std::mem::size_of::<TPacketGCSkillLevelNew>(), 2554);
    assert_eq!(std::mem::size_of::<TPacketGCSkillCoolTimeEnd>(), 5);
}

#[test]
fn test_skill_level_short_buffer() {
    let mut data = vec![0u8; 10];
    assert!(!m2_recv_skill_level_sync(data.as_ptr(), data.len()));
    assert!(!m2_recv_skill_level_new_sync(data.as_ptr(), data.len()));
    
    let short_cooltime = vec![0u8; 2];
    assert!(!m2_recv_skill_cooltime_end_sync(short_cooltime.as_ptr(), short_cooltime.len()));
}

#[test]
fn test_skill_level_sync() {
    let mut packet_data = vec![0u8; std::mem::size_of::<TPacketGCSkillLevel>()];
    
    let packet = unsafe { &mut *(packet_data.as_mut_ptr() as *mut TPacketGCSkillLevel) };
    packet.header = 1;
    packet.length = std::mem::size_of::<TPacketGCSkillLevel>() as u16;
    
    packet.skill_levels[0] = 5; // power: 12, next: 14
    packet.skill_levels[10] = 12; // power: 26, next: 28
    packet.skill_levels[200] = 45; // level > MAX (40) -> power: 0.0

    let res = m2_recv_skill_level_sync(packet_data.as_ptr(), packet_data.len());
    assert!(res);

    let state = GLOBAL_SKILL_STATE.read().unwrap();
    assert_eq!(state.skills.get(&0).unwrap().level, 5);
    assert_eq!(state.skills.get(&0).unwrap().grade, -1);
    assert_eq!(state.skills.get(&0).unwrap().cur_efficient_percentage, 0.12);
    assert_eq!(state.skills.get(&0).unwrap().next_efficient_percentage, 0.14);
    
    assert_eq!(state.skills.get(&10).unwrap().level, 12);
    assert_eq!(state.skills.get(&10).unwrap().grade, -1);
    assert_eq!(state.skills.get(&10).unwrap().cur_efficient_percentage, 0.26);
    assert_eq!(state.skills.get(&10).unwrap().next_efficient_percentage, 0.28);
    
    assert_eq!(state.skills.get(&200).unwrap().level, 45);
    assert_eq!(state.skills.get(&200).unwrap().grade, -1);
    assert_eq!(state.skills.get(&200).unwrap().cur_efficient_percentage, 0.0);
    assert_eq!(state.skills.get(&200).unwrap().next_efficient_percentage, 0.0);
}

#[test]
fn test_skill_level_new_sync_grades_and_combo() {
    let mut packet_data = vec![0u8; std::mem::size_of::<TPacketGCSkillLevelNew>()];
    
    let packet = unsafe { &mut *(packet_data.as_mut_ptr() as *mut TPacketGCSkillLevelNew) };
    packet.header = 2;
    packet.length = std::mem::size_of::<TPacketGCSkillLevelNew>() as u16;

    // Normal skill
    packet.skills[1].master_type = 0;
    packet.skills[1].level = 15;

    // Master skill
    packet.skills[2].master_type = 1;
    packet.skills[2].level = 25; // should become 25 - 20 + 1 = 6

    // Grand Master skill
    packet.skills[3].master_type = 2;
    packet.skills[3].level = 38; // should become 38 - 30 + 1 = 9

    // Perfect Master skill
    packet.skills[4].master_type = 3;
    packet.skills[4].level = 40; // should become 40 - 40 + 1 = 1

    // Combo
    packet.skills[114].master_type = 0;
    packet.skills[114].level = 2;

    // Riding
    packet.skills[117].master_type = 0;
    packet.skills[117].level = 10;

    let res = m2_recv_skill_level_new_sync(packet_data.as_ptr(), packet_data.len());
    assert!(res);

    let state = GLOBAL_SKILL_STATE.read().unwrap();

    assert_eq!(state.skills.get(&1).unwrap().grade, 0);
    assert_eq!(state.skills.get(&1).unwrap().level, 15);
    assert_eq!(state.skills.get(&1).unwrap().cur_efficient_percentage, 0.32); // level 15 power

    assert_eq!(state.skills.get(&2).unwrap().grade, 1);
    assert_eq!(state.skills.get(&2).unwrap().level, 6);
    assert_eq!(state.skills.get(&2).unwrap().cur_efficient_percentage, 0.60); // level 25 power

    assert_eq!(state.skills.get(&3).unwrap().grade, 2);
    assert_eq!(state.skills.get(&3).unwrap().level, 9);

    assert_eq!(state.skills.get(&4).unwrap().grade, 3);
    assert_eq!(state.skills.get(&4).unwrap().level, 1);

    assert_eq!(state.combo_skill_index, 114);
    assert_eq!(state.riding_skill_index, 117);
}

#[test]
fn test_skill_cooltime_end() {
    let mut state = GLOBAL_SKILL_STATE.write().unwrap();
    let mut skill = state.skills.entry(5).or_default();
    skill.is_cooltime = true;
    drop(state); // Drop lock to allow C function to acquire it
    
    let mut packet_data = vec![0u8; std::mem::size_of::<TPacketGCSkillCoolTimeEnd>()];
    let packet = unsafe { &mut *(packet_data.as_mut_ptr() as *mut TPacketGCSkillCoolTimeEnd) };
    packet.header = 3;
    packet.length = std::mem::size_of::<TPacketGCSkillCoolTimeEnd>() as u16;
    packet.skill = 5;
    
    let res = m2_recv_skill_cooltime_end_sync(packet_data.as_ptr(), packet_data.len());
    assert!(res);
    
    let state = GLOBAL_SKILL_STATE.read().unwrap();
    assert_eq!(state.skills.get(&5).unwrap().is_cooltime, false);
}
