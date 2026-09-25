//! Modul `player_core` — Kompletny stan glownego gracza CPythonPlayer
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayer.cpp`
//! Opis: Odwzorowanie CPythonPlayer: tablica statystyk Point[255], 180 slotow ekwipunku, zalozone przedmioty, zloto (Yang), aktualny cel (target VID), stan zycia/smierci.

#[derive(Debug, Clone, Default)]
pub struct PlayerCore {
    pub initialized: bool,
}

impl PlayerCore {
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
    fn test_skeleton_player_core() {
        let instance = PlayerCore::new();
        assert!(instance.is_ready());
    }
}
