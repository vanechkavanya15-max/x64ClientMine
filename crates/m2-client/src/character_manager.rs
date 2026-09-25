//! Modul `character_manager` — Centralny rejestr wszystkich postaci w swiecie CPythonCharacterManager
//! Wzorzec referencyjny C++: `source/UserInterface/PythonCharacterManager.cpp`
//! Opis: Odwzorowanie CPythonCharacterManager: CreateInstance, DeleteInstance, GetInstancePtr(vid), GetMainInstancePtr(), iteracja i aktualizacja wszystkich aktorow w klatce.

#[derive(Debug, Clone, Default)]
pub struct CharacterManager {
    pub initialized: bool,
}

impl CharacterManager {
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
    fn test_skeleton_character_manager() {
        let instance = CharacterManager::new();
        assert!(instance.is_ready());
    }
}
