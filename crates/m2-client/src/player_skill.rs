//! Modul `player_skill` — Wykonywanie umiejetnosci bojowych i odliczanie cooldownow
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayerSkill.cpp`
//! Opis: Odwzorowanie UseSkill: sprawdzanie wymaganego SP, odliczanie czasu odnowienia (cooldown), obsluga skilli przelaczanych (Aura Miecza, Czarowane Ostrze, Silne Cialo).

#[derive(Debug, Clone, Default)]
pub struct PlayerSkill {
    pub initialized: bool,
}

impl PlayerSkill {
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
    fn test_skeleton_player_skill() {
        let instance = PlayerSkill::new();
        assert!(instance.is_ready());
    }
}
