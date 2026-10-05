

use m2_client_bridge::ffi_guard::safe_ffi_call;

use crate::py_bridge_texttail_runtime_complete::{get_text_tail_state, Color};

#[no_mangle]
pub extern "C" fn m2_text_tail_get_position(
    virtual_id: u32,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
) -> bool {
    safe_ffi_call(false, || {
        let state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        if let Some(tail) = state.character_text_tail_map.get(&virtual_id) {
            unsafe {
                if !x.is_null() {
                    *x = tail.x;
                }
                if !y.is_null() {
                    *y = tail.y;
                }
                if !z.is_null() {
                    *z = tail.z;
                }
            }
            return true;
        }

        false
    })
}

#[no_mangle]
pub extern "C" fn m2_text_tail_show_item_text_tail(virtual_id: u32) {
    safe_ffi_call((), || {
        let mut state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        if state.item_text_tail_map.contains_key(&virtual_id) {
            if !state.item_text_tail_list.contains(&virtual_id) {
                state.item_text_tail_list.push(virtual_id);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_text_tail_select_item_name(virtual_id: u32) {
    safe_ffi_call((), || {
        let mut state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        if let Some(tail) = state.item_text_tail_map.get_mut(&virtual_id) {
            tail.color = Color {
                r: 0.1,
                g: 0.9,
                b: 0.1,
                a: 1.0,
            };
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_text_tail_is_chat(virtual_id: u32) -> bool {
    safe_ffi_call(false, || {
        let state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        state.chat_tail_map.contains_key(&virtual_id)
    })
}

#[no_mangle]
pub extern "C" fn m2_text_tail_clear() {
    safe_ffi_call((), || {
        let mut state = match get_text_tail_state().lock() {
            Ok(s) => s,
            Err(e) => e.into_inner(),
        };

        state.clear();
    });
}
