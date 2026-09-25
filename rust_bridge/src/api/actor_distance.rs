use crate::ffi_guard::safe_ffi_call;
use std::ffi::c_void;

// Zwraca obliczona odleglosc aktora od gracza. Zwraca 0.0 jako stub.
#[no_mangle]
pub extern "C" fn m2_actor_get_distance(vid: *const c_void) -> f32 {
    safe_ffi_call(0.0, || {
        if vid.is_null() {
            return 0.0;
        }
        
        // Stub: Zwracamy przykladowy wynik
        0.0
    })
}
