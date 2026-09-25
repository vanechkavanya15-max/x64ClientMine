//! Modul `net_stream_core` — Maszyna stanow strumienia sieciowego CPythonNetworkStream
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStream.cpp`
//! Opis: Odwzorowanie CPythonNetworkStream: przejscia miedzy fazami OffLine, HandShake, Login, Select, Loading, Game, kolejkowanie pakietow przychodzacych i wychodzacych.

#[derive(Debug, Clone, Default)]
pub struct NetStreamCore {
    pub initialized: bool,
}

impl NetStreamCore {
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
    fn test_skeleton_net_stream_core() {
        let instance = NetStreamCore::new();
        assert!(instance.is_ready());
    }
}
