use std::sync::atomic::{AtomicU32, Ordering};
use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::character_manager_core::get_character_manager;
use crate::instance_fading_queue::m2_chr_delete_by_fade;

/// Store the selected instance VID globally in this module.
pub static SELECTED_INSTANCE_VID: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn m2_py_chr_create_instance(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        
        let race = 0;
        let x = 0.0;
        let y = 0.0;
        let z = 0.0;
        
        mgr.create_instance(vid, race, x, y, z)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_delete_instance(vid: u32) -> bool {
    safe_ffi_call(false, || {
        if SELECTED_INSTANCE_VID.load(Ordering::SeqCst) == vid {
            SELECTED_INSTANCE_VID.store(0, Ordering::SeqCst);
        }

        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        mgr.delete_instance(vid)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_delete_instance_by_fade(vid: u32) -> bool {
    safe_ffi_call(false, || {
        m2_chr_delete_by_fade(vid)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_select(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        
        if mgr.get_instance(vid).is_some() {
            SELECTED_INSTANCE_VID.store(vid, Ordering::SeqCst);
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_unselect() -> bool {
    safe_ffi_call(false, || {
        let vid = SELECTED_INSTANCE_VID.load(Ordering::SeqCst);
        if vid != 0 {
            SELECTED_INSTANCE_VID.store(0, Ordering::SeqCst);
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_destroy() -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        mgr.clear();
        SELECTED_INSTANCE_VID.store(0, Ordering::SeqCst);
        true
    })
}
