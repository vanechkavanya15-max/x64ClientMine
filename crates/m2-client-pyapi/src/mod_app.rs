//! Modul mod_app dla skrzynki m2-client-pyapi
//! Modul Pythona app: obsluga petli gry, czasu i rozdzielczosci
//! Metody app.GetGlobalTime, app.GetTime, app.GetCursorPosition, app.Exit

use std::time::Instant;

/// Glowna struktura implementacyjna
#[derive(Debug, Clone)]
pub struct ModApp {
    pub initialized: bool,
    start_time: Instant,
    exit_requested: bool,
    cursor_position: (i32, i32),
}

impl Default for ModApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ModApp {
    pub fn new() -> Self {
        Self {
            initialized: true,
            start_time: Instant::now(),
            exit_requested: false,
            cursor_position: (0, 0),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Zwraca czas w milisekundach od uruchomienia modulu
    pub fn get_global_time(&self) -> u32 {
        self.start_time.elapsed().as_millis() as u32
    }

    /// Zwraca czas w sekundach (z ulamkami) od uruchomienia modulu
    pub fn get_time(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    /// Zwraca aktualna pozycje kursora (x, y)
    pub fn get_cursor_position(&self) -> (i32, i32) {
        self.cursor_position
    }

    /// Ustawia aktualna pozycje kursora (x, y)
    pub fn set_cursor_position(&mut self, x: i32, y: i32) {
        self.cursor_position = (x, y);
    }

    /// Zamyka aplikacje poprzez flage
    pub fn exit(&mut self) {
        self.exit_requested = true;
    }

    /// Sprawdza, czy zazadano zamkniecia aplikacji
    pub fn is_exit_requested(&self) -> bool {
        self.exit_requested
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
