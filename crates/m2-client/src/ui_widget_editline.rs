// ui_widget_editline.rs - EditLine UI widget implementation for Metin2 Pure Rust

use crate::ui_font_renderer::get_global_font_atlas;

#[derive(Debug, Clone)]
pub struct EditLineWidget {
    pub text: String,
    pub cursor_pos: usize,
    pub max_len: usize,
    pub is_secret: bool,
    pub has_focus: bool,
    pub cursor_blink_timer: f32,
    pub selection_start: Option<usize>,
}

impl Default for EditLineWidget {
    fn default() -> Self {
        Self::new(32, false)
    }
}

impl EditLineWidget {
    pub fn new(max_len: usize, is_secret: bool) -> Self {
        Self {
            text: String::new(),
            cursor_pos: 0,
            max_len,
            is_secret,
            has_focus: false,
            cursor_blink_timer: 0.0,
            selection_start: None,
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.text.chars().count() < self.max_len {
            // Find byte index for cursor position
            let byte_idx = self.text.char_indices().nth(self.cursor_pos).map(|(i, _)| i).unwrap_or(self.text.len());
            self.text.insert(byte_idx, ch);
            self.cursor_pos += 1;
            self.cursor_blink_timer = 0.0;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            let byte_idx = self.text.char_indices().nth(self.cursor_pos).map(|(i, _)| i).unwrap();
            self.text.remove(byte_idx);
            self.cursor_blink_timer = 0.0;
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_pos < self.text.chars().count() {
            let byte_idx = self.text.char_indices().nth(self.cursor_pos).map(|(i, _)| i).unwrap();
            self.text.remove(byte_idx);
            self.cursor_blink_timer = 0.0;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.cursor_blink_timer = 0.0;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.text.chars().count() {
            self.cursor_pos += 1;
            self.cursor_blink_timer = 0.0;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.cursor_pos = 0;
        self.cursor_blink_timer = 0.0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_pos = self.text.chars().count();
        self.cursor_blink_timer = 0.0;
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_pos = 0;
        self.cursor_blink_timer = 0.0;
    }

    pub fn get_visible_text(&self) -> String {
        if self.is_secret {
            "*".repeat(self.text.chars().count())
        } else {
            self.text.clone()
        }
    }

    pub fn calculate_cursor_offset_x(&self) -> f32 {
        let visible_text = self.get_visible_text();
        let font = get_global_font_atlas();
        
        let text_to_cursor: String = visible_text.chars().take(self.cursor_pos).collect();
        font.measure_text_width(&text_to_cursor)
    }

    pub fn update_blink_timer(&mut self, delta_time: f32) {
        self.cursor_blink_timer += delta_time;
        if self.cursor_blink_timer >= 1.0 {
            self.cursor_blink_timer -= 1.0;
        }
    }
}
