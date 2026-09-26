use m2_client::skill_data_cache_v2::*;

#[test]
fn test_skill_registration_and_retrieval() {
    let mut cache = SkillDataCacheV2::new();
    let mut data = SkillData::default();
    data.skill_id = 1;
    data.skill_type = SKILL_TYPE_ACTIVE;
    data.max_level = 20;
    
    cache.register_skill(data);

    let skill = cache.get_skill(1).expect("Skill should be registered");
    assert_eq!(skill.skill_id, 1);
    assert_eq!(skill.skill_type, SKILL_TYPE_ACTIVE);
    assert_eq!(skill.max_level, 20);
    
    assert!(cache.get_skill(999).is_none());
}

#[test]
fn test_calculate_actual_cooldown_no_bonus() {
    let mut cache = SkillDataCacheV2::new();
    let mut data = SkillData::default();
    data.skill_id = 1;
    data.base_cooldown_ms[1] = 10000; // 10s cooldown
    cache.register_skill(data);

    // No bonus cast speed (iCastingSpeed = 0)
    // Formula: iSpd = 10000 / (100 + 0) = 100
    // actual_cd = 10000 * 100 / 100 = 10000
    let actual_cd = cache.calculate_actual_cooldown(1, 1, 0);
    assert_eq!(actual_cd, 10000);
}

#[test]
fn test_calculate_actual_cooldown_positive_bonus() {
    let mut cache = SkillDataCacheV2::new();
    let mut data = SkillData::default();
    data.skill_id = 1;
    data.base_cooldown_ms[1] = 10000;
    cache.register_skill(data);

    // Bonus cast speed = 25
    // Formula: iSpd = 10000 / (100 + 25) = 80
    // actual_cd = 10000 * 80 / 100 = 8000
    let actual_cd = cache.calculate_actual_cooldown(1, 1, 25);
    assert_eq!(actual_cd, 8000);
}

#[test]
fn test_calculate_actual_cooldown_high_bonus() {
    let mut cache = SkillDataCacheV2::new();
    let mut data = SkillData::default();
    data.skill_id = 1;
    data.base_cooldown_ms[1] = 10000;
    cache.register_skill(data);

    // Bonus cast speed = 100 (double casting speed)
    // Formula: iSpd = 10000 / (100 + 100) = 50
    // actual_cd = 10000 * 50 / 100 = 5000
    let actual_cd = cache.calculate_actual_cooldown(1, 1, 100);
    assert_eq!(actual_cd, 5000);
}

#[test]
fn test_cooldown_pct_progress() {
    let mut cache = SkillDataCacheV2::new();
    let mut data = SkillData::default();
    data.skill_id = 1;
    data.base_cooldown_ms[1] = 10000;
    cache.register_skill(data);

    let cur_time = 1000;
    // casting speed 100 -> cd = 5000 ms
    cache.start_cooldown(1, 1, cur_time, 100); 

    // Zaraz po uzyciu, pct = 1.0
    let pct_start = cache.get_cooldown_pct(1, cur_time);
    assert_eq!(pct_start, 1.0);

    // W polowie czasu (2500ms po), pct = 0.5
    let pct_half = cache.get_cooldown_pct(1, cur_time + 2500);
    assert!((pct_half - 0.5).abs() < f32::EPSILON);

    // Po zakonczeniu cooldownu (5000ms), pct = 0.0
    let pct_end = cache.get_cooldown_pct(1, cur_time + 5000);
    assert_eq!(pct_end, 0.0);
    
    // Grubo po zakonczeniu cooldownu (15000ms), pct = 0.0
    let pct_over = cache.get_cooldown_pct(1, cur_time + 15000);
    assert_eq!(pct_over, 0.0);
}

#[test]
fn test_cooldown_pct_unregistered_skill() {
    let cache = SkillDataCacheV2::new();
    let pct = cache.get_cooldown_pct(999, 1000);
    assert_eq!(pct, 0.0);
}

#[test]
fn test_ffi_exports() {
    let cd_pct = m2_skill_get_cooldown_pct(99, 0);
    assert_eq!(cd_pct, 0.0);
}

#[test]
fn test_sp_cost_level_clamping() {
    let mut cache = SkillDataCacheV2::new();
    let mut data = SkillData::default();
    data.skill_id = 2;
    data.sp_cost[40] = 500;
    cache.register_skill(data);

    let cost = cache.get_skill(2).unwrap().sp_cost[40];
    assert_eq!(cost, 500);
    
    let calc_cd = cache.calculate_actual_cooldown(2, 255, 0); 
    assert_eq!(calc_cd, 0); 
}
