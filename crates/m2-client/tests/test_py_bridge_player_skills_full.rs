#[path = "../src/py_bridge_player_skills_full.rs"]
mod py_bridge_player_skills_full;

use m2_client::py_bridge_player_skills::PLAYER_SKILLS;
use py_bridge_player_skills_full::*;

#[test]
fn test_get_skill_index() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[1].dwIndex = 4;
        skills[2].dwIndex = 19;
    }
    
    assert_eq!(m2_py_player_get_skill_index_full(1), 4);
    assert_eq!(m2_py_player_get_skill_index_full(2), 19);
    assert_eq!(m2_py_player_get_skill_index_full(-1), 0);
    assert_eq!(m2_py_player_get_skill_index_full(300), 0);
}

#[test]
fn test_get_skill_grade() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[3].iGrade = 2; // Grand Master
    }
    
    assert_eq!(get_skill_grade(3), 2);
    assert_eq!(get_skill_grade(-1), 0);
    assert_eq!(get_skill_grade(255), 0);
}

#[test]
fn test_get_skill_level() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[4].iLevel = 17; 
    }
    
    assert_eq!(get_skill_level(4), 17);
    assert_eq!(get_skill_level(-5), 0);
    assert_eq!(get_skill_level(255), 0);
}

#[test]
fn test_get_skill_cool_time() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[5].fCoolTime = 12.5; 
    }
    
    assert_eq!(get_skill_cool_time(5), 12.5);
    assert_eq!(get_skill_cool_time(-1), 0.0);
    assert_eq!(get_skill_cool_time(255), 0.0);
}

#[test]
fn test_is_skill_active() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[6].bActive = true; 
        skills[7].bActive = false; 
    }
    
    assert_eq!(is_skill_active(6), 1);
    assert_eq!(is_skill_active(7), 0);
    assert_eq!(is_skill_active(-1), 0);
    assert_eq!(is_skill_active(255), 0);
}

#[test]
fn test_can_use_skill() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[8].dwIndex = 1; // Aura of Sword (example)
    }
    
    // Testing the default behavior of m2_py_ui_skill_can_use_skill through FFI
    // Typically the mock logic in testing will return a default false if not populated, but it relies on bridge implementations.
    let _ = can_use_skill(8); // just ensure it doesn't panic
    assert_eq!(can_use_skill(-1), false);
    assert_eq!(can_use_skill(255), false);
}

#[test]
fn test_get_skill_max_level() {
    // FFI call into table bridge
    let _ = get_skill_max_level(1);
    assert_eq!(get_skill_max_level(-1), 0);
}
