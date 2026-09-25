use crate::ffi_guard::safe_ffi_call;

/// Zwraca VNUM przedmiotu w podanym slocie i oknie.
/// Zgodnie z wymaganiami funkcji FFI, chronimy przed panika przez catch_unwind.
#[no_mangle]
pub extern "C" fn m2_player_get_item_vnum(_window: u8, _slot: u16) -> u32 {
    safe_ffi_call(0, || {
        // Implementacja zalezy od integracji ze stanem gracza.
        // Obecnie zwraca 0.
        0
    })
}

/// Zwraca ilosc przedmiotu w podanym slocie i oknie.
/// Zgodnie z wymaganiami funkcji FFI, chronimy przed panika przez catch_unwind.
#[no_mangle]
pub extern "C" fn m2_player_get_item_count(_window: u8, _slot: u16) -> u32 {
    safe_ffi_call(0, || {
        // Implementacja zalezy od integracji ze stanem gracza.
        // Obecnie zwraca 0.
        0
    })
}
