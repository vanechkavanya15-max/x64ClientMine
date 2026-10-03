use std::collections::HashMap;
use std::ffi::{c_char, CString};
use std::sync::{Mutex, OnceLock};

#[cfg(not(test))]
use crate::item_proto_binary_loader::{
    TItemTable, ITEM_APPLY_MAX_NUM, ITEM_SOCKET_MAX_NUM, ITEM_VALUES_MAX_NUM,
};
#[cfg(not(test))]
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[cfg(test)]
use crate::item_proto_binary_loader::{
    TItemTable, ITEM_APPLY_MAX_NUM, ITEM_SOCKET_MAX_NUM, ITEM_VALUES_MAX_NUM,
};
#[cfg(test)]
use m2_client_bridge::ffi_guard::safe_ffi_call;

// Required constants from other parts of the client for get_affect logic
const APPLY_ATT_SPEED: u8 = 7;
const ITEM_TYPE_WEAPON: u8 = 1;
const WEAPON_TWO_HANDED: u8 = 3;

static TWOHANDED_WEAPON_ATT_SPEED_DECREASE_VALUE: OnceLock<Mutex<i32>> = OnceLock::new();

pub fn get_twohanded_decrease_value() -> i32 {
    let m = TWOHANDED_WEAPON_ATT_SPEED_DECREASE_VALUE.get_or_init(|| Mutex::new(0));
    *m.lock().unwrap()
}

#[no_mangle]
pub extern "C" fn m2_py_item_set_twohanded_decrease_value(val: i32) {
    let m = TWOHANDED_WEAPON_ATT_SPEED_DECREASE_VALUE.get_or_init(|| Mutex::new(0));
    *m.lock().unwrap() = val;
}

pub struct ItemDetailsManager {
    selected_vnum: Option<u32>,
    items: HashMap<u32, TItemTable>,
}

impl ItemDetailsManager {
    pub fn new() -> Self {
        Self {
            selected_vnum: None,
            items: HashMap::new(),
        }
    }

    pub fn insert_item(&mut self, vnum: u32, item: TItemTable) {
        self.items.insert(vnum, item);
    }

    pub fn select_item(&mut self, vnum: u32) -> bool {
        if self.items.contains_key(&vnum) {
            self.selected_vnum = Some(vnum);
            true
        } else {
            false
        }
    }

    pub fn get_selected_item(&self) -> Option<&TItemTable> {
        self.selected_vnum.and_then(|vnum| self.items.get(&vnum))
    }
}

static ITEM_DETAILS_MANAGER: OnceLock<Mutex<ItemDetailsManager>> = OnceLock::new();

pub fn get_item_details_manager() -> &'static Mutex<ItemDetailsManager> {
    ITEM_DETAILS_MANAGER.get_or_init(|| Mutex::new(ItemDetailsManager::new()))
}

#[no_mangle]
pub extern "C" fn m2_py_item_details_select_item(vnum: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = get_item_details_manager().lock().unwrap();
        mgr.select_item(vnum)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_affect(index: usize, out_type: *mut u8, out_value: *mut i32) -> bool {
    safe_ffi_call(false, || {
        if index >= ITEM_APPLY_MAX_NUM || out_type.is_null() || out_value.is_null() {
            return false;
        }

        let mgr = get_item_details_manager().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            #[cfg(not(test))]
            let mut apply_bytes = [0u8; std::mem::size_of::<crate::item_proto_binary_loader::TItemApply>()];
            #[cfg(test)]
            let mut apply_bytes = [0u8; std::mem::size_of::<crate::item_proto_binary_loader::TItemApply>()];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(item.aApplies[index]) as *const u8,
                    apply_bytes.as_mut_ptr(),
                    apply_bytes.len(),
                );
                *out_type = apply_bytes[0];
                let val_bytes: [u8; 4] = apply_bytes[1..5].try_into().unwrap();
                let mut apply_value = i32::from_le_bytes(val_bytes);

                // Two-handed weapon attack speed penalty
                let b_type = {
                    let mut b_type_bytes = [0u8; 1];
                    std::ptr::copy_nonoverlapping(
                        std::ptr::addr_of!(item.bType) as *const u8,
                        b_type_bytes.as_mut_ptr(),
                        1,
                    );
                    b_type_bytes[0]
                };

                let b_sub_type = {
                    let mut b_sub_type_bytes = [0u8; 1];
                    std::ptr::copy_nonoverlapping(
                        std::ptr::addr_of!(item.bSubType) as *const u8,
                        b_sub_type_bytes.as_mut_ptr(),
                        1,
                    );
                    b_sub_type_bytes[0]
                };

                if *out_type == APPLY_ATT_SPEED && b_type == ITEM_TYPE_WEAPON && b_sub_type == WEAPON_TWO_HANDED {
                    apply_value -= get_twohanded_decrease_value();
                }

                *out_value = apply_value;
            }
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_socket(index: usize, out_value: *mut i32) -> bool {
    safe_ffi_call(false, || {
        if index >= ITEM_SOCKET_MAX_NUM || out_value.is_null() {
            return false;
        }

        let mgr = get_item_details_manager().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            let mut val_bytes = [0u8; 4];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(item.alSockets[index]) as *const u8,
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

#[no_mangle]
pub extern "C" fn m2_py_item_get_value(index: usize, out_value: *mut i32) -> bool {
    safe_ffi_call(false, || {
        if index >= ITEM_VALUES_MAX_NUM || out_value.is_null() {
            return false;
        }

        let mgr = get_item_details_manager().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            let mut val_bytes = [0u8; 4];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(item.alValues[index]) as *const u8,
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

#[no_mangle]
pub extern "C" fn m2_py_item_get_item_size(out_size: *mut u8) -> bool {
    safe_ffi_call(false, || {
        if out_size.is_null() {
            return false;
        }

        let mgr = get_item_details_manager().lock().unwrap();
        if let Some(item) = mgr.get_selected_item() {
            let mut val_bytes = [0u8; 1];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(item.bSize) as *const u8,
                    val_bytes.as_mut_ptr(),
                    1,
                );
                let mut size = val_bytes[0];
                if size == 0 {
                    size = 1; // Fallback
                }
                *out_size = size;
            }
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_icon_image_file_name(out_buf: *mut c_char, out_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || out_len == 0 {
            return false;
        }

        let mgr = get_item_details_manager().lock().unwrap();
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
