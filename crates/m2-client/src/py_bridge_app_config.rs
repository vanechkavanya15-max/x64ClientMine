// crates/m2-client/src/py_bridge_app_config.rs
//! Mostek konfiguracji aplikacji, czasu i losowosci.
//! Integruje i re-eksportuje natywne metody z py_master_runtime_wire_final.

pub use crate::py_master_runtime_wire_final::{
    m2_app_set_frame_skip,
    m2_app_get_random,
    m2_app_get_global_time_stamp,
    m2_app_set_hair_color_enable,
    m2_app_set_armor_specular_enable,
    m2_app_set_weapon_specular_enable,
    FRAME_SKIP,
    HAIR_COLOR_ENABLE,
};

/// Pobiera biezacy stan frame skip.
pub fn m2_app_get_frame_skip() -> bool {
    if let Ok(lock) = FRAME_SKIP.lock() {
        *lock
    } else {
        false
    }
}

/// Pobiera biezacy stan hair color.
pub fn m2_app_get_hair_color_enable() -> bool {
    if let Ok(lock) = HAIR_COLOR_ENABLE.lock() {
        *lock
    } else {
        false
    }
}
