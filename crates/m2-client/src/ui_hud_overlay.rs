//! Modul `ui_hud_overlay` — Glowny interfejs HUD w trakcie gry (Kule HP/SP, EXP, Quicksloty, Pasek Celu)
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayer.cpp`
//! Opis: Zbudowanie pelnego HUD gry: dolny panel z czerwonym paskiem HP, niebieskim SP, zoltym Stamina, 4 kulami EXP (0..100%), 8 polami Quickslot (1-4, F1-F4) oraz gornym paskiem HP wybranego celu/potwora.

#[derive(Debug, Clone, Default)]
pub struct UiHudOverlay {
    pub initialized: bool,
}

impl UiHudOverlay {
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
    fn test_skeleton_ui_hud_overlay() {
        let instance = UiHudOverlay::new();
        assert!(instance.is_ready());
    }
}
