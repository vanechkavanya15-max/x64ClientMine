use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::chat_buffer_v2::CHAT_TYPE_MAX_NUM;

pub const BOARD_STATE_VIEW: i32 = 0;
pub const BOARD_STATE_EDIT: i32 = 1;
pub const BOARD_STATE_LOG: i32 = 2;

#[derive(Debug, Clone)]
pub struct ChatSet {
    pub ix: i32,
    pub iy: i32,
    pub width: i32,
    pub height: i32,
    pub step: i32,
    pub end_pos: f32,
    pub align: i32,
    pub board_state: i32,
    pub mode: Vec<bool>,
}

impl Default for ChatSet {
    fn default() -> Self {
        Self {
            ix: 0,
            iy: 0,
            width: 0,
            height: 0,
            step: 15,
            end_pos: 1.0,
            align: 0,
            board_state: BOARD_STATE_VIEW,
            mode: vec![true; CHAT_TYPE_MAX_NUM as usize],
        }
    }
}

static CHAT_SET_MAP: OnceLock<RwLock<HashMap<u32, ChatSet>>> = OnceLock::new();
static ACTIVE_CHAT_SET: OnceLock<RwLock<u32>> = OnceLock::new();

pub fn get_chat_set_map() -> &'static RwLock<HashMap<u32, ChatSet>> {
    CHAT_SET_MAP.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn get_active_chat_set() -> &'static RwLock<u32> {
    ACTIVE_CHAT_SET.get_or_init(|| RwLock::new(0))
}

#[no_mangle]
pub extern "C" fn m2_py_chat_set_chat_set(id: u32) {
    safe_ffi_call((), || {
        let mut active = get_active_chat_set().write().unwrap();
        *active = id;
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_create_chat_set(id: u32) -> u32 {
    safe_ffi_call(id, || {
        let mut map = get_chat_set_map().write().unwrap();
        map.insert(id, ChatSet::default());
        id
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_set_board_state(id: u32, state: i32) {
    safe_ffi_call((), || {
        let mut map = get_chat_set_map().write().unwrap();
        if let Some(chat_set) = map.get_mut(&id) {
            chat_set.board_state = state;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_set_position(id: u32, x: i32, y: i32) {
    safe_ffi_call((), || {
        let mut map = get_chat_set_map().write().unwrap();
        if let Some(chat_set) = map.get_mut(&id) {
            chat_set.ix = x;
            chat_set.iy = y;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_set_width(id: u32, width: i32) {
    safe_ffi_call((), || {
        let mut map = get_chat_set_map().write().unwrap();
        if let Some(chat_set) = map.get_mut(&id) {
            chat_set.width = width;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_set_height(id: u32, height: i32) {
    safe_ffi_call((), || {
        let mut map = get_chat_set_map().write().unwrap();
        if let Some(chat_set) = map.get_mut(&id) {
            chat_set.height = height;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_set_step(id: u32, step: i32) {
    safe_ffi_call((), || {
        let mut map = get_chat_set_map().write().unwrap();
        if let Some(chat_set) = map.get_mut(&id) {
            chat_set.step = step;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_set_align(id: u32, align: i32) {
    safe_ffi_call((), || {
        let mut map = get_chat_set_map().write().unwrap();
        if let Some(chat_set) = map.get_mut(&id) {
            chat_set.align = if align != 0 { 1 } else { 0 };
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_toggle_chat_mode(id: u32, mode: u8) {
    safe_ffi_call((), || {
        if mode >= CHAT_TYPE_MAX_NUM {
            return;
        }
        let mut map = get_chat_set_map().write().unwrap();
        if let Some(chat_set) = map.get_mut(&id) {
            chat_set.mode[mode as usize] = !chat_set.mode[mode as usize];
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_enable_chat_mode(id: u32, mode: u8) {
    safe_ffi_call((), || {
        if mode >= CHAT_TYPE_MAX_NUM {
            return;
        }
        let mut map = get_chat_set_map().write().unwrap();
        if let Some(chat_set) = map.get_mut(&id) {
            chat_set.mode[mode as usize] = true;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_disable_chat_mode(id: u32, mode: u8) {
    safe_ffi_call((), || {
        if mode >= CHAT_TYPE_MAX_NUM {
            return;
        }
        let mut map = get_chat_set_map().write().unwrap();
        if let Some(chat_set) = map.get_mut(&id) {
            chat_set.mode[mode as usize] = false;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chat_set_end_pos(id: u32, pos: f32) {
    safe_ffi_call((), || {
        let mut map = get_chat_set_map().write().unwrap();
        if let Some(chat_set) = map.get_mut(&id) {
            chat_set.end_pos = pos.clamp(0.0, 1.0);
        }
    })
}
