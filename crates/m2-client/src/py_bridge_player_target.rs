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

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_alignment_data(out_point: *mut i32, out_grade: *mut i32) -> bool {
    safe_ffi_call(false, || {
        if out_point.is_null() || out_grade.is_null() {
            return false;
        }
        let mgr = get_character_manager().read().unwrap();
        if let Some(main_vid) = mgr.main_actor_vid {
            if let Some(main_actor) = mgr.alive_instances.get(&main_vid) {
                let alignment = main_actor.network_data.as_ref().map(|d| d.alignment).unwrap_or(0);
                let grade = if alignment >= 12000 {
                    0
                } else if alignment >= 8000 {
                    1
                } else if alignment >= 4000 {
                    2
                } else if alignment >= 1000 {
                    3
                } else if alignment >= 0 {
                    4
                } else if alignment > -4000 {
                    5
                } else if alignment > -8000 {
                    6
                } else if alignment > -12000 {
                    7
                } else {
                    8
                };
                *out_point = alignment as i32;
                *out_grade = grade;
                return true;
            }
        }
        *out_point = 0;
        *out_grade = 4;
        true
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_guild_id() -> u32 {
    safe_ffi_call(0, || {
        let mgr = get_character_manager().read().unwrap();
        if let Some(main_vid) = mgr.main_actor_vid {
            if let Some(main_actor) = mgr.alive_instances.get(&main_vid) {
                return main_actor.network_data.as_ref().map(|d| d.guild_id).unwrap_or(0);
            }
        }
        0
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_character_guild_name(out_buf: *mut std::os::raw::c_char, max_len: usize) -> i32 {
    safe_ffi_call(-1, || {
        if out_buf.is_null() || max_len == 0 {
            return -1;
        }
        let guild_id = m2_py_player_get_guild_id();
        if guild_id == 0 {
            *out_buf = 0;
            return 0;
        }
        if let Ok(state) = crate::recv_guild_logic_v2::get_guild_state().lock() {
            if state.is_enabled && !state.name.is_empty() {
                let bytes = state.name.as_bytes();
                let copy_len = bytes.len().min(max_len - 1);
                std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const std::os::raw::c_char, out_buf, copy_len);
                *out_buf.add(copy_len) = 0;
                return copy_len as i32;
            }
        }
        *out_buf = 0;
        0
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_is_mounting_horse() -> bool {
    safe_ffi_call(false, || {
        let mgr = get_character_manager().read().unwrap();
        if let Some(main_vid) = mgr.main_actor_vid {
            crate::actor_mount_synchronizer::m2_is_actor_mounted(main_vid)
        } else {
            false
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_combo_attack() -> bool {
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

        let target_vid = get_player_target_state().lock().unwrap().target_vid;
        if target_vid != 0 {
            if let Some(target_actor) = mgr.alive_instances.get(&target_vid) {
                let mut knockback_x = 0.0f32;
                let mut knockback_y = 0.0f32;
                return crate::melee_combat_hit_resolver::m2_combat_test_melee_hit(
                    main_actor.x, main_actor.y, main_actor.z,
                    main_actor.rot_deg,
                    150.0,
                    target_vid,
                    target_actor.x, target_actor.y, target_actor.z,
                    50.0,
                    false,
                    &mut knockback_x,
                    &mut knockback_y,
                );
            }
        }
        true
    })
}



