pub trait ClipboardProvider {
    fn set_text(&mut self, text: &str);
    fn get_text(&mut self) -> Option<String>;
}

pub struct EditLineWidget {
    text: Vec<char>,
    cursor_pos: usize,
    selection_anchor: Option<usize>,
    max_length: usize,
    is_secret: bool,
    cursor_blink_timer: f32,
    cursor_visible: bool,
    blink_interval: f32,
}

impl EditLineWidget {
    pub fn new(max_length: usize) -> Self {
        Self {
            text: Vec::new(),
            cursor_pos: 0,
            selection_anchor: None,
            max_length,
            is_secret: false,
            cursor_blink_timer: 0.0,
            cursor_visible: true,
            blink_interval: 0.5,
        }
    }

    pub fn set_secret(&mut self, is_secret: bool) {
        self.is_secret = is_secret;
    }

    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = max_length;
        if self.text.len() > self.max_length {
            self.text.truncate(self.max_length);
            if self.cursor_pos > self.max_length {
                self.cursor_pos = self.max_length;
            }
            if let Some(anchor) = self.selection_anchor {
                if anchor > self.max_length {
                    self.selection_anchor = Some(self.max_length);
                }
            }
        }
    }

    pub fn get_text(&self) -> String {
        self.text.iter().collect()
    }

    pub fn get_display_text(&self) -> String {
        if self.is_secret {
            "*".repeat(self.text.len())
        } else {
            self.get_text()
        }
    }

    pub fn get_cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    pub fn is_cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn update(&mut self, dt: f32) {
        self.cursor_blink_timer += dt;
        while self.cursor_blink_timer >= self.blink_interval {
            self.cursor_blink_timer -= self.blink_interval;
            self.cursor_visible = !self.cursor_visible;
        }
    }

    fn reset_blink(&mut self) {
        self.cursor_blink_timer = 0.0;
        self.cursor_visible = true;
    }

    fn get_selection_range(&self) -> Option<(usize, usize)> {
        if let Some(anchor) = self.selection_anchor {
            if anchor != self.cursor_pos {
                let start = anchor.min(self.cursor_pos);
                let end = anchor.max(self.cursor_pos);
                return Some((start, end));
            }
        }
        None
    }

    fn delete_selection(&mut self) -> bool {
        if let Some((start, end)) = self.get_selection_range() {
            self.text.drain(start..end);
            self.cursor_pos = start;
            self.selection_anchor = None;
            self.reset_blink();
            return true;
        }
        false
    }

    pub fn insert_char(&mut self, c: char) {
        self.delete_selection();
        if self.text.len() < self.max_length {
            self.text.insert(self.cursor_pos, c);
            self.cursor_pos += 1;
        }
        self.reset_blink();
    }

    pub fn insert_text(&mut self, text: &str) {
        // Check if pasting a hyperlink (contains |H tag)
        if text.contains("|H") {
            self.cursor_pos = self.text.len();
        }

        self.delete_selection();
        for c in text.chars() {
            if self.text.len() < self.max_length {
                self.text.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
            } else {
                break;
            }
        }
        self.reset_blink();
    }

    pub fn delete_left(&mut self) {
        if self.delete_selection() {
            return;
        }
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.text.remove(self.cursor_pos);
            self.reset_blink();
        }
    }

    pub fn delete_right(&mut self) {
        if self.delete_selection() {
            return;
        }
        if self.cursor_pos < self.text.len() {
            self.text.remove(self.cursor_pos);
            self.reset_blink();
        }
    }

    fn handle_shift(&mut self, shift: bool) {
        if shift {
            if self.selection_anchor.is_none() {
                self.selection_anchor = Some(self.cursor_pos);
            }
        } else {
            self.selection_anchor = None;
        }
    }

    pub fn move_cursor_left(&mut self, shift: bool) {
        self.handle_shift(shift);
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.reset_blink();
        } else if !shift {
            self.selection_anchor = None;
        }
    }

    pub fn move_cursor_right(&mut self, shift: bool) {
        self.handle_shift(shift);
        if self.cursor_pos < self.text.len() {
            self.cursor_pos += 1;
            self.reset_blink();
        } else if !shift {
            self.selection_anchor = None;
        }
    }

    pub fn move_cursor_home(&mut self, shift: bool) {
        self.handle_shift(shift);
        self.cursor_pos = 0;
        self.reset_blink();
    }

    pub fn move_cursor_end(&mut self, shift: bool) {
        self.handle_shift(shift);
        self.cursor_pos = self.text.len();
        self.reset_blink();
    }

    pub fn select_all(&mut self) {
        self.selection_anchor = Some(0);
        self.cursor_pos = self.text.len();
        self.reset_blink();
    }

    pub fn copy(&self, clipboard: &mut dyn ClipboardProvider) {
        if self.is_secret {
            return;
        }
        if let Some((start, end)) = self.get_selection_range() {
            let selected_text: String = self.text[start..end].iter().collect();
            clipboard.set_text(&selected_text);
        }
    }

    pub fn cut(&mut self, clipboard: &mut dyn ClipboardProvider) {
        if self.is_secret {
            return;
        }
        if let Some((start, end)) = self.get_selection_range() {
            let selected_text: String = self.text[start..end].iter().collect();
            clipboard.set_text(&selected_text);
            self.delete_selection();
        }
    }

    pub fn paste(&mut self, clipboard: &mut dyn ClipboardProvider) {
        if let Some(text) = clipboard.get_text() {
            self.insert_text(&text);
        }
    }
}
