#[path = "../src/skill_calculator.rs"]
pub mod skill_calculator;

use skill_calculator::{PlayerClass, PlayerStats, SkillCalculator};

#[test]
fn test_get_skill_multiplier() {
    assert_eq!(SkillCalculator::get_skill_multiplier(0), 0.0);
    assert_eq!(SkillCalculator::get_skill_multiplier(1), 0.52);
    assert_eq!(SkillCalculator::get_skill_multiplier(19), 0.88);
    assert_eq!(SkillCalculator::get_skill_multiplier(20), 1.0);
    assert_eq!(SkillCalculator::get_skill_multiplier(29), 1.36);
    assert_eq!(SkillCalculator::get_skill_multiplier(30), 1.5);
    assert_eq!(SkillCalculator::get_skill_multiplier(39), 2.04);
    assert_eq!(SkillCalculator::get_skill_multiplier(40), 2.5);
    assert_eq!(SkillCalculator::get_skill_multiplier(50), 2.5); // Max P limit test
}

#[test]
fn test_calculate_damage_level_1_vs_perfect_master() {
    let stats = PlayerStats { str: 90, dex: 10, int: 5, vit: 10 };
    let base_atk = 100.0;
    let k = 1.2;
    
    // Poziom 1 (Zwykly)
    let dmg_lv1 = SkillCalculator::calculate_damage(base_atk, k, 1, &stats, PlayerClass::Warrior);
    // Poziom 40 (Perfect Master)
    let dmg_p = SkillCalculator::calculate_damage(base_atk, k, 40, &stats, PlayerClass::Warrior);
    
    assert!(dmg_p > dmg_lv1, "Obrazenia na poziomie P powinny byc znacznie wieksze od poziomu 1");
    
    // Warrior bonus: (90 * 2 + 10) = 190
    // Lvl 1 mult: 0.52
    // Lvl 40 mult: 2.5
    // dmg1 = (100 + 190 * 1.2) * 0.52 = (100 + 228) * 0.52 = 328 * 0.52 = 170.56
    assert!((dmg_lv1 - 170.56).abs() < f32::EPSILON);
    
    // dmg40 = (100 + 190 * 1.2) * 2.5 = 328 * 2.5 = 820.0
    assert!((dmg_p - 820.0).abs() < f32::EPSILON);
}

#[test]
fn test_sura_int_based_scaling() {
    let stats_high_int = PlayerStats { str: 10, dex: 10, int: 90, vit: 10 };
    let stats_low_int = PlayerStats { str: 10, dex: 10, int: 10, vit: 10 };
    let base_atk = 50.0;
    let k = 1.5;
    
    let dmg_high = SkillCalculator::calculate_damage(base_atk, k, 20, &stats_high_int, PlayerClass::Sura);
    let dmg_low = SkillCalculator::calculate_damage(base_atk, k, 20, &stats_low_int, PlayerClass::Sura);
    
    assert!(dmg_high > dmg_low, "Obrazenia Sury powinny wzrastac wraz z Inteligencja (INT)");
    
    // Sura bonus (High INT): (90 * 2 + 10) = 190
    // Lvl 20 mult: 1.0
    // Dmg High = (50 + 190 * 1.5) * 1.0 = 50 + 285 = 335.0
    assert!((dmg_high - 335.0).abs() < f32::EPSILON);
    
    // Sura bonus (Low INT): (10 * 2 + 10) = 30
    // Dmg Low = (50 + 30 * 1.5) * 1.0 = 50 + 45 = 95.0
    assert!((dmg_low - 95.0).abs() < f32::EPSILON);
}

#[test]
fn test_strong_body_defense_g1() {
    let stats = PlayerStats { str: 10, dex: 10, int: 10, vit: 90 };
    // Silne Cialo na G1 (poziom 30)
    let def = SkillCalculator::calculate_strong_body_defense(30, &stats);
    
    // Mult: 1.5
    // Base def: 90 * 2.0 + 30.0 = 210.0
    // Total def: 210.0 * 1.5 = 315.0
    assert!((def - 315.0).abs() < f32::EPSILON);
}

#[test]
fn test_aura_attack_zero_stats() {
    let stats = PlayerStats::default(); // 0 wszzedzie
    let atk = SkillCalculator::calculate_aura_attack(40, &stats);
    
    // Lvl 40 Mult: 2.5
    // Base aura: 0 * 1.5 + 50.0 = 50.0
    // Total atk: 50.0 * 2.5 = 125.0
    assert!((atk - 125.0).abs() < f32::EPSILON);
}

#[test]
fn test_edge_case_level_0() {
    let stats = PlayerStats { str: 90, dex: 10, int: 5, vit: 10 };
    
    let dmg = SkillCalculator::calculate_damage(100.0, 1.0, 0, &stats, PlayerClass::Warrior);
    let aura = SkillCalculator::calculate_aura_attack(0, &stats);
    let strong_body = SkillCalculator::calculate_strong_body_defense(0, &stats);
    
    assert_eq!(dmg, 0.0);
    assert_eq!(aura, 0.0);
    assert_eq!(strong_body, 0.0);
}
