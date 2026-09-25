use crate::ffi_guard::safe_ffi_call;

/// FFI: Wysyla pakiet zadania podniesienia przedmiotu o podanym VID.
#[no_mangle]
pub extern "C" fn m2_net_send_pickup(_vid: u32) -> bool {
    safe_ffi_call(false, || {
        // Mock logiki sieciowej Rusta dla Fazy A.
        true
    })
}
