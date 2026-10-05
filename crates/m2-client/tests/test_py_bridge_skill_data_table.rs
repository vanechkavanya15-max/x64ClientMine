use m2_client::py_bridge_skill_data_table::*;
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
fn test_get_skill_name_and_desc() {
    let mut table = get_table();
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 1,
        skill_type: 1,
        name: "Test Skill".to_string(),
        description: "Test Desc".to_string(),
        affects: vec![],
        skill_attribute: 0,
        need_weapon: 0,
    });
    drop(table);

    assert_eq!(get_str_from_ptr(m2_py_skill_get_skill_name_v2(1)), "Test Skill");
    assert_eq!(get_str_from_ptr(m2_py_skill_get_skill_description_v2(1)), "Test Desc");
    assert_eq!(m2_py_skill_get_skill_type_v2(1), 1);
}

#[test]
fn test_affect_description() {
    let mut table = get_table();
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 2,
        skill_type: 2,
        name: "Buff".to_string(),
        description: "Buff Desc".to_string(),
        affects: vec![
            AffectData {
                description: "Increases attack by %.0f - %.0f%%".to_string(),
                min_formula: "10 * 2".to_string(), // 20
                max_formula: "20 * 2".to_string(), // 40
            }
        ],
        skill_attribute: 0,
        need_weapon: 0,
    });
    drop(table);

    assert_eq!(get_str_from_ptr(m2_py_skill_get_skill_affect_description(2, 0, 1.0)), "Increases attack by 20 - 40%");
    assert_eq!(m2_py_skill_get_skill_affect_power(2, 0, 1.0, false), 20.0);
    assert_eq!(m2_py_skill_get_skill_affect_power(2, 0, 1.0, true), 40.0);
}

#[test]
fn test_can_use_skill() {
    let mut table = get_table();
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 3,
        skill_type: 1,
        name: "Active Skill".to_string(),
        description: "".to_string(),
        affects: vec![],
        skill_attribute: 0, // Not passive
        need_weapon: 0,
    });
    
    table.register_skill(SkillDataTableEntry {
        skill_vnum: 4,
        skill_type: 1,
        name: "Passive Skill".to_string(),
        description: "".to_string(),
        affects: vec![],
        skill_attribute: 1 << 17, // SKILL_ATTRIBUTE_PASSIVE
        need_weapon: 0,
    });
    drop(table);

    assert_eq!(m2_py_skill_can_use_skill_v2(3), true);
    assert_eq!(m2_py_skill_can_use_skill_v2(4), false);
}
