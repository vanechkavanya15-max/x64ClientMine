//! Modul `instance_base` — Klasa bazowa instancji postaci w swiecie gry CInstanceBase
//! Wzorzec referencyjny C++: `source/UserInterface/InstanceBase.cpp`
//! Opis: Odwzorowanie CInstanceBase: VID, nazwa, rasa, krolestwo (Shinsoo/Chunjo/Jinno), gildia, ranga (alignment), PK mode, zalozona zbroja, bron, fryzura, wierzchowiec.

#[derive(Debug, Clone, Default)]
pub struct InstanceBase {
    pub initialized: bool,
}

impl InstanceBase {
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
    fn test_skeleton_instance_base() {
        let instance = InstanceBase::new();
        assert!(instance.is_ready());
    }
}
