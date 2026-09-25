//! Modul player_pos - eksportuje FFI umozliwiajace pobranie pozycji gracza
//! Uzywane do minimapy i radaru.
//! Wszystkie funkcje sa chronione przed panika.

use crate::ffi_guard::safe_ffi_call;

/// FFI: Pobiera pozycje i rotacje gracza.
/// Zwraca true jesli pomyslnie, false jesli wystapil blad (np. null pointer).
#[no_mangle]
pub extern "C" fn m2_player_get_position(out_x: *mut f32, out_y: *mut f32, out_rot: *mut f32) -> bool {
    safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_rot.is_null() {
            return false;
        }

        // Faza A: Zwracamy przykladowe zera jako pozycje bazowa przed integracja silnika.
        unsafe {
            *out_x = 0.0;
            *out_y = 0.0;
            *out_rot = 0.0;
        }

        true
    })
}
