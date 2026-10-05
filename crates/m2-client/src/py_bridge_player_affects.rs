use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::sync::RwLock;
use once_cell::sync::Lazy;

use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;
use crate::recv_exchange_logic_v2::{get_exchange_logic, EXCHANGE_ITEM_MAX_NUM, ITEM_SOCKET_SLOT_MAX_NUM, ITEM_ATTRIBUTE_SLOT_MAX_NUM};

pub use crate::game_auto_potion_manager::{m2_py_player_get_auto_potion_info, m2_py_player_set_auto_potion_info};

pub static PARTY_AFFECTS: Lazy<RwLock<HashMap<u32, [i16; 7]>>> = Lazy::new(|| RwLock::new(HashMap::new()));

#[no_mangle]
pub extern "C" fn m2_py_player_set_party_member_affects(
    vid: u32,
    a0: i16, a1: i16, a2: i16, a3: i16, a4: i16, a5: i16, a6: i16
) -> bool {
    safe_ffi_call(false, || {
        let mut map = PARTY_AFFECTS.write().unwrap();
        map.insert(vid, [a0, a1, a2, a3, a4, a5, a6]);
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_party_member_affects_by_vid(
    vid: u32,
    out_a0: *mut i16, out_a1: *mut i16, out_a2: *mut i16, out_a3: *mut i16,
    out_a4: *mut i16, out_a5: *mut i16, out_a6: *mut i16
) -> bool {
    safe_ffi_call(false, || {
        let map = PARTY_AFFECTS.read().unwrap();
        if let Some(affects) = map.get(&vid) {
            unsafe {
                if !out_a0.is_null() { *out_a0 = affects[0]; }
                if !out_a1.is_null() { *out_a1 = affects[1]; }
                if !out_a2.is_null() { *out_a2 = affects[2]; }
                if !out_a3.is_null() { *out_a3 = affects[3]; }
                if !out_a4.is_null() { *out_a4 = affects[4]; }
                if !out_a5.is_null() { *out_a5 = affects[5]; }
                if !out_a6.is_null() { *out_a6 = affects[6]; }
            }
            true
        } else {
            false
        }
    })
}

extern "C" {
    fn m2_py_item_select_item(vnum: u32) -> bool;
    fn m2_py_item_get_item_name(out_buf: *mut c_char, out_len: usize) -> bool;
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_item_link(
    window: u8,
    cell: u16,
    out_buf: *mut c_char,
    out_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || out_len == 0 {
            return false;
        }
        let state = GLOBAL_GAME_STATE.read().unwrap();
        if let Some(item) = state.get_item(window, cell) {
            let mut name_buf = [0u8; 128];
            let mut item_name = String::new();
            
            // Get item name using native item bridge
            unsafe {
                m2_py_item_select_item(item.vnum);
                if m2_py_item_get_item_name(name_buf.as_mut_ptr() as *mut c_char, name_buf.len()) {
                    if let Ok(cstr) = CStr::from_ptr(name_buf.as_ptr() as *const c_char).to_str() {
                        item_name = cstr.to_string();
                    }
                }
            }
            
            let mut itemlink = format!(
                "item:{:x}:{:x}:{:x}:{:x}:{:x}",
                item.vnum, item.flags, item.sockets[0], item.sockets[1], item.sockets[2]
            );
            
            let mut is_attr = false;
            for i in 0..ITEM_ATTRIBUTE_SLOT_MAX_NUM {
                let attr = item.attrs[i];
                if attr.0 != 0 {
                    itemlink.push_str(&format!(":{:x}:{}", attr.0, attr.1));
                    is_attr = true;
                }
            }
            
            let formatted_str = if is_attr {
                format!("|cffffc700|H{}|h[{}]|h|r", itemlink, item_name)
            } else {
                format!("|cfff1e6c0|H{}|h[{}]|h|r", itemlink, item_name)
            };
            
            let c_formatted = std::ffi::CString::new(formatted_str).unwrap();
            let bytes = c_formatted.as_bytes_with_nul();
            if bytes.len() > out_len {
                return false;
            }
            
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, bytes.len());
            }
            true
        } else {
            unsafe { *out_buf = 0; }
            true
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_exchange_item_attribute(
    owner: u8,
    pos: u8,
    attr_idx: u8,
    out_type: *mut u8,
    out_value: *mut i16,
) -> bool {
    safe_ffi_call(false, || {
        let pos = pos as usize;
        let attr_idx = attr_idx as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM || attr_idx >= ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            return false;
        }
        
        let logic = get_exchange_logic().read().unwrap();
        let data = if owner == 0 {
            &logic.self_data
        } else {
            &logic.target_data
        };
        
        let attr = data.item_attr[pos][attr_idx];
        unsafe {
            if !out_type.is_null() { *out_type = attr.b_type; }
            if !out_value.is_null() { *out_value = attr.s_value; }
        }
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_exchange_item_metin_socket(
    owner: u8,
    pos: u8,
    socket_idx: u8,
    out_socket: *mut u32,
) -> bool {
    safe_ffi_call(false, || {
        let pos = pos as usize;
        let socket_idx = socket_idx as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM || socket_idx >= ITEM_SOCKET_SLOT_MAX_NUM {
            return false;
        }
        
        let logic = get_exchange_logic().read().unwrap();
        let data = if owner == 0 {
            &logic.self_data
        } else {
            &logic.target_data
        };
        
        unsafe {
            if !out_socket.is_null() {
                *out_socket = data.item_metin[pos][socket_idx];
            }
        }
        true
    })
}
