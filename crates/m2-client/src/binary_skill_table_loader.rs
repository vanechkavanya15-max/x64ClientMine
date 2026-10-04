//! Binary Skill Table Loader Module
//!
//! Ladowanie i indeksowanie tabeli umiejetnosci `SkillTable`:
//! Formuly obrazen, wymogi many (SP), czas odnowienia (Cooltime), zasieg obszarowy (Splash)
//! oraz poziomy zaawansowania: Podstawowy (1-16), Master (17-29), Grand Master (30-39), Perfect Master (40).

pub use crate::skill_table_core::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillGradeLevel {
    Normal,
    Master,
    GrandMaster,
    PerfectMaster,
}

#[derive(Debug, Clone)]
pub struct DetailedSkillEntry {
    pub vnum: u32,
    pub name: String,
    pub skill_type: u8,
    pub max_level: u8,
    pub splash_range: u32,
    pub base_sp: u32,
    pub sp_per_level: u32,
    pub base_cooltime: f32,
    pub damage_multiplier_base: f32,
}

impl DetailedSkillEntry {
    pub fn new(vnum: u32, name: &str, skill_type: u8) -> Self {
        Self {
            vnum,
            name: name.to_string(),
            skill_type,
            max_level: 40,
            splash_range: 200,
            base_sp: 20,
            sp_per_level: 2,
            base_cooltime: 15.0,
            damage_multiplier_base: 1.0,
        }
    }

    /// Okresla stopien zaawansowania umiejetnosci (Grade).
    pub fn get_grade(&self, level: u8) -> SkillGradeLevel {
        if level >= 40 {
            SkillGradeLevel::PerfectMaster
        } else if level >= 30 {
            SkillGradeLevel::GrandMaster
        } else if level >= 17 {
            SkillGradeLevel::Master
        } else {
            SkillGradeLevel::Normal
        }
    }

    /// Oblicza koszt SP dla zadanego poziomu skilla.
    pub fn calculate_sp_cost(&self, level: u8) -> u32 {
        let lvl = (level as u32).min(self.max_level as u32);
        self.base_sp + lvl * self.sp_per_level
    }

    /// Oblicza czas odnowienia.
    pub fn calculate_cooltime(&self, _level: u8) -> f32 {
        self.base_cooltime
    }
}

/// Ladowarka i baza danych tabeli umiejetnosci.
#[derive(Debug, Default, Clone)]
pub struct BinarySkillTableLoader {
    skills: HashMap<u32, DetailedSkillEntry>,
}

impl BinarySkillTableLoader {
    pub fn new() -> Self {
        let mut loader = Self {
            skills: HashMap::new(),
        };
        loader.register_default_skills();
        loader
    }

    pub fn register_default_skills(&mut self) {
        // Trzystopniowe Ciecie (Tripple Slash)
        let mut tripple = DetailedSkillEntry::new(1, "Tripple Slash", SKILL_TYPE_ATTACK);
        tripple.base_sp = 30;
        tripple.sp_per_level = 3;
        tripple.base_cooltime = 12.0;
        tripple.splash_range = 250;
        self.skills.insert(1, tripple);

        // Aura Miecza (Sword Aura)
        let mut aura = DetailedSkillEntry::new(4, "Aura of the Sword", SKILL_TYPE_BUFF);
        aura.base_sp = 50;
        aura.sp_per_level = 5;
        aura.base_cooltime = 60.0;
        aura.splash_range = 0;
        self.skills.insert(4, aura);
    }

    pub fn add_skill(&mut self, skill: DetailedSkillEntry) {
        self.skills.insert(skill.vnum, skill);
    }

    pub fn get_skill(&self, vnum: u32) -> Option<&DetailedSkillEntry> {
        self.skills.get(&vnum)
    }

    pub fn len(&self) -> usize {
        self.skills.len()
    }

    pub fn is_empty(&self) -> bool {
        self.skills.is_empty()
    }
}
