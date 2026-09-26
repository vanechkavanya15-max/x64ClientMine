use crate::ffi_guard::safe_ffi_call;
use crate::state::game_state::GLOBAL_GAME_STATE;

/// Wyszukuje VID najblizszego aktora spelniajacego kryteria typu oraz zasiegu.
/// Wywoluje find_nearest_actor na centralnym stanie gry.
#[no_mangle]
pub extern "C" fn m2_find_nearest_actor(actor_type: u32, max_distance: f32) -> u32 {
    safe_ffi_call(0, || {
        GLOBAL_GAME_STATE.read().unwrap().find_nearest_actor(actor_type, max_distance)
    })
}
