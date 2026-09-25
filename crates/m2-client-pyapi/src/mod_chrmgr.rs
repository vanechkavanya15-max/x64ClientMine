//! Modul mod_chrmgr dla skrzynki m2-client-pyapi
//! Modul Pythona chrmgr: menedzer postaci i mobow
//! Metody chrmgr.GetVIDName, chrmgr.GetRace, chrmgr.SelectCharacter

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModChrmgr {
    pub initialized: bool,
}

impl ModChrmgr {
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
    fn test_skeleton_mod_chrmgr() {
        let obj = ModChrmgr::new();
        assert!(obj.is_ready());
    }
}
