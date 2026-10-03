// ui_widget_editline_ime.rs - Full IME EditLine widget implementation for Metin2 Pure Rust
// Corresponds 1:1 with CPythonIME and CIME in the original C++ Client.

#[derive(Debug, Clone)]
pub struct ImeEditLineWidget {
    pub text: String,
    pub cursor_pos: usize,
    pub max_len: usize,

    pub is_secret: bool,
    pub number_only: bool,
    pub has_focus: bool,

    pub cursor_blink_timer: f32,

    // Selection state
    pub selection_start: usize,
    pub selection_end: usize,

    // Undo/Redo stack
    pub undo_stack: Vec<SUndoState>,
    pub redo_stack: Vec<SUndoState>,

    // Win32 Mock Clipboard for tests
    clipboard: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SUndoState {
    pub text: String,
    pub cursor_pos: usize,
    pub selection_start: usize,
    pub selection_end: usize,
}

pub const MAX_UNDO: usize = 64;

impl Default for ImeEditLineWidget {
    fn default() -> Self {
        Self::new(32, false)
    }
}

impl ImeEditLineWidget {
    pub fn new(max_len: usize, is_secret: bool) -> Self {
        Self {
            text: String::new(),
            cursor_pos: 0,
            max_len,
            is_secret,
            number_only: false,
            has_focus: false,
            cursor_blink_timer: 0.0,
            selection_start: 0,
            selection_end: 0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            clipboard: None,
        }
    }

    pub fn set_number_mode(&mut self, number_only: bool) {
        self.number_only = number_only;
    }

    pub fn get_visible_text(&self) -> String {
        if self.is_secret {
            "*".repeat(self.text.chars().count())
        } else {
            self.text.clone()
        }
    }

    fn reset_blink(&mut self) {
        self.cursor_blink_timer = 0.0;
    }

    pub fn update_blink_timer(&mut self, delta_time: f32) {
        self.cursor_blink_timer += delta_time;
        if self.cursor_blink_timer >= 1.0 {
            // 500ms on, 500ms off essentially, or wrap at 1s
            self.cursor_blink_timer -= 1.0;
        }
    }

    // --- Cursor Navigation ---

    pub fn move_cursor_left(&mut self, shift_pressed: bool) {
        if !shift_pressed {
            self.clear_selection();
        } else if !self.has_selection() {
            self.selection_start = self.cursor_pos;
            self.selection_end = self.cursor_pos;
        }

        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }

        if shift_pressed {
            self.selection_end = self.cursor_pos;
        }
        self.reset_blink();
    }

    pub fn move_cursor_right(&mut self, shift_pressed: bool) {
        if !shift_pressed {
            self.clear_selection();
        } else if !self.has_selection() {
            self.selection_start = self.cursor_pos;
            self.selection_end = self.cursor_pos;
        }

        let char_count = self.text.chars().count();
        if self.cursor_pos < char_count {
            self.cursor_pos += 1;
        }

        if shift_pressed {
            self.selection_end = self.cursor_pos;
        }
        self.reset_blink();
    }

    pub fn move_cursor_home(&mut self, shift_pressed: bool) {
        if !shift_pressed {
            self.clear_selection();
        } else if !self.has_selection() {
            self.selection_start = self.cursor_pos;
            self.selection_end = self.cursor_pos;
        }
        self.cursor_pos = 0;
        if shift_pressed {
            self.selection_end = self.cursor_pos;
        }
        self.reset_blink();
    }

    pub fn move_cursor_end(&mut self, shift_pressed: bool) {
        if !shift_pressed {
            self.clear_selection();
        } else if !self.has_selection() {
            self.selection_start = self.cursor_pos;
            self.selection_end = self.cursor_pos;
        }
        self.cursor_pos = self.text.chars().count();
        if shift_pressed {
            self.selection_end = self.cursor_pos;
        }
        self.reset_blink();
    }

    pub fn set_cursor_position(&mut self, position: usize) {
        self.cursor_pos = position.min(self.text.chars().count());
        self.reset_blink();
    }

