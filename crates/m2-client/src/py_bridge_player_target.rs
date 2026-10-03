use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::character_manager_core::get_character_manager;
use std::sync::{Mutex, OnceLock};

/// Global state representing the player's current target (m_dwTargetVID in C++)
#[derive(Default, Clone, Copy, Debug)]
pub struct PlayerTargetState {
    pub target_vid: u32,
}

static PLAYER_TARGET_STATE: OnceLock<Mutex<PlayerTargetState>> = OnceLock::new();

pub fn get_player_target_state() -> &'static Mutex<PlayerTargetState> {
    PLAYER_TARGET_STATE.get_or_init(|| Mutex::new(PlayerTargetState::default()))
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_set_target(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_player_target_state().lock().unwrap();
        state.target_vid = vid;
        
        // C++: SendTargetPacket etc., here we just set the UI state.
        // The actual network packet might be handled elsewhere or added if required.
        true
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_clear_target() -> bool {
    safe_ffi_call(false, || {
        let mut state = get_player_target_state().lock().unwrap();
        state.target_vid = 0;
        true
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_target_vid() -> u32 {
    safe_ffi_call(0, || {
        let state = get_player_target_state().lock().unwrap();
        state.target_vid
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_pk_mode() -> u8 {
    safe_ffi_call(0, || {
        let mgr = get_character_manager().read().unwrap();
        if let Some(main_vid) = mgr.main_actor_vid {
            if let Some(main_actor) = mgr.alive_instances.get(&main_vid) {
                if let Some(net_data) = &main_actor.network_data {
                    return net_data.pk_mode;
                }
            }
        }
        0
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_is_same_empire(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = get_character_manager().read().unwrap();
        let main_vid = match mgr.main_actor_vid {
            Some(v) => v,
            None => return false,
        };

        let main_actor = match mgr.alive_instances.get(&main_vid) {
            Some(a) => a,
            None => return false,
        };

        let target_actor = match mgr.alive_instances.get(&vid) {
            Some(a) => a,
            None => return false,
        };

        let main_empire = main_actor.network_data.as_ref().map(|d| d.empire_id).unwrap_or(0);
        let target_empire = target_actor.network_data.as_ref().map(|d| d.empire_id).unwrap_or(0);

        if target_empire == 0 || main_empire == 0 {
            return true;
        }

        main_empire == target_empire
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_is_in_safe_area() -> bool {
    safe_ffi_call(false, || {
        let mgr = get_character_manager().read().unwrap();
        if let Some(main_vid) = mgr.main_actor_vid {
            if let Some(main_actor) = mgr.alive_instances.get(&main_vid) {
                return crate::terrain_ffi_bridge_v2::m2_terrain_is_attr(
                    main_actor.x, 
                    main_actor.y, 
                    crate::terrain_attr_map::ATTR_BANPK
                );
            }
        }
        false
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_is_pvp_instance(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = get_character_manager().read().unwrap();
        let main_vid = match mgr.main_actor_vid {
            Some(v) => v,
            None => return false,
        };

        let main_actor = match mgr.alive_instances.get(&main_vid) {
            Some(a) => a,
            None => return false,
        };

        let target_actor = match mgr.alive_instances.get(&vid) {
            Some(a) => a,
            None => return false,
        };
        
        let my_guild = main_actor.network_data.as_ref().map(|d| d.guild_id).unwrap_or(0);
        let target_guild = target_actor.network_data.as_ref().map(|d| d.guild_id).unwrap_or(0);

        // Check if a duel is active
        if let Ok(mgr) = crate::pvp_gvg_manager_v2::get_pvp_gvg_manager().read() {
            if mgr.has_duel_key(main_vid, vid) {
                return true;
            }
            if my_guild != 0 && target_guild != 0 && mgr.has_gvg_key(my_guild, target_guild) {
                return true;
            }
        }
        
        false
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_set_target_vid(vid: u32) {
    m2_py_player_set_target(vid);
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_clear_target_vid() {
    m2_py_player_clear_target();
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_is_attack_target(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = match get_character_manager().read() {
            Ok(m) => m,
            Err(_) => return false,
        };
        if let Some(actor) = mgr.alive_instances.get(&vid) {
            actor.is_enemy()
        } else {
            false
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_target_hp(out_hp: *mut i32, out_max_hp: *mut i32) -> bool {
    safe_ffi_call(false, || {
        let state = get_player_target_state().lock().unwrap();
        if state.target_vid == 0 {
            return false;
        }
        if let Ok(g_state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.read() {
            if let Some(actor) = g_state.actors.get(&state.target_vid) {
                if !out_hp.is_null() && !out_max_hp.is_null() {
                    *out_hp = actor.hp;
                    *out_max_hp = actor.max_hp.max(1);
                    return true;
                }
            }
        }
        false
    })
}


