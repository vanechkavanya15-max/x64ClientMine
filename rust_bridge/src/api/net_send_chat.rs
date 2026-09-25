use crate::ffi_guard::safe_ffi_call;
use std::os::raw::c_char;
use std::ffi::CStr;

/// Funkcja C FFI obslugujaca wysylanie wiadomosci czatu.
///
/// chat_type: Typ czatu (BYTE w C++, tutaj u8)
/// msg_ptr: Wskaznik na string w stylu C
/// Zwraca true, jesli operacja sie powiedzie lub gdy parametry sa poprawne,
/// a false, jezeli wskaznik jest nullem (lub w przypadku paniki).
#[no_mangle]
pub extern "C" fn m2_net_send_chat(_chat_type: u8, msg_ptr: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if msg_ptr.is_null() {
            return false;
        }

        // Bezpieczne utworzenie odniesienia do C-stringa.
        // CStr::from_ptr moze spowinodowac UB, jezeli msg_ptr nie jest prawidlowym,
        // zakonczonym zerem C-stringiem, ale w FFI to standardowy sposob.
        let _c_str = unsafe { CStr::from_ptr(msg_ptr) };

        // Na chwile obecna symulujemy tylko weryfikacje poprawnosci wskaznika
        // i prawidlowe odebranie parametrow. Wlasciwa logika moze
        // zostac dopisana w przyszlosci.

        true
    })
}
