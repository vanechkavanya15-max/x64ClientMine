use crate::ffi_guard::safe_ffi_call;
use crate::state::game_state::GLOBAL_GAME_STATE;

/// Zwraca VNUM przedmiotu w podanym oknie i slocie z centralnego stanu gry.
#[no_mangle]
pub extern "C" fn m2_player_get_item_vnum(window: u8, slot: u16) -> u32 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.get_item(window, slot).map(|item| item.vnum).unwrap_or(0)
    })
}

/// Zwraca ilosc przedmiotu w podanym oknie i slocie z centralnego stanu gry.
#[no_mangle]
pub extern "C" fn m2_player_get_item_count(window: u8, slot: u16) -> u32 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.get_item(window, slot).map(|item| item.count as u32).unwrap_or(0)
    })
}
