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

    // W silniku Metin2 C++ (char_battle.cpp CalcMeleeDamage):
    // 1. Obrazenia bazowe: dam = base_damage - victim_defense
    let mut dam = base_damage - victim_defense;

    // 2. Jesli krytyk: podwojenie czystych obrazen
    if is_critical {
        dam *= 2;
    }

    // 3. Jesli przeszywka: anulowanie odjetej obrony (ignorowanie pancerza)
    // dam += victim_defense
    if is_penetrating {
        dam += victim_defense;
    }

    // Minimalne obrazenia w grze to 1
    let final_damage = if dam < 1 { 1 } else { dam };

    DamageCalculationResult {
        final_damage,
        is_critical,
        is_penetrating,
    }
}
