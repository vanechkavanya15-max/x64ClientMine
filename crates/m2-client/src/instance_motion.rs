//! Modul `instance_motion` — Kolejka animacji i trybow broni aktora
//! Wzorzec referencyjny C++: `source/UserInterface/InstanceBaseMotion.cpp oraz source/GameLib/ActorInstanceMotion.cpp`
//! Opis: Odwzorowanie kolejki animacji: tryby broni (GENERAL, ONEHAND_SWORD, TWOHAND_SWORD, BOW, BELL, FAN, HORSE), stany WAIT, WALK, RUN, COMBO_ATTACK, DEAD.

#[derive(Debug, Clone, Default)]
pub struct InstanceMotion {
    pub initialized: bool,
}

impl InstanceMotion {
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
    fn test_skeleton_instance_motion() {
        let instance = InstanceMotion::new();
        assert!(instance.is_ready());
    }
}
