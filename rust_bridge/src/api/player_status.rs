use crate::ffi_guard::safe_ffi_call;
use crate::state::game_state::GLOBAL_GAME_STATE;

/// Pobiera wartosc punktu statusu gracza dla danego typu z centralnego stanu gry.
#[no_mangle]
pub extern "C" fn m2_player_get_status(status_type: u32) -> i64 {
    safe_ffi_call(0, || {
        if status_type > 255 {
            panic!("Nieprawidlowy typ statusu: {}", status_type);
        }
        GLOBAL_GAME_STATE.read().unwrap().get_point(status_type as u8) as i64
    })
}
