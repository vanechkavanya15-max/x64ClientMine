//! Modul `app_loop` — Glowna petla gry Process/Update/Render i licznik FPS
//! Wzorzec referencyjny C++: `source/UserInterface/PythonApplication.cpp`
//! Opis: Odwzorowanie metody CPythonApplication::Process(): synchronizacja klatek, obliczanie delta_time, licznik FPS, harmonogram wywolan UpdateGame() i RenderGame().

#[derive(Debug, Clone, Default)]
pub struct AppLoop {
    pub initialized: bool,
}

impl AppLoop {
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
    fn test_skeleton_app_loop() {
        let instance = AppLoop::new();
        assert!(instance.is_ready());
    }
}
