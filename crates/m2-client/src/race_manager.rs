//! Modul `race_manager` — Zarzadca ras grywalnych i potworow CRaceManager
//! Wzorzec referencyjny C++: `source/GameLib/RaceManager.cpp`
//! Opis: Odwzorowanie CRaceManager: rejestr mapujacy ID rasy (0..7 wojownik/ninja/sura/szaman oraz ID potworow i NPC) na obiekty CRaceData.

#[derive(Debug, Clone, Default)]
pub struct RaceManager {
    pub initialized: bool,
}

impl RaceManager {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_race_manager() {
        let instance = RaceManager::new();
        assert!(instance.is_ready());
    }
}
