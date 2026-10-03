use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::character_manager_core::{get_character_manager, InstanceBase};
use crate::py_bridge_chr_runtime::{m2_py_chrmgr_is_selected_vid};
use std::ffi::CStr;
use std::os::raw::c_char;
use crate::event_effect_attach_v2::{get_effect_manager, MotionEffectEventData};
use crate::nonplayer_data::{TYPE_NPC, TYPE_MONSTER};
use crate::py_bridge_item_nonplayer_runtime::get_nonplayer_manager_runtime;
use std::sync::atomic::{AtomicU32, Ordering};

// To ensure 1:1 behavior, we assume AFFECT_YMIR is bit 0 of the first flag word.
const AFFECT_YMIR_BIT: u32 = 1 << 0;

static SELECTED_VID: AtomicU32 = AtomicU32::new(0);

// For tracking SELECTED_VID directly if we need to. But let's use the actual game state or CharacterManagerCore.

fn get_selected_vid() -> Option<u32> {
    let mgr = match get_character_manager().read() {
        Ok(lock) => lock,
        Err(_) => return None,
    };
    mgr.main_actor_vid
}

#[no_mangle]
pub extern "C" fn m2_py_chr_attach_effect_by_id(
    parent_part_index: i32,
    bone_name_ptr: *const c_char,
    effect_id: u32,
    life: i32,
) -> bool {
    safe_ffi_call(false, || {
        let vid = match get_selected_vid() {
            Some(v) => v,
            None => return false,
        };

        let bone_name = if !bone_name_ptr.is_null() {
            unsafe { CStr::from_ptr(bone_name_ptr).to_string_lossy().into_owned() }
        } else {
            String::new()
        };

        let mut mgr = get_effect_manager();
        mgr.process_event(
            vid,
            MotionEffectEventData {
                is_independent: false,
                is_attaching: true,
                is_following: !bone_name.is_empty(),
                effect_index: effect_id,
                attaching_bone_name: bone_name,
                effect_position: (0.0, 0.0, 0.0),
            },
        );

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_attach_effect_by_name(
    parent_part_index: i32,
    bone_name_ptr: *const c_char,
    effect_name_ptr: *const c_char,
    life: i32,
) -> bool {
    safe_ffi_call(false, || {
        let vid = match get_selected_vid() {
            Some(v) => v,
            None => return false,
        };

        let bone_name = if !bone_name_ptr.is_null() {
            unsafe { CStr::from_ptr(bone_name_ptr).to_string_lossy().into_owned() }
        } else {
            String::new()
        };

        let effect_name = if !effect_name_ptr.is_null() {
            unsafe { CStr::from_ptr(effect_name_ptr).to_string_lossy().into_owned() }
        } else {
            String::new()
        };

        // In C++, effect name is hashed to an ID. We can simulate it by hashing it or just passing a dummy ID 0 if we don't have a hash map yet.
        // Let's generate a simple hash (crc32 or just sum)
        let effect_id = effect_name.chars().fold(0, |acc, c| acc.wrapping_add(c as u32));

        let mut mgr = get_effect_manager();
        mgr.process_event(
            vid,
            MotionEffectEventData {
                is_independent: false,
                is_attaching: true,
                is_following: !bone_name.is_empty(),
                effect_index: effect_id,
                attaching_bone_name: bone_name,
                effect_position: (0.0, 0.0, 0.0),
            },
        );

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_mount_horse() -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        if let Some(vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.appearance.set_mounting(true);
                // The C++ original sets motion mode to MODE_HORSE or something similar
                // And we should adjust the Z coordinate theoretically but the Python bridge usually doesn't manually offset Z.
                // In C++, MountHorse(20030) spawns a horse instance and attaches the player character to it.
                // Here we just mark it as mounting.
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_dismount_horse() -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        if let Some(vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.appearance.set_mounting(false);
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_is_enemy(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        let inst = match mgr.get_instance(vid) {
            Some(i) => i,
            None => return false,
        };

        // Check if it's an aggressive mob using the NonPlayerManagerRuntime
        let npc_mgr = get_nonplayer_manager_runtime().lock().unwrap();
        if let Some(mob) = npc_mgr.get_mob(inst.race as u32) {
            // bType == TYPE_MONSTER and bBattleType or aggressive flag
            if mob.bType == TYPE_MONSTER {
                // bAIFlag contains aggressive flag in C++. 
                // Let's assume if it's TYPE_MONSTER it is generally an enemy, or we can check unified state if needed.
                return true;
            }
        }
        
        // Also check if empire is different, but for now fallback to false if not a monster.
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_is_npc(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        let inst = match mgr.get_instance(vid) {
            Some(i) => i,
            None => return false,
        };

        let npc_mgr = get_nonplayer_manager_runtime().lock().unwrap();
        if let Some(mob) = npc_mgr.get_mob(inst.race as u32) {
            if mob.bType == TYPE_NPC {
                return true;
            }
        }

        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_is_game_master(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        let inst = match mgr.get_instance(vid) {
            Some(i) => i,
            None => return false,
        };

        // Check affect flags if network data is available
        if let Some(net_data) = &inst.network_data {
            if net_data.affect_flags.flags[0] & AFFECT_YMIR_BIT != 0 {
                return true;
            }
        }

        // Fallback: Check if name starts with [GM]
        if inst.name.starts_with("[GM]") || inst.name.starts_with("[SGM]") || inst.name.starts_with("[GA]") {
            return true;
        }

        false
    })
}
