use crate::item_manager::get_item_manager;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::ffi::CStr;
use std::os::raw::c_char;
use crate::py_bridge_item::ITEM_LIMIT_MAX_NUM;

#[no_mangle]
pub extern "C" fn m2_py_item_get_limit_type(limit_index: u8) -> u8 {
    safe_ffi_call(0, || {
        if limit_index as usize >= ITEM_LIMIT_MAX_NUM {
            return 0;
        }
        let mgr = get_item_manager().lock().unwrap();
        mgr.get_selected_item().map(|i| i.limits[limit_index as usize].limit_type).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_limit_value(limit_index: u8) -> i32 {
    safe_ffi_call(0, || {
        if limit_index as usize >= ITEM_LIMIT_MAX_NUM {
            return 0;
        }
        let mgr = get_item_manager().lock().unwrap();
        mgr.get_selected_item().map(|i| i.limits[limit_index as usize].limit_value).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_is_wearable_flag(flag: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = get_item_manager().lock().unwrap();
        mgr.get_selected_item().map(|i| (i.wear_flags & flag) != 0).unwrap_or(false)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_set_use_sound_file_name(item_type: u32, file_name: *const c_char) {
    safe_ffi_call((), || {
        if file_name.is_null() {
            return;
        }
        let c_str = unsafe { CStr::from_ptr(file_name) };
        if let Ok(s) = c_str.to_str() {
            if item_type < 8 { // USESOUND_NUM in C++ is 8 (USESOUND_NONE=0...USESOUND_PORTAL=7, USESOUND_NUM=8)
                let mut mgr = get_item_manager().lock().unwrap();
                mgr.use_sound_filenames.insert(item_type, s.to_string());
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_item_set_drop_sound_file_name(item_type: u32, file_name: *const c_char) {
    safe_ffi_call((), || {
        if file_name.is_null() {
            return;
        }
        let c_str = unsafe { CStr::from_ptr(file_name) };
        if let Ok(s) = c_str.to_str() {
            if item_type < 5 { // DROPSOUND_NUM in C++ is 5 (DROPSOUND_DEFAULT=0...DROPSOUND_ACCESSORY=4, DROPSOUND_NUM=5)
                let mut mgr = get_item_manager().lock().unwrap();
                mgr.drop_sound_filenames.insert(item_type, s.to_string());
            }
        }
    });
}
