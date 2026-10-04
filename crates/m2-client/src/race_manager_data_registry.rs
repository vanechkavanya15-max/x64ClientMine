//! Race Manager Data Registry Module
//!
//! Centralny rejestr ras i modeli CRaceManager: mapuje identyfikatory ras (race_id)
//! na sciezki katalogow modeli, pliki .msm, dane postaci oraz powiazania animacji ruchu.

use std::collections::HashMap;

/// Glowne typy ras postaci i potworow.
pub const RACE_WARRIOR_M: u32 = 0;
pub const RACE_ASSASSIN_W: u32 = 1;
pub const RACE_SURA_M: u32 = 2;
pub const RACE_SHAMAN_W: u32 = 3;
pub const RACE_WARRIOR_W: u32 = 4;
pub const RACE_ASSASSIN_M: u32 = 5;
pub const RACE_SURA_W: u32 = 6;
pub const RACE_SHAMAN_M: u32 = 7;
pub const RACE_WOLFMAN_M: u32 = 8;

/// Kody animacji ruchu (Motion Codes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotionType {
    Wait,
    Walk,
    Run,
    Combo,
    Damage,
    Dead,
    Special,
}

/// Dane rasy zarejestrowane w menedzerze ras.
#[derive(Debug, Clone)]
pub struct RaceData {
    pub race_id: u32,
    pub name: String,
    pub base_path: String,
    pub msm_filename: String,
    pub motions: HashMap<MotionType, String>,
}

impl RaceData {
    pub fn new(race_id: u32, name: &str, base_path: &str, msm_filename: &str) -> Self {
        Self {
            race_id,
            name: name.to_string(),
            base_path: base_path.to_string(),
            msm_filename: msm_filename.to_string(),
            motions: HashMap::new(),
        }
    }

    pub fn set_motion(&mut self, motion: MotionType, filename: &str) {
        self.motions.insert(motion, filename.to_string());
    }

    pub fn get_motion_path(&self, motion: MotionType) -> Option<String> {
        self.motions.get(&motion).map(|file| {
            format!("{}/{}", self.base_path.trim_end_matches('/'), file)
        })
    }

    pub fn get_msm_full_path(&self) -> String {
        format!("{}/{}", self.base_path.trim_end_matches('/'), self.msm_filename)
    }
}

/// Centralny rejestr ras i powiazan zasobow.
#[derive(Debug, Default, Clone)]
pub struct RaceManagerDataRegistry {
    races: HashMap<u32, RaceData>,
}

impl RaceManagerDataRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            races: HashMap::new(),
        };
        registry.register_default_pc_races();
        registry
    }

    /// Rejestruje domyslne rasy postaci graczy (PC).
    pub fn register_default_pc_races(&mut self) {
        // Wojownik mezczyzna
        let mut warrior_m = RaceData::new(RACE_WARRIOR_M, "warrior_m", "d:/ymir work/pc/warrior", "warrior_m.msm");
        warrior_m.set_motion(MotionType::Wait, "wait.msa");
        warrior_m.set_motion(MotionType::Walk, "walk.msa");
        warrior_m.set_motion(MotionType::Run, "run.msa");
        warrior_m.set_motion(MotionType::Combo, "combo_01.msa");
        self.races.insert(RACE_WARRIOR_M, warrior_m);

        // Ninja kobieta
        let mut assassin_w = RaceData::new(RACE_ASSASSIN_W, "assassin_w", "d:/ymir work/pc/assassin", "assassin_w.msm");
        assassin_w.set_motion(MotionType::Wait, "wait.msa");
        assassin_w.set_motion(MotionType::Walk, "walk.msa");
        assassin_w.set_motion(MotionType::Run, "run.msa");
        self.races.insert(RACE_ASSASSIN_W, assassin_w);

        // Sura mezczyzna
        let mut sura_m = RaceData::new(RACE_SURA_M, "sura_m", "d:/ymir work/pc/sura", "sura_m.msm");
        sura_m.set_motion(MotionType::Wait, "wait.msa");
        sura_m.set_motion(MotionType::Run, "run.msa");
        self.races.insert(RACE_SURA_M, sura_m);

        // Szaman kobieta
        let mut shaman_w = RaceData::new(RACE_SHAMAN_W, "shaman_w", "d:/ymir work/pc/shaman", "shaman_w.msm");
        shaman_w.set_motion(MotionType::Wait, "wait.msa");
        shaman_w.set_motion(MotionType::Run, "run.msa");
        self.races.insert(RACE_SHAMAN_W, shaman_w);
    }

    /// Rejestruje niestandardowa rase (np. moba, bossa lub wierzchowca).
    pub fn register_race(&mut self, race: RaceData) {
        self.races.insert(race.race_id, race);
    }

    /// Pobiera dane rasy o danym identyfikatorze.
    pub fn get_race(&self, race_id: u32) -> Option<&RaceData> {
        self.races.get(&race_id)
    }

    /// Liczba zarejestrowanych ras.
    pub fn len(&self) -> usize {
        self.races.len()
    }

    /// Czy rejestr jest pusty.
    pub fn is_empty(&self) -> bool {
        self.races.is_empty()
    }
}
