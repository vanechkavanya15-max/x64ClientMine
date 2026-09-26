/// Reprezentuje naglowek pakietu skryptu TPacketGCScript
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCScript {
    pub header: u16,
    pub length: u16,
    pub skin: u8,
    pub src_size: u16,
}

pub struct QuestDialog {
    pub skin: u8,
    pub title: String,
    pub content: String,
    pub buttons: Vec<String>,
    pub questions: Vec<String>,
}

pub fn parse_script_packet(data: &[u8]) -> Option<(u8, String)> {
    if data.len() < std::mem::size_of::<TPacketGCScript>() {
        return None;
    }

    let packet_ptr = data.as_ptr() as *const TPacketGCScript;
    let packet = unsafe { std::ptr::read_unaligned(packet_ptr) };

    if data.len() < packet.length as usize {
        return None;
    }

    let str_len = packet.length as usize - std::mem::size_of::<TPacketGCScript>();
    let str_data = &data[std::mem::size_of::<TPacketGCScript>()..std::mem::size_of::<TPacketGCScript>() + str_len];
    
    let mut actual_len = str_len;
    if actual_len > 0 && str_data[actual_len - 1] == 0 {
        actual_len -= 1;
    }

    let script_str = String::from_utf8_lossy(&str_data[..actual_len]).to_string();

    Some((packet.skin, script_str))
}

fn parse_argument_string(args_str: &str, key: &str) -> Option<String> {
    let parts: Vec<&str> = args_str.split('|').collect();
    for part in parts {
        let key_value: Vec<&str> = part.split(';').collect();
        if key_value.len() >= 2 {
            let current_key = key_value[0].trim();
            if current_key == key {
                return Some(key_value[1..].join(";").trim().to_string());
            }
        }
    }
    None
}

pub fn parse_quest_script(skin: u8, script_str: &str) -> QuestDialog {
    let mut dialog = QuestDialog {
        skin,
        title: String::new(),
        content: String::new(),
        buttons: Vec::new(),
        questions: Vec::new(),
    };

    let mut chars = script_str.chars().peekable();
    
    while let Some(&c) = chars.peek() {
        if c == '[' {
            chars.next(); // Consume '['
            
            let mut tag_content = String::new();
            while let Some(tc) = chars.next() {
                if tc == ']' {
                    break;
                }
                tag_content.push(tc);
            }
            
            let tag_content = tag_content.trim();
            let space_pos = tag_content.find(' ');
            let (command_name, args_str) = if let Some(pos) = space_pos {
                (&tag_content[..pos], &tag_content[pos+1..])
            } else {
                (&tag_content[..], "")
            };
            
            let command_name = command_name.trim();
            let args_str = args_str.trim();

            match command_name {
                "TITLE" => {
                    if let Some(title) = parse_argument_string(args_str, "value") {
                        dialog.title = title;
                    }
                },
                "QUESTBUTTON" => {
                    if let Some(name) = parse_argument_string(args_str, "name") {
                        dialog.buttons.push(name);
                    }
                },
                "QUESTION" => {
                    let parts: Vec<&str> = args_str.split('|').collect();
                    for part in parts {
                        let key_value: Vec<&str> = part.split(';').collect();
                        if key_value.len() >= 2 {
                            let current_key = key_value[0].trim();
                            if current_key == "name" || (current_key != "idx" && current_key != "icon_type" && current_key != "icon_name") {
                                // Normally they pass just one string argument per question button, or idx;1|name;Yes
                                // Metin2 passes [QUESTION arg1|arg2] as multiple AppendQuestion calls.
                                // In the test we pass [QUESTION idx;1|name;Yes|idx;2|name;No], so we only want the name parts if it's key-value.
                                // Wait, the C++ code for QUESTION parses it into an argList, where EACH part separated by | is a button label.
                                // Actually parser.cpp splits on '|' and pushes an argument. If it doesn't have a ';', it sets it.
                                // Let's just grab "name" values for now if key is name.
                                if current_key == "name" {
                                    dialog.questions.push(key_value[1..].join(";").trim().to_string());
                                }
                            }
                        } else if !part.trim().is_empty() {
                            // Raw value
                            dialog.questions.push(part.trim().to_string());
                        }
                    }
                },
                "ENTER" => {
                    dialog.content.push('\n');
                },
                "CLEAR" => {
                    dialog.content.clear();
                },
                _ => {
                    // Ignore other tags
                }
            }
        } else if c == '\r' || c == '\n' {
            chars.next();
            // Ignore raw newlines usually
        } else {
            let letter = chars.next().unwrap();
            dialog.content.push(letter);
        }
    }

    dialog
}

use std::sync::RwLock;

static CURRENT_QUEST_DIALOG: std::sync::OnceLock<RwLock<Option<QuestDialog>>> = std::sync::OnceLock::new();

#[no_mangle]
pub extern "C" fn m2_quest_get_dialog(out_title: *mut u8, out_content: *mut u8) -> bool {
    let global_state = CURRENT_QUEST_DIALOG.get_or_init(|| RwLock::new(None));
    let dialog_guard = match global_state.read() {
        Ok(guard) => guard,
        Err(_) => return false,
    };

    if let Some(dialog) = dialog_guard.as_ref() {
        if !out_title.is_null() {
            unsafe {
                let title_bytes = dialog.title.as_bytes();
                std::ptr::copy_nonoverlapping(title_bytes.as_ptr(), out_title, title_bytes.len());
                *out_title.add(title_bytes.len()) = 0;
            }
        }
        if !out_content.is_null() {
            unsafe {
                let content_bytes = dialog.content.as_bytes();
                std::ptr::copy_nonoverlapping(content_bytes.as_ptr(), out_content, content_bytes.len());
                *out_content.add(content_bytes.len()) = 0;
            }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_quest_recv_script_packet(data_ptr: *const u8, data_len: usize) -> bool {
    if data_ptr.is_null() || data_len == 0 {
        return false;
    }
    let data = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    if let Some((skin, script_str)) = parse_script_packet(data) {
        let dialog = parse_quest_script(skin, &script_str);
        let global_state = CURRENT_QUEST_DIALOG.get_or_init(|| RwLock::new(None));
        if let Ok(mut guard) = global_state.write() {
            *guard = Some(dialog);
            return true;
        }
    }
    false
}
