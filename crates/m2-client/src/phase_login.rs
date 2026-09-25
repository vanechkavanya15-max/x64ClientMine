//! Modul `phase_login` — Obsluga fazy logowania do serwera gry LoginPhase
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseLogin.cpp`
//! Opis: Odwzorowanie LoginPhase: wysylanie HEADER_CG_LOGIN2/LOGIN3 z kluczem autoryzacji, obsluga HEADER_GC_LOGIN_SUCCESS i kodow bledow HEADER_GC_LOGIN_FAILURE (WRONGPWD, NOID, ALREADY, BLOCK).

#[derive(Debug, Clone, Default)]
pub struct PhaseLogin {
    pub initialized: bool,
}

impl PhaseLogin {
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
    fn test_skeleton_phase_login() {
        let instance = PhaseLogin::new();
        assert!(instance.is_ready());
    }
}
