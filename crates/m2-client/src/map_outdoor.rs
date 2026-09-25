//! Modul `map_outdoor` — Zarzadca otwartej mapy swiata CMapOutdoor
//! Wzorzec referencyjny C++: `source/GameLib/MapOutdoor.cpp oraz source/GameLib/MapOutdoorUpdate.cpp`
//! Opis: Odwzorowanie CMapOutdoor: zarzadzanie aktywnymi terenami wokol gracza, obliczanie wysokosci terenu GetHeight(x, y), konfiguracja nieba i mgly mapy.

#[derive(Debug, Clone, Default)]
pub struct MapOutdoor {
    pub initialized: bool,
}

impl MapOutdoor {
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
    fn test_skeleton_map_outdoor() {
        let instance = MapOutdoor::new();
        assert!(instance.is_ready());
    }
}
