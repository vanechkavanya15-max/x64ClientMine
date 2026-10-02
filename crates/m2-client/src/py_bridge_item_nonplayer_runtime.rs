use std::collections::HashMap;
use std::ffi::{c_char, c_void, CString};
use std::sync::{Mutex, OnceLock};

use crate::item_proto_binary_loader::{
    TItemTable, ITEM_APPLY_MAX_NUM, ITEM_LIMIT_MAX_NUM, ITEM_NAME_MAX_LEN, ITEM_VALUES_MAX_NUM,
};
use crate::mob_proto_binary_loader::{TMobTable, CHARACTER_NAME_MAX_LEN};
use crate::py_bootstrapper::{py_init_module, PyMethodDef};
use m2_client_bridge::ffi_guard::safe_ffi_call;

// ---------------------------------------------------------
// ItemManager Runtime
// ---------------------------------------------------------

pub struct ItemManagerRuntime {
    pub items: HashMap<u32, TItemTable>,
    pub selected_vnum: u32,
}

impl ItemManagerRuntime {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            selected_vnum: 0,
        }
    }

    pub fn get_selected_item(&self) -> Option<&TItemTable> {
        self.items.get(&self.selected_vnum)
    }

    pub fn insert_item(&mut self, vnum: u32, item: TItemTable) {
        self.items.insert(vnum, item);
    }
}

static ITEM_MANAGER_RUNTIME: OnceLock<Mutex<ItemManagerRuntime>> = OnceLock::new();

pub fn get_item_manager_runtime() -> &'static Mutex<ItemManagerRuntime> {
    ITEM_MANAGER_RUNTIME.get_or_init(|| Mutex::new(ItemManagerRuntime::new()))
}

pub extern "C" fn m2_py_item_select_item(vnum: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = get_item_manager_runtime().lock().unwrap();
        if mgr.items.contains_key(&vnum) {
            mgr.selected_vnum = vnum;
            true
        } else {
            false
        }
    })
}

pub extern "C" fn m2_py_item_get_item_name(out_buf: *mut c_char, out_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || out_len == 0 {
            return false;
        }
        let mgr = get_item_manager_runtime().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            let mut name_bytes = [0u8; ITEM_NAME_MAX_LEN + 1];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(item.szLocaleName) as *const u8,
                    name_bytes.as_mut_ptr(),
                    ITEM_NAME_MAX_LEN + 1,
                );
            }
            
            let mut len = 0;
            while len < ITEM_NAME_MAX_LEN + 1 && name_bytes[len] != 0 {
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

pub extern "C" fn m2_py_item_get_item_type() -> u8 {
    safe_ffi_call(0, || {
        let mgr = get_item_manager_runtime().lock().unwrap();
        mgr.get_selected_item().map(|i| {
            let mut val_bytes = [0u8; 1];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(i.bType) as *const u8,
                    val_bytes.as_mut_ptr(),
                    1,
                );
            }
            val_bytes[0]
        }).unwrap_or(0)
    })
}

pub extern "C" fn m2_py_item_get_item_sub_type() -> u8 {
    safe_ffi_call(0, || {
        let mgr = get_item_manager_runtime().lock().unwrap();
        mgr.get_selected_item().map(|i| {
            let mut val_bytes = [0u8; 1];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(i.bSubType) as *const u8,
                    val_bytes.as_mut_ptr(),
                    1,
                );
            }
            val_bytes[0]
        }).unwrap_or(0)
    })
}

pub extern "C" fn m2_py_item_get_item_size() -> u8 {
    safe_ffi_call(1, || {
        let mgr = get_item_manager_runtime().lock().unwrap();
        mgr.get_selected_item().map(|i| {
            let mut val_bytes = [0u8; 1];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(i.bSize) as *const u8,
                    val_bytes.as_mut_ptr(),
                    1,
                );
            }
            let size = val_bytes[0];
            if size == 0 { 1 } else { size }
        }).unwrap_or(1)
    })
}

