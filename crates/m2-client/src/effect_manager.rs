//! Modul `effect_manager` — Menedzer efektow czasteczkowych i blyskow CEffectManager
//! Wzorzec referencyjny C++: `source/EffectLib/EffectManager.cpp oraz source/EffectLib/EffectInstance.cpp`
//! Opis: Odwzorowanie CEffectManager: rejestr aktywnych efektow w swiecie 3D (uderzenia miecza, rozblyski skilli, dym, swiatla), czas zycia czasteczek i aktualizacja klatek.

#[derive(Debug, Clone, Default)]
pub struct EffectManager {
    pub initialized: bool,
}

impl EffectManager {
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
    fn test_skeleton_effect_manager() {
        let instance = EffectManager::new();
        assert!(instance.is_ready());
    }
}
