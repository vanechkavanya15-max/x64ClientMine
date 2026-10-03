use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::state::game_state::{Actor, GLOBAL_GAME_STATE};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicU32, Ordering};

// Zmienna przechowujaca identyfikator zaznaczonego aktora (VID).
static SELECTED_VID: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_create_instance_legacy(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        let mut actor = Actor::default();
        actor.vid = vid;
        state.add_or_update_actor(actor);
        true
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_select_instance(vid: u32) -> bool {
    safe_ffi_call(false, || {
        SELECTED_VID.store(vid, Ordering::SeqCst);
        true
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_delete_instance_legacy(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        state.remove_actor(vid);
        true
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_set_instance_type(instance_type: u32) -> bool {
    safe_ffi_call(false, || {
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        if let Some(actor) = state.actors.get_mut(&vid) {
            actor.b_type = instance_type as u8;
            return true;
        }
        false
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_rt_set_pixel_position(x: i32, y: i32, z: i32) -> bool {
    safe_ffi_call(false, || {
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        if let Some(actor) = state.actors.get_mut(&vid) {
            actor.x = x as f32;
            actor.y = y as f32;
            actor.z = z as f32;
            return true;
        }
        false
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_rt_set_rotation(angle: f32) -> bool {
    safe_ffi_call(false, || {
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        if let Some(actor) = state.actors.get_mut(&vid) {
            actor.rot = angle;
            return true;
        }
        false
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_set_motion_mode_legacy(mode: i32) -> bool {
    safe_ffi_call(false, || {
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        if let Some(actor) = state.actors.get_mut(&vid) {
            actor.motion_mode = mode;
            return true;
        }
        false
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_rt_push_once_motion(motion_idx: i32) -> bool {
    safe_ffi_call(false, || {
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        if let Some(actor) = state.actors.get_mut(&vid) {
            actor.current_motion = motion_idx;
            return true;
        }
        false
    })
}

// Global variable to keep the last name requested by `m2_py_chr_get_name` alive.
// Real clients would use the string inside the instance base, but returning a pointer
// into the RwLock protected state directly is undefined behavior (the pointer becomes invalid
// once the read lock drops). So we cache the last name globally.
static mut LAST_NAME: Vec<u8> = Vec::new();

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_get_name() -> *const c_char {
    safe_ffi_call(std::ptr::null(), || {
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        let state = GLOBAL_GAME_STATE.read().unwrap();
        if let Some(actor) = state.get_actor(vid) {
            // Need to drop the lock before returning a pointer, so we copy the string to a static buffer.
            let mut name_bytes = actor.name.as_bytes().to_vec();
            name_bytes.push(0); // null terminator
            LAST_NAME = name_bytes;
            return LAST_NAME.as_ptr() as *const c_char;
        }
        
        LAST_NAME = vec![b'N', b'o', b'n', b'e', 0];
        LAST_NAME.as_ptr() as *const c_char
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_is_dead() -> bool {
    safe_ffi_call(false, || {
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        let state = GLOBAL_GAME_STATE.read().unwrap();
        if let Some(actor) = state.get_actor(vid) {
            return actor.is_dead;
        }
        false
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chrmgr_get_vid_by_name(name_ptr: *const c_char) -> u32 {
    safe_ffi_call(0, || {
        if name_ptr.is_null() {
            return 0;
        }
        
        let c_str = CStr::from_ptr(name_ptr);
        let name = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        };

        let state = GLOBAL_GAME_STATE.read().unwrap();
        for actor in state.actors.values() {
            if actor.name == name {
                return actor.vid;
            }
        }
        0
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chrmgr_is_selected_vid(vid: u32) -> bool {
    safe_ffi_call(false, || {
        SELECTED_VID.load(Ordering::SeqCst) == vid
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_get_instance_type(vid: u32) -> u32 {
    safe_ffi_call(0, || {
        let target_vid = if vid != 0 { vid } else { SELECTED_VID.load(Ordering::SeqCst) };
        let state = GLOBAL_GAME_STATE.read().unwrap();
        if let Some(actor) = state.get_actor(target_vid) {
            actor.b_type as u32
        } else {
            0
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_get_name_by_vid(vid: u32) -> *const c_char {
    safe_ffi_call(std::ptr::null(), || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        if let Some(actor) = state.get_actor(vid) {
            let mut name_bytes = actor.name.as_bytes().to_vec();
            name_bytes.push(0);
            LAST_NAME = name_bytes;
            return LAST_NAME.as_ptr() as *const c_char;
        }
        LAST_NAME = vec![b'N', b'o', b'n', b'e', 0];
        LAST_NAME.as_ptr() as *const c_char
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_chr_get_pixel_position(
    vid: u32,
    out_x: *mut f32,
    out_y: *mut f32,
    out_z: *mut f32,
) -> bool {
    crate::py_bridge_chr::m2_chr_get_pixel_position(vid, out_x, out_y, out_z)
}

