//! Modul player_pos - eksportuje FFI umozliwiajace pobranie pozycji gracza
//! Uzywane do minimapy i radaru.
//! Wszystkie funkcje sa chronione przed panika.

use crate::ffi_guard::safe_ffi_call;
use crate::state::game_state::GLOBAL_GAME_STATE;

/// FFI: Pobiera pozycje i rotacje gracza z centralnego stanu gry.
/// Zwraca true jesli pomyslnie, false jesli wystapil blad (np. null pointer).
#[no_mangle]
pub extern "C" fn m2_player_get_position(out_x: *mut f32, out_y: *mut f32, out_rot: *mut f32) -> bool {
    safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_rot.is_null() {
            return false;
        }

        let state = GLOBAL_GAME_STATE.read().unwrap();
        unsafe {
            *out_x = state.player.x;
            *out_y = state.player.y;
            *out_rot = state.player.rot;
        }

        true
    })
}
