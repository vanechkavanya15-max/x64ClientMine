use std::os::raw::c_char;
use std::sync::RwLock;
use std::panic::{catch_unwind, AssertUnwindSafe};
use once_cell::sync::Lazy;

pub const M2_NET_CALLBACKS: u32 = 0;

#[repr(C)]
#[derive(Clone, Default)]
pub struct M2NetCallbacks {
    pub on_char_spawn: Option<extern "C" fn(vid: u32, x: i32, y: i32, race: u16)>,
    pub on_char_move: Option<extern "C" fn(vid: u32, x: i32, y: i32, rot: u8)>,
    pub on_chat: Option<extern "C" fn(chat_type: u8, sender_vid: u32, msg: *const c_char)>,
    pub on_item_set: Option<extern "C" fn(window: u8, cell: u16, vnum: u32, count: u8)>,
    pub on_damage: Option<extern "C" fn(victim_vid: u32, damage: i32, flag: u8)>,
}

static CALLBACKS: Lazy<RwLock<M2NetCallbacks>> = Lazy::new(|| RwLock::new(M2NetCallbacks::default()));

pub fn register_callbacks(cbs: M2NetCallbacks) {
    if let Ok(mut lock) = CALLBACKS.write() {
        *lock = cbs;
    }
}

pub fn dispatch_char_spawn(vid: u32, x: i32, y: i32, race: u16) {
    if let Ok(lock) = CALLBACKS.read() {
        if let Some(cb) = lock.on_char_spawn {
            cb(vid, x, y, race);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_set_callbacks(cbs: *const M2NetCallbacks) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        if cbs.is_null() {
            return false;
        }
        let callbacks = (*cbs).clone();
        register_callbacks(callbacks);
        true
    })).unwrap_or(false)
}
