use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

use m2_client_bridge::ffi_guard::safe_ffi_call;

// --- Data Structures ---

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TextTail {
    pub virtual_id: u32,
    pub text: String,
    pub owner_text: Option<String>,
    pub color: Color,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub x_start: f32,
    pub y_start: f32,
    pub x_end: f32,
    pub y_end: f32,
    pub f_distance_from_player: f32,
    pub b_name_flag: bool,
    pub living_time: u32,
    pub f_height: f32,
}

impl TextTail {
    pub fn new(virtual_id: u32, text: String, color: Color, f_height: f32) -> Self {
        // Mocked size estimation similar to CPythonTextTail::RegisterTextTail
        let x_size = text.len() as f32 * 6.0;
        let y_size = 12.0;

        Self {
            virtual_id,
            text,
            owner_text: None,
            color,
            x: -100.0,
            y: -100.0,
            z: 0.0,
            x_start: (-x_size / 2.0) - 2.0,
            y_start: -2.0,
            x_end: (x_size / 2.0) + 2.0,
            y_end: y_size,
            f_distance_from_player: 0.0,
            b_name_flag: false,
            living_time: 0,
            f_height,
        }
    }

    pub fn is_in(&self, target: &TextTail) -> bool {
        let x1_source = self.x + self.x_start;
        let y1_source = self.y + self.y_start;
        let x2_source = self.x + self.x_end;
        let y2_source = self.y + self.y_end;

        let x1_target = target.x + target.x_start;
        let y1_target = target.y + target.y_start;
        let x2_target = target.x + target.x_end;
        let y2_target = target.y + target.y_end;

        x1_source <= x2_target
            && x2_source >= x1_target
            && y1_source <= y2_target
            && y2_source >= y1_target
    }
}

// --- State Management ---

pub struct TextTailState {
    pub character_text_tail_map: HashMap<u32, TextTail>,
    pub item_text_tail_map: HashMap<u32, TextTail>,
    pub chat_tail_map: HashMap<u32, TextTail>,

    pub character_text_tail_list: Vec<u32>, // Store VIDs of tails currently shown
    pub item_text_tail_list: Vec<u32>,
}

impl TextTailState {
    pub fn new() -> Self {
        Self {
            character_text_tail_map: HashMap::new(),
            item_text_tail_map: HashMap::new(),
            chat_tail_map: HashMap::new(),
            character_text_tail_list: Vec::new(),
            item_text_tail_list: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.character_text_tail_map.clear();
        self.item_text_tail_map.clear();
        self.chat_tail_map.clear();
        self.character_text_tail_list.clear();
        self.item_text_tail_list.clear();
    }
}

impl Default for TextTailState {
    fn default() -> Self {
        Self::new()
    }
}

static TEXT_TAIL_STATE: OnceLock<Mutex<TextTailState>> = OnceLock::new();

pub fn get_text_tail_state() -> &'static Mutex<TextTailState> {
    TEXT_TAIL_STATE.get_or_init(|| Mutex::new(TextTailState::new()))
}

// Helper to simulate time (since we can't rely on global CTimer)
pub fn current_millisecond() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u32
}

const TEXTTAIL_LIVINGTIME: u32 = 5000;

// --- FFI Exports ---

