use std::collections::HashMap;
use std::ffi::c_char;
use std::sync::{Mutex, OnceLock};

use crate::mob_proto_binary_loader::{TMobTable, CHARACTER_NAME_MAX_LEN};
use m2_client_bridge::ffi_guard::safe_ffi_call;

// ---------------------------------------------------------
// NonPlayerManagerDataComplete Runtime
// ---------------------------------------------------------

pub struct NonPlayerManagerDataComplete {
    pub mobs: HashMap<u32, TMobTable>,
}

impl NonPlayerManagerDataComplete {
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

static NONPLAYER_MANAGER_RUNTIME: OnceLock<Mutex<NonPlayerManagerDataComplete>> = OnceLock::new();

pub fn get_nonplayer_manager_runtime() -> &'static Mutex<NonPlayerManagerDataComplete> {
    NONPLAYER_MANAGER_RUNTIME.get_or_init(|| Mutex::new(NonPlayerManagerDataComplete::new()))
}

// ---------------------------------------------------------
// Python FFI Methods
// ---------------------------------------------------------

#[no_mangle]
pub extern "C" fn m2_py_nonplayer_get_monster_name_complete(vnum: u32, out_buf: *mut c_char, out_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || out_len == 0 {
            return false;
        }
        let mgr = get_nonplayer_manager_runtime().lock().unwrap();
        if let Some(mob) = mgr.get_mob(vnum) {
            let mut name_bytes = [0u8; CHARACTER_NAME_MAX_LEN + 1];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(mob.szLocaleName) as *const u8,
                    name_bytes.as_mut_ptr(),
                    CHARACTER_NAME_MAX_LEN + 1,
                );
            }
            
            let mut len = 0;
            while len < CHARACTER_NAME_MAX_LEN + 1 && name_bytes[len] != 0 {
                len += 1;
            }
            
            if len < out_len {
                unsafe {
                    std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), out_buf as *mut u8, len);
                    *out_buf.add(len) = 0;
                }
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_nonplayer_get_monster_level_complete(vnum: u32) -> u8 {
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
pub extern "C" fn m2_py_nonplayer_get_monster_grade_complete(vnum: u32) -> u8 {
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
pub extern "C" fn m2_py_nonplayer_get_monster_type_complete(vnum: u32) -> u8 {
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
pub extern "C" fn m2_py_nonplayer_get_monster_color_complete(vnum: u32) -> u32 {
    safe_ffi_call(0, || {
        let mgr = get_nonplayer_manager_runtime().lock().unwrap();
        mgr.get_mob(vnum).map(|m| {
            let mut val_bytes = [0u8; 4];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(m.dwMonsterColor) as *const u8,
                    val_bytes.as_mut_ptr(),
                    4,
                );
            }
            u32::from_le_bytes(val_bytes)
        }).unwrap_or(0)
    })
}
