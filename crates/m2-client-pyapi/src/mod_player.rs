//! Modul mod_player dla skrzynki m2-client-pyapi
//! Modul Pythona player: statystyki, ekwipunek i skille
//! Metody player.GetStatus, player.GetItemIndex, player.GetItemCount, player.GetSkillLevel

use std::collections::HashMap;

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModPlayer {
    pub initialized: bool,
    pub statuses: HashMap<u32, i32>,
    pub inventory_index: HashMap<u32, u32>,
    pub inventory_count: HashMap<u32, u32>,
    pub skills: HashMap<u32, i32>,
}

impl ModPlayer {
    pub fn new() -> Self {
        Self {
            initialized: true,
            statuses: HashMap::new(),
            inventory_index: HashMap::new(),
            inventory_count: HashMap::new(),
            skills: HashMap::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Zwraca wartosc statusu (np. HP, MP).
    /// Jesli status nie istnieje, zwraca 0.
    pub fn get_status(&self, status_type: u32) -> i32 {
        self.statuses.get(&status_type).copied().unwrap_or(0)
    }

    /// Ustawia wartosc statusu.
    pub fn set_status(&mut self, status_type: u32, value: i32) {
        self.statuses.insert(status_type, value);
    }

    /// Zwraca indeks przedmiotu na danym slocie w ekwipunku.
    /// Jesli slot jest pusty (nie istnieje), zwraca 0.
    pub fn get_item_index(&self, slot_index: u32) -> u32 {
        self.inventory_index.get(&slot_index).copied().unwrap_or(0)
    }

    /// Zwraca ilosc sztuk przedmiotu na danym slocie w ekwipunku.
    /// Jesli slot jest pusty (nie istnieje), zwraca 0.
    pub fn get_item_count(&self, slot_index: u32) -> u32 {
        self.inventory_count.get(&slot_index).copied().unwrap_or(0)
    }

    /// Ustawia informacje o przedmiocie na danym slocie w ekwipunku.
    pub fn set_item(&mut self, slot_index: u32, item_index: u32, count: u32) {
        self.inventory_index.insert(slot_index, item_index);
        self.inventory_count.insert(slot_index, count);
    }

    /// Zwraca poziom umiejetnosci.
    /// Jesli umiejetnosc nie istnieje, zwraca 0.
    pub fn get_skill_level(&self, skill_index: u32) -> i32 {
        self.skills.get(&skill_index).copied().unwrap_or(0)
    }

    /// Ustawia poziom umiejetnosci.
    pub fn set_skill_level(&mut self, skill_index: u32, level: i32) {
        self.skills.insert(skill_index, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_mod_player() {
        let obj = ModPlayer::new();
        assert!(obj.is_ready());
    }
}
