//! Modul `app_cursor` — Menedzer kursorow myszy i stanow interakcji
//! Wzorzec referencyjny C++: `source/UserInterface/PythonApplicationCursor.cpp`
//! Opis: Odwzorowanie kursorow gry: Normal, Attack, Target, Talk, CantGo, Pick, Door, Chair, Magic, Buy, Sell, CameraRotate oraz logiki ich przelaczania.

#[derive(Debug, Clone, Default)]
pub struct AppCursor {
    pub initialized: bool,
}

impl AppCursor {
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
    fn test_skeleton_app_cursor() {
        let instance = AppCursor::new();
        assert!(instance.is_ready());
    }
}
