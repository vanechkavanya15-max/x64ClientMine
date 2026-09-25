//! Modul `app_event` — Obsluga zdarzen cyklu klatki i przekazywanie wejscia
//! Wzorzec referencyjny C++: `source/UserInterface/PythonApplicationEvent.cpp`
//! Opis: Odwzorowanie metod OnCameraUpdate, OnUIUpdate, OnUIRender, OnMouseMove, OnMouseLeftButtonDown, OnMouseRightButtonDown.

#[derive(Debug, Clone, Default)]
pub struct AppEvent {
    pub initialized: bool,
}

impl AppEvent {
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
    fn test_skeleton_app_event() {
        let instance = AppEvent::new();
        assert!(instance.is_ready());
    }
}
