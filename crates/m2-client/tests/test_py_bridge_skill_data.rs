#[path = "../src/py_bridge_skill_data.rs"]
mod py_bridge_skill_data;

use py_bridge_skill_data::{
    m2_py_skill_get_skill_need_sp, m2_py_skill_get_skill_level_up_point, m2_py_skill_is_toggle_skill,
    m2_py_skill_is_standing_skill, m2_py_skill_get_skill_type, SkillDataV3, get_bridge,
};

#[test]
fn test_skill_need_sp() {
    let mut skill = SkillDataV3::default();
    skill.skill_id = 1;
    skill.sp_cost[1] = 10;
    skill.sp_cost[40] = 100;

    get_bridge().register_skill(skill);

    assert_eq!(m2_py_skill_get_skill_need_sp(1, 1.0), 10);
    assert_eq!(m2_py_skill_get_skill_need_sp(1, 40.0), 100);
    assert_eq!(m2_py_skill_get_skill_need_sp(1, 50.0), 100); // capped
    assert_eq!(m2_py_skill_get_skill_need_sp(999, 1.0), 0); // not found
}

#[test]
fn test_skill_level_up_point() {
    let mut skill = SkillDataV3::default();
    skill.skill_id = 2;
    skill.level_up_point = 17;

    get_bridge().register_skill(skill);

    assert_eq!(m2_py_skill_get_skill_level_up_point(2), 17);
    assert_eq!(m2_py_skill_get_skill_level_up_point(999), 0);
}

#[test]
fn test_is_toggle_skill() {
    let mut skill1 = SkillDataV3::default();
    skill1.skill_id = 3;
    skill1.skill_attribute = 1 << 1; // TOGGLE

    let mut skill2 = SkillDataV3::default();
    skill2.skill_id = 4;
    skill2.skill_attribute = 1 << 2; // NOT TOGGLE

    get_bridge().register_skill(skill1);
    get_bridge().register_skill(skill2);

    assert!(m2_py_skill_is_toggle_skill(3));
    assert!(!m2_py_skill_is_toggle_skill(4));
    assert!(!m2_py_skill_is_toggle_skill(999));
}

#[test]
fn test_is_standing_skill() {
    let mut skill1 = SkillDataV3::default();
    skill1.skill_id = 5;
    skill1.skill_attribute = 1 << 6; // STANDING

    let mut skill2 = SkillDataV3::default();
    skill2.skill_id = 6;
    skill2.skill_attribute = 1 << 1; // TOGGLE

    get_bridge().register_skill(skill1);
    get_bridge().register_skill(skill2);

    assert!(m2_py_skill_is_standing_skill(5));
    assert!(!m2_py_skill_is_standing_skill(6));
    assert!(!m2_py_skill_is_standing_skill(999));
}

#[test]
fn test_skill_type() {
    let mut skill = SkillDataV3::default();
    skill.skill_id = 7;
    skill.skill_type = 2; // SUPPORT

    get_bridge().register_skill(skill);

    assert_eq!(m2_py_skill_get_skill_type(7), 2);
    assert_eq!(m2_py_skill_get_skill_type(999), 0);
}

#[test]
fn test_default() {
    let skill = SkillDataV3::default();
    assert_eq!(skill.skill_id, 0);
    assert_eq!(skill.skill_type, 0);
    assert_eq!(skill.level_up_point, 0);
    assert_eq!(skill.max_level, 40);
    assert_eq!(skill.skill_attribute, 0);
    assert_eq!(skill.sp_cost[0], 0);
}
