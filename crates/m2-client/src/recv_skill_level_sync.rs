use std::sync::{LazyLock, RwLock};
use std::collections::HashMap;

pub const SKILL_MAX_NUM: usize = 255;
pub const SKILL_MAX_LEVEL: u8 = 40;

const SKILL_POWERS: [u32; 50] = [
    0,
    5,  6,  8, 10, 12,
    14, 16, 18, 20, 22,
    24, 26, 28, 30, 32,
    34, 36, 38, 40, 50,
    52, 54, 56, 58, 60,
    63, 66, 69, 72, 82,
    85, 88, 91, 94, 98,
    102, 106, 110, 115, 125,
    125, 0, 0, 0, 0, 0, 0, 0, 0
];

pub fn get_skill_power(level: u8) -> u32 {
    if (level as usize) < SKILL_POWERS.len() {
        SKILL_POWERS[level as usize]
    } else {
        0
    }
}

#[repr(C, packed)]
pub struct TPacketGCSkillLevel {
    pub header: u16,
    pub length: u16,
    pub skill_levels: [u8; SKILL_MAX_NUM],
}

// In Metin2 MSVC C++20 for x64, time_t is an 8-byte integer (i64).
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPlayerSkill {
    pub master_type: u8,
    pub level: u8,
    pub next_read: i64,
}

#[repr(C, packed)]
pub struct TPacketGCSkillLevelNew {
    pub header: u16,
    pub length: u16,
    pub skills: [TPlayerSkill; SKILL_MAX_NUM],
}

#[repr(C, packed)]
pub struct TPacketGCSkillCoolTimeEnd {
    pub header: u16,
    pub length: u16,
    pub skill: u8,
}

#[derive(Clone, Debug)]
pub struct SkillSlotData {
    pub grade: i32,
    pub level: u8,
    pub cur_efficient_percentage: f32,
    pub next_efficient_percentage: f32,
    pub is_cooltime: bool,
    pub sp_cost: i32,
    pub stamina_cost: i32,
}

impl Default for SkillSlotData {
    fn default() -> Self {
        Self {
            grade: -1,
            level: 0,
            cur_efficient_percentage: 0.0,
            next_efficient_percentage: 0.0,
            is_cooltime: false,
            sp_cost: 0,
            stamina_cost: 0,
        }
    }
}

#[derive(Clone, Default)]
pub struct PlayerSkillState {
    pub skills: HashMap<u32, SkillSlotData>,
    pub combo_skill_index: u32,
    pub riding_skill_index: u32,
}

pub static GLOBAL_SKILL_STATE: LazyLock<RwLock<PlayerSkillState>> = LazyLock::new(|| RwLock::new(PlayerSkillState::default()));

pub fn process_skill_level_sync(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < std::mem::size_of::<TPacketGCSkillLevel>() {
        return Err("Buffer too short for TPacketGCSkillLevel");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCSkillLevel) };

    let mut state = GLOBAL_SKILL_STATE.write().map_err(|_| "Failed to lock state")?;

    for i in 0..SKILL_MAX_NUM {
        let level = packet.skill_levels[i];
        let mut skill_data = state.skills.entry(i as u32).or_default().clone();
        
        skill_data.grade = -1;
        skill_data.level = level;
        
        if level <= 0 {
            skill_data.is_cooltime = false; // ResetSkillCoolTimeForSlot
        }

        if level > SKILL_MAX_LEVEL {
            skill_data.cur_efficient_percentage = 0.0;
            skill_data.next_efficient_percentage = 0.0;
        } else {
            skill_data.cur_efficient_percentage = get_skill_power(level) as f32 / 100.0;
            skill_data.next_efficient_percentage = get_skill_power(level.saturating_add(1)) as f32 / 100.0;
        }

        state.skills.insert(i as u32, skill_data);
    }

    Ok(())
}

pub fn process_skill_level_new_sync(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < std::mem::size_of::<TPacketGCSkillLevelNew>() {
        return Err("Buffer too short for TPacketGCSkillLevelNew");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCSkillLevelNew) };

    let mut state = GLOBAL_SKILL_STATE.write().map_err(|_| "Failed to lock state")?;

    state.combo_skill_index = 0;
    state.riding_skill_index = 0;

    for i in 0..SKILL_MAX_NUM {
        let skill = packet.skills[i];
        
        if i >= 112 && i <= 115 && skill.level > 0 {
            state.combo_skill_index = i as u32;
        }

        if i >= 116 && i <= 119 && skill.level > 0 {
            state.riding_skill_index = i as u32;
        }

        let mut final_level = skill.level;

        match skill.master_type {
            0 => { }
            1 => { final_level = final_level.saturating_sub(20) + 1; }
            2 => { final_level = final_level.saturating_sub(30) + 1; }
            3 => { final_level = final_level.saturating_sub(40) + 1; }
            _ => { }
        }

        let mut skill_data = state.skills.entry(i as u32).or_default().clone();
        
        skill_data.grade = skill.master_type as i32;
        skill_data.level = final_level;

        if final_level <= 0 {
            skill_data.is_cooltime = false; // ResetSkillCoolTimeForSlot
        }

        if skill.level > SKILL_MAX_LEVEL {
            skill_data.cur_efficient_percentage = 0.0;
            skill_data.next_efficient_percentage = 0.0;
        } else {
            skill_data.cur_efficient_percentage = get_skill_power(skill.level) as f32 / 100.0;
            skill_data.next_efficient_percentage = get_skill_power(skill.level.saturating_add(1)) as f32 / 100.0;
        }

        state.skills.insert(i as u32, skill_data);
    }

    Ok(())
}

pub fn process_skill_cooltime_end_sync(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < std::mem::size_of::<TPacketGCSkillCoolTimeEnd>() {
        return Err("Buffer too short for TPacketGCSkillCoolTimeEnd");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCSkillCoolTimeEnd) };

    let mut state = GLOBAL_SKILL_STATE.write().map_err(|_| "Failed to lock state")?;

    if let Some(skill) = state.skills.get_mut(&(packet.skill as u32)) {
        skill.is_cooltime = false;
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn m2_recv_skill_level_sync(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_skill_level_sync(slice).is_ok()
}

#[no_mangle]
pub extern "C" fn m2_recv_skill_level_new_sync(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_skill_level_new_sync(slice).is_ok()
}

#[no_mangle]
pub extern "C" fn m2_recv_skill_cooltime_end_sync(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_skill_cooltime_end_sync(slice).is_ok()
}
