// chrmgr_registration_bridge.rs - Rejestracja danych postaci, animacji, kosci, efektow i kolorow
// Zgodnosc z CPythonCharacterManager oraz CRaceManager w Metin2 C++.
// Obsluguje wywolania z playersettingmodule.py eliminujac atrapy (stubs).
// Komentarze w jezyku polskim bez polskich znakow diakrytycznych.

use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Default)]
pub struct RaceDefinition {
    pub race_id: u32,
    pub race_name: String,
    pub src_name: String,
    pub attaching_bones: HashMap<u32, String>,
    pub motion_modes: Vec<u32>,
    pub motion_data: HashMap<(u32, u32), String>, // (mode, motion) -> path
    pub motion_weights: HashMap<(u32, u32), u32>,
    pub combo_attack: HashMap<(u32, u32), u32>,   // (mode, combo) -> count
}

#[derive(Debug, Clone, Default)]
pub struct EffectRegistration {
    pub effect_type: u32,
    pub bone_name: String,
    pub file_path: String,
    pub is_cached: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CharacterManagerSettings {
    pub races: HashMap<u32, RaceDefinition>,
    pub race_name_to_id: HashMap<String, u32>,
    pub effects: HashMap<u32, EffectRegistration>,
    pub dust_gap: u32,
    pub horse_dust_gap: u32,
    pub title_names: HashMap<u32, String>,
    pub name_colors: HashMap<u32, [u8; 3]>,
    pub title_colors: HashMap<u32, [u8; 3]>,
}

static CHRMGR_SETTINGS: OnceLock<Mutex<CharacterManagerSettings>> = OnceLock::new();

pub fn get_chrmgr_settings() -> &'static Mutex<CharacterManagerSettings> {
    CHRMGR_SETTINGS.get_or_init(|| Mutex::new(CharacterManagerSettings::default()))
}

