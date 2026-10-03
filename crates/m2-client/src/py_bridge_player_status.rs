use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, POINT_GOLD};
use crate::character_manager_core::get_character_manager;
use std::ffi::CString;

pub const POINT_EXP: u8 = 3;
pub const POINT_PLAYTIME: u8 = 31;

static LAST_PLAYER_NAME: std::sync::Mutex<Vec<u8>> = std::sync::Mutex::new(Vec::new());

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_exp() -> i64 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.get_point(POINT_EXP)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_money() -> i64 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.get_point(POINT_GOLD)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_elk() -> i64 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.get_point(POINT_GOLD)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_play_time() -> i64 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.get_point(POINT_PLAYTIME)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_job() -> u32 {
    safe_ffi_call(0, || {
        let mgr = match get_character_manager().read() {
            Ok(m) => m,
            Err(_) => return 0,
        };
        if let Some(main_inst) = mgr.get_main_instance() {
            (main_inst.race % 4) as u32
        } else {
            0
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_race() -> u32 {
    safe_ffi_call(0, || {
        let mgr = match get_character_manager().read() {
            Ok(m) => m,
            Err(_) => return 0,
        };
        if let Some(main_inst) = mgr.get_main_instance() {
            main_inst.race as u32
        } else {
            0
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_name() -> *mut std::ffi::c_char {
    safe_ffi_call(std::ptr::null_mut(), || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let vid = state.get_player_vid();
        let mut name = String::new();
        
        let mgr_lock = match get_character_manager().read() {
            Ok(lock) => Some(lock),
            Err(_) => None,
        };
        
        if let Some(mgr) = mgr_lock {
            if let Some(main_inst) = mgr.get_main_instance() {
                name = main_inst.name.clone();
            } else if let Some(actor) = state.get_actor(vid) {
                name = actor.name.clone();
            }
        } else if let Some(actor) = state.get_actor(vid) {
            name = actor.name.clone();
        }

        let mut name_bytes = name.into_bytes();
        name_bytes.push(0);
        let mut cache = LAST_PLAYER_NAME.lock().unwrap();
        *cache = name_bytes;
        cache.as_mut_ptr() as *mut std::ffi::c_char
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_character_distance(vid: u32) -> f32 {
    safe_ffi_call(0.0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.actor_distance(vid)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_main_character_index() -> u32 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.get_player_vid()
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_main_character_name() -> *mut std::ffi::c_char {
    m2_py_player_get_name()
}

