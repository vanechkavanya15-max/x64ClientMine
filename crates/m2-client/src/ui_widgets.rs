//! Modul `ui_widgets` — Kontrolki graficzne UI: CButton, CEditLine, CTextLine, CImageBox, CBar
//! Wzorzec referencyjny C++: `source/EterPythonLib/PythonWindow.cpp`
//! Opis: Odwzorowanie kontrolek: przycisk (stany Up, Over, Down, Disable), pole edycji tekstu, linia tekstu z kolorem, obrazek, pasek postepu (HP/SP/EXP).

#[derive(Debug, Clone, Default)]
pub struct UiWidgets {
    pub initialized: bool,
}

impl UiWidgets {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

// Baza dla kontrolek
#[derive(Debug, Clone)]
pub struct WindowBase {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub show: bool,
    pub name: String,
}

impl Default for WindowBase {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            show: false,
            name: String::new(),
        }
    }
}

// CBar - pasek uzywany do HP/SP/EXP
#[derive(Debug, Clone)]
pub struct CBar {
    pub base: WindowBase,
    pub color: u32,
}

impl CBar {
    pub fn new() -> Self {
        Self {
            base: WindowBase::default(),
            color: 0xff000000,
        }
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }
}

// CTextLine - linia tekstu
#[derive(Debug, Clone)]
pub struct CTextLine {
    pub base: WindowBase,
    pub max_len: i32,
    pub horizontal_align: i32,
    pub vertical_align: i32,
    pub secret: bool,
    pub outline: bool,
    pub feather: bool,
    pub multi_line: bool,
    pub font_name: String,
    pub font_color: u32,
    pub limit_width: f32,
    pub text: String,
}

impl CTextLine {
    pub fn new() -> Self {
        Self {
            base: WindowBase::default(),
            max_len: 0,
            horizontal_align: 0,
            vertical_align: 0,
            secret: false,
            outline: false,
            feather: false,
            multi_line: false,
            font_name: String::new(),
            font_color: 0xffcccccc, // Okolo 0.78f dla R, G, B
            limit_width: 0.0,
            text: String::new(),
        }
    }

    pub fn set_max(&mut self, max: i32) {
        self.max_len = max;
    }

    pub fn set_horizontal_align(&mut self, align: i32) {
        self.horizontal_align = align;
    }

    pub fn set_vertical_align(&mut self, align: i32) {
        self.vertical_align = align;
    }

    pub fn set_secret(&mut self, secret: bool) {
        self.secret = secret;
    }

    pub fn set_outline(&mut self, outline: bool) {
        self.outline = outline;
    }

    pub fn set_feather(&mut self, feather: bool) {
        self.feather = feather;
    }

    pub fn set_multi_line(&mut self, multi_line: bool) {
        self.multi_line = multi_line;
    }

    pub fn set_font_name(&mut self, name: &str) {
        self.font_name = format!("{}.fnt", name);
    }

    pub fn set_font_color(&mut self, color: u32) {
        self.font_color = color;
    }

    pub fn set_limit_width(&mut self, width: f32) {
        self.limit_width = width;
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}

// CEditLine - pole edycji tekstu
#[derive(Debug, Clone)]
pub struct CEditLine {
    pub text_line: CTextLine,
    pub cursor_pos: usize,
    pub is_focused: bool,
}

impl CEditLine {
    pub fn new() -> Self {
        Self {
            text_line: CTextLine::new(),
            cursor_pos: 0,
            is_focused: false,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text_line.set_text(text);
        self.cursor_pos = self.text_line.text.chars().count();
    }

    pub fn get_text(&self) -> &str {
        self.text_line.get_text()
    }

    pub fn set_max(&mut self, max: i32) {
        self.text_line.set_max(max);
    }
    
    pub fn set_secret(&mut self, secret: bool) {
        self.text_line.set_secret(secret);
    }

    // Wprowadzanie znakow, uwzglednia limit znakow
    pub fn type_char(&mut self, ch: char) {
        if !self.is_focused {
            return;
        }
        let max = self.text_line.max_len as usize;
        let char_count = self.text_line.text.chars().count();
        if max > 0 && char_count >= max {
            return; // Osiagnieto limit znakow
        }
        
        // Znajdz indeks w bajtach na podstawie indeksu w znakach
        let byte_pos = self.text_line.text.char_indices().nth(self.cursor_pos).map(|(pos, _)| pos).unwrap_or(self.text_line.text.len());
        self.text_line.text.insert(byte_pos, ch);
        self.cursor_pos += 1;
    }

    // Kasowanie znakow
    pub fn backspace(&mut self) {
        if !self.is_focused || self.cursor_pos == 0 {
            return;
        }
        self.cursor_pos -= 1;
        
        let byte_pos = self.text_line.text.char_indices().nth(self.cursor_pos).map(|(pos, _)| pos).unwrap_or(self.text_line.text.len());
        self.text_line.text.remove(byte_pos);
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.is_focused = focus;
    }
}

// CImageBox - kontrolka wyswietlajaca obraz
#[derive(Debug, Clone)]
pub struct CImageBox {
    pub base: WindowBase,
    pub image_path: String,
    pub diffuse_color: (f32, f32, f32, f32),
    pub loaded: bool,
    pub width: i32,
    pub height: i32,
}

impl CImageBox {
    pub fn new() -> Self {
        Self {
            base: WindowBase::default(),
            image_path: String::new(),
            diffuse_color: (1.0, 1.0, 1.0, 1.0), // Domyslnie bialy, pelna nieprzezroczystosc
            loaded: false,
            width: 0,
            height: 0,
        }
    }

