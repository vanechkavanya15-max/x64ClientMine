use crate::bridge::ffi_guard::safe_ffi_call;
use crate::recv_exchange_logic_v2::{get_exchange_logic, EXCHANGE_ITEM_MAX_NUM, ITEM_SOCKET_SLOT_MAX_NUM, ITEM_ATTRIBUTE_SLOT_MAX_NUM, ExchangeState};
use std::sync::atomic::{AtomicBool, Ordering};

static EXCHANGE_ELK_MODE: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "C" fn m2_py_exchange_init_trading() -> bool {
    safe_ffi_call(false, || true)
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_is_trading() -> bool {
    safe_ffi_call(false, || {
        if let Ok(logic) = get_exchange_logic().read() {
            logic.state != ExchangeState::Idle
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_elk_from_self() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(logic) = get_exchange_logic().read() {
            logic.self_data.elk
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_elk_from_target() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(logic) = get_exchange_logic().read() {
            logic.target_data.elk
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_item_vnum_from_target(slot: u8) -> u32 {
    safe_ffi_call(0, || {
        let slot = slot as usize;
        if slot >= EXCHANGE_ITEM_MAX_NUM {
            return 0;
        }
        if let Ok(logic) = get_exchange_logic().read() {
            logic.target_data.item_vnum[slot]
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_item_count_from_self(slot: u8) -> u8 {
    safe_ffi_call(0, || {
        let slot = slot as usize;
        if slot >= EXCHANGE_ITEM_MAX_NUM {
            return 0;
        }
        if let Ok(logic) = get_exchange_logic().read() {
            logic.self_data.item_count[slot]
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_item_count_from_target(slot: u8) -> u8 {
    safe_ffi_call(0, || {
        let slot = slot as usize;
        if slot >= EXCHANGE_ITEM_MAX_NUM {
            return 0;
        }
        if let Ok(logic) = get_exchange_logic().read() {
            logic.target_data.item_count[slot]
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_accept_from_self() -> bool {
    safe_ffi_call(false, || {
        if let Ok(logic) = get_exchange_logic().read() {
            logic.self_data.accept != 0
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_accept_from_target() -> bool {
    safe_ffi_call(false, || {
        if let Ok(logic) = get_exchange_logic().read() {
            logic.target_data.accept != 0
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_name_from_self(out_buf: *mut std::os::raw::c_char, max_len: usize) -> i32 {
    safe_ffi_call(-1, || {
        if out_buf.is_null() || max_len == 0 {
            return -1;
        }
        if let Ok(mgr) = crate::character_manager_core::get_character_manager().read() {
            if let Some(main_vid) = mgr.main_actor_vid {
                if let Some(inst) = mgr.alive_instances.get(&main_vid) {
                    let bytes = inst.name.as_bytes();
                    let copy_len = bytes.len().min(max_len - 1);
                    unsafe {
                        std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const std::os::raw::c_char, out_buf, copy_len);
                        *out_buf.add(copy_len) = 0;
                    }
                    return copy_len as i32;
                }
            }
        }
        unsafe { *out_buf = 0; }
        0
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_name_from_target(out_buf: *mut std::os::raw::c_char, max_len: usize) -> i32 {
    safe_ffi_call(-1, || {
        if out_buf.is_null() || max_len == 0 {
            return -1;
        }
        let target_vid = if let Ok(logic) = get_exchange_logic().read() {
            logic.target_vid
        } else {
            0
        };
        if target_vid != 0 {
            if let Ok(mgr) = crate::character_manager_core::get_character_manager().read() {
                if let Some(inst) = mgr.alive_instances.get(&target_vid) {
                    let bytes = inst.name.as_bytes();
                    let copy_len = bytes.len().min(max_len - 1);
                    unsafe {
                        std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const std::os::raw::c_char, out_buf, copy_len);
                        *out_buf.add(copy_len) = 0;
                    }
                    return copy_len as i32;
                }
            }
        }
        unsafe { *out_buf = 0; }
        0
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_item_metin_from_self(slot: u8, metin_idx: u8) -> u32 {
    safe_ffi_call(0, || {
        let slot = slot as usize;
        let metin_idx = metin_idx as usize;
        if slot >= EXCHANGE_ITEM_MAX_NUM || metin_idx >= ITEM_SOCKET_SLOT_MAX_NUM {
            return 0;
        }
        if let Ok(logic) = get_exchange_logic().read() {
            logic.self_data.item_metin[slot][metin_idx]
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_item_metin_from_target(slot: u8, metin_idx: u8) -> u32 {
    safe_ffi_call(0, || {
        let slot = slot as usize;
        let metin_idx = metin_idx as usize;
        if slot >= EXCHANGE_ITEM_MAX_NUM || metin_idx >= ITEM_SOCKET_SLOT_MAX_NUM {
            return 0;
        }
        if let Ok(logic) = get_exchange_logic().read() {
            logic.target_data.item_metin[slot][metin_idx]
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_item_attr_from_self(slot: u8, attr_idx: u8, out_type: *mut u8, out_val: *mut i16) -> bool {
    safe_ffi_call(false, || {
        let slot = slot as usize;
        let attr_idx = attr_idx as usize;
        if slot >= EXCHANGE_ITEM_MAX_NUM || attr_idx >= ITEM_ATTRIBUTE_SLOT_MAX_NUM || out_type.is_null() || out_val.is_null() {
            return false;
        }
        if let Ok(logic) = get_exchange_logic().read() {
            let attr = logic.self_data.item_attr[slot][attr_idx];
            unsafe {
                *out_type = attr.b_type;
                *out_val = attr.s_value;
            }
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_item_attr_from_target(slot: u8, attr_idx: u8, out_type: *mut u8, out_val: *mut i16) -> bool {
    safe_ffi_call(false, || {
        let slot = slot as usize;
        let attr_idx = attr_idx as usize;
        if slot >= EXCHANGE_ITEM_MAX_NUM || attr_idx >= ITEM_ATTRIBUTE_SLOT_MAX_NUM || out_type.is_null() || out_val.is_null() {
            return false;
        }
        if let Ok(logic) = get_exchange_logic().read() {
            let attr = logic.target_data.item_attr[slot][attr_idx];
            unsafe {
                *out_type = attr.b_type;
                *out_val = attr.s_value;
            }
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_get_elk_mode() -> bool {
    EXCHANGE_ELK_MODE.load(Ordering::SeqCst)
}

#[no_mangle]
pub extern "C" fn m2_py_exchange_set_elk_mode(mode: bool) -> bool {
    EXCHANGE_ELK_MODE.store(mode, Ordering::SeqCst);
    true
}

