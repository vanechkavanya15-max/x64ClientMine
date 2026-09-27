use std::collections::{VecDeque, HashSet};
use std::sync::{Mutex, OnceLock};

pub const CHAT_TYPE_TALKING: u8 = 0;
pub const CHAT_TYPE_INFO: u8 = 1;
pub const CHAT_TYPE_NOTICE: u8 = 2;
pub const CHAT_TYPE_PARTY: u8 = 3;
pub const CHAT_TYPE_GUILD: u8 = 4;
pub const CHAT_TYPE_COMMAND: u8 = 5;
pub const CHAT_TYPE_SHOUT: u8 = 6;
pub const CHAT_TYPE_WHISPER: u8 = 7;
pub const CHAT_TYPE_BIG_NOTICE: u8 = 8;
pub const CHAT_TYPE_MAX_NUM: u8 = 9;

pub const CHAT_LINE_MAX_NUM: usize = 1000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemHyperlink {
    pub vnum: u32,
    pub flags: u32,
    pub sockets: [i32; 3],
    pub attrs: Vec<(u8, i16)>,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ChatLine {
    pub id: u64,
    pub chat_type: u8,
    pub text: String,
    pub sender_name: String,
    pub timestamp: u32,
}

pub struct ChatBuffer {
    pub lines: VecDeque<ChatLine>,
    pub next_id: u64,
    pub last_msg_time: u32,
    pub forbidden_words: HashSet<String>,
}

impl ChatBuffer {
    pub fn new() -> Self {
        Self {
            lines: VecDeque::with_capacity(CHAT_LINE_MAX_NUM),
            next_id: 1,
            last_msg_time: 0,
            forbidden_words: HashSet::new(),
        }
    }

    pub fn set_forbidden_words(&mut self, words: Vec<String>) {
        self.forbidden_words = words.into_iter().collect();
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.last_msg_time = 0;
    }

    pub fn append_line(&mut self, chat_type: u8, text: &str, timestamp: u32) -> bool {
        // Anti-flood: 500ms min delay for player speech (CHAT_TYPE_TALKING / CHAT_TYPE_SHOUT)
        let is_flood_checked = chat_type == CHAT_TYPE_TALKING || chat_type == CHAT_TYPE_SHOUT;
        if is_flood_checked && self.last_msg_time != 0 && timestamp.wrapping_sub(self.last_msg_time) < 500 {
            return false;
        }

        let mut filtered_text = text.to_string();
        for word in &self.forbidden_words {
            if filtered_text.contains(word) {
                let replacement = "*".repeat(word.chars().count());
                filtered_text = filtered_text.replace(word, &replacement);
            }
        }

        let mut sender_name = String::new();
        let mut final_text = filtered_text.as_str();

        // Extract sender name from "Name : Message"
        if let Some(colon_idx) = filtered_text.find(" : ") {
            sender_name = filtered_text[..colon_idx].to_string();
            final_text = &filtered_text[colon_idx + 3..];
        }

        let line = ChatLine {
            id: self.next_id,
            chat_type,
            text: final_text.to_string(),
            sender_name,
            timestamp,
        };

        self.next_id += 1;
        if is_flood_checked {
            self.last_msg_time = timestamp;
        }

        self.lines.push_back(line);

        while self.lines.len() > CHAT_LINE_MAX_NUM {
            self.lines.pop_front();
        }

        true
    }
}

pub fn parse_item_hyperlink(text: &str) -> Option<ItemHyperlink> {
    // Format: |cffffc700|Hitem:vnum:flags:socket0:socket1:socket2:attr0_type:attr0_val:...|h[NazwaPrzedmiotu]|h|r
    if !text.starts_with("|c") || text.len() < 10 {
        return None;
    }
    
    let color_end = 10;
    if !text[color_end..].starts_with("|Hitem:") {
        return None;
    }
    
    let parts_start = color_end + 7;
    let end_idx = text[parts_start..].find("|h[")?;
    
    let parts_str = &text[parts_start..parts_start + end_idx];
    let mut parts = parts_str.split(':');
    
    let vnum = u32::from_str_radix(parts.next()?, 16).ok()?;
    let flags = u32::from_str_radix(parts.next()?, 16).ok()?;
    
    let mut sockets = [0i32; 3];
    for i in 0..3 {
        if let Some(p) = parts.next() {
            // C++ uses %x for formatting, which prints negative signed ints as 32-bit hex
            sockets[i] = u32::from_str_radix(p, 16).ok()? as i32;
        } else {
            return None;
        }
    }
    
    let mut attrs = Vec::new();
    while let Some(attr_type_str) = parts.next() {
        if let Some(attr_val_str) = parts.next() {
            let attr_type = u32::from_str_radix(attr_type_str, 16).ok()? as u8;
            let attr_val = u32::from_str_radix(attr_val_str, 16).ok()? as i16;
            attrs.push((attr_type, attr_val));
        } else {
            break;
        }
    }
    
    let name_start = parts_start + end_idx + 3; // +3 for "|h["
    let name_end_offset = text[name_start..].find("]|h|r")?;
    let name = text[name_start..name_start + name_end_offset].to_string();
    
    Some(ItemHyperlink {
        vnum,
        flags,
        sockets,
        attrs,
        name,
    })
}

static CHAT_BUFFER: OnceLock<Mutex<ChatBuffer>> = OnceLock::new();

pub fn get_chat_buffer() -> &'static Mutex<ChatBuffer> {
    CHAT_BUFFER.get_or_init(|| Mutex::new(ChatBuffer::new()))
}

#[no_mangle]
pub extern "C" fn m2_chat_append_line(chat_type: u8, text: *const std::ffi::c_char) -> bool {
    if text.is_null() {
        return false;
    }
    
    let text_cstr = unsafe { std::ffi::CStr::from_ptr(text) };
    let text_str = match text_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u32;
        
    let mut buffer = get_chat_buffer().lock().unwrap();
    buffer.append_line(chat_type, text_str, timestamp)
}
