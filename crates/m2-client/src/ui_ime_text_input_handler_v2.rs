#[cfg(windows)]
mod clipboard {
    use std::ptr;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    const CF_UNICODETEXT: u32 = 13;
    const GMEM_MOVEABLE: u32 = 0x0002;

    extern "system" {
        fn OpenClipboard(hwnd: *mut std::ffi::c_void) -> i32;
        fn CloseClipboard() -> i32;
        fn GetClipboardData(uFormat: u32) -> *mut std::ffi::c_void;
        fn SetClipboardData(uFormat: u32, hMem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
        fn EmptyClipboard() -> i32;
        fn GlobalAlloc(uFlags: u32, dwBytes: usize) -> *mut std::ffi::c_void;
        fn GlobalLock(hMem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
        fn GlobalUnlock(hMem: *mut std::ffi::c_void) -> i32;
    }

    pub fn get_clipboard_text() -> Option<String> {
        unsafe {
            if OpenClipboard(ptr::null_mut()) == 0 {
                return None;
            }
            let handle = GetClipboardData(CF_UNICODETEXT);
            if handle.is_null() {
                CloseClipboard();
                return None;
            }
            let ptr = GlobalLock(handle) as *const u16;
            if ptr.is_null() {
                CloseClipboard();
                return None;
            }
            let mut len = 0;
            while *ptr.offset(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(ptr, len as usize);
            let s = OsString::from_wide(slice).into_string().ok();
            GlobalUnlock(handle);
            CloseClipboard();
            s
        }
    }

    pub fn set_clipboard_text(text: &str) {
        unsafe {
            if OpenClipboard(ptr::null_mut()) == 0 {
                return;
            }
            EmptyClipboard();
            let mut wchars: Vec<u16> = text.encode_utf16().collect();
            wchars.push(0);
            let bytes = wchars.len() * 2;
            let handle = GlobalAlloc(GMEM_MOVEABLE, bytes);
            if !handle.is_null() {
                let ptr = GlobalLock(handle) as *mut u16;
                if !ptr.is_null() {
                    ptr::copy_nonoverlapping(wchars.as_ptr(), ptr, wchars.len());
                    GlobalUnlock(handle);
                    SetClipboardData(CF_UNICODETEXT, handle);
                }
            }
            CloseClipboard();
        }
    }
}

#[cfg(not(windows))]
mod clipboard {
    use std::sync::Mutex;
    use std::sync::OnceLock;

    static CLIPBOARD: OnceLock<Mutex<String>> = OnceLock::new();

    fn get_clipboard() -> &'static Mutex<String> {
        CLIPBOARD.get_or_init(|| Mutex::new(String::new()))
    }

    pub fn get_clipboard_text() -> Option<String> {
        if let Ok(guard) = get_clipboard().lock() {
            if guard.is_empty() {
                None
            } else {
                Some(guard.clone())
            }
        } else {
            None
        }
    }

    pub fn set_clipboard_text(text: &str) {
        if let Ok(mut guard) = get_clipboard().lock() {
            *guard = text.to_string();
        }
    }
}

pub const IMESTR_MAXLEN: usize = 1024;
pub const MAX_UNDO: usize = 64;

pub const VK_BACK: u32 = 0x08;
pub const VK_LEFT: u32 = 0x25;
pub const VK_RIGHT: u32 = 0x27;
pub const VK_HOME: u32 = 0x24;
pub const VK_END: u32 = 0x23;
pub const VK_DELETE: u32 = 0x2E;
pub const VK_C: u32 = 0x43;
pub const VK_V: u32 = 0x56;
pub const VK_X: u32 = 0x58;
pub const VK_A: u32 = 0x41;

#[derive(Clone, Debug, PartialEq)]
pub struct UndoState {
    pub text: Vec<char>,
    pub curpos: usize,
    pub lastpos: usize,
    pub selbegin: usize,
    pub selend: usize,
}

pub struct ImeTextInputHandlerV2 {
    text: Vec<char>,
    curpos: usize,
    lastpos: usize,
    selbegin: usize,
    selend: usize,
    undo: Vec<UndoState>,
    redo: Vec<UndoState>,
    is_secret_mode: bool,
    max_len: usize,
    enable_paste: bool,
}

impl Default for ImeTextInputHandlerV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ImeTextInputHandlerV2 {
    pub fn new() -> Self {
        Self {
            text: Vec::with_capacity(IMESTR_MAXLEN),
            curpos: 0,
            lastpos: 0,
            selbegin: 0,
            selend: 0,
            undo: Vec::with_capacity(MAX_UNDO),
            redo: Vec::with_capacity(MAX_UNDO),
            is_secret_mode: false,
            max_len: IMESTR_MAXLEN - 1, // -1 to match C++ reserving room for NUL
            enable_paste: true,
        }
    }

    pub fn set_secret_mode(&mut self, is_secret: bool) {
        self.is_secret_mode = is_secret;
    }

    pub fn is_secret_mode(&self) -> bool {
        self.is_secret_mode
    }

    pub fn set_enable_paste(&mut self, enable: bool) {
        self.enable_paste = enable;
    }

    pub fn get_text(&self) -> String {
        self.text.iter().collect()
    }

    pub fn get_render_text(&self) -> String {
        if self.is_secret_mode {
            "*".repeat(self.lastpos)
        } else {
            self.text.iter().collect()
        }
    }

    pub fn get_curpos(&self) -> usize {
        self.curpos
    }

    pub fn get_lastpos(&self) -> usize {
        self.lastpos
    }

    pub fn get_selbegin(&self) -> usize {
        self.selbegin
    }

    pub fn get_selend(&self) -> usize {
        self.selend
    }

    pub fn set_text(&mut self, text: &str) {
        self.clear();
        self.insert_string(&text.chars().collect::<Vec<char>>());
        self.undo.clear();
        self.redo.clear();
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.curpos = 0;
        self.lastpos = 0;
        self.selbegin = 0;
        self.selend = 0;
    }

    pub fn has_selection(&self) -> bool {
        self.selbegin != self.selend
    }

    fn normalize_selection(&mut self) {
        if self.selbegin > self.selend {
            std::mem::swap(&mut self.selbegin, &mut self.selend);
        }
    }

    pub fn clear_selection(&mut self) {
        self.selbegin = self.curpos;
        self.selend = self.curpos;
    }

    pub fn select_all(&mut self) {
        self.selbegin = 0;
        self.selend = self.lastpos;
        self.curpos = self.lastpos;
    }

    pub fn set_selection(&mut self, begin: usize, end: usize) {
        self.selbegin = begin.min(self.lastpos);
        self.selend = end.min(self.lastpos);
        self.curpos = self.selend;
    }

    fn push_undo_state(&mut self) {
        let state = UndoState {
            text: self.text.clone(),
            curpos: self.curpos,
            lastpos: self.lastpos,
            selbegin: self.selbegin,
            selend: self.selend,
        };
        self.undo.push(state);
        if self.undo.len() > MAX_UNDO {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    pub fn undo(&mut self) {
        if let Some(state) = self.undo.pop() {
            let redo_state = UndoState {
                text: self.text.clone(),
                curpos: self.curpos,
                lastpos: self.lastpos,
                selbegin: self.selbegin,
                selend: self.selend,
            };
            self.redo.push(redo_state);
            if self.redo.len() > MAX_UNDO {
                self.redo.remove(0);
            }
            self.text = state.text;
            self.curpos = state.curpos;
            self.lastpos = state.lastpos;
            self.selbegin = state.selbegin;
            self.selend = state.selend;
        }
    }

    pub fn redo(&mut self) {
        if let Some(state) = self.redo.pop() {
            let undo_state = UndoState {
                text: self.text.clone(),
                curpos: self.curpos,
                lastpos: self.lastpos,
                selbegin: self.selbegin,
                selend: self.selend,
            };
            self.undo.push(undo_state);
            if self.undo.len() > MAX_UNDO {
                self.undo.remove(0);
            }
            self.text = state.text;
            self.curpos = state.curpos;
            self.lastpos = state.lastpos;
            self.selbegin = state.selbegin;
            self.selend = state.selend;
        }
    }

    pub fn delete_selection(&mut self) {
        self.normalize_selection();
        if !self.has_selection() {
            return;
        }

        self.push_undo_state();

        let a = self.selbegin;
        let b = self.selend;
        let sel_len = b - a;

        if sel_len == 0 {
            return;
        }

        self.text.drain(a..b);
        self.lastpos -= sel_len;
        self.curpos = a;
        self.selbegin = a;
        self.selend = a;
    }

    pub fn copy_selection_to_clipboard(&mut self) {
        self.normalize_selection();
        if !self.has_selection() {
            return;
        }
        if self.is_secret_mode {
            return;
        }

        let a = self.selbegin;
        let b = self.selend;
        let sel_text: String = self.text[a..b].iter().collect();
        clipboard::set_clipboard_text(&sel_text);
    }

    pub fn cut_selection(&mut self) {
        if !self.is_secret_mode {
            self.copy_selection_to_clipboard();
        }
        self.delete_selection();
    }

    pub fn paste_text_from_clipboard(&mut self) {
        if !self.enable_paste {
            return;
        }
        if let Some(text) = clipboard::get_clipboard_text() {
            self.paste_string(&text);
        }
    }

    pub fn paste_string(&mut self, s: &str) {
        let chars: Vec<char> = s.chars().filter(|&c| c != '\r' && c != '\n').collect();
        self.insert_string(&chars);
    }

    pub fn insert_string(&mut self, chars: &[char]) {
        self.push_undo_state();

        if chars.is_empty() {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
            self.undo.pop(); // Remove the extra undo pushed by delete_selection
        }

        let insert_len = chars.len().min(self.max_len.saturating_sub(self.lastpos));
        if insert_len == 0 {
            return;
        }

        let chars_to_insert = &chars[0..insert_len];

        self.text.splice(self.curpos..self.curpos, chars_to_insert.iter().copied());
        self.curpos += insert_len;
        self.lastpos += insert_len;
        self.selbegin = self.curpos;
        self.selend = self.curpos;
    }

    pub fn on_char(&mut self, c: char) {
        if c == '\x16' || c == '\r' || c == '\n' || c == '\t' || c == '\x08' || c == '\x7f' {
            return; // Ignore control characters commonly passed to WM_CHAR
        }
        self.insert_string(&[c]);
    }

    pub fn move_left(&mut self) {
        if self.curpos > 0 {
            self.curpos -= 1;
        }
        self.clear_selection();
    }

    pub fn move_right(&mut self) {
        if self.curpos < self.lastpos {
            self.curpos += 1;
        }
        self.clear_selection();
    }

    pub fn move_home(&mut self) {
        self.curpos = 0;
        self.clear_selection();
    }

    pub fn move_end(&mut self) {
        self.curpos = self.lastpos;
        self.clear_selection();
    }

    pub fn on_backspace(&mut self) {
        if self.has_selection() {
            self.delete_selection();
            return;
        }

        if self.curpos > 0 {
            self.push_undo_state();
            self.curpos -= 1;
            self.del_cur_pos_internal();
        }
    }

    pub fn on_delete(&mut self) {
        if self.has_selection() {
            self.delete_selection();
            return;
        }

        if self.curpos < self.lastpos {
            self.push_undo_state();
            self.del_cur_pos_internal();
        }
    }

    fn del_cur_pos_internal(&mut self) {
        if self.curpos < self.lastpos {
            self.text.remove(self.curpos);
            self.lastpos -= 1;
            self.curpos = self.curpos.min(self.lastpos);
        }
    }

    pub fn on_key_down(&mut self, key: u32, ctrl_pressed: bool, shift_pressed: bool) -> bool {
        if ctrl_pressed {
            match key {
                VK_A => {
                    self.select_all();
                    return true;
                }
                VK_C => {
                    self.copy_selection_to_clipboard();
                    return true;
                }
                VK_X => {
                    self.cut_selection();
                    return true;
                }
                VK_V => {
                    self.paste_text_from_clipboard();
                    return true;
                }
                _ => {}
            }
        }

        // Shift selection logic (simplified version of IME text selection via arrows)
        let previous_curpos = self.curpos;

        match key {
            VK_LEFT => {
                if self.curpos > 0 {
                    self.curpos -= 1;
                }
                if shift_pressed {
                    if self.selbegin == self.selend {
                        self.selbegin = previous_curpos;
                    }
                    self.selend = self.curpos;
                } else {
                    self.clear_selection();
                }
                true
            }
            VK_RIGHT => {
                if self.curpos < self.lastpos {
                    self.curpos += 1;
                }
                if shift_pressed {
                    if self.selbegin == self.selend {
                        self.selbegin = previous_curpos;
                    }
                    self.selend = self.curpos;
                } else {
                    self.clear_selection();
                }
                true
            }
            VK_HOME => {
                self.curpos = 0;
                if shift_pressed {
                    if self.selbegin == self.selend {
                        self.selbegin = previous_curpos;
                    }
                    self.selend = self.curpos;
                } else {
                    self.clear_selection();
                }
                true
            }
            VK_END => {
                self.curpos = self.lastpos;
                if shift_pressed {
                    if self.selbegin == self.selend {
                        self.selbegin = previous_curpos;
                    }
                    self.selend = self.curpos;
                } else {
                    self.clear_selection();
                }
                true
            }
            VK_BACK => {
                self.on_backspace();
                true
            }
            VK_DELETE => {
                self.on_delete();
                true
            }
            _ => false,
        }
    }

    pub fn is_cursor_visible(&self, time_ms: u64) -> bool {
        // Blinking cursor: visible for 500ms, hidden for 500ms
        (time_ms / 500) % 2 == 0
    }
}
