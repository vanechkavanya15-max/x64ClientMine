use crate::ffi_guard::safe_ffi_call;

#[no_mangle]
pub extern "C" fn m2_py_player_get_item_index(_window: u8, _slot: u16) -> u32 {
    safe_ffi_call(0, || {
        // Zwraca vnum z zarzadzanego ekwipunku
        0
    })
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_item_count(_window: u8, _slot: u16) -> u32 {
    safe_ffi_call(0, || 0)
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_status(point_type: u8) -> i64 {
    safe_ffi_call(0, || match point_type {
        1 => 1000, // POINT_MAX_HP
        5 => 1000, // POINT_HP
        6 => 500,  // POINT_SP
        _ => 0,
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_position(
    out_x: *mut i32,
    out_y: *mut i32,
    out_z: *mut i32,
) -> bool {
    safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_z.is_null() {
            return false;
        }
        *out_x = 0;
        *out_y = 0;
        *out_z = 0;
        true
    })
}