    // Ladowanie obrazu
    pub fn load_image(&mut self, filename: &str) -> bool {
        if filename.is_empty() {
            return false;
        }
        self.image_path = filename.to_string();
        self.loaded = true;
        
        // Mock wymiarow z zaladowanego obrazu (zastepuje logike C++)
        self.width = 100;
        self.height = 100;
        self.base.width = 100;
        self.base.height = 100;
        
        true
    }

    pub fn set_diffuse_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.diffuse_color = (r, g, b, a);
    }

    pub fn get_width(&self) -> i32 {
        if !self.loaded { 0 } else { self.width }
    }

    pub fn get_height(&self) -> i32 {
        if !self.loaded { 0 } else { self.height }
    }
}

// CButton - przycisk ze stanami
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonState {
    Up,
    Over,
    Down,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct CButton {
    pub base: WindowBase,
    pub state: ButtonState,
    pub is_enabled: bool,
    pub is_pressed: bool,
    pub is_flash: bool,
    pub up_visual: String,
    pub over_visual: String,
    pub down_visual: String,
    pub disable_visual: String,
}

impl CButton {
    pub fn new() -> Self {
        Self {
            base: WindowBase::default(),
            state: ButtonState::Up,
            is_enabled: true,
            is_pressed: false,
            is_flash: false,
            up_visual: String::new(),
            over_visual: String::new(),
            down_visual: String::new(),
            disable_visual: String::new(),
        }
    }

    pub fn set_up_visual(&mut self, filename: &str) -> bool {
        self.up_visual = filename.to_string();
        true
    }

    pub fn set_over_visual(&mut self, filename: &str) -> bool {
        self.over_visual = filename.to_string();
        true
    }

    pub fn set_down_visual(&mut self, filename: &str) -> bool {
        self.down_visual = filename.to_string();
        true
    }

    pub fn set_disable_visual(&mut self, filename: &str) -> bool {
        self.disable_visual = filename.to_string();
        true
    }

    pub fn get_up_visual_file_name(&self) -> &str {
        &self.up_visual
    }

    pub fn get_over_visual_file_name(&self) -> &str {
        &self.over_visual
    }

    pub fn get_down_visual_file_name(&self) -> &str {
        &self.down_visual
    }

    pub fn flash(&mut self) {
        self.is_flash = true;
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
        self.set_up();
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
        self.state = ButtonState::Disabled;
    }

    pub fn is_disable(&self) -> bool {
        !self.is_enabled
    }

    pub fn is_enable(&self) -> bool {
        self.is_enabled
    }

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    pub fn set_up(&mut self) {
        self.up();
    }

    pub fn up(&mut self) {
        if self.is_enabled {
            self.state = ButtonState::Up;
        }
    }

    pub fn over(&mut self) {
        if self.is_enabled {
            self.state = ButtonState::Over;
        }
    }

    pub fn down(&mut self) {
        if self.is_enabled {
            self.state = ButtonState::Down;
        }
    }

    pub fn on_mouse_left_button_down(&mut self) -> bool {
        if !self.is_enabled {
            return false;
        }
        self.is_pressed = true;
        self.down();
        true
    }

    pub fn on_mouse_left_button_double_click(&mut self) -> bool {
        self.on_mouse_left_button_down()
    }

    pub fn on_mouse_left_button_up(&mut self) -> bool {
        if !self.is_enabled {
            return false;
        }
        if !self.is_pressed {
            return false;
        }
        self.is_pressed = false;
        self.over();
        true
    }

    pub fn on_mouse_over_in(&mut self) {
        if !self.is_enabled {
            return;
        }
        self.over();
    }

    pub fn on_mouse_over_out(&mut self) {
        if !self.is_enabled {
            return;
        }
        self.up();
        self.is_pressed = false;
    }
}
