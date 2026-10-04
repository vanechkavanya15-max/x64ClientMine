use std::sync::{LazyLock, RwLock};

use crate::app_timing_clock::m2_get_time;
use crate::game_state_unified::{SKILL_MAX_NUM, TSkillInstance};

pub static PLAYER_SKILLS: LazyLock<RwLock<[TSkillInstance; SKILL_MAX_NUM]>> =
    LazyLock::new(|| RwLock::new([TSkillInstance::default(); SKILL_MAX_NUM]));

#[no_mangle]
pub extern "C" fn m2_py_player_get_skill_grade(slot_index: i32) -> i32 {
    if slot_index < 0 || slot_index as usize >= SKILL_MAX_NUM {
        return 0;
    }
    if let Ok(skills) = PLAYER_SKILLS.read() {
        return skills[slot_index as usize].iGrade;
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_skill_index(slot_index: i32) -> i32 {
    if slot_index < 0 || slot_index as usize >= SKILL_MAX_NUM {
        return 0;
    }
    if let Ok(skills) = PLAYER_SKILLS.read() {
        return skills[slot_index as usize].dwIndex as i32;
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_skill_slot_index(skill_index: i32) -> i32 {
    if let Ok(skills) = PLAYER_SKILLS.read() {
        for (i, skill) in skills.iter().enumerate() {
            if skill.dwIndex == skill_index as u32 {
                return i as i32;
            }
        }
    }
    0 // Match CPythonPlayer returning 0 or false, FFI layer returns 0 when not found.
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_skill_current_efficient_percentage(slot_index: i32) -> f32 {
    if slot_index < 0 || slot_index as usize >= SKILL_MAX_NUM {
        return 0.0;
    }
    if let Ok(skills) = PLAYER_SKILLS.read() {
        return skills[slot_index as usize].fcurEfficientPercentage;
    }
    0.0
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_skill_next_efficient_percentage(slot_index: i32) -> f32 {
    if slot_index < 0 || slot_index as usize >= SKILL_MAX_NUM {
        return 0.0;
    }
    if let Ok(skills) = PLAYER_SKILLS.read() {
        return skills[slot_index as usize].fnextEfficientPercentage;
    }
    0.0
}

#[no_mangle]
pub extern "C" fn m2_py_player_is_skill_active(slot_index: i32) -> i32 {
    if slot_index < 0 || slot_index as usize >= SKILL_MAX_NUM {
        return 0;
    }
    if let Ok(skills) = PLAYER_SKILLS.read() {
        return if skills[slot_index as usize].bActive { 1 } else { 0 };
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_py_player_is_skill_cool_time(slot_index: i32) -> i32 {
    if slot_index < 0 || slot_index as usize >= SKILL_MAX_NUM {
        return 0;
    }
    
    if let Ok(skills) = PLAYER_SKILLS.read() {
        let skill = &skills[slot_index as usize];
        let current_time = m2_get_time() as f32 / 1000.0;
        let elapsed = current_time - skill.fLastUsedTime;
        
        if elapsed >= skill.fCoolTime {
            return 0;
        } else {
            return 1;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_skill_level(slot_index: u32) -> u32 {
    if (slot_index as usize) >= SKILL_MAX_NUM {
        return 0;
    }
    if let Ok(skills) = PLAYER_SKILLS.read() {
        return skills[slot_index as usize].iLevel as u32;
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_skill_cool_time(slot_index: u32) -> f32 {
    if (slot_index as usize) >= SKILL_MAX_NUM {
        return 0.0;
    }
    if let Ok(skills) = PLAYER_SKILLS.read() {
        return skills[slot_index as usize].fCoolTime;
    }
    0.0
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_skill_elapsed_cool_time(slot_index: u32) -> f32 {
    if (slot_index as usize) >= SKILL_MAX_NUM {
        return 0.0;
    }
    if let Ok(skills) = PLAYER_SKILLS.read() {
        let skill = &skills[slot_index as usize];
        if skill.fCoolTime <= 0.0 || skill.fLastUsedTime <= 0.0 {
            return 0.0;
        }
        let current_time = m2_get_time() as f32 / 1000.0;
        let elapsed = current_time - skill.fLastUsedTime;
        return elapsed.clamp(0.0, skill.fCoolTime);
    }
    0.0
}


