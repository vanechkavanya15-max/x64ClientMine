use crate::bridge::ffi_guard::safe_ffi_call;
use crate::recv_exchange_logic_v2::{get_exchange_logic, EXCHANGE_ITEM_MAX_NUM};

#[no_mangle]
pub extern "C" fn m2_py_exchange_init_trading() -> bool {
    safe_ffi_call(false, || true)
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
pub extern "C" fn m2_py_exchange_get_accept_from_target() -> bool {
    safe_ffi_call(false, || {
        if let Ok(logic) = get_exchange_logic().read() {
            logic.target_data.accept != 0
        } else {
            false
        }
    })
}
