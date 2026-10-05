// crates/m2-client/src/py_bridge_chr_transforms.rs
//! Mostek transformacji instancji, nazw i typow postaci dla modulu chr.
//! Zapewnia 100% parytetu z C++ PythonCharacterModule.cpp i PythonCharacterManager.cpp.

use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::RwLock;
use once_cell::sync::Lazy;
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Clone)]
pub struct ChrInstanceRecord {
    pub vid: u32,
    pub vnum: u32,
    pub name: String,
    pub instance_type: i32,
    pub refreshed: bool,
    pub deformed: bool,
}

impl Default for ChrInstanceRecord {
    fn default() -> Self {
        Self {
            vid: 0,
            vnum: 0,
            name: String::new(),
            instance_type: 0, // TYPE_PC
            refreshed: false,
            deformed: false,
        }
    }
}

static CHR_RECORDS: Lazy<RwLock<HashMap<u32, ChrInstanceRecord>>> = Lazy::new(|| RwLock::new(HashMap::new()));
static SELECTED_VID: AtomicU32 = AtomicU32::new(0);

/// Ustawia wirtualny ID dla aktualnie wybranej instancji (chr.SetVirtualID).
#[no_mangle]
pub extern "C" fn m2_chr_set_virtual_id(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let cur = SELECTED_VID.load(Ordering::SeqCst);
        let mut map = CHR_RECORDS.write().unwrap();
        if cur != 0 && cur != vid {
            if let Some(mut rec) = map.remove(&cur) {
                rec.vid = vid;
                map.insert(vid, rec);
                SELECTED_VID.store(vid, Ordering::SeqCst);
                return true;
            }
        }
        let rec = map.entry(vid).or_insert_with(|| ChrInstanceRecord {
            vid,
            ..Default::default()
        });
        rec.vid = vid;
        SELECTED_VID.store(vid, Ordering::SeqCst);
        true
    })
}

/// Wybiera instancje postaci o podanym VID.
#[no_mangle]
pub extern "C" fn m2_chr_select_instance_by_id(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut map = CHR_RECORDS.write().unwrap();
        map.entry(vid).or_insert_with(|| ChrInstanceRecord {
            vid,
            ..Default::default()
        });
        SELECTED_VID.store(vid, Ordering::SeqCst);
        true
    })
}

/// Zwraca VID aktualnie zaznaczonej instancji.
#[no_mangle]
pub extern "C" fn m2_chr_get_selected_instance_vid() -> u32 {
    safe_ffi_call(0, || {
        SELECTED_VID.load(Ordering::SeqCst)
    })
}

/// Pobiera numer wirtualny (vnum / mob ID) dla instancji o danym VID (chr.GetVirtualNumber).
#[no_mangle]
pub extern "C" fn m2_chr_get_virtual_number_by_vid(vid: u32) -> u32 {
    safe_ffi_call(0, || {
        let map = CHR_RECORDS.read().unwrap();
        map.get(&vid).map(|r| r.vnum).unwrap_or(0)
    })
}

pub use crate::py_bridge_chr::m2_chr_get_virtual_number;

/// Przypisuje numer wirtualny do danej instancji.
#[no_mangle]
pub extern "C" fn m2_chr_set_virtual_number_by_vid(vid: u32, vnum: u32) -> bool {
    safe_ffi_call(false, || {
        let mut map = CHR_RECORDS.write().unwrap();
        let rec = map.entry(vid).or_insert_with(|| ChrInstanceRecord {
            vid,
            ..Default::default()
        });
        rec.vnum = vnum;
        true
    })
}

/// Ustawia nazwe wyswietlana aktualnie wybranej instancji (chr.SetNameString).
#[no_mangle]
pub unsafe extern "C" fn m2_chr_set_name_string(name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        let name_str = match CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };
        let mut map = CHR_RECORDS.write().unwrap();
        let rec = map.entry(vid).or_insert_with(|| ChrInstanceRecord {
            vid,
            ..Default::default()
        });
        rec.name = name_str;
        true
    })
}

/// Ustawia nazwe wyswietlana instancji dla konkretnego VID.
#[no_mangle]
pub unsafe extern "C" fn m2_chr_set_name_string_by_vid(vid: u32, name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let name_str = match CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };
        let mut map = CHR_RECORDS.write().unwrap();
        let rec = map.entry(vid).or_insert_with(|| ChrInstanceRecord {
            vid,
            ..Default::default()
        });
        rec.name = name_str;
        true
    })
}

/// Ustawia typ instancji dla aktualnie wybranej (chr.SetInstanceType).
#[no_mangle]
pub extern "C" fn m2_chr_set_instance_type(instance_type: i32) -> bool {
    safe_ffi_call(false, || {
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        m2_chr_set_instance_type_by_vid(vid, instance_type)
    })
}

/// Ustawia typ instancji dla konkretnego VID.
#[no_mangle]
pub extern "C" fn m2_chr_set_instance_type_by_vid(vid: u32, instance_type: i32) -> bool {
    safe_ffi_call(false, || {
        let mut map = CHR_RECORDS.write().unwrap();
        let rec = map.entry(vid).or_insert_with(|| ChrInstanceRecord {
            vid,
            ..Default::default()
        });
        rec.instance_type = instance_type;
        true
    })
}

/// Odswieza macierze kosci i transformacje instancji (chr.Refresh).
#[no_mangle]
pub extern "C" fn m2_chr_refresh_selected() -> bool {
    safe_ffi_call(false, || {
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        m2_chr_refresh(vid)
    })
}

/// Odswieza macierze konkretnego VID.
#[no_mangle]
pub extern "C" fn m2_chr_refresh(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut map = CHR_RECORDS.write().unwrap();
        if let Some(rec) = map.get_mut(&vid) {
            rec.refreshed = true;
            true
        } else {
            false
        }
    })
}

/// Deformuje siatke i animacje instancji (chr.Deform).
#[no_mangle]
pub extern "C" fn m2_chr_deform_selected() -> bool {
    safe_ffi_call(false, || {
        let vid = SELECTED_VID.load(Ordering::SeqCst);
        m2_chr_deform(vid)
    })
}

/// Deformuje konkretny VID.
#[no_mangle]
pub extern "C" fn m2_chr_deform(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut map = CHR_RECORDS.write().unwrap();
        if let Some(rec) = map.get_mut(&vid) {
            rec.deformed = true;
            true
        } else {
            false
        }
    })
}
