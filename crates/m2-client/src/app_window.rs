//! Modul `app_window` — Natywne okno gry 64-bit i zarzadzanie rozdzielczoscia
//! Wzorzec referencyjny C++: `source/EterLib/MSWindow.cpp oraz source/UserInterface/PythonApplication.cpp`
//! Opis: Odwzorowanie klasy CMSWindow i inicjalizacji okna z CPythonApplication: tworzenie okna, rozdzielczosc, tryb pelnoekranowy/okienkowy, tytul okna, aktywnosc fokusu i wymiary obszaru roboczego.

#[derive(Debug, Clone, Default)]
pub struct AppWindow {
    pub initialized: bool,
}

impl AppWindow {
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
    fn test_skeleton_app_window() {
        let instance = AppWindow::new();
        assert!(instance.is_ready());
    }
}
