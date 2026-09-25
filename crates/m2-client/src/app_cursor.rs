//! Modul `app_cursor` — Menedzer kursorow myszy i stanow interakcji
//! Wzorzec referencyjny C++: `source/UserInterface/PythonApplicationCursor.cpp`
//! Opis: Odwzorowanie kursorow gry: Normal, Attack, Target, Talk, CantGo, Pick, Door, Chair, Magic, Buy, Sell, CameraRotate oraz logiki ich przelaczania.

use std::collections::HashMap;

/// Tryb kursora (sprzetowy lub programowy)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorMode {
    Hardware,
    Software,
}

/// Ksztalt kursora (Normal, Attack, itp.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorShape {
    Normal,
    Attack,
    Target,
    Talk,
    CantGo,
    Pick,
    Door,
    Chair,
    Magic,
    Buy,
    Sell,
    CameraRotate,
    HSize,
    VSize,
    HVSize,
}

#[derive(Debug, Clone)]
pub struct AppCursor {
    pub initialized: bool,
    pub cursor_visible: bool,
    pub liar_cursor_on: bool,
    pub cursor_mode: CursorMode,
    pub cursor_num: CursorShape,
    pub continuous_cursor_num: CursorShape,
    pub cursor_handle_map: HashMap<CursorShape, isize>, // represents HANDLE mapping
}

impl Default for AppCursor {
    fn default() -> Self {
        Self::new()
    }
}

impl AppCursor {
    pub fn new() -> Self {
        Self {
            initialized: true,
            cursor_visible: true,
            liar_cursor_on: false,
            cursor_mode: CursorMode::Hardware,
            cursor_num: CursorShape::Normal,
            continuous_cursor_num: CursorShape::Normal,
            cursor_handle_map: HashMap::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Tworzy (inicjalizuje) kursory
    pub fn create_cursors(&mut self) -> bool {
        self.cursor_visible = true;
        self.liar_cursor_on = false;
        
        self.cursor_handle_map.clear();
        
        // Mocking the behavior of loading cursors.
        // In real win32 it would be LoadImage. We just assign some dummy numbers.
        let cursors = [
            CursorShape::Normal,
            CursorShape::Attack,
            CursorShape::Target,
            CursorShape::Talk,
            CursorShape::CantGo,
            CursorShape::Pick,
            CursorShape::Door,
            CursorShape::Chair,
            CursorShape::Magic,
            CursorShape::Buy,
            CursorShape::Sell,
            CursorShape::CameraRotate,
            CursorShape::HSize,
            CursorShape::VSize,
            CursorShape::HVSize,
        ];
        
        for (i, shape) in cursors.iter().enumerate() {
            self.cursor_handle_map.insert(*shape, (i + 1) as isize); // dummy handle
        }
        
        self.initialized = true;
        true
    }

    /// Niszczy kursory
    pub fn destroy_cursors(&mut self) {
        self.cursor_handle_map.clear();
        self.initialized = false;
    }

    /// Ustawia widocznosc kursora
    pub fn set_cursor_visible(&mut self, flag: bool, liar_cursor_on: bool) {
        self.cursor_visible = flag;
        self.liar_cursor_on = liar_cursor_on;
        
        // In hardware mode, it would loop ShowCursor, but here we just update state
        if self.cursor_mode == CursorMode::Hardware {
            // mock hardware show/hide cursor loop
        }
    }

    /// Zwraca informacje o tym, czy kursor jest widoczny
    pub fn get_cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    /// Zwraca status ukrytego kursora klamliwego
    pub fn get_liar_cursor_on(&self) -> bool {
        self.liar_cursor_on
    }

    /// Zwraca aktualny tryb kursora
    pub fn get_cursor_mode(&self) -> CursorMode {
        self.cursor_mode
    }

    /// Sprawdza czy dany ksztalt jest typem ciaglym (continuous change type)
    pub fn is_continuous_change_type_cursor(&self, shape: CursorShape) -> bool {
        matches!(
            shape,
            CursorShape::Normal
                | CursorShape::Attack
                | CursorShape::Target
                | CursorShape::Magic
                | CursorShape::Buy
                | CursorShape::Sell
        )
    }

    /// Ustawia numer/ksztalt kursora
    pub fn set_cursor_num(&mut self, mut shape: CursorShape) -> bool {
        if shape == CursorShape::Normal {
            if !self.is_continuous_change_type_cursor(self.cursor_num) {
                shape = self.continuous_cursor_num;
            }
        } else {
            if self.is_continuous_change_type_cursor(self.cursor_num) {
                self.continuous_cursor_num = self.cursor_num;
            }
        }

        if self.cursor_mode == CursorMode::Hardware {
            if !self.cursor_handle_map.contains_key(&shape) {
                return false;
            }
            // In C++ here it does SetCursor(hCursor)
        }

        self.cursor_num = shape;
        
        // In C++ it calls Python UI script method
        true
    }

    /// Ustawia tryb kursora
    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        match mode {
            CursorMode::Hardware => {
                self.cursor_mode = CursorMode::Hardware;
                // ShowCursor(true)
            }
            CursorMode::Software => {
                self.cursor_mode = CursorMode::Software;
                // SetCursor(NULL); ShowCursor(false)
            }
        }
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
