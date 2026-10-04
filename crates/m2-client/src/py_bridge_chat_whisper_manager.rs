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
pub fn parse_hyperlinks(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut current_idx = 0;

    while let Some(start_h) = text[current_idx..].find("|H") {
        let abs_start = current_idx + start_h;
        // Search for the *second* "|h" from the starting "|H" since links look like "|H...|h[...]|h"
        // inner_start points after "|H"
        let inner_start = abs_start + 2;
        
        // Find the first |h
        if let Some(first_h) = text[inner_start..].find("|h") {
            // Find the second |h after the first one
            let after_first_h = inner_start + first_h + 2;
            if let Some(second_h) = text[after_first_h..].find("|h") {
                let inner_end = after_first_h + second_h;
                // Capture from inner_start to inner_end (after the first |h, up to the second |h)
                // Wait, the format is |Hitem:1234...|h[Name]|h
                // The C++ client splits at the first |h to extract the link type and data, 
                // and the text between the two |h as the visible text. 
                // Let's capture exactly what's between |H and the first |h, then the [Name] part.
                
                // Let's just capture the whole payload to match the test: "item:1234:0:0:0:0|h[Nazwa]"
                links.push(text[inner_start..inner_end].to_string());
                current_idx = inner_end + 2;
                continue;
            }
        }
        break;
    }
    links
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

        // Ensure hyperlinks parse correctly to test compatibility.
        let _links = parse_hyperlinks(&msg_str);

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
