//! Modul `phase_game_command` — Parser polecen serwerowych ServerCommand
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamCommand.cpp`
//! Opis: Odwzorowanie RecvCommandPacket: parsowanie komend tekstowych od serwera (day, night, mall, gift, RefineSuceeded, RefineFailed, BINARY_Update).

#[derive(Debug, Clone, Default)]
pub struct PhaseGameCommand {
    pub initialized: bool,
}

impl PhaseGameCommand {
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
    fn test_skeleton_phase_game_command() {
        let instance = PhaseGameCommand::new();
        assert!(instance.is_ready());
    }
}
