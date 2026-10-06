use std::collections::HashMap;
use std::ffi::c_char;
use std::sync::{Mutex, OnceLock};

use crate::mob_proto_binary_loader::{TMobTable, CHARACTER_NAME_MAX_LEN};
use m2_client_bridge::ffi_guard::safe_ffi_call;

// ---------------------------------------------------------
// NonPlayerManager Database Runtime
// ---------------------------------------------------------

pub struct NonPlayerManagerDataFull {
    pub mobs: HashMap<u32, TMobTable>,
}

impl NonPlayerManagerDataFull {
    pub fn new() -> Self {
        Self {
            mobs: HashMap::new(),
        }
    }

    pub fn insert_mob(&mut self, vnum: u32, mob: TMobTable) {
        self.mobs.insert(vnum, mob);
    }
    
    pub fn get_mob(&self, vnum: u32) -> Option<&TMobTable> {
        self.mobs.get(&vnum)
    }
    
    pub fn clear(&mut self) {
        self.mobs.clear();
    }
}

static NONPLAYER_MANAGER_RUNTIME: OnceLock<Mutex<NonPlayerManagerDataFull>> = OnceLock::new();

pub fn get_nonplayer_manager_runtime() -> &'static Mutex<NonPlayerManagerDataFull> {
    NONPLAYER_MANAGER_RUNTIME.get_or_init(|| Mutex::new(NonPlayerManagerDataFull::new()))
}

// ---------------------------------------------------------
// Python FFI Methods
// ---------------------------------------------------------

#[no_mangle]
pub extern "C" fn m2_py_nonplayer_get_monster_name_full(vnum: u32) -> *const c_char {
    safe_ffi_call(std::ptr::null(), || {
        let mgr = get_nonplayer_manager_runtime().lock().unwrap();
        if let Some(mob) = mgr.get_mob(vnum) {
            mob.szLocaleName.as_ptr() as *const c_char
        } else {
            static EMPTY_STRING: [u8; 1] = [0];
            EMPTY_STRING.as_ptr() as *const c_char
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_nonplayer_get_monster_level_full(vnum: u32) -> u8 {
    safe_ffi_call(0, || {
        let mgr = get_nonplayer_manager_runtime().lock().unwrap();
        mgr.get_mob(vnum).map(|m| {
            let mut val_bytes = [0u8; 1];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(m.bLevel) as *const u8,
                    val_bytes.as_mut_ptr(),
                    1,
                );
            }
            val_bytes[0]
        }).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_nonplayer_get_monster_grade_full(vnum: u32) -> u8 {
    safe_ffi_call(0, || {
        let mgr = get_nonplayer_manager_runtime().lock().unwrap();
        mgr.get_mob(vnum).map(|m| {
            let mut val_bytes = [0u8; 1];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(m.bRank) as *const u8,
                    val_bytes.as_mut_ptr(),
                    1,
                );
            }
            val_bytes[0]
        }).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_nonplayer_get_monster_type_full(vnum: u32) -> u8 {
    safe_ffi_call(0, || {
        let mgr = get_nonplayer_manager_runtime().lock().unwrap();
        mgr.get_mob(vnum).map(|m| {
            let mut val_bytes = [0u8; 1];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(m.bType) as *const u8,
                    val_bytes.as_mut_ptr(),
                    1,
                );
            }
            val_bytes[0]
        }).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_nonplayer_get_monster_race_flag_full(vnum: u32) -> u32 {
    safe_ffi_call(0, || {
        let mgr = get_nonplayer_manager_runtime().lock().unwrap();
        mgr.get_mob(vnum).map(|m| {
            let mut val_bytes = [0u8; 4];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(m.dwRaceFlag) as *const u8,
                    val_bytes.as_mut_ptr(),
                    4,
                );
            }
            u32::from_le_bytes(val_bytes)
        }).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_nonplayer_is_monster_aggressive_full(vnum: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = get_nonplayer_manager_runtime().lock().unwrap();
        mgr.get_mob(vnum).map(|m| {
            let mut val_bytes = [0u8; 4];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(m.dwAIFlag) as *const u8,
                    val_bytes.as_mut_ptr(),
                    4,
                );
            }
            let flags = u32::from_le_bytes(val_bytes);
            (flags & (1 << 0)) != 0
        }).unwrap_or(false)
    })
}
