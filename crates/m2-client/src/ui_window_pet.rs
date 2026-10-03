// ui_window_pet.rs - Okno statusu zwierzaka (UiWindowPet) w Metin2 Pure Rust
// Wyswietlanie poziomu, wieku, czasu zycia, slotow na karmy i ewolucje.

use std::sync::{Mutex, OnceLock};
use crate::ui_window_core::{Window, WindowType};

pub const PET_FEED_SLOTS_MAX: usize = 3;
pub const PET_EVOLUTION_SLOTS_MAX: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetEvolutionStage {
    Young = 1,   // Maly zwierzak (lvl 1-40)
    Wild = 2,    // Dziki zwierzak (lvl 41-80)
    Brave = 3,   // Dzielny zwierzak (lvl 81-105)
    Heroic = 4,  // Bohaterski zwierzak (odblokowane umiejetnosci)
}

impl PetEvolutionStage {
    pub fn name(&self) -> &'static str {
        match self {
            PetEvolutionStage::Young => "Maly",
            PetEvolutionStage::Wild => "Dziki",
            PetEvolutionStage::Brave => "Dzielny",
            PetEvolutionStage::Heroic => "Bohaterski",
        }
    }

    pub fn min_level(&self) -> u32 {
        match self {
            PetEvolutionStage::Young => 1,
            PetEvolutionStage::Wild => 41,
            PetEvolutionStage::Brave => 81,
            PetEvolutionStage::Heroic => 81,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PetFeedSlot {
    pub item_vnum: u32,
    pub count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PetEvolutionSlot {
    pub item_vnum: u32,
    pub count: u32,
    pub is_ready: bool,
}

#[derive(Debug, Clone)]
pub struct UiWindowPet {
    pub base: Window,
    pub is_visible: bool,
    pub pet_vid: u32,
    pub pet_name: String,
    pub level: u32,
    pub exp: u64,
    pub max_exp: u64,
    pub age_days: u32,
    pub duration_seconds: u32,
    pub max_duration_seconds: u32,
    pub evolution_stage: PetEvolutionStage,
    pub feed_slots: [Option<PetFeedSlot>; PET_FEED_SLOTS_MAX],
    pub evolution_slots: [Option<PetEvolutionSlot>; PET_EVOLUTION_SLOTS_MAX],
    pub bonus_hp_pct: f32,
    pub bonus_def_pct: f32,
    pub bonus_sp_pct: f32,
}

impl Default for UiWindowPet {
    fn default() -> Self {
        let mut base = Window::new(0, WindowType::ThinBoard);
        base.width = 250;
        base.height = 380;
        base.name = "PetStatusWindow".to_string();
        base.is_show = false;

        Self {
            base,
            is_visible: false,
            pet_vid: 0,
            pet_name: "Zwierzak".to_string(),
            level: 1,
            exp: 0,
            max_exp: 1000,
            age_days: 1,
            duration_seconds: 86400, // 1 dzien
            max_duration_seconds: 604800, // 7 dni
            evolution_stage: PetEvolutionStage::Young,
            feed_slots: [None; PET_FEED_SLOTS_MAX],
            evolution_slots: [None; PET_EVOLUTION_SLOTS_MAX],
            bonus_hp_pct: 1.5,
            bonus_def_pct: 1.0,
            bonus_sp_pct: 1.0,
        }
    }
}

impl UiWindowPet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
        self.base.is_show = self.is_visible;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
        self.base.is_show = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
        self.base.is_show = false;
    }

    pub fn set_pet_info(
        &mut self,
        pet_vid: u32,
        pet_name: impl Into<String>,
        level: u32,
        exp: u64,
        max_exp: u64,
        age_days: u32,
        duration_seconds: u32,
        max_duration_seconds: u32,
        stage: PetEvolutionStage,
    ) {
        self.pet_vid = pet_vid;
        self.pet_name = pet_name.into();
        self.level = level;
        self.exp = exp;
        self.max_exp = max_exp.max(1);
        self.age_days = age_days;
        self.duration_seconds = duration_seconds;
        self.max_duration_seconds = max_duration_seconds;
        self.evolution_stage = stage;
    }

    pub fn format_age(&self) -> String {
        format!("Wiek: {} dni", self.age_days)
    }

    pub fn format_duration(&self) -> String {
        let days = self.duration_seconds / 86400;
        let hours = (self.duration_seconds % 86400) / 3600;
        let minutes = (self.duration_seconds % 3600) / 60;
        if days > 0 {
            format!("Czas zycia: {}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("Czas zycia: {}h {}m", hours, minutes)
        } else {
            format!("Czas zycia: {}m", minutes)
        }
    }

    pub fn format_level(&self) -> String {
        format!("Poziom: {}", self.level)
    }

    pub fn format_exp(&self) -> String {
        let pct = if self.max_exp > 0 {
            (self.exp as f64 / self.max_exp as f64) * 100.0
        } else {
            0.0
        };
        format!("EXP: {} / {} ({:.1}%)", self.exp, self.max_exp, pct)
    }

    pub fn format_evolution(&self) -> String {
        format!("Ewolucja: {}", self.evolution_stage.name())
    }

    /// Ustawia karme w slocie karmy (0..2)
    pub fn set_feed_slot(
        &mut self,
        slot_idx: usize,
        item_vnum: u32,
        count: u32,
    ) -> Result<(), &'static str> {
        if slot_idx >= PET_FEED_SLOTS_MAX {
            return Err("Nieprawidlowy indeks slotu karmy");
        }
        self.feed_slots[slot_idx] = Some(PetFeedSlot { item_vnum, count });
        Ok(())
    }

    /// Pobiera lub czyści slot karmy
    pub fn clear_feed_slot(&mut self, slot_idx: usize) -> Option<PetFeedSlot> {
        if slot_idx < PET_FEED_SLOTS_MAX {
            self.feed_slots[slot_idx].take()
        } else {
            None
        }
    }

    /// Karmienie zwierzaka zawartoscia danego slotu karmy (odnawia czas zycia lub daje exp)
    pub fn feed_pet_from_slot(&mut self, slot_idx: usize, added_duration: u32) -> Result<u32, &'static str> {
        if slot_idx >= PET_FEED_SLOTS_MAX {
            return Err("Nieprawidlowy indeks slotu karmy");
        }

        let slot = self.feed_slots[slot_idx].as_mut().ok_or("Slot karmy jest pusty")?;
        if slot.count > 1 {
            slot.count -= 1;
        } else {
            self.feed_slots[slot_idx] = None;
        }

        self.duration_seconds = (self.duration_seconds + added_duration).min(self.max_duration_seconds);
        Ok(self.duration_seconds)
    }

    /// Ustawia material do ewolucji w slocie (0..3)
    pub fn set_evolution_slot(
        &mut self,
        slot_idx: usize,
        item_vnum: u32,
        count: u32,
        is_ready: bool,
    ) -> Result<(), &'static str> {
        if slot_idx >= PET_EVOLUTION_SLOTS_MAX {
            return Err("Nieprawidlowy indeks slotu ewolucji");
        }
        self.evolution_slots[slot_idx] = Some(PetEvolutionSlot {
            item_vnum,
            count,
            is_ready,
        });
        Ok(())
    }

    /// Sprawdza czy zwierzak moze przejsc na wyzszy stopien ewolucji
    pub fn can_evolve(&self) -> bool {
        match self.evolution_stage {
            PetEvolutionStage::Young => self.level >= 40,
            PetEvolutionStage::Wild => self.level >= 80,
            PetEvolutionStage::Brave => self.level >= 81,
            PetEvolutionStage::Heroic => false, // Osiagnieto maksymalny stopien
        }
    }

    /// Przeprowadza ewolucje na nastepny stopien
    pub fn evolve(&mut self) -> Result<PetEvolutionStage, &'static str> {
        if !self.can_evolve() {
            return Err("Zwierzak nie spelnia wymagan do ewolucji");
        }

        // Sprawdzenie czy materialy sa gotowe
        for slot_opt in &self.evolution_slots {
            if let Some(slot) = slot_opt {
                if !slot.is_ready {
                    return Err("Wymagane materialy nie sa jeszcze gotowe");
                }
            }
        }

        // Czyszczenie slotow ewolucji po udanej ewolucji
        self.evolution_slots = [None; PET_EVOLUTION_SLOTS_MAX];

        let next_stage = match self.evolution_stage {
            PetEvolutionStage::Young => PetEvolutionStage::Wild,
            PetEvolutionStage::Wild => PetEvolutionStage::Brave,
            PetEvolutionStage::Brave => PetEvolutionStage::Heroic,
            PetEvolutionStage::Heroic => PetEvolutionStage::Heroic,
        };

        self.evolution_stage = next_stage;
        // Skalowanie bonusow peta po ewolucji
        self.bonus_hp_pct += 1.0;
        self.bonus_def_pct += 0.8;
        self.bonus_sp_pct += 0.5;

        Ok(self.evolution_stage)
    }
}

