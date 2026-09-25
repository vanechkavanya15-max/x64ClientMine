use crate::ffi_guard::safe_ffi_call;
use std::os::raw::{c_float, c_uchar};

// Funkcja C FFI do wysylania pakietu ruchu przez klienta do serwera
// Zgodnie z wymaganiami wywolanie owiniete jest w safe_ffi_call z catch_unwind
#[no_mangle]
pub extern "C" fn m2_net_send_move(_x: c_float, _y: c_float, _rot: c_float, _move_type: c_uchar) -> bool {
    safe_ffi_call(false, || {
        // W tej funkcji nie uzywamy wskaznikow jako parametrow, wiec nie musimy
        // sprawdzac czy sa nullami przed dereferencja, ale w przypadku dodania
        // takowych nalezy stosowac ptr.is_null().

        // Zwracamy na razie prawde, zeby potwierdzic poprawne wykonanie FFI.
        true
    })
}
