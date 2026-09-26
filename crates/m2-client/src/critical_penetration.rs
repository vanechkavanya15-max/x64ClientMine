pub struct DamageCalculationResult {
    pub final_damage: i32,
    pub is_critical: bool,
    pub is_penetrating: bool,
}

/// Calculates the final damage based on critical and penetrating chances.
///
/// # Arguments
/// * `base_damage` - The raw base damage.
/// * `victim_defense` - The target's defense.
/// * `critical_chance` - The chance to land a critical hit (0-100+).
/// * `penetrate_chance` - The chance to land a penetrating hit (0-100+).
/// * `rand_pct_critical` - A random value between 0 and 99 used to evaluate the critical chance.
/// * `rand_pct_penetrate` - A random value between 0 and 99 used to evaluate the penetrating chance.
pub fn calculate_damage(
    base_damage: i32,
    victim_defense: i32,
    critical_chance: i32,
    penetrate_chance: i32,
    rand_pct_critical: i32,
    rand_pct_penetrate: i32,
) -> DamageCalculationResult {
    // Determine if hits are critical or penetrating
    let is_critical = rand_pct_critical < critical_chance;
    let is_penetrating = rand_pct_penetrate < penetrate_chance;

    let mut final_damage = base_damage;

    // Normal base damage calculation without critical or penetration
    if !is_penetrating {
        final_damage -= victim_defense;
    } else {
        // Penetrating hits add the target's defense to the base damage
        final_damage += victim_defense;
    }

    if is_critical {
        // Critical hits double the resulting damage
        final_damage *= 2;
    }

    // Damage cannot be less than 1
    if final_damage < 1 {
        final_damage = 1;
    }

    DamageCalculationResult {
        final_damage,
        is_critical,
        is_penetrating,
    }
}
