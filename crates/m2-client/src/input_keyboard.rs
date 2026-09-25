//! Modul `input_keyboard` — Sterowanie postacia z klawiatury (WASD, skille, quicksloty)
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayerInputKeyboard.cpp`
//! Opis: Odwzorowanie sterowania klawiatura: ruch kierunkowy WASD/Strzalki, klawisz Spacji (atak ciagly), skroty 1-4 i F1-F4 (uzycie slotow podrecznych), podnoszenie przedmiotow klawiszem Z/`.

#[derive(Debug, Clone, Default)]
pub struct InputKeyboard {
    pub initialized: bool,
}

impl InputKeyboard {
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
    fn test_skeleton_input_keyboard() {
        let instance = InputKeyboard::new();
        assert!(instance.is_ready());
    }
}
