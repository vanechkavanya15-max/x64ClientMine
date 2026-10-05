use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Default)]
pub struct WhisperLine {
    pub chat_type: u8,
    pub name: String,
    pub message: String,
    pub sender: Option<String>,
    pub display_text: String,
    pub links: Vec<ParsedHyperlink>,
}

#[derive(Debug, Clone, Default)]
pub struct WhisperConversation {
    pub name: String,
    pub lines: Vec<WhisperLine>,
}

static WHISPER_MANAGER: OnceLock<Mutex<HashMap<String, WhisperConversation>>> = OnceLock::new();

pub fn get_whisper_manager() -> &'static Mutex<HashMap<String, WhisperConversation>> {
    WHISPER_MANAGER.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Helper function to parse an item hyperlink like `|cffffc700|Hitem:1234:0:0:0:0|h[Nazwa]|h|r`.
/// Since we just need to ensure the system supports reading/validating hyperlinked items,
/// we will parse it from the string if present, or just pass the string through.

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedHyperlink {
    pub link_data: String,
    pub display_text: String,
}

pub fn parse_message_tags(text: &str) -> (String, Vec<ParsedHyperlink>) {
    let mut display_text = String::new();
    let mut links = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '|' {
            if let Some(&next_c) = chars.peek() {
                match next_c {
                    'c' => {
                        // Skip |c and the 8 color hex chars
                        chars.next(); // consume 'c'
                        for _ in 0..8 {
                            chars.next();
                        }
                    }
                    'r' => {
                        // Skip |r
                        chars.next();
                    }
                    'H' => {
                        // Hyperlink parsing: |Hlink_data|h[display_name]|h
                        chars.next(); // consume 'H'
                        
                        let mut link_data = String::new();
                        while let Some(&inner_c) = chars.peek() {
                            if inner_c == '|' {
                                break;
                            }
                            link_data.push(inner_c);
                            chars.next();
                        }

                        // Expecting |h
                        if chars.next() == Some('|') && chars.next() == Some('h') {
                            let mut link_display = String::new();
                            while let Some(&inner_c) = chars.peek() {
                                if inner_c == '|' {
                                    break;
                                }
                                link_display.push(inner_c);
                                chars.next();
                            }
                            // Expecting ending |h
                            if chars.next() == Some('|') && chars.next() == Some('h') {
                                display_text.push_str(&link_display);
                                links.push(ParsedHyperlink {
                                    link_data,
                                    display_text: link_display,
                                });
                            }
                        }
                    }
                    _ => {
                        display_text.push(c);
                    }
                }
            } else {
                display_text.push(c);
            }
        } else {
            display_text.push(c);
        }
    }

    (display_text, links)
}

pub fn parse_hyperlinks(text: &str) -> Vec<String> {
    parse_message_tags(text).1.into_iter().map(|l| format!("{}|h{}", l.link_data, l.display_text)).collect()
}

#[no_mangle]
pub extern "C" fn m2_py_chat_create_whisper(name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(name) };
        let name_str = match c_str.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };
        let mut map = match get_whisper_manager().lock() {
            Ok(m) => m,
            Err(e) => e.into_inner(),
        };
        map.entry(name_str.clone()).or_insert_with(|| WhisperConversation {
            name: name_str,
            lines: Vec::new(),
        });
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_append_whisper(
    chat_type: u8,
    name: *const c_char,
    message: *const c_char,
) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() || message.is_null() {
            return false;
        }
        let c_name = unsafe { CStr::from_ptr(name) };
        let c_msg = unsafe { CStr::from_ptr(message) };

        let name_str = match c_name.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };

        let msg_str = match c_msg.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };

        let mut sender = None;
        let mut msg_text = msg_str.as_str();

        // BiDi format handling: check if message starts with "name : message"
        if let Some(idx) = msg_text.find(" : ") {
            if idx > 0 {
                sender = Some(msg_text[..idx].to_string());
                msg_text = &msg_text[idx + 3..];
            }
        }

        let (display_text, links) = parse_message_tags(msg_text);

        let mut map = match get_whisper_manager().lock() {
            Ok(m) => m,
            Err(e) => e.into_inner(),
        };

        let conversation = map.entry(name_str.clone()).or_insert_with(|| WhisperConversation {
            name: name_str.clone(),
            lines: Vec::new(),
        });

        conversation.lines.push(WhisperLine {
            chat_type,
            name: name_str,
            message: msg_str,
            sender,
            links,
            display_text,
        });

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_render_whisper(name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(name) };
        let name_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };
        let map = match get_whisper_manager().lock() {
            Ok(m) => m,
            Err(e) => e.into_inner(),
        };
        map.contains_key(name_str)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_clear_whisper(name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(name) };
        let name_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };
        let mut map = match get_whisper_manager().lock() {
            Ok(m) => m,
            Err(e) => e.into_inner(),
        };
        map.remove(name_str).is_some()
    })
}
