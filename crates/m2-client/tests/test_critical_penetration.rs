#[path = "../src/critical_penetration.rs"]
mod critical_penetration;

use critical_penetration::calculate_damage;

#[test]
fn test_normal_attack() {
    let result = calculate_damage(
        100, // base_damage
        20,  // victim_defense
        10,  // critical_chance
        10,  // penetrate_chance
        50,  // rand_pct_critical (50 >= 10, so no crit)
        50,  // rand_pct_penetrate (50 >= 10, so no pen)
    );
    assert!(!result.is_critical);
    assert!(!result.is_penetrating);
    assert_eq!(result.final_damage, 80); // 100 - 20 = 80
}

#[test]
fn test_critical_hit() {
    let result = calculate_damage(
        100, // base_damage
        20,  // victim_defense
        30,  // critical_chance
        10,  // penetrate_chance
        10,  // rand_pct_critical (10 < 30, so crit)
        50,  // rand_pct_penetrate (50 >= 10, so no pen)
    );
    assert!(result.is_critical);
    assert!(!result.is_penetrating);
    assert_eq!(result.final_damage, 160); // (100 - 20) * 2 = 160
}

#[test]
fn test_penetrating_hit() {
    let result = calculate_damage(
        100, // base_damage
        20,  // victim_defense
        10,  // critical_chance
        30,  // penetrate_chance
        50,  // rand_pct_critical (50 >= 10, so no crit)
        10,  // rand_pct_penetrate (10 < 30, so pen)
    );
    assert!(!result.is_critical);
    assert!(result.is_penetrating);
    assert_eq!(result.final_damage, 120); // 100 + 20 = 120
}

#[test]
fn test_critical_and_penetrating_hit() {
    let result = calculate_damage(
        100, // base_damage
        20,  // victim_defense
        30,  // critical_chance
        30,  // penetrate_chance
        10,  // rand_pct_critical (10 < 30, so crit)
        10,  // rand_pct_penetrate (10 < 30, so pen)
    );
    assert!(result.is_critical);
    assert!(result.is_penetrating);
    assert_eq!(result.final_damage, 240); // (100 + 20) * 2 = 240
}

#[test]
fn test_minimum_damage_normal() {
    let result = calculate_damage(
        20,  // base_damage
        100, // victim_defense
        0,   // critical_chance
        0,   // penetrate_chance
        50,  // rand_pct_critical
        50,  // rand_pct_penetrate
    );
    assert!(!result.is_critical);
    assert!(!result.is_penetrating);
    assert_eq!(result.final_damage, 1); // 20 - 100 = -80 => 1
}

#[test]
fn test_minimum_damage_critical() {
    let result = calculate_damage(
        20,  // base_damage
        100, // victim_defense
        100, // critical_chance
        0,   // penetrate_chance
        50,  // rand_pct_critical
        50,  // rand_pct_penetrate
    );
    assert!(result.is_critical);
    assert!(!result.is_penetrating);
    assert_eq!(result.final_damage, 1); // (20 - 100) * 2 = -160 => 1
}

#[test]
fn test_penetrating_with_high_defense() {
    let result = calculate_damage(
        20,  // base_damage
        100, // victim_defense
        0,   // critical_chance
        100, // penetrate_chance
        50,  // rand_pct_critical
        50,  // rand_pct_penetrate
    );
    assert!(!result.is_critical);
    assert!(result.is_penetrating);
    assert_eq!(result.final_damage, 120); // 20 + 100 = 120
}

#[test]
fn test_zero_defense() {
    let result = calculate_damage(
        100, // base_damage
        0,   // victim_defense
        0,   // critical_chance
        0,   // penetrate_chance
        50,  // rand_pct_critical
        50,  // rand_pct_penetrate
    );
    assert!(!result.is_critical);
    assert!(!result.is_penetrating);
    assert_eq!(result.final_damage, 100);
}

#[test]
fn test_negative_base_damage() {
    let result = calculate_damage(
        -50, // base_damage
        20,  // victim_defense
        0,   // critical_chance
        0,   // penetrate_chance
        50,  // rand_pct_critical
        50,  // rand_pct_penetrate
    );
    assert_eq!(result.final_damage, 1); // Minimum damage constraint
}
