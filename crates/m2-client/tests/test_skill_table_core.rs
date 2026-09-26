use m2_client::skill_table_core::{SkillTableCore, SkillData, SKILL_TYPE_ATTACK, SKILL_TYPE_BUFF};

#[test]
fn test_register_and_get_skill() {
    let mut table = SkillTableCore::new();
    table.register_skill(SkillData {
        skill_vnum: 1,
        skill_type: SKILL_TYPE_ATTACK,
        base_sp_cost: 50,
        cooldown_base_ms: 1000,
        max_level: 40,
    });

    let skill = table.get_skill(1).unwrap();
    assert_eq!(skill.skill_vnum, 1);
    assert_eq!(skill.skill_type, SKILL_TYPE_ATTACK);
    assert_eq!(skill.base_sp_cost, 50);
}

#[test]
fn test_sp_cost_scaling() {
    let mut table = SkillTableCore::new();
    table.register_skill(SkillData {
        skill_vnum: 2,
        skill_type: SKILL_TYPE_BUFF,
        base_sp_cost: 100,
        cooldown_base_ms: 2000,
        max_level: 40,
    });

    // Level 0: 0 cost
    assert_eq!(table.calculate_sp_cost(2, 0), Some(0));
    
    // Level 1: Base cost
    assert_eq!(table.calculate_sp_cost(2, 1), Some(100));
    
    // Level 2: Base cost + 10% = 110
    assert_eq!(table.calculate_sp_cost(2, 2), Some(110));
    
    // Level 11: Base cost + 100% = 200
    assert_eq!(table.calculate_sp_cost(2, 11), Some(200));

    // Level 40: Base cost + 390% = 490
    assert_eq!(table.calculate_sp_cost(2, 40), Some(490));

    // Over max level: Caps at max level
    assert_eq!(table.calculate_sp_cost(2, 50), Some(490));
}

#[test]
fn test_can_use_skill() {
    let mut table = SkillTableCore::new();
    table.register_skill(SkillData {
        skill_vnum: 3,
        skill_type: SKILL_TYPE_ATTACK,
        base_sp_cost: 100,
        cooldown_base_ms: 1000,
        max_level: 40,
    });

    // Level 1 cost is 100 SP
    assert_eq!(table.can_use_skill(3, 1, 150), Ok(true));
    assert_eq!(table.can_use_skill(3, 1, 100), Ok(true));
    assert_eq!(table.can_use_skill(3, 1, 99), Ok(false));

    // Level 11 cost is 200 SP
    assert_eq!(table.can_use_skill(3, 11, 250), Ok(true));
    assert_eq!(table.can_use_skill(3, 11, 150), Ok(false));

    // Skill not found
    assert_eq!(table.can_use_skill(999, 1, 1000), Err("Skill not found"));
}
