//! Modul `actor_sync` — Synchronizacja odrzutu (knockback) i kolizji miedzy aktorami
//! Wzorzec referencyjny C++: `source/GameLib/ActorInstanceSync.cpp oraz source/GameLib/ActorInstancePosition.cpp`
//! Opis: Odwzorowanie fizyki odrzutu po ostatnim ciosie combo lub skillu, kolizja sferyczna zapobiegajaca przenikaniu przez potwory i budynki.

#[derive(Debug, Clone, Default)]
pub struct ActorSync {
    pub initialized: bool,
}

impl ActorSync {
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
    fn test_skeleton_actor_sync() {
        let instance = ActorSync::new();
        assert!(instance.is_ready());
    }
}
