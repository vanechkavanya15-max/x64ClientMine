use crate::character_manager_core::{get_character_manager, InstanceBase};
use m2_client_bridge::ffi_guard::safe_ffi_call;

/// Pobiera referencje do aktora o danym VID i zwraca u32 VID, jesli istnieje
#[no_mangle]
pub extern "C" fn m2_chr_get_instance(vid: u32) -> u32 {
    safe_ffi_call(0, || {
        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return 0,
        };
        
        if mgr.get_instance(vid).is_some() {
            vid
        } else {
            0
        }
    })
}

/// Zaznacza instancje (ustawia jako glownego aktora w menedzerze)
#[no_mangle]
pub extern "C" fn m2_chr_select_instance(vid: u32) -> bool {
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

/// Ustawia zapetlanie danej animacji (Blend Loop Motion) dla aktora
#[no_mangle]
pub extern "C" fn m2_chr_blend_loop_motion(vid: u32, motion_id: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        
        if let Some(inst) = mgr.get_instance_mut(vid) {
            inst.active_loop_motion = Some(motion_id);
            true
        } else {
            false
        }
    })
}

/// Zwraca pozycje pixelowa instancji uzywajac wskaznikow do C
#[no_mangle]
pub extern "C" fn m2_chr_get_pixel_position(vid: u32, out_x: *mut f32, out_y: *mut f32, out_z: *mut f32) -> bool {
    safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_z.is_null() {
            return false;
        }

        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        
        if let Some(inst) = mgr.get_instance(vid) {
            unsafe {
                *out_x = inst.x;
                *out_y = inst.y;
                *out_z = inst.z;
            }
            true
        } else {
            false
        }
    })
}

/// Wypelnia bufor nazwa gracza z InstanceBase (C-String).
/// Zwraca dlugosc skopiowanego tekstu lub -1 jesli postac nie istnieje.
#[no_mangle]
pub extern "C" fn m2_chr_get_name(vid: u32, out_buffer: *mut u8, buffer_len: usize) -> i32 {
    safe_ffi_call(-1, || {
        if out_buffer.is_null() || buffer_len == 0 {
            return -1;
        }

        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return -1,
        };
        
        if let Some(inst) = mgr.get_instance(vid) {
            let name_bytes = inst.name.as_bytes();
            let copy_len = std::cmp::min(name_bytes.len(), buffer_len - 1);
            
            unsafe {
                std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), out_buffer, copy_len);
                *out_buffer.add(copy_len) = 0; // null-terminator
            }
            copy_len as i32
        } else {
            -1
        }
    })
}

/// Zwraca rase (race) po VID.
#[no_mangle]
pub extern "C" fn m2_chr_get_race(vid: u32) -> u16 {
    safe_ffi_call(0, || {
        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return 0,
        };
        
        if let Some(inst) = mgr.get_instance(vid) {
            inst.race
        } else {
            0
        }
    })
}

/// Sprawdza czy aktor jest martwy.
#[no_mangle]
pub extern "C" fn m2_chr_is_dead(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        
        if let Some(inst) = mgr.get_instance(vid) {
            inst.is_dead
        } else {
            false
        }
    })
}
