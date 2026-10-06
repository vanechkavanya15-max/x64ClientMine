// To make it compile both when included in `lib.rs` (where it would be `crate::character_manager_core`)
// and when included via `#[path]` in tests (where it's `m2_client::character_manager_core`),
// we can use `m2_client::character_manager_core` explicitly, or just `use crate::...`
// Wait, if it's included via #[path] in the test crate, it acts as if the file is part of the test crate.
// The test crate depends on `m2_client`. So `m2_client::character_manager_core` works.
// However, if the user or lib.rs later includes it, it will fail if it's not registered properly or if it expects `crate::`.
// To be safe for both:
use crate::character_manager_core::get_character_manager;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// Map for tracking visibility state of actors
static VISIBILITY_MAP: OnceLock<RwLock<HashMap<u32, bool>>> = OnceLock::new();

/// Map for tracking point effects on actors
static POINT_MAP: OnceLock<RwLock<HashMap<u32, (f32, f32)>>> = OnceLock::new();

fn get_visibility_map() -> &'static RwLock<HashMap<u32, bool>> {
    VISIBILITY_MAP.get_or_init(|| RwLock::new(HashMap::new()))
}

fn get_point_map() -> &'static RwLock<HashMap<u32, (f32, f32)>> {
    POINT_MAP.get_or_init(|| RwLock::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_create_character(vid: u32) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut lock) = get_character_manager().write() {
            lock.create_instance(vid, 0, 0.0, 0.0, 0.0)
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_destroy_character(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let deleted = if let Ok(mut lock) = get_character_manager().write() {
            lock.delete_instance(vid)
        } else {
            false
        };
        
        if deleted {
            if let Ok(mut vis_lock) = get_visibility_map().write() {
                vis_lock.remove(&vid);
            }
            if let Ok(mut pt_lock) = get_point_map().write() {
                pt_lock.remove(&vid);
            }
        }
        deleted
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_select_character(vid: u32) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut lock) = get_character_manager().write() {
            if lock.alive_instances.contains_key(&vid) {
                lock.main_actor_vid = Some(vid);
                return true;
            }
            false
        } else {
            false
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chrmgr_get_character_position(
    vid: u32,
    out_x: *mut f32,
    out_y: *mut f32,
    out_z: *mut f32,
) -> bool {
    safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_z.is_null() {
            return false;
        }

        if let Ok(lock) = get_character_manager().read() {
            if let Some(inst) = lock.get_instance(vid) {
                unsafe {
                    *out_x = inst.x;
                    *out_y = inst.y;
                    *out_z = inst.z;
                }
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_show_character(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let exists = get_character_manager()
            .read()
            .map(|mgr| mgr.alive_instances.contains_key(&vid))
            .unwrap_or(false);
            
        if exists {
            if let Ok(mut lock) = get_visibility_map().write() {
                lock.insert(vid, true);
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_hide_character(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let exists = get_character_manager()
            .read()
            .map(|mgr| mgr.alive_instances.contains_key(&vid))
            .unwrap_or(false);
            
        if exists {
            if let Ok(mut lock) = get_visibility_map().write() {
                lock.insert(vid, false);
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_point(vid: u32, x: f32, y: f32) -> bool {
    safe_ffi_call(false, || {
        let exists = get_character_manager()
            .read()
            .map(|mgr| mgr.alive_instances.contains_key(&vid))
            .unwrap_or(false);
            
        if exists {
            if let Ok(mut lock) = get_point_map().write() {
                lock.insert(vid, (x, y));
                return true;
            }
        }
        false
    })
}
