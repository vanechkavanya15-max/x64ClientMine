//! Modul `phase_game_actor` — Obsluga pakietow pojawiania sie, ruchu i aktualizacji postaci
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseGameActor.cpp`
//! Opis: Odwzorowanie RecvCharacterAppendPacket, RecvCharacterAdditionalInfo, RecvCharacterUpdatePacket, RecvCharacterDeletePacket, RecvCharacterMovePacket.

#[derive(Debug, Clone, Default)]
pub struct PhaseGameActor {
    pub initialized: bool,
}

impl PhaseGameActor {
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
    fn test_skeleton_phase_game_actor() {
        let instance = PhaseGameActor::new();
        assert!(instance.is_ready());
    }
}
