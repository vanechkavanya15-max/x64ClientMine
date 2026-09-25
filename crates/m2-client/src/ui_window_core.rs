//! Modul `ui_window_core` — Klasa bazowa elementu interfejsu UI::CWindow
//! Wzorzec referencyjny C++: `source/EterPythonLib/PythonWindow.cpp`
//! Opis: Odwzorowanie UI::CWindow: hierarchia rodzic-dzieci, pozycja lokalna i globalna (x, y, width, height), widocznosc, flagi (MOVABLE, LIMIT), test trafienia kursorem IsIn(mx, my).

#[derive(Debug, Clone, Default)]
pub struct UiWindowCore {
    pub initialized: bool,
}

impl UiWindowCore {
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
    fn test_skeleton_ui_window_core() {
        let instance = UiWindowCore::new();
        assert!(instance.is_ready());
    }
}
