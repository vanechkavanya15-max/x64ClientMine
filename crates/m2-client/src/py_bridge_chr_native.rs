use crate::character_manager_core::get_character_manager;
use m2_client_bridge::ffi_guard::safe_ffi_call;

// --- chrmgr module methods ---

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_create_instance(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        // Default spawn at 0, 0, 0 with race 0
        mgr.create_instance(vid, 0, 0.0, 0.0, 0.0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_delete_instance(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        mgr.delete_instance(vid)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_select_instance(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        if mgr.get_instance(vid).is_some() {
            mgr.main_actor_vid = Some(vid);
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_is_instance(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        mgr.get_instance(vid).is_some()
    })
}

// --- chr module methods ---

#[no_mangle]
pub extern "C" fn m2_py_chr_set_race(race: u16) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        if let Some(vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.race = race;
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_set_armor(vnum: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        if let Some(vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.armor_vnum = vnum;
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_set_weapon(vnum: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        if let Some(vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.weapon_vnum = vnum;
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_set_hair(vnum: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        if let Some(vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.hair_vnum = vnum;
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_set_rotation(angle: f32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        if let Some(vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.rot_deg = angle;
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_set_pixel_position(x: f32, y: f32, z: f32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        if let Some(vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.x = x;
                inst.y = y;
                inst.z = z;
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_set_motion(motion_id: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        if let Some(vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.active_loop_motion = Some(motion_id);
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chr_push_once_motion(motion_id: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        if let Some(vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.active_once_motion = Some(motion_id);
                return true;
            }
        }
        false
    })
}