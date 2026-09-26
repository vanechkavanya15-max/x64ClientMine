use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;

/// Stan polimorfii dla konkretnej postaci (VID).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolymorphState {
    pub mob_vnum: u32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub equipment_hidden: bool,
}

static POLYMORPH_STATES: OnceLock<Mutex<HashMap<u32, PolymorphState>>> = OnceLock::new();

fn get_states() -> &'static Mutex<HashMap<u32, PolymorphState>> {
    POLYMORPH_STATES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Aplikuje lub zdejmuje stan polimorfii z postaci.
/// Jezeli mob_vnum > 0, postac zmienia rase, skaluje sie, a ludzki ekwipunek jest ukryty.
/// Jezeli mob_vnum == 0, polimorfia zostaje zdjeta.
#[no_mangle]
pub extern "C" fn m2_poly_apply(
    vid: u32,
    mob_vnum: u32,
    scale_x: f32,
    scale_y: f32,
    scale_z: f32,
) -> bool {
    safe_ffi_call(false, || {
        let mut states = get_states().lock().unwrap();

        if mob_vnum > 0 {
            states.insert(
                vid,
                PolymorphState {
                    mob_vnum,
                    scale_x,
                    scale_y,
                    scale_z,
                    equipment_hidden: true,
                },
            );
        } else {
            states.remove(&vid);
        }

        true
    })
}

/// Zwraca informacje, czy postac o danym VID jest aktualnie w stanie polimorfii.
#[no_mangle]
pub extern "C" fn m2_poly_is_active(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let states = get_states().lock().unwrap();
        states.contains_key(&vid)
    })
}

/// Zwraca ID potwora (mob_vnum), w ktorego zamieniona jest postac.
/// Jezeli postac nie jest w polimorfii, zwraca 0.
#[no_mangle]
pub extern "C" fn m2_poly_get_vnum(vid: u32) -> u32 {
    safe_ffi_call(0, || {
        let states = get_states().lock().unwrap();
        if let Some(state) = states.get(&vid) {
            state.mob_vnum
        } else {
            0
        }
    })
}

/// Pobiera aktualne wektory skalowania dla postaci w polimorfii.
/// Zwraca false, jezeli postac nie jest w polimorfii (skala to standardowo 1.0).
#[no_mangle]
pub extern "C" fn m2_poly_get_scale(
    vid: u32,
    out_x: *mut f32,
    out_y: *mut f32,
    out_z: *mut f32,
) -> bool {
    safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_z.is_null() {
            return false;
        }

        let states = get_states().lock().unwrap();
        if let Some(state) = states.get(&vid) {
            unsafe {
                *out_x = state.scale_x;
                *out_y = state.scale_y;
                *out_z = state.scale_z;
            }
            true
        } else {
            false
        }
    })
}

/// Zwraca true, jezeli ekwipunek czlowieka (bron, zbroja, wlosy) ma byc ukryty z powodu polimorfii.
#[no_mangle]
pub extern "C" fn m2_poly_is_equipment_hidden(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let states = get_states().lock().unwrap();
        if let Some(state) = states.get(&vid) {
            state.equipment_hidden
        } else {
            false
        }
    })
}
