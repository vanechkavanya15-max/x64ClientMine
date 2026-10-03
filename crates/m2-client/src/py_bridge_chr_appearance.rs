use m2_client::character_manager_core::get_character_manager;
use m2_client_bridge::ffi_guard::safe_ffi_call;

/// Ustawia rase (race) glownego aktora. Odpowiada funkcji `chrSetRace` w Pythonie.
#[no_mangle]
pub extern "C" fn m2_py_chr_set_race(race: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        if let Some(main_vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(main_vid) {
                inst.race = race as u16;
                return true;
            }
        }
        false
    })
}

/// Ustawia pancerz glownego aktora. Odpowiada funkcji `chrSetArmor` w Pythonie.
#[no_mangle]
pub extern "C" fn m2_py_chr_set_armor(armor: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        if let Some(main_vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(main_vid) {
                inst.appearance.change_armor(armor);
                return true;
            }
        }
        false
    })
}

/// Zmienia pancerz glownego aktora. Odpowiada funkcji `chrChangeShape` w Pythonie.
/// W silniku C++ `chrChangeShape` wywoluje `ChangeArmor`.
#[no_mangle]
pub extern "C" fn m2_py_chr_change_shape(armor: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        if let Some(main_vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(main_vid) {
                inst.appearance.change_armor(armor);
                return true;
            }
        }
        false
    })
}

/// Ustawia bron glownego aktora. Odpowiada funkcji `chrSetWeapon` w Pythonie.
#[no_mangle]
pub extern "C" fn m2_py_chr_set_weapon(weapon: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        if let Some(main_vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(main_vid) {
                inst.appearance.attach_weapon(weapon);
                return true;
            }
        }
        false
    })
}

/// Ustawia fryzure glownego aktora. Odpowiada funkcji `chrSetHair` w Pythonie.
#[no_mangle]
pub extern "C" fn m2_py_chr_set_hair(hair: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        if let Some(main_vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(main_vid) {
                inst.appearance.set_hair(hair);
                return true;
            }
        }
        false
    })
}

/// Zmienia fryzure glownego aktora. Odpowiada funkcji `chrChangeHair` w Pythonie.
#[no_mangle]
pub extern "C" fn m2_py_chr_change_hair(hair: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };

        if let Some(main_vid) = mgr.main_actor_vid {
            if let Some(inst) = mgr.get_instance_mut(main_vid) {
                inst.appearance.set_hair(hair);
                return true;
            }
        }
        false
    })
}
