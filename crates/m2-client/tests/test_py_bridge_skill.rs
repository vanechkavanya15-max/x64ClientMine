use m2_client::py_bridge_skill::*;
use m2_client::skill_table_core::{SkillData, SKILL_TYPE_ATTACK, SKILL_TYPE_BUFF};
use std::ffi::CStr;

fn get_str_from_ptr(ptr: *const std::os::raw::c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

#[test]
fn test_py_skill_bridge_warrior_aura() {
    let mut bridge = get_bridge();
    bridge.register_skill(
        SkillData {
            skill_vnum: 1, // Aura of the Sword
            skill_type: SKILL_TYPE_BUFF,
            base_sp_cost: 50,
            cooldown_base_ms: 10000,
            max_level: 40,
        },
        "Aura of the Sword",
        "Increases attack power.",
    );
    drop(bridge);

    assert_eq!(get_str_from_ptr(m2_py_skill_get_name(1)), "Aura of the Sword");
    assert_eq!(get_str_from_ptr(m2_py_skill_get_description(1)), "Increases attack power.");
    assert_eq!(m2_py_skill_get_type(1), SKILL_TYPE_BUFF);
    assert_eq!(m2_py_skill_get_max_level(1), 40);
}

#[test]
fn test_py_skill_bridge_ninja_ambush() {
    let mut bridge = get_bridge();
    bridge.register_skill(
        SkillData {
            skill_vnum: 31, // Ambush
            skill_type: SKILL_TYPE_ATTACK,
            base_sp_cost: 35,
            cooldown_base_ms: 15000,
            max_level: 40,
        },
        "Ambush",
        "Backstab attack with high damage.",
    );
    drop(bridge);

    assert_eq!(get_str_from_ptr(m2_py_skill_get_name(31)), "Ambush");
    assert_eq!(get_str_from_ptr(m2_py_skill_get_description(31)), "Backstab attack with high damage.");
    assert_eq!(m2_py_skill_get_type(31), SKILL_TYPE_ATTACK);
}

#[test]
fn test_py_skill_bridge_sura_dark_strike() {
    let mut bridge = get_bridge();
    bridge.register_skill(
        SkillData {
            skill_vnum: 71, // Dark Strike
            skill_type: SKILL_TYPE_ATTACK,
            base_sp_cost: 20,
            cooldown_base_ms: 7000,
            max_level: 40,
        },
        "Dark Strike",
        "Shoots a dark energy projectile.",
    );
    drop(bridge);

    assert_eq!(get_str_from_ptr(m2_py_skill_get_name(71)), "Dark Strike");
    assert_eq!(m2_py_skill_get_type(71), SKILL_TYPE_ATTACK);
    assert_eq!(m2_py_skill_get_max_level(71), 40);
}

#[test]
fn test_py_skill_bridge_shaman_dragon_roar() {
    let mut bridge = get_bridge();
    bridge.register_skill(
        SkillData {
            skill_vnum: 91, // Dragon's Roar
            skill_type: SKILL_TYPE_ATTACK,
            base_sp_cost: 60,
            cooldown_base_ms: 20000,
            max_level: 40,
        },
        "Dragon's Roar",
        "AoE fire attack.",
    );
    drop(bridge);

    assert_eq!(get_str_from_ptr(m2_py_skill_get_name(91)), "Dragon's Roar");
    assert_eq!(m2_py_skill_get_type(91), SKILL_TYPE_ATTACK);
}

#[test]
fn test_py_skill_bridge_grade_names() {
    assert_eq!(get_str_from_ptr(m2_py_skill_get_grade_name(0)), "Normal");
    assert_eq!(get_str_from_ptr(m2_py_skill_get_grade_name(1)), "Master");
    assert_eq!(get_str_from_ptr(m2_py_skill_get_grade_name(2)), "Grand Master");
    assert_eq!(get_str_from_ptr(m2_py_skill_get_grade_name(3)), "Perfect Master");
}

#[test]
fn test_py_skill_bridge_unregistered() {
    // Test unregistered skills return defaults/null pointers without crashing
    assert!(m2_py_skill_get_name(999).is_null());
    assert!(m2_py_skill_get_description(999).is_null());
    assert_eq!(m2_py_skill_get_type(999), 0);
    assert_eq!(m2_py_skill_get_max_level(999), 0);
    assert!(m2_py_skill_get_grade_name(99).is_null());
}
