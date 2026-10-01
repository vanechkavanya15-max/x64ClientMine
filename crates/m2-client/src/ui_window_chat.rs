use crate::chat_buffer_v2::{
    CHAT_TYPE_BIG_NOTICE, CHAT_TYPE_COMMAND, CHAT_TYPE_GUILD, CHAT_TYPE_INFO, CHAT_TYPE_NOTICE,
    CHAT_TYPE_PARTY, CHAT_TYPE_SHOUT, CHAT_TYPE_TALKING, CHAT_TYPE_WHISPER,
};
use crate::send_chat_packets_v2::{m2_send_chat, m2_send_whisper};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatTab {
    All,
    Normal,
    Party,
    Guild,
}

#[derive(Debug, Clone)]
pub struct UiChatLine {
    pub chat_type: u8,
    pub text: String,
    pub timestamp: u32,
    pub color: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct UiChatWindow {
    pub visible_lines: usize, // 3 to 12
    pub active_tab: ChatTab,
    pub is_edit_mode: bool,
    pub input_text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub screen_width: i32,
    pub screen_height: i32,
    pub lines: Vec<UiChatLine>,
}

impl UiChatWindow {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        let width = 300;
        let visible_lines = 5;
        let line_height = 15;
        let is_edit_mode = false;
        let height = visible_lines as i32 * line_height + if is_edit_mode { 20 } else { 0 };
        let x = 0;
        let y = screen_height - height;

        Self {
            visible_lines,
            active_tab: ChatTab::All,
            is_edit_mode,
            input_text: String::new(),
            x,
            y,
            width,
            height,
            screen_width,
            screen_height,
            lines: Vec::new(),
        }
    }

    pub fn append_chat_line(&mut self, chat_type: u8, text: String, timestamp: u32) {
        let color = get_chat_color(chat_type);
        self.lines.push(UiChatLine {
            chat_type,
            text,
            timestamp,
            color,
        });

        // Cap at 300 lines max like CHAT_LINE_MAX_NUM
        if self.lines.len() > 300 {
            self.lines.remove(0);
        }
    }

    pub fn get_visible_chat_lines(&self) -> Vec<&UiChatLine> {
        self.lines
            .iter()
            .filter(|line| self.is_chat_visible(line.chat_type))
            .rev()
            .take(self.visible_lines)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    pub fn set_visible_lines(&mut self, lines: usize) {
        let clamped_lines = lines.clamp(3, 12);
        self.visible_lines = clamped_lines;
        self.update_layout();
    }

    pub fn set_screen_size(&mut self, width: i32, height: i32) {
        self.screen_width = width;
        self.screen_height = height;
        self.update_layout();
    }

    fn update_layout(&mut self) {
        let line_height = 15;
        self.height = self.visible_lines as i32 * line_height + if self.is_edit_mode { 20 } else { 0 };
        self.y = self.screen_height - self.height;
    }

    pub fn set_tab(&mut self, tab: ChatTab) {
        self.active_tab = tab;
    }

    pub fn is_chat_visible(&self, chat_type: u8) -> bool {
        match self.active_tab {
            ChatTab::All => true,
            ChatTab::Normal => matches!(
                chat_type,
                CHAT_TYPE_TALKING | CHAT_TYPE_SHOUT | CHAT_TYPE_INFO | CHAT_TYPE_NOTICE
            ),
            ChatTab::Party => chat_type == CHAT_TYPE_PARTY,
            ChatTab::Guild => chat_type == CHAT_TYPE_GUILD,
        }
    }

    pub fn handle_key_down(&mut self, key_code: u32) -> bool {
        const VK_RETURN: u32 = 0x0D;
        const VK_ESCAPE: u32 = 0x1B;
        const VK_BACK: u32 = 0x08;

        if !self.is_edit_mode {
            if key_code == VK_RETURN {
                self.is_edit_mode = true;
                self.update_layout();
                return true;
            }
            return false;
        }

        if key_code == VK_ESCAPE {
            self.is_edit_mode = false;
            self.input_text.clear();
            self.update_layout();
            return true;
        }

        if key_code == VK_RETURN {
            self.process_input_text();
            self.is_edit_mode = false;
            self.input_text.clear();
            self.update_layout();
            return true;
        }

        if key_code == VK_BACK {
            self.input_text.pop();
            return true;
        }

        true
    }

    pub fn append_input_char(&mut self, c: char) {
        if self.is_edit_mode && !c.is_control() {
            self.input_text.push(c);
        }
    }

    fn process_input_text(&mut self) {
        let text = self.input_text.trim();
        if text.is_empty() {
            return;
        }

        // Check if it's a whisper "/w Name Message" or "/whisper Name Message"
        if text.starts_with("/w ") || text.starts_with("/whisper ") {
            let parts: Vec<&str> = text.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                let name = parts[1];
                let msg = parts[2];
                
                let mut out_buf = [0u8; 1024];
                unsafe {
                    m2_send_whisper(
                        name.as_ptr(),
                        name.len(),
                        msg.as_ptr(),
                        msg.len(),
                        out_buf.as_mut_ptr(),
                    );
                }
                return;
            }
        }

        let mut chat_type = CHAT_TYPE_TALKING;
        let mut msg_to_send = text;

        if text.starts_with('!') {
            chat_type = CHAT_TYPE_SHOUT;
            msg_to_send = &text[1..];
        } else if text.starts_with('#') {
            chat_type = CHAT_TYPE_PARTY;
            msg_to_send = &text[1..];
        } else if text.starts_with('%') {
            chat_type = CHAT_TYPE_GUILD;
            msg_to_send = &text[1..];
        }

        let mut out_buf = [0u8; 1024];
        unsafe {
            m2_send_chat(
                chat_type,
                msg_to_send.as_ptr(),
                msg_to_send.len(),
                out_buf.as_mut_ptr(),
            );
        }
    }
}

pub fn get_chat_color(chat_type: u8) -> [f32; 4] {
    match chat_type {
        CHAT_TYPE_TALKING => [1.0, 1.0, 1.0, 1.0],           // White
        CHAT_TYPE_INFO => [1.0, 0.785, 0.785, 1.0],          // Info yellow/pinkish
        CHAT_TYPE_NOTICE => [1.0, 0.902, 0.730, 1.0],        // Notice
        CHAT_TYPE_PARTY => [0.542, 1.0, 0.949, 1.0],         // Party blue
        CHAT_TYPE_GUILD => [0.906, 0.847, 1.0, 1.0],         // Guild green/yellow
        CHAT_TYPE_COMMAND => [0.658, 1.0, 0.835, 1.0],       // Command
        CHAT_TYPE_SHOUT => [0.658, 1.0, 0.835, 1.0],         // Shout
        CHAT_TYPE_WHISPER => [74.0 / 255.0, 225.0 / 255.0, 74.0 / 255.0, 1.0], // Whisper 0xff4AE14A
        CHAT_TYPE_BIG_NOTICE => [1.0, 0.902, 0.730, 1.0],    // Big notice
        _ => [1.0, 1.0, 1.0, 1.0],
    }
}