static UI_WINDOW_PET: OnceLock<Mutex<UiWindowPet>> = OnceLock::new();

pub fn get_ui_window_pet() -> &'static Mutex<UiWindowPet> {
    UI_WINDOW_PET.get_or_init(|| Mutex::new(UiWindowPet::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pet_window_creation_and_visibility() {
        let mut win = UiWindowPet::new();
        assert!(!win.is_visible);
        assert!(!win.base.is_show);

        win.toggle_visibility();
        assert!(win.is_visible);
        assert!(win.base.is_show);

        win.hide();
        assert!(!win.is_visible);
    }

    #[test]
    fn test_formatters() {
        let mut win = UiWindowPet::new();
        win.set_pet_info(
            5001,
            "Smoczek",
            45,
            3000,
            10000,
            12,
            90000, // 1d 1h 0m
            604800,
            PetEvolutionStage::Wild,
        );

        assert_eq!(win.format_age(), "Wiek: 12 dni");
        assert_eq!(win.format_duration(), "Czas zycia: 1d 1h 0m");
        assert_eq!(win.format_level(), "Poziom: 45");
        assert_eq!(win.format_exp(), "EXP: 3000 / 10000 (30.0%)");
        assert_eq!(win.format_evolution(), "Ewolucja: Dziki");
    }

    #[test]
    fn test_feed_slots() {
        let mut win = UiWindowPet::new();
        win.duration_seconds = 1000;
        win.max_duration_seconds = 5000;

        assert!(win.set_feed_slot(0, 55001, 2).is_ok());
        assert!(win.set_feed_slot(3, 55001, 1).is_err()); // Poza zakresem

        // Karmienie
        let new_dur = win.feed_pet_from_slot(0, 1500).unwrap();
        assert_eq!(new_dur, 2500);
        assert_eq!(win.feed_slots[0].unwrap().count, 1);

        // Drugie karmienie zuzywa ostatnia sztuke
        win.feed_pet_from_slot(0, 1500).unwrap();
        assert_eq!(win.feed_slots[0], None);
    }

    #[test]
    fn test_evolution_progression() {
        let mut win = UiWindowPet::new();
        win.level = 39;
        assert!(!win.can_evolve());

        win.level = 40;
        assert!(win.can_evolve());

        assert!(win.set_evolution_slot(0, 70001, 1, true).is_ok());
        let next = win.evolve().unwrap();
        assert_eq!(next, PetEvolutionStage::Wild);
        assert_eq!(win.evolution_stage, PetEvolutionStage::Wild);
        assert_eq!(win.evolution_slots[0], None);
    }
}
