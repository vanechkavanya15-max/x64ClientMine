//! Modul `app_window` — Natywne okno gry 64-bit i zarzadzanie rozdzielczoscia
//! Wzorzec referencyjny C++: `source/EterLib/MSWindow.cpp oraz source/UserInterface/PythonApplication.cpp`
//! Opis: Odwzorowanie klasy CMSWindow i inicjalizacji okna z CPythonApplication: tworzenie okna, rozdzielczosc, tryb pelnoekranowy/okienkowy, tytul okna, aktywnosc fokusu i wymiary obszaru roboczego.

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Default)]
pub struct AppWindow {
    pub initialized: bool,
    pub is_visible: bool,
    pub is_active: bool,
    pub is_windowed: bool,
    pub is_window_fullscreen_enable: bool,
    
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    
    pub title: String,
    
    // Symulowane style Windows
    pub window_mode: u32,
}

pub const WS_OVERLAPPEDWINDOW: u32 = 0x00CF0000;
pub const WS_POPUP: u32 = 0x80000000;
pub const SM_CXSCREEN: i32 = 1920; // Rozdzielczosc pozioma dla testow
pub const SM_CYSCREEN: i32 = 1080; // Rozdzielczosc pionowa dla testow

impl AppWindow {
    pub fn new() -> Self {
        Self { 
            initialized: true,
            is_visible: false,
            is_active: false,
            is_windowed: false,
            is_window_fullscreen_enable: false,
            width: 800,
            height: 600,
            x: 0,
            y: 0,
            title: String::new(),
            window_mode: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    pub fn create(&mut self, name: &str, width: i32, height: i32, windowed: bool) -> bool {
        self.title = name.to_string();
        self.width = width;
        self.height = height;
        
        self.window_mode = if windowed {
            WS_OVERLAPPEDWINDOW
        } else {
            WS_POPUP
        };
        
        self.is_windowed = windowed;
        
        // Odwzorowanie logiki inicjalizacji z PythonApplication::Create
        let use_default_ime = true; 
        let is_europe = false; 

        if !windowed && (use_default_ime || is_europe) {
            self.is_windowed = false;
            self.is_window_fullscreen_enable = true;
            // Metoda ustawiajaca pelny ekran okna gry
        } else {
            self.adjust_size(width, height);
            
            if windowed {
                self.is_windowed = true;
                
                // SetPosition(-8, 0); in PythonApplication.cpp
                self.set_position(-8, 0); 
            } else {
                self.is_windowed = false;
                self.set_position(0, 0);
            }
        }
        
        // Okno utworzone pomyslnie
        self.initialized = true;
        true
    }
    
    pub fn destroy(&mut self) {
        self.is_visible = false;
        self.is_active = false;
        self.initialized = false;
    }
    
    pub fn show(&mut self) {
        self.is_visible = true;
        self.is_active = true;
    }
    
    pub fn hide(&mut self) {
        self.is_visible = false;
        self.is_active = false;
    }
    
    pub fn set_visible_mode(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }
    
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
    
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
    
    pub fn get_screen_width(&self) -> i32 {
        SM_CXSCREEN
    }
    
    pub fn get_screen_height(&self) -> i32 {
        SM_CYSCREEN
    }
    
    pub fn get_window_rect(&self) -> Rect {
        Rect {
            left: self.x,
            top: self.y,
            right: self.x + self.width,
            bottom: self.y + self.height,
        }
    }
    
    pub fn get_client_rect(&self) -> Rect {
        Rect {
            left: 0,
            top: 0,
            right: self.width,
            bottom: self.height,
        }
    }
    
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    
    pub fn set_center_position(&mut self) {
        let rc = self.get_client_rect();
        let window_width = rc.right - rc.left;
        let window_height = rc.bottom - rc.top;
        
        self.set_position(
            (self.get_screen_width() - window_width) / 2,
            (self.get_screen_height() - window_height) / 2,
        );
    }
    
    pub fn adjust_size(&mut self, width: i32, height: i32) {
        // Logika z AdjustWindowRectEx 
        // Okno bez obramowania nie zmienia rozmiaru wzgledem klienta
        if self.window_mode == WS_OVERLAPPEDWINDOW {
            // Symulacja dekoracji okna w systemie Windows
            self.width = width + 16;
            self.height = height + 39;
        } else {
            self.width = width;
            self.height = height;
        }
    }
    
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }
    
    pub fn set_text(&mut self, text: &str) {
        self.title = text.to_string();
    }
    
    pub fn on_size(&mut self, minimized: bool) {
        if minimized {
            self.is_active = false;
            self.is_visible = false;
        } else {
            self.is_active = true;
            self.is_visible = true;
        }
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
