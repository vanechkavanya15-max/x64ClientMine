use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::ffi::CString;
use std::os::raw::c_char;
use crate::item_manager::get_item_manager;
pub use crate::item_proto_loader::ItemProto;

pub fn register_item_proto(item: ItemProto) {
    if let Ok(mut mgr) = get_item_manager().lock() {
        mgr.items.insert(item.vnum, item);
    }
}

pub const ITEM_LIMIT_MAX_NUM: usize = 2;
pub const ITEM_APPLY_MAX_NUM: usize = 3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemLimit {
    pub limit_type: u8,
    pub limit_value: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemApply {
    pub apply_type: u8,
    pub apply_value: i16,
}

impl Default for ItemLimit {
    fn default() -> Self {
        Self { limit_type: 0, limit_value: 0 }
    }
}

impl Default for ItemApply {
    fn default() -> Self {
        Self { apply_type: 0, apply_value: 0 }
    }
}

#[no_mangle]
pub extern "C" fn m2_py_item_select_item(vnum: u32) {
    safe_ffi_call((), || {
        if let Ok(mut mgr) = get_item_manager().lock() {
            mgr.selected_vnum = vnum;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_item_name(out_buf: *mut c_char, out_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || out_len == 0 {
            return false;
        }
        let mgr = get_item_manager().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            if let Ok(c_str) = CString::new(item.name.as_str()) {
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

#[no_mangle]
pub extern "C" fn m2_py_item_get_item_type() -> u8 {
    safe_ffi_call(0, || {
        let mgr = get_item_manager().lock().unwrap();
        mgr.get_selected_item().map(|i| i.item_type).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_item_sub_type() -> u8 {
    safe_ffi_call(0, || {
        let mgr = get_item_manager().lock().unwrap();
        mgr.get_selected_item().map(|i| i.item_sub_type).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_ibuy_item_price() -> i64 {
    safe_ffi_call(0, || {
        let mgr = get_item_manager().lock().unwrap();
        mgr.get_selected_item().map(|i| i.buy_price).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_isell_item_price() -> i64 {
    safe_ffi_call(0, || {
        let mgr = get_item_manager().lock().unwrap();
        mgr.get_selected_item().map(|i| i.sell_price).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_icon_image(out_buf: *mut c_char, out_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || out_len == 0 {
            return false;
        }
        let mgr = get_item_manager().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            if let Ok(c_str) = CString::new(item.icon_image.as_str()) {
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

#[no_mangle]
pub extern "C" fn m2_py_item_get_limit(limit_index: u8, out_type: *mut u8, out_value: *mut i32) -> bool {
    safe_ffi_call(false, || {
        if limit_index as usize >= ITEM_LIMIT_MAX_NUM {
            return false;
        }
        let mgr = get_item_manager().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            let limit = item.limits[limit_index as usize];
            unsafe {
                if !out_type.is_null() {
                    *out_type = limit.limit_type;
                }
                if !out_value.is_null() {
                    *out_value = limit.limit_value;
                }
            }
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_apply(apply_index: u8, out_type: *mut u8, out_value: *mut i16) -> bool {
    safe_ffi_call(false, || {
        if apply_index as usize >= ITEM_APPLY_MAX_NUM {
            return false;
        }
        let mgr = get_item_manager().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            let apply = item.applies[apply_index as usize];
            unsafe {
                if !out_type.is_null() {
                    *out_type = apply.apply_type;
                }
                if !out_value.is_null() {
                    *out_value = apply.apply_value;
                }
            }
            return true;
        }
        false
    })
}

static ITEM_DESC: &[u8] = b"Item Description\0";

#[no_mangle]
pub extern "C" fn m2_py_item_get_item_description() -> *const c_char {
    ITEM_DESC.as_ptr() as *const c_char
}

