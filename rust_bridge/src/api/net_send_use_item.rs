use crate::ffi_guard::safe_ffi_call;

/// FFI Bridge dla wysylania pakietu uzycia przedmiotu.
/// 
/// Argumenty:
/// window - identyfikator okna (np. INVENTORY)
/// slot - indeks komorki w ekwipunku
#[no_mangle]
pub extern "C" fn m2_net_send_use_item(_window: u8, _slot: u16) -> bool {
    safe_ffi_call(false, || {
        // Implementacja zablokowana do kolejnej fazy integracji z C++.
        // Zwracamy true, zeby potwierdzic symulowane wyslanie pakietu.
        
        true
    })
}