pub extern "C" fn m2_py_item_get_icon_image(out_buf: *mut c_char, out_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || out_len == 0 {
            return false;
        }
        let mgr = get_item_manager_runtime().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            let mut val_bytes = [0u8; 4];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(item.dwVnum) as *const u8,
                    val_bytes.as_mut_ptr(),
                    4,
                );
            }
            let vnum = u32::from_le_bytes(val_bytes);
            
            let icon_path = format!("icon/item/{:05}.tga", vnum);
            if let Ok(c_str) = CString::new(icon_path) {
                let bytes = c_str.as_bytes_with_nul();
                if bytes.len() <= out_len {
                    unsafe {
                        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, bytes.len());
                    }
                    return true;
                }
            }
        }
        false
    })
}

pub extern "C" fn m2_py_item_get_limit(limit_idx: usize, out_type: *mut u8, out_value: *mut i32) -> bool {
    safe_ffi_call(false, || {
        if limit_idx >= ITEM_LIMIT_MAX_NUM || out_type.is_null() || out_value.is_null() {
            return false;
        }
        let mgr = get_item_manager_runtime().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            let mut limit_bytes = [0u8; std::mem::size_of::<crate::item_proto_binary_loader::TItemLimit>()];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(item.aLimits[limit_idx]) as *const u8,
                    limit_bytes.as_mut_ptr(),
                    limit_bytes.len(),
                );
                
                *out_type = limit_bytes[0];
                let val_bytes: [u8; 4] = limit_bytes[1..5].try_into().unwrap();
                *out_value = i32::from_le_bytes(val_bytes);
            }
            return true;
        }
        false
    })
}

pub extern "C" fn m2_py_item_get_apply(apply_idx: usize, out_type: *mut u8, out_value: *mut i32) -> bool {
    safe_ffi_call(false, || {
        if apply_idx >= ITEM_APPLY_MAX_NUM || out_type.is_null() || out_value.is_null() {
            return false;
        }
        let mgr = get_item_manager_runtime().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            let mut apply_bytes = [0u8; std::mem::size_of::<crate::item_proto_binary_loader::TItemApply>()];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(item.aApplies[apply_idx]) as *const u8,
                    apply_bytes.as_mut_ptr(),
                    apply_bytes.len(),
                );
                
                *out_type = apply_bytes[0];
                let val_bytes: [u8; 4] = apply_bytes[1..5].try_into().unwrap();
                *out_value = i32::from_le_bytes(val_bytes);
            }
            return true;
        }
        false
    })
}

pub extern "C" fn m2_py_item_get_value(value_idx: usize, out_value: *mut i32) -> bool {
    safe_ffi_call(false, || {
        if value_idx >= ITEM_VALUES_MAX_NUM || out_value.is_null() {
            return false;
        }
        let mgr = get_item_manager_runtime().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            let mut val_bytes = [0u8; 4];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(item.alValues[value_idx]) as *const u8,
                    val_bytes.as_mut_ptr(),
                    4,
                );
                *out_value = i32::from_le_bytes(val_bytes);
            }
            return true;
        }
        false
    })
}

// ---------------------------------------------------------
// NonPlayerManager Runtime
// ---------------------------------------------------------

pub struct NonPlayerManagerRuntime {
    pub mobs: HashMap<u32, TMobTable>,
}

impl NonPlayerManagerRuntime {
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
}

static NONPLAYER_MANAGER_RUNTIME: OnceLock<Mutex<NonPlayerManagerRuntime>> = OnceLock::new();

pub fn get_nonplayer_manager_runtime() -> &'static Mutex<NonPlayerManagerRuntime> {
    NONPLAYER_MANAGER_RUNTIME.get_or_init(|| Mutex::new(NonPlayerManagerRuntime::new()))
}

