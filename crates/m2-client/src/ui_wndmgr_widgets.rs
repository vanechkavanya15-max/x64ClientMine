// ui_wndmgr_widgets.rs - Text Controls, EditLine and ScrollBar for wndMgr

// ==========================================
// 1. EDITLINE WIDGET
// ==========================================

#[derive(Debug, Clone)]
pub struct EditLine {
    pub text: String,
    pub cursor_pos: usize, // character index, not byte index
    pub max_len: usize,
    pub is_secret: bool,
    pub has_focus: bool,
    pub selection_start: Option<usize>, // character index
}

impl Default for EditLine {
    fn default() -> Self {
        Self::new(32, false)
    }
}

impl EditLine {
    pub fn new(max_len: usize, is_secret: bool) -> Self {
        Self {
            text: String::new(),
            cursor_pos: 0,
            max_len,
            is_secret,
            has_focus: false,
            selection_start: None,
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.has_selection() {
            self.delete_selection();
        }

        if self.text.chars().count() < self.max_len {
            let byte_idx = self.char_index_to_byte_index(self.cursor_pos);
            self.text.insert(byte_idx, ch);
            self.cursor_pos += 1;
        }
    }
    
    pub fn insert_string(&mut self, s: &str) {
        for ch in s.chars() {
            self.insert_char(ch);
        }
    }

    pub fn backspace(&mut self) {
        if self.has_selection() {
            self.delete_selection();
            return;
        }

        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            let byte_idx = self.char_index_to_byte_index(self.cursor_pos);
            self.text.remove(byte_idx);
        }
    }

    pub fn delete_char(&mut self) {
        if self.has_selection() {
            self.delete_selection();
            return;
        }

        if self.cursor_pos < self.text.chars().count() {
            let byte_idx = self.char_index_to_byte_index(self.cursor_pos);
            self.text.remove(byte_idx);
        }
    }

    pub fn move_cursor_left(&mut self, shift_pressed: bool) {
        if !shift_pressed {
            self.selection_start = None;
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_pos);
        }

        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn move_cursor_right(&mut self, shift_pressed: bool) {
        if !shift_pressed {
            self.selection_start = None;
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_pos);
        }

        if self.cursor_pos < self.text.chars().count() {
            self.cursor_pos += 1;
        }
    }

    pub fn move_cursor_home(&mut self, shift_pressed: bool) {
         if !shift_pressed {
            self.selection_start = None;
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_pos);
        }
        self.cursor_pos = 0;
    }

    pub fn move_cursor_end(&mut self, shift_pressed: bool) {
         if !shift_pressed {
            self.selection_start = None;
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_pos);
        }
        self.cursor_pos = self.text.chars().count();
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_pos = 0;
        self.selection_start = None;
    }

    pub fn select_all(&mut self) {
        let len = self.text.chars().count();
        if len > 0 {
            self.selection_start = Some(0);
            self.cursor_pos = len;
        }
    }

    pub fn get_visible_text(&self) -> String {
        if self.is_secret {
            "*".repeat(self.text.chars().count())
        } else {
            self.text.clone()
        }
    }

    pub fn has_selection(&self) -> bool {
        if let Some(start) = self.selection_start {
            start != self.cursor_pos
        } else {
            false
        }
    }

    pub fn delete_selection(&mut self) {
        if let Some(start) = self.selection_start {
            if start != self.cursor_pos {
                let min_pos = start.min(self.cursor_pos);
                let max_pos = start.max(self.cursor_pos);

                let start_byte = self.char_index_to_byte_index(min_pos);
                let end_byte = self.char_index_to_byte_index(max_pos);

                self.text.replace_range(start_byte..end_byte, "");
                self.cursor_pos = min_pos;
            }
        }
        self.selection_start = None;
    }

    fn char_index_to_byte_index(&self, char_idx: usize) -> usize {
        self.text.char_indices().nth(char_idx).map(|(i, _)| i).unwrap_or(self.text.len())
    }
}


// ==========================================
// 2. SCROLLBAR WIDGET
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollBarOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub struct ScrollBar {
    pub orientation: ScrollBarOrientation,
    pub pos: f32, // 0.0 to 1.0
    pub step: f32, // Amount to scroll per wheel click/arrow
    pub page_step: f32, // Amount to scroll per track click
    
    // Geometry
    pub track_length: f32,
    pub thumb_length: f32,
    
    // Drag state
    pub is_dragging: bool,
    pub drag_start_mouse: f32,
    pub drag_start_pos: f32,
}

impl Default for ScrollBar {
    fn default() -> Self {
        Self::new(ScrollBarOrientation::Vertical)
    }
}