// ============================================================================
// Eksporty C-ABI (FFI) dla py_master_runtime_bridge i playersettingmodule.py
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_attaching_bone_name(race: u32, part: u32, bone: *const c_char) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if bone.is_null() {
            return false;
        }
        let bone_str = unsafe { CStr::from_ptr(bone) }.to_string_lossy().to_string();
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            let race_def = settings.races.entry(race).or_insert_with(|| RaceDefinition {
                race_id: race,
                ..Default::default()
            });
            race_def.attaching_bones.insert(part, bone_str);
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_motion_mode(race: u32, mode: u32) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            let race_def = settings.races.entry(race).or_insert_with(|| RaceDefinition {
                race_id: race,
                ..Default::default()
            });
            if !race_def.motion_modes.contains(&mode) {
                race_def.motion_modes.push(mode);
            }
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_set_motion_random_weight(race: u32, mode: u32, motion: u32, weight: u32) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            let race_def = settings.races.entry(race).or_insert_with(|| RaceDefinition {
                race_id: race,
                ..Default::default()
            });
            race_def.motion_weights.insert((mode, motion), weight);
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_normal_attack(race: u32, mode: u32, count: u32) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            let race_def = settings.races.entry(race).or_insert_with(|| RaceDefinition {
                race_id: race,
                ..Default::default()
            });
            race_def.combo_attack.insert((mode, 0), count);
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_combo_attack_new(race: u32, mode: u32, combo: u32, count: u32) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            let race_def = settings.races.entry(race).or_insert_with(|| RaceDefinition {
                race_id: race,
                ..Default::default()
            });
            race_def.combo_attack.insert((mode, combo), count);
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_motion_data(race: u32, mode: u32, motion: u32, path: *const c_char) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if path.is_null() {
            return false;
        }
        let path_str = unsafe { CStr::from_ptr(path) }.to_string_lossy().to_string();
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            let race_def = settings.races.entry(race).or_insert_with(|| RaceDefinition {
                race_id: race,
                ..Default::default()
            });
            race_def.motion_data.insert((mode, motion), path_str);
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_race_name(race: u32, name: *const c_char) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let name_str = unsafe { CStr::from_ptr(name) }.to_string_lossy().to_string();
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            settings.race_name_to_id.insert(name_str.clone(), race);
            let race_def = settings.races.entry(race).or_insert_with(|| RaceDefinition {
                race_id: race,
                ..Default::default()
            });
            race_def.race_name = name_str;
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_race_src_name(name: *const c_char, src: *const c_char) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if name.is_null() || src.is_null() {
            return false;
        }
        let name_str = unsafe { CStr::from_ptr(name) }.to_string_lossy().to_string();
        let src_str = unsafe { CStr::from_ptr(src) }.to_string_lossy().to_string();
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            if let Some(&race_id) = settings.race_name_to_id.get(&name_str) {
                if let Some(race_def) = settings.races.get_mut(&race_id) {
                    race_def.src_name = src_str;
                    return true;
                }
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_effect(etype: u32, bone: *const c_char, file: *const c_char) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        let bone_str = if !bone.is_null() {
            unsafe { CStr::from_ptr(bone) }.to_string_lossy().to_string()
        } else {
            String::new()
        };
        let file_str = if !file.is_null() {
            unsafe { CStr::from_ptr(file) }.to_string_lossy().to_string()
        } else {
            String::new()
        };
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            settings.effects.insert(etype, EffectRegistration {
                effect_type: etype,
                bone_name: bone_str,
                file_path: file_str,
                is_cached: false,
            });
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_cache_effect(etype: u32, bone: *const c_char, file: *const c_char) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        let bone_str = if !bone.is_null() {
            unsafe { CStr::from_ptr(bone) }.to_string_lossy().to_string()
        } else {
            String::new()
        };
        let file_str = if !file.is_null() {
            unsafe { CStr::from_ptr(file) }.to_string_lossy().to_string()
        } else {
            String::new()
        };
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            settings.effects.insert(etype, EffectRegistration {
                effect_type: etype,
                bone_name: bone_str,
                file_path: file_str,
                is_cached: true,
            });
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_set_dust_gap(gap: u32) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            settings.dust_gap = gap;
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_set_horse_dust_gap(gap: u32) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            settings.horse_dust_gap = gap;
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_title_name(index: u32, name: *const c_char) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let name_str = unsafe { CStr::from_ptr(name) }.to_string_lossy().to_string();
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            settings.title_names.insert(index, name_str);
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_name_color(index: u32, r: u8, g: u8, b: u8) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            settings.name_colors.insert(index, [r, g, b]);
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_chrmgr_register_title_color(index: u32, r: u8, g: u8, b: u8) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if let Ok(mut settings) = get_chrmgr_settings().lock() {
            settings.title_colors.insert(index, [r, g, b]);
            return true;
        }
        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_chrmgr_register_attaching_bone() {
        let bone = CString::new("Bip01 R Hand").unwrap();
        assert!(m2_py_chrmgr_register_attaching_bone_name(0, 1, bone.as_ptr()));

        let settings = get_chrmgr_settings().lock().unwrap();
        let race_def = settings.races.get(&0).unwrap();
        assert_eq!(race_def.attaching_bones.get(&1).unwrap(), "Bip01 R Hand");
    }

    #[test]
    fn test_chrmgr_register_effect_and_dust() {
        let bone = CString::new("Bip01 Foot").unwrap();
        let file = CString::new("d:/ymir work/effect/dust.mse").unwrap();
        assert!(m2_py_chrmgr_register_effect(10, bone.as_ptr(), file.as_ptr()));
        assert!(m2_py_chrmgr_set_dust_gap(30));

        let settings = get_chrmgr_settings().lock().unwrap();
        assert_eq!(settings.dust_gap, 30);
        let eff = settings.effects.get(&10).unwrap();
        assert_eq!(eff.file_path, "d:/ymir work/effect/dust.mse");
    }
}
