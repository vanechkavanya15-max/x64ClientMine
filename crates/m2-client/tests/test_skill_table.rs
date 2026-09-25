use m2_client::skill_table::{SkillTable, SkillNeedWeapon, SkillType};
 
 #[test]
fn test_integration_skill_table_ready() {
     let instance = SkillTable::new();
     assert!(instance.is_ready());
 }

#[test]
fn test_register_skill_desc() {
    let mut table = SkillTable::new();
    
    // Mocked TSV content based on DESC_TOKEN_TYPE
    // VNUM | JOB | NAME1 | NAME2 | NAME3 | DESCRIPTION | ... | ATTRIBUTE (10) | WEAPON (11) | ICON_NAME (12)
    let content = "1\twarrior\tTrzy Ciecia\t\t\tPotrojne uderzenie.\t\t\t\t\tMELEE_ATTACK|USE_HP|ATTACK_SKILL\tSWORD|TWO_HANDED\ticon/item/skill_1.tga\n";
    
    table.register_skill_desc(content);
    
    let skill_data = table.skill_data_map.get(&1).expect("Skill 1 should be registered");
    assert_eq!(skill_data.dw_skill_index, 1);
    assert_eq!(skill_data.by_type, SkillType::Active as u8);
    assert_eq!(skill_data.str_name, "Trzy Ciecia");
    assert_eq!(skill_data.str_description, "Potrojne uderzenie.");
    assert_eq!(skill_data.str_icon_file_name, "icon/item/skill_1.tga");
    
    assert!(skill_data.is_melee_skill());
    assert!(skill_data.is_use_hp_skill());
    assert!(skill_data.is_attack_skill());
    
    assert!(!skill_data.is_toggle_skill());
    
    assert!(skill_data.dw_need_weapon & (SkillNeedWeapon::Sword as u32) != 0);
    assert!(skill_data.dw_need_weapon & (SkillNeedWeapon::TwoHanded as u32) != 0);
    assert!(skill_data.dw_need_weapon & (SkillNeedWeapon::Bow as u32) == 0);
}

#[test]
fn test_register_skill_table() {
    let mut table = SkillTable::new();
    
    // First register desc to ensure the skill exists
    let desc_content = "1\twarrior\tTrzy Ciecia\t\t\tOpis\t\t\t\t\tMELEE_ATTACK\tSWORD\ticon.tga\n";
    table.register_skill_desc(desc_content);
    
    // Mocked TSV content based on TABLE_TOKEN_TYPE
    // 0:VNUM 1:NAME 2:TYPE 3:LEVEL_STEP 4:MAX_LEVEL 5:LEVEL_LIMIT 6:POINT_ON 7:POINT_POLY 8:SP_COST_POLY 9:DURATION_POLY 10:DURATION_SP_COST_POLY 11:COOLDOWN_POLY 12:MASTER_BONUS_POLY 13:ATTACK_GRADE_POLY 14:FLAG 15:AFFECT_FLAG 16:POINT_ON2 17:POINT_POLY2 18:DURATION_POLY2 19:AFFECT_FLAG2 20:PREREQUISITE_SKILL_VNUM 21:PREREQUISITE_SKILL_LEVEL 22:SKILL_TYPE 23:MAX_HIT 24:SPLASH_AROUND_DAMAGE_ADJUST_POLY 25:TARGET_RANGE 26:SPLASH_RANGE 27:MAX_NUM
    let table_content = "1\tTrzy_Ciecia\t1\t1\t40\t1\t0\t0\t50+5*k\t10\t0\t12\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t1\t1\t0\t250\t0\t1\n";
    
    table.register_skill_table(table_content);
    
    let skill_data = table.skill_data_map.get(&1).expect("Skill 1 should exist");
    assert_eq!(skill_data.by_max_level, 40);
    assert_eq!(skill_data.by_level_limit, 1);
    assert_eq!(skill_data.str_need_sp_formula, "50+5*k");
    assert_eq!(skill_data.str_duration, "10");
    assert_eq!(skill_data.str_continuation_sp_formula, "0");
    assert_eq!(skill_data.str_cool_time_formula, "12");
    assert_eq!(skill_data.dw_target_range, 250);
}

#[test]
fn test_skill_attributes_flags() {
    let mut table = SkillTable::new();
    let desc_content = "2\tsura\tCzarowane Ostrze\t\t\tOpis\t\t\t\t\tTOGGLE|USE_HP|STANDING_SKILL\tSWORD\ticon.tga\n";
    table.register_skill_desc(desc_content);
    
    let skill_data = table.skill_data_map.get(&2).unwrap();
    assert!(skill_data.is_toggle_skill());
    assert!(skill_data.is_use_hp_skill());
    assert!(skill_data.is_standing_skill());
    assert!(!skill_data.is_horse_skill());
}

#[test]
fn test_poly_formulas() {
    let mut table = SkillTable::new();
    let desc_content = "1\twarrior\tTrzy Ciecia\t\t\tOpis\t\t\t\t\tMELEE_ATTACK\tSWORD\ticon.tga\n";
    table.register_skill_desc(desc_content);
    let table_content = "1\tTrzy_Ciecia\t1\t1\t40\t1\t0\t0\t50+5*k\t10\t0\t12\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t1\t1\t0\t250\t100\t1\n";
    table.register_skill_table(table_content);
    
    let skill_data = table.skill_data_map.get(&1).unwrap();
    
    assert_eq!(skill_data.dw_splash_range, 100);
    assert_eq!(skill_data.get_skill_cool_time(10.0), 12);
    assert_eq!(skill_data.get_duration(10.0), 10);
    assert_eq!(skill_data.get_need_sp(10.0), 100); // 50+5*10
}