impl ScrollBar {
    pub fn new(orientation: ScrollBarOrientation) -> Self {
        Self {
            orientation,
            pos: 0.0,
            step: 0.05,
            page_step: 0.2,
            track_length: 100.0,
            thumb_length: 20.0,
            is_dragging: false,
            drag_start_mouse: 0.0,
            drag_start_pos: 0.0,
        }
    }

    pub fn set_pos(&mut self, pos: f32) {
        self.pos = pos.clamp(0.0, 1.0);
    }

    pub fn get_pos(&self) -> f32 {
        self.pos
    }

    pub fn scroll_up(&mut self) {
        self.set_pos(self.pos - self.step);
    }

    pub fn scroll_down(&mut self) {
        self.set_pos(self.pos + self.step);
    }

    pub fn handle_mouse_wheel(&mut self, delta: i32) {
        if delta > 0 {
            self.scroll_up();
        } else if delta < 0 {
            self.scroll_down();
        }
    }

    pub fn handle_track_click(&mut self, mouse_coord: f32) {
        let movable_track = self.track_length - self.thumb_length;
        if movable_track <= 0.0 { return; }

        let thumb_pixel_pos = self.pos * movable_track;

        if mouse_coord < thumb_pixel_pos {
            // Clicked above/left of thumb
            self.set_pos(self.pos - self.page_step);
        } else if mouse_coord > thumb_pixel_pos + self.thumb_length {
            // Clicked below/right of thumb
            self.set_pos(self.pos + self.page_step);
        }
    }

    pub fn begin_drag(&mut self, mouse_coord: f32) {
        self.is_dragging = true;
        self.drag_start_mouse = mouse_coord;
        self.drag_start_pos = self.pos;
    }

    pub fn update_drag(&mut self, mouse_coord: f32) {
        if !self.is_dragging { return; }

        let movable_track = self.track_length - self.thumb_length;
        if movable_track > 0.0 {
            let pixel_delta = mouse_coord - self.drag_start_mouse;
            let pos_delta = pixel_delta / movable_track;
            self.set_pos(self.drag_start_pos + pos_delta);
        }
    }

    pub fn end_drag(&mut self) {
        self.is_dragging = false;
    }
}


// ==========================================
// 3. BUTTON WIDGET
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Over,
    Down,
    Disable,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub state: ButtonState,
    pub up_visual: String,
    pub over_visual: String,
    pub down_visual: String,
    pub disable_visual: String,
    pub click_sound: String,
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Button {
    pub fn new() -> Self {
        Self {
            state: ButtonState::Normal,
            up_visual: String::new(),
            over_visual: String::new(),
            down_visual: String::new(),
            disable_visual: String::new(),
            click_sound: "sound/ui/click.wav".to_string(),
        }
    }

    pub fn set_state(&mut self, new_state: ButtonState) {
        if self.state == ButtonState::Disable && new_state != ButtonState::Normal {
            return; // Can only transition out of disable to normal explicitly
        }
        self.state = new_state;
    }
    
    pub fn enable(&mut self) {
        self.state = ButtonState::Normal;
    }

    pub fn disable(&mut self) {
        self.state = ButtonState::Disable;
    }
    
    pub fn is_disabled(&self) -> bool {
        self.state == ButtonState::Disable
    }

    pub fn get_current_visual(&self) -> &str {
        match self.state {
            ButtonState::Normal => &self.up_visual,
            ButtonState::Over => &self.over_visual,
            ButtonState::Down => &self.down_visual,
            ButtonState::Disable => {
                if self.disable_visual.is_empty() {
                    &self.up_visual // fallback
                } else {
                    &self.disable_visual
                }
            }
        }
    }

    pub fn handle_mouse_enter(&mut self) {
        if self.state == ButtonState::Normal {
            self.state = ButtonState::Over;
        }
    }

    pub fn handle_mouse_leave(&mut self) {
        if self.state == ButtonState::Over || self.state == ButtonState::Down {
            self.state = ButtonState::Normal;
        }
    }

    pub fn handle_mouse_down(&mut self) {
        if self.state == ButtonState::Over || self.state == ButtonState::Normal {
            self.state = ButtonState::Down;
        }
    }

    pub fn handle_mouse_up(&mut self, is_over: bool) -> bool {
        if self.state == ButtonState::Down {
            if is_over {
                self.state = ButtonState::Over;
                // W prawdziwym kodzie tu by byl dzwiek:
                // crate::ui_sound_event_router::play_sound(&self.click_sound);
                return true; // Click registered
            } else {
                self.state = ButtonState::Normal;
            }
        }
        false
    }
}
