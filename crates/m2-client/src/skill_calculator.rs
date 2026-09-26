#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerClass {
    Warrior,
    Ninja,
    Sura,
    Shaman,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerStats {
    pub str: u32,
    pub dex: u32,
    pub int: u32,
    pub vit: u32,
}

pub struct SkillCalculator;

impl SkillCalculator {
    /// Oblicza mnoznik skilla na podstawie jego poziomu (1..40).
    /// Poziomy:
    /// 1..19: Zwykly skill
    /// 20..29: Master (M1..M10)
    /// 30..39: Grand Master (G1..G10)
    /// 40: Perfect Master (P)
    pub fn get_skill_multiplier(level: u8) -> f32 {
        match level {
            0 => 0.0,
            1..=19 => 0.5 + (level as f32) * 0.02,
            20..=29 => 1.0 + (level as f32 - 20.0) * 0.04,
            30..=39 => 1.5 + (level as f32 - 30.0) * 0.06,
            40..=u8::MAX => 2.5,
        }
    }

    /// Oblicza bonus do statystyk dla obrazen bazowych zaleznie od klasy
    pub fn get_stat_bonus(stats: &PlayerStats, class: PlayerClass) -> f32 {
        match class {
            PlayerClass::Warrior => (stats.str * 2 + stats.dex) as f32,
            PlayerClass::Ninja => (stats.dex * 2 + stats.str) as f32,
            PlayerClass::Sura => (stats.int * 2 + stats.str) as f32,
            PlayerClass::Shaman => (stats.int * 2 + stats.dex) as f32,
        }
    }

    /// Oblicza koncowe obrazenia ze skilla: (base_atk + stat_bonus * k) * skill_level_multiplier
    pub fn calculate_damage(base_atk: f32, k: f32, skill_level: u8, stats: &PlayerStats, class: PlayerClass) -> f32 {
        if skill_level == 0 {
            return 0.0;
        }
        let stat_bonus = Self::get_stat_bonus(stats, class);
        let multiplier = Self::get_skill_multiplier(skill_level);
        
        (base_atk + stat_bonus * k) * multiplier
    }

    /// Oblicza wartosc dodawanego ataku przez buff Aura Miecza
    pub fn calculate_aura_attack(skill_level: u8, stats: &PlayerStats) -> f32 {
        if skill_level == 0 {
            return 0.0;
        }
        let multiplier = Self::get_skill_multiplier(skill_level);
        // Aura bazuje glownie na Sile (STR)
        let base_aura = (stats.str as f32) * 1.5 + 50.0;
        base_aura * multiplier
    }

    /// Oblicza wartosc dodawanej obrony przez buff Silne Cialo
    pub fn calculate_strong_body_defense(skill_level: u8, stats: &PlayerStats) -> f32 {
        if skill_level == 0 {
            return 0.0;
        }
        let multiplier = Self::get_skill_multiplier(skill_level);
        // Silne Cialo bazuje glownie na Witalnosci (VIT)
        let base_def = (stats.vit as f32) * 2.0 + 30.0;
        base_def * multiplier
    }
}
