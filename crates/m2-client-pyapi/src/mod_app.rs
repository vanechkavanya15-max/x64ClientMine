//! Modul mod_app dla skrzynki m2-client-pyapi
//! Modul Pythona app: obsluga petli gry, czasu i rozdzielczosci
//! Metody app.GetGlobalTime, app.GetTime, app.GetCursorPosition, app.Exit

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModApp {
    pub initialized: bool,
}

impl ModApp {
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
    fn test_skeleton_mod_app() {
        let obj = ModApp::new();
        assert!(obj.is_ready());
    }
}
