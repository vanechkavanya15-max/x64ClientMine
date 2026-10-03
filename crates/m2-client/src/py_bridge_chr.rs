use crate::character_manager_core::{get_character_manager, InstanceBase};
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{OnceLock, RwLock};

static PICKED_VID: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, Default)]
pub struct RaceData {
    pub race_id: u32,
    pub path: String,
    pub loaded: bool,
}

static RACE_DATA_REGISTRY: OnceLock<RwLock<HashMap<u32, RaceData>>> = OnceLock::new();

pub fn get_race_registry() -> &'static RwLock<HashMap<u32, RaceData>> {
    RACE_DATA_REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

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

/// Ustawia identyfikator kliknietego aktora (VID).
#[no_mangle]
pub extern "C" fn m2_py_chr_set_picked_vid(vid: u32) {
    PICKED_VID.store(vid, Ordering::SeqCst);
}

/// Pobiera identyfikator kliknietego aktora (VID).
#[no_mangle]
pub extern "C" fn m2_py_chr_pick() -> u32 {
    safe_ffi_call(0, || {
        let picked = PICKED_VID.load(Ordering::SeqCst);
        if picked != 0 {
            return picked;
        }
        if let Ok(mgr) = get_character_manager().read() {
            if let Some(main_vid) = mgr.main_actor_vid {
                return main_vid;
            }
        }
        0
    })
}

/// Rejestruje nowa rase w menedzerze ras postaci (chrmgr.CreateRace).
#[no_mangle]
pub extern "C" fn m2_py_chrmgr_create_race(race_id: u32) -> bool {
    safe_ffi_call(false, || {
        let mut reg = match get_race_registry().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        reg.entry(race_id).or_insert(RaceData {
            race_id,
            path: String::new(),
            loaded: false,
        });
        true
    })
}

/// Laduje sciezke i dane dla zarejestrowanej rasy (chrmgr.LoadRaceData).
#[no_mangle]
pub extern "C" fn m2_py_chrmgr_load_race_data(race_id: u32, path: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if path.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(path) };
        let path_str = match c_str.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };

        let mut reg = match get_race_registry().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        reg.insert(race_id, RaceData {
            race_id,
            path: path_str,
            loaded: true,
        });
        true
    })
}