    // --- Undo / Redo ---
    pub fn push_undo_state(&mut self) {
        let state = SUndoState {
            text: self.text.clone(),
            cursor_pos: self.cursor_pos,
            selection_start: self.selection_start,
            selection_end: self.selection_end,
        };
        self.undo_stack.push(state);
        if self.undo_stack.len() > MAX_UNDO {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(st) = self.undo_stack.pop() {
            let cur = SUndoState {
                text: self.text.clone(),
                cursor_pos: self.cursor_pos,
                selection_start: self.selection_start,
                selection_end: self.selection_end,
            };
            self.redo_stack.push(cur);
            if self.redo_stack.len() > MAX_UNDO {
                self.redo_stack.remove(0);
            }

            self.text = st.text;
            self.cursor_pos = st.cursor_pos;
            self.selection_start = st.selection_start;
            self.selection_end = st.selection_end;
            
            // Note: C++ OnUpdate triggered here in ms_pEvent, which we abstract away or let the framework handle.
            self.reset_blink();
        }
    }

    pub fn redo(&mut self) {
        if let Some(st) = self.redo_stack.pop() {
            let cur = SUndoState {
                text: self.text.clone(),
                cursor_pos: self.cursor_pos,
                selection_start: self.selection_start,
                selection_end: self.selection_end,
            };
            self.undo_stack.push(cur);
            if self.undo_stack.len() > MAX_UNDO {
                self.undo_stack.remove(0);
            }

            self.text = st.text;
            self.cursor_pos = st.cursor_pos;
            self.selection_start = st.selection_start;
            self.selection_end = st.selection_end;

            self.reset_blink();
        }
    }

    // --- Typing and Deletion ---

    pub fn insert_char(&mut self, ch: char) {
        if self.number_only && !ch.is_ascii_digit() {
            return;
        }

        self.push_undo_state();

        if self.has_selection() {
            self.delete_selection_internal();
        }

        if self.text.chars().count() < self.max_len {
            let byte_idx = self.char_idx_to_byte_idx(self.cursor_pos);
            self.text.insert(byte_idx, ch);
            self.cursor_pos += 1;
            self.reset_blink();
        }
    }

    pub fn insert_string(&mut self, s: &str) {
        self.push_undo_state();
        for ch in s.chars() {
            if self.number_only && !ch.is_ascii_digit() {
                continue;
            }
            if self.has_selection() {
                self.delete_selection_internal();
            }
            if self.text.chars().count() < self.max_len {
                let byte_idx = self.char_idx_to_byte_idx(self.cursor_pos);
                self.text.insert(byte_idx, ch);
                self.cursor_pos += 1;
            }
        }
        self.reset_blink();
    }

    pub fn backspace(&mut self) {
        if self.has_selection() {
            self.push_undo_state();
            self.delete_selection_internal();
            return;
        }

        if self.cursor_pos > 0 {
            self.push_undo_state();
            self.cursor_pos -= 1;
            let byte_idx = self.char_idx_to_byte_idx(self.cursor_pos);
            self.text.remove(byte_idx);
            self.reset_blink();
        }
    }

    pub fn delete_char(&mut self) {
        if self.has_selection() {
            self.push_undo_state();
            self.delete_selection_internal();
            return;
        }

        if self.cursor_pos < self.text.chars().count() {
            self.push_undo_state();
            let byte_idx = self.char_idx_to_byte_idx(self.cursor_pos);
            self.text.remove(byte_idx);
            self.reset_blink();
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_pos = 0;
        self.clear_selection();
        self.reset_blink();
    }

    // --- Selection ---

    pub fn has_selection(&self) -> bool {
        self.selection_start != self.selection_end
    }

    pub fn clear_selection(&mut self) {
        self.selection_start = self.cursor_pos;
        self.selection_end = self.cursor_pos;
    }

    pub fn select_all(&mut self) {
        self.selection_start = 0;
        self.selection_end = self.text.chars().count();
        self.cursor_pos = self.selection_end;
        self.reset_blink();
    }

    pub fn delete_selection(&mut self) {
        if !self.has_selection() {
            return;
        }
        self.push_undo_state();
        self.delete_selection_internal();
    }

    fn delete_selection_internal(&mut self) {
        if !self.has_selection() {
            return;
        }

        let start = self.selection_start.min(self.selection_end);
        let end = self.selection_start.max(self.selection_end);

        let byte_start = self.char_idx_to_byte_idx(start);
        let byte_end = self.char_idx_to_byte_idx(end);

        self.text.replace_range(byte_start..byte_end, "");

        self.cursor_pos = start;
        self.clear_selection();
        self.reset_blink();
    }

    fn get_selected_text(&self) -> String {
        if !self.has_selection() {
            return String::new();
        }

        let start = self.selection_start.min(self.selection_end);
        let end = self.selection_start.max(self.selection_end);

        let byte_start = self.char_idx_to_byte_idx(start);
        let byte_end = self.char_idx_to_byte_idx(end);

        self.text[byte_start..byte_end].to_string()
    }

    // --- Clipboard Operations ---

    pub fn copy_selection_to_clipboard(&mut self) {
        if self.is_secret {
            return; // Don't copy secrets!
        }
        if self.has_selection() {
            let selected = self.get_selected_text();
            self.set_clipboard_text(&selected);
        }
    }

    pub fn cut_selection(&mut self) {
        if !self.is_secret && self.has_selection() {
            self.copy_selection_to_clipboard();
        }
        if self.has_selection() {
            self.push_undo_state();
            self.delete_selection_internal();
        }
    }

    pub fn paste_text_from_clipboard(&mut self) {
        if let Some(text) = self.get_clipboard_text() {
            let mut valid_text = String::new();
            for ch in text.chars() {
                if self.number_only && !ch.is_ascii_digit() {
                    continue;
                }
                valid_text.push(ch);
            }
            if !valid_text.is_empty() {
                self.push_undo_state();
                if self.has_selection() {
                    self.delete_selection_internal();
                }
                for ch in valid_text.chars() {
                    if self.text.chars().count() < self.max_len {
                        let byte_idx = self.char_idx_to_byte_idx(self.cursor_pos);
                        self.text.insert(byte_idx, ch);
                        self.cursor_pos += 1;
                    }
                }
                self.reset_blink();
            }
        }
    }

    // Platform specific clipboard stubs for testing purely in the engine memory
    // In actual client, this would bridge to winapi / x11 clipboards.
    fn set_clipboard_text(&mut self, text: &str) {
        self.clipboard = Some(text.to_string());
    }

    fn get_clipboard_text(&self) -> Option<String> {
        self.clipboard.clone()
    }

    // --- Events ---

    pub fn on_return(&self) {
        // Trigger OnReturn event in UI framework
    }

    pub fn on_escape(&self) {
        // Trigger OnEscape event in UI framework
    }
    
    pub fn on_tab(&self) {
        // Trigger OnTab event in UI framework
    }

    pub fn on_wm_char(&mut self, char_code: u32, ctrl_down: bool, shift_down: bool) -> bool {
        if ctrl_down {
            match char_code {
                0x01 => {
                    // Ctrl+A (0x01)
                    self.select_all();
                    return true;
                }
                0x03 => {
                    // Ctrl+C (0x03)
                    self.copy_selection_to_clipboard();
                    return true;
                }
                0x16 => {
                    // Ctrl+V (0x16)
                    self.paste_text_from_clipboard();
                    return true;
                }
                0x18 => {
                    // Ctrl+X (0x18)
                    self.cut_selection();
                    return true;
                }
                0x1A => {
                    // Ctrl+Z (0x1A)
                    self.undo();
                    return true;
                }
                0x19 => {
                    // Ctrl+Y (0x19)
                    self.redo();
                    return true;
                }
                _ => {}
            }
        }

        match char_code {
            0x0D => {
                // Return (VK_RETURN in win32 translates to \r in WM_CHAR)
                self.on_return();
                return true;
            }
            0x1B => {
                // Escape
                self.on_escape();
                return true;
            }
            0x09 => {
                // Tab
                self.on_tab();
                return true;
            }
            0x24 => {
                // Home (often not directly via WM_CHAR, but mapping for completness of the IME spec)
                self.move_cursor_home(shift_down);
                return true;
            }
            0x23 => {
                // End
                self.move_cursor_end(shift_down);
                return true;
            }
            0x08 => {
                // Backspace
                self.backspace();
                return true;
            }
            0x2E => {
                // Delete
                self.delete_char();
                return true;
            }
            _ => {
                if !ctrl_down {
                    if let Some(ch) = char::from_u32(char_code) {
                        if !ch.is_control() {
                            self.insert_char(ch);
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    // Utility
    fn char_idx_to_byte_idx(&self, char_idx: usize) -> usize {
        self.text
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.text.len())
    }
}
