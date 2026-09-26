use crate::ffi_guard::safe_ffi_call;
use crate::state::game_state::GLOBAL_GAME_STATE;
use std::ffi::c_void;

/// Oblicza odleglosc aktora od gracza na podstawie identyfikatora VID.
#[no_mangle]
pub extern "C" fn m2_actor_get_distance(vid: *const c_void) -> f32 {
    safe_ffi_call(0.0, || {
        if vid.is_null() {
            return 0.0;
        }

        let vid_u32 = vid as usize as u32;
        GLOBAL_GAME_STATE.read().unwrap().actor_distance(vid_u32)
    })
}
