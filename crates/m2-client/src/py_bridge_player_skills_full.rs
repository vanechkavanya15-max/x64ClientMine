//! Python Bridge Player Skills Full Module
//! 
//! Implementuje API umiejetnosci gracza do integracji FFI z C++.
//! Zapewnia pobieranie wlasciwosci umiejetnosci bazujac na GameState.


use m2_client::game_state_unified::{SKILL_MAX_NUM, TSkillInstance};
use m2_client::py_bridge_player_skills::PLAYER_SKILLS;
use m2_client::py_bridge_skill::m2_py_skill_get_max_level;
use m2_client::py_bridge_ui_subsystems::m2_py_ui_skill_can_use_skill;

/// Pomocnicza funkcja do pobierania skilla w celu lepszej obslugi bledow
fn get_skill_instance(slot: i32) -> Result<TSkillInstance, &'static str> {
    if slot < 0 || slot as usize >= SKILL_MAX_NUM {
        return Err("Invalid slot index");
    }
    
    if let Ok(skills) = PLAYER_SKILLS.read() {
        Ok(skills[slot as usize])
    } else {
        Err("Failed to acquire read lock on PLAYER_SKILLS")
    }
}

/// Zwraca index umiejetnosci w danym slocie
#[no_mangle]
pub extern "C" fn m2_py_player_get_skill_index_full(slot_index: i32) -> i32 {
    match get_skill_instance(slot_index) {
        Ok(skill) => skill.dwIndex as i32,
        Err(_) => 0,
    }
}

/// Zwraca grade (stopien) umiejetnosci w danym slocie
#[no_mangle]
pub extern "C" fn get_skill_grade(slot_index: i32) -> i32 {
    match get_skill_instance(slot_index) {
        Ok(skill) => skill.iGrade,
        Err(_) => 0,
    }
}

/// Zwraca poziom umiejetnosci w danym slocie
#[no_mangle]
pub extern "C" fn get_skill_level(slot_index: i32) -> i32 {
    match get_skill_instance(slot_index) {
        Ok(skill) => skill.iLevel,
        Err(_) => 0,
    }
}

/// Zwraca czas odnowienia umiejetnosci
#[no_mangle]
pub extern "C" fn get_skill_cool_time(slot_index: i32) -> f32 {
    match get_skill_instance(slot_index) {
        Ok(skill) => skill.fCoolTime,
        Err(_) => 0.0,
    }
}

/// Sprawdza czy gracz moze uzyc umiejetnosci
#[no_mangle]
pub extern "C" fn can_use_skill(slot_index: i32) -> bool {
    match get_skill_instance(slot_index) {
        Ok(skill) => {
            // Logika can_use_skill przekazuje wywolanie dalej
            m2_py_ui_skill_can_use_skill(skill.dwIndex)
        },
        Err(_) => false,
    }
}

/// Sprawdza czy umiejetnosc jest aktywna
#[no_mangle]
pub extern "C" fn is_skill_active(slot_index: i32) -> i32 {
    match get_skill_instance(slot_index) {
        Ok(skill) => if skill.bActive { 1 } else { 0 },
        Err(_) => 0,
    }
}

/// Zwraca maksymalny poziom umiejetnosci
#[no_mangle]
pub extern "C" fn get_skill_max_level(vnum: i32) -> i32 {
    if vnum < 0 {
        return 0;
    }
    m2_py_skill_get_max_level(vnum as u32) as i32
}
