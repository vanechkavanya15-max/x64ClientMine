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

#[test]
fn test_py_skill_data_table_affect_description() {
    use m2_client::py_bridge_skill_data_table::{get_table, SkillDataTableEntry, AffectData, m2_py_skill_get_skill_affect_description};

    let mut table = get_table();
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 150,
        skill_type: 1,
        name: "Test Affect".to_string(),
        description: "Test Desc".to_string(),
        affects: vec![AffectData {
            description: "Damage increased by %.0f%% to %.0f%%".to_string(),
            min_formula: "10 * k".to_string(),
            max_formula: "20 * k".to_string(),
        }],
        skill_attribute: 0,
        need_weapon: 0,
    });
    drop(table);

    let desc_ptr = m2_py_skill_get_skill_affect_description(150, 0, 2.0);
    assert_eq!(get_str_from_ptr(desc_ptr), "Damage increased by 20% to 40%");
}

#[test]
fn test_py_skill_data_table_affect_power() {
    use m2_client::py_bridge_skill_data_table::{get_table, SkillDataTableEntry, AffectData, m2_py_skill_get_skill_affect_power};

    let mut table = get_table();
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 151,
        skill_type: 1,
        name: "Test Power".to_string(),
        description: "".to_string(),
        affects: vec![AffectData {
            description: "".to_string(),
            min_formula: "15 * k".to_string(),
            max_formula: "30 * k".to_string(),
        }],
        skill_attribute: 0,
        need_weapon: 0,
    });
    drop(table);

    let min_power = m2_py_skill_get_skill_affect_power(151, 0, 3.0, false);
    let max_power = m2_py_skill_get_skill_affect_power(151, 0, 3.0, true);
    
    assert_eq!(min_power, 45.0);
    assert_eq!(max_power, 90.0);
}

#[test]
fn test_py_skill_data_table_can_use_skill() {
    use m2_client::py_bridge_skill_data_table::{get_table, SkillDataTableEntry, m2_py_skill_can_use_skill};

    let mut table = get_table();
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 152,
        skill_type: 1,
        name: "Active Skill".to_string(),
        description: "".to_string(),
        affects: vec![],
        skill_attribute: 0, // Not passive
        need_weapon: 0,
    });
    
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 153,
        skill_type: 1,
        name: "Passive Skill".to_string(),
        description: "".to_string(),
        affects: vec![],
        skill_attribute: 1 << 17, // SKILL_ATTRIBUTE_PASSIVE
        need_weapon: 0,
    });
    drop(table);

    assert_eq!(m2_py_skill_can_use_skill_v2(152), true);
    assert_eq!(m2_py_skill_can_use_skill_v2(153), false);
}

#[test]
fn test_py_skill_data_table_get_name_and_desc() {
    use m2_client::py_bridge_skill_data_table::{get_table, SkillDataTableEntry, m2_py_skill_get_skill_name, m2_py_skill_get_skill_description, m2_py_skill_get_skill_type};

    let mut table = get_table();
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 156,
        skill_type: 2,
        name: "Heal".to_string(),
        description: "Restores HP".to_string(),
        affects: vec![],
        skill_attribute: 0,
        need_weapon: 0,
    });
    drop(table);

    assert_eq!(get_str_from_ptr(m2_py_skill_get_skill_name(156)), "Heal");
    assert_eq!(get_str_from_ptr(m2_py_skill_get_skill_description(156)), "Restores HP");
    assert_eq!(m2_py_skill_get_skill_type(156), 2);
}

#[test]
fn test_py_skill_data_table_can_use_weapon() {
    use m2_client::py_bridge_skill_data_table::{get_table, SkillDataTableEntry, m2_py_skill_can_use_weapon};

    let mut table = get_table();
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 154,
        skill_type: 1,
        name: "Sword Only Skill".to_string(),
        description: "".to_string(),
        affects: vec![],
        skill_attribute: 1 << 2, // SKILL_ATTRIBUTE_WEAPON_LIMITATION
        need_weapon: 1 << 1, // Let's say 1 is Sword
    });
    
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 155,
        skill_type: 1,
        name: "Any Weapon Skill".to_string(),
        description: "".to_string(),
        affects: vec![],
        skill_attribute: 0, // No WEAPON_LIMITATION
        need_weapon: 0, // Any
    });
    drop(table);

    assert_eq!(m2_py_skill_can_use_weapon(154, 1), true);  // Sword
    assert_eq!(m2_py_skill_can_use_weapon(154, 2), false); // Dagger
    assert_eq!(m2_py_skill_can_use_weapon(155, 5), true);  // Any
}