#[no_mangle]
pub extern "C" fn m2_text_tail_arrange_text_tail() {
    safe_ffi_call((), || {
        let mut state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        // We process item tails first, matching ArrangeTextTail in PythonTextTail.cpp
        let item_vids = state.item_text_tail_list.clone();
        
        for i in 0..item_vids.len() {
            let vid_insert = item_vids[i];
            
            // We need to mutate the items, so we'll do this carefully to avoid borrow checker issues.
            // We'll extract the text tails, modify their positions, and put them back.
            
            let mut limit_count = 0;
            let y_temp = 5.0;

            loop {
                let mut collision_found = false;
                let mut compare_y = 0.0;
                let mut compare_y_end = 0.0;
                
                // Get current state of the inserting tail
                let mut insert_tail = match state.item_text_tail_map.get(&vid_insert) {
                    Some(t) => t.clone(),
                    None => break,
                };

                for j in 0..item_vids.len() {
                    let vid_compare = item_vids[j];
                    if vid_compare == vid_insert {
                        continue;
                    }

                    if let Some(compare_tail) = state.item_text_tail_map.get(&vid_compare) {
                        if insert_tail.is_in(compare_tail) {
                            collision_found = true;
                            compare_y = compare_tail.y;
                            compare_y_end = compare_tail.y_end;
                            break;
                        }
                    }
                }

                if collision_found && limit_count < 20 {
                    if let Some(t) = state.item_text_tail_map.get_mut(&vid_insert) {
                        t.y = compare_y + compare_y_end + y_temp;
                        // update insert_tail y to match the new one for next iteration
                        insert_tail.y = t.y;
                    }
                    limit_count += 1;
                } else {
                    break;
                }
            }
        }
        
        // C++ ArrangeTextTail then does title/name refactoring for CharacterTextTailList
        // which mainly adjusts positions of attached widgets. We'll update the main coordinates if chat exists.
        // The actual logic for shifting character text tails due to chat tails is in UpdateShowingTextTail
        // but we'll apply a basic update here to represent the state shift.
        let char_vids = state.character_text_tail_list.clone();
        for vid in char_vids {
            if let Some(_char_tail) = state.character_text_tail_map.get(&vid) {
                 // In C++, ArrangeTextTail mostly does Z-sorting and widget repositioning for characters.
                 // We will skip pure visual offsets since we don't have instances.
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_text_tail_show_all_text_tail() {
    safe_ffi_call((), || {
        let mut state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        // C++:
        // for each character text tail: if distance < 3500.0f -> ShowCharacterTextTail
        // for each item text tail: if distance < 3500.0f -> ShowItemTextTail
        
        let mut chars_to_show = Vec::new();
        for (vid, tail) in state.character_text_tail_map.iter() {
            if tail.f_distance_from_player < 3500.0 {
                chars_to_show.push(*vid);
            }
        }
        
        let mut items_to_show = Vec::new();
        for (vid, tail) in state.item_text_tail_map.iter() {
            if tail.f_distance_from_player < 3500.0 {
                items_to_show.push(*vid);
            }
        }

        for vid in chars_to_show {
            if !state.character_text_tail_list.contains(&vid) {
                 state.character_text_tail_list.push(vid);
            }
        }
        
        for vid in items_to_show {
            if !state.item_text_tail_list.contains(&vid) {
                 state.item_text_tail_list.push(vid);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_text_tail_hide_all_text_tail() {
    safe_ffi_call((), || {
        let mut state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        // CPythonTextTail::HideAllTextTail()
        // m_CharacterTextTailList.clear();
        // m_ItemTextTailList.clear();
        state.character_text_tail_list.clear();
        state.item_text_tail_list.clear();
    });
}

#[no_mangle]
pub extern "C" fn m2_text_tail_show_character_text_tail(virtual_id: u32) {
    safe_ffi_call((), || {
        let mut state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        if state.character_text_tail_map.contains_key(&virtual_id) {
            if !state.character_text_tail_list.contains(&virtual_id) {
                // In C++, it checks if instance exists, isn't a guild wall, and can be picked.
                // We assume these are true here if it's in the map.
                state.character_text_tail_list.push(virtual_id);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_text_tail_hide_character_text_tail(virtual_id: u32) {
    safe_ffi_call((), || {
        let mut state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        if let Some(pos) = state.character_text_tail_list.iter().position(|&x| x == virtual_id) {
            state.character_text_tail_list.remove(pos);
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_text_tail_register_chat_tail(virtual_id: u32, chat_text: *const c_char) {
    safe_ffi_call((), || {
        if chat_text.is_null() {
            return;
        }

        let c_str = unsafe { CStr::from_ptr(chat_text) };
        let text = match c_str.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return,
        };

        let mut state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        let color = Color {
            r: 1.0,
            g: 1.0,
            b: 0.0, // Yellow chat text color roughly
            a: 1.0,
        };
        
        let living_time = current_millisecond() + TEXTTAIL_LIVINGTIME;

        if let Some(tail) = state.chat_tail_map.get_mut(&virtual_id) {
            tail.text = text;
            tail.color = color;
            tail.living_time = living_time;
            tail.b_name_flag = true;
        } else {
            let mut new_tail = TextTail::new(virtual_id, text, color, 10.0);
            new_tail.living_time = living_time;
            new_tail.b_name_flag = true;
            state.chat_tail_map.insert(virtual_id, new_tail);
        }
    });
}

// Utility functions for tests to bypass pointers
pub fn test_add_character_text_tail(virtual_id: u32, text: &str, distance: f32) {
    let mut state = match get_text_tail_state().lock() {
        Ok(s) => s,
        Err(e) => e.into_inner(),
    };
    let mut tail = TextTail::new(virtual_id, text.to_string(), Color::default(), 10.0);
    tail.f_distance_from_player = distance;
    state.character_text_tail_map.insert(virtual_id, tail);
}

pub fn test_add_item_text_tail(virtual_id: u32, text: &str, distance: f32, x: f32, y: f32) {
    let mut state = match get_text_tail_state().lock() {
        Ok(s) => s,
        Err(e) => e.into_inner(),
    };
    let mut tail = TextTail::new(virtual_id, text.to_string(), Color::default(), 0.0);
    tail.f_distance_from_player = distance;
    tail.x = x;
    tail.y = y;
    state.item_text_tail_map.insert(virtual_id, tail);
}