#[no_mangle]
pub extern "C" fn m2_py_nonplayer_get_monster_name(vnum: u32, out_buf: *mut c_char, out_len: usize) -> bool {
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
pub extern "C" fn m2_py_nonplayer_get_monster_level(vnum: u32) -> u8 {
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
pub extern "C" fn m2_py_nonplayer_get_monster_max_hp(vnum: u32) -> u32 {
    safe_ffi_call(0, || {
        let mgr = get_nonplayer_manager_runtime().lock().unwrap();
        mgr.get_mob(vnum).map(|m| {
            let mut val_bytes = [0u8; 4];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(m.dwMaxHP) as *const u8,
                    val_bytes.as_mut_ptr(),
                    4,
                );
            }
            u32::from_le_bytes(val_bytes)
        }).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_nonplayer_get_grade_by_name(vnum: u32) -> u8 {
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

// ---------------------------------------------------------
// Module Initialization
// ---------------------------------------------------------

pub fn register_module_item_nonplayer_runtime() -> (*mut c_void, *mut c_void) {
    static ITEM_METHODS: &[PyMethodDef] = &[
        PyMethodDef {
            ml_name: b"SelectItem\0".as_ptr() as *const c_char,
            ml_meth: m2_py_item_select_item as *const c_void,
            ml_flags: 1,
            ml_doc: b"SelectItem(vnum) -> bool\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetItemName\0".as_ptr() as *const c_char,
            ml_meth: m2_py_item_get_item_name as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetItemName(out_buf, len) -> bool\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetItemType\0".as_ptr() as *const c_char,
            ml_meth: m2_py_item_get_item_type as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetItemType() -> int\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetItemSubType\0".as_ptr() as *const c_char,
            ml_meth: m2_py_item_get_item_sub_type as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetItemSubType() -> int\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetItemSize\0".as_ptr() as *const c_char,
            ml_meth: m2_py_item_get_item_size as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetItemSize() -> int\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetIconImage\0".as_ptr() as *const c_char,
            ml_meth: m2_py_item_get_icon_image as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetIconImage(out_buf, len) -> bool\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetLimit\0".as_ptr() as *const c_char,
            ml_meth: m2_py_item_get_limit as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetLimit(limit_idx, out_type, out_value) -> bool\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetApply\0".as_ptr() as *const c_char,
            ml_meth: m2_py_item_get_apply as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetApply(apply_idx, out_type, out_value) -> bool\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetValue\0".as_ptr() as *const c_char,
            ml_meth: m2_py_item_get_value as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetValue(value_idx, out_value) -> bool\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: std::ptr::null(),
            ml_meth: std::ptr::null(),
            ml_flags: 0,
            ml_doc: std::ptr::null(),
        },
    ];

    static NONPLAYER_METHODS: &[PyMethodDef] = &[
        PyMethodDef {
            ml_name: b"GetMonsterName\0".as_ptr() as *const c_char,
            ml_meth: m2_py_nonplayer_get_monster_name as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetMonsterName(vnum, out_buf, len) -> bool\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetMonsterLevel\0".as_ptr() as *const c_char,
            ml_meth: m2_py_nonplayer_get_monster_level as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetMonsterLevel(vnum) -> int\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetMonsterMaxHP\0".as_ptr() as *const c_char,
            ml_meth: m2_py_nonplayer_get_monster_max_hp as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetMonsterMaxHP(vnum) -> int\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: b"GetGradeByName\0".as_ptr() as *const c_char,
            ml_meth: m2_py_nonplayer_get_grade_by_name as *const c_void,
            ml_flags: 1,
            ml_doc: b"GetGradeByName(vnum) -> int\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: std::ptr::null(),
            ml_meth: std::ptr::null(),
            ml_flags: 0,
            ml_doc: std::ptr::null(),
        },
    ];

    let item_mod = unsafe { py_init_module("item", ITEM_METHODS.as_ptr()) };
    let nonplayer_mod = unsafe { py_init_module("nonplayer", NONPLAYER_METHODS.as_ptr()) };

    (item_mod, nonplayer_mod)
}
