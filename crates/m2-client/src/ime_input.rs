//! Modul `ime_input` — Edytor tekstu IME dla pol logowania, hasla i czatu
//! Wzorzec referencyjny C++: `source/EterLib/IME.cpp oraz source/UserInterface/PythonIME.cpp`
//! Opis: Odwzorowanie bufora wprowadzania tekstu: dopisywanie znakow ASCII/UTF-8, Backspace, Delete, kursor tekstowy, maskowanie gwiazdkami (*) dla pola hasla, historia wpisow.

pub const IMESTR_MAXLEN: usize = 1024;

#[derive(Debug, Clone)]
pub struct ImeInput {
    pub initialized: bool,
    text: String,
    cursor_pos: usize, // Pozycja w bajtach (w UTF-8 to wazne)
    max_len: usize, // Limit znakow (char count)
    is_password_mode: bool,
    is_number_mode: bool,
    history: Vec<String>,
    history_index: usize,
}

impl Default for ImeInput {
    fn default() -> Self {
        Self::new()
    }
}

impl ImeInput {
    pub fn new() -> Self {
        Self { 
            initialized: true,
            text: String::new(),
            cursor_pos: 0,
            max_len: IMESTR_MAXLEN,
            is_password_mode: false,
            is_number_mode: false,
            history: Vec::new(),
            history_index: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

impl ImeInput {
    // ---- Ustawienia ----

    pub fn set_max_len(&mut self, max: usize) {
        self.max_len = max;
    }

    pub fn set_password_mode(&mut self, mode: bool) {
        self.is_password_mode = mode;
    }

    pub fn set_number_mode(&mut self, mode: bool) {
        self.is_number_mode = mode;
    }

    // ---- Manipulacja tekstem ----

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_pos = 0;
    }

    pub fn set_text(&mut self, text: &str) {
        self.text.clear();
        let mut char_count = 0;
        for c in text.chars() {
            if char_count >= self.max_len {
                break;
            }
            self.text.push(c);
            char_count += 1;
        }
        self.cursor_pos = self.text.len();
    }

    pub fn get_text(&self) -> String {
        if self.is_password_mode {
            let char_count = self.text.chars().count();
            "*".repeat(char_count)
        } else {
            self.text.clone()
        }
    }

    pub fn get_raw_text(&self) -> &str {
        &self.text
    }

    pub fn on_char(&mut self, c: char) {
        if self.is_number_mode && !c.is_ascii_digit() {
            return;
        }

        // Ignoruj znaki sterujace (oprocz tabulacji, backspace, etc. - tu w uproszczeniu tylko drukowalne)
        if c.is_control() {
            return;
        }

        if self.text.chars().count() >= self.max_len {
            return;
        }

        self.text.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
    }

    pub fn paste_text(&mut self, text: &str) {
        for c in text.chars() {
            self.on_char(c);
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            // Znajdz poczatek znaku przed kursorem
            let mut new_pos = self.cursor_pos - 1;
            while !self.text.is_char_boundary(new_pos) {
                new_pos -= 1;
            }
            self.text.remove(new_pos);
            self.cursor_pos = new_pos;
        }
    }

    pub fn delete(&mut self) {
        if self.cursor_pos < self.text.len() {
            self.text.remove(self.cursor_pos);
        }
    }

    // ---- Kursor ----

    pub fn move_left(&mut self) {
        if self.cursor_pos > 0 {
            let mut new_pos = self.cursor_pos - 1;
            while !self.text.is_char_boundary(new_pos) {
                new_pos -= 1;
            }
            self.cursor_pos = new_pos;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_pos < self.text.len() {
            let mut new_pos = self.cursor_pos + 1;
            while !self.text.is_char_boundary(new_pos) {
                new_pos += 1;
            }
            self.cursor_pos = new_pos;
        }
    }

    pub fn move_home(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor_pos = self.text.len();
    }

    pub fn set_cursor_position(&mut self, char_pos: usize) {
        let mut byte_pos = 0;
        for (i, c) in self.text.chars().enumerate() {
            if i == char_pos {
                break;
            }
            byte_pos += c.len_utf8();
        }
        self.cursor_pos = byte_pos;
    }

    pub fn get_cursor_position(&self) -> usize {
        // Zwraca pozycje w znakach dla UI
        self.text[..self.cursor_pos].chars().count()
    }

    // ---- Historia ----

    pub fn add_to_history(&mut self) {
        if !self.text.is_empty() {
            // Nie dodawaj duplikatow z rzedu
            if self.history.last() != Some(&self.text) {
                self.history.push(self.text.clone());
            }
        }
        self.history_index = self.history.len();
    }

    pub fn get_history_prev(&mut self) -> Option<String> {
        if self.history.is_empty() {
            return None;
        }
        if self.history_index > 0 {
            self.history_index -= 1;
            return Some(self.history[self.history_index].clone());
        }
        None
    }

    pub fn get_history_next(&mut self) -> Option<String> {
        if self.history.is_empty() || self.history_index >= self.history.len() {
            return None;
        }
        self.history_index += 1;
        if self.history_index < self.history.len() {
            Some(self.history[self.history_index].clone())
        } else {
            Some(String::new()) // Pusty string po przejsciu konca historii
        }
    }
}
