use crate::ffi_guard::safe_ffi_call;
use std::sync::RwLock;

// Naglowek pakietu / stala
pub const POINTS_REGISTRY: u8 = 0;

// Stale punktow postaci
pub const POINT_HP: u8 = 5;
pub const POINT_SP: u8 = 6;
pub const POINT_MAX_HP: u8 = 7;
pub const POINT_ATT_SPEED: u8 = 19;
pub const POINT_MOV_SPEED: u8 = 20;
pub const POINT_DEF_GRADE: u8 = 21;

// Struktura rejestru punktow
#[derive(Debug)]
pub struct PointsRegistry {
    points: [i64; 130],
}

impl PointsRegistry {
    // Tworzy nowa instancje
    pub fn new() -> Self {
        PointsRegistry { points: [0; 130] }
    }

    // Pobiera wartosc na danym indeksie, zwraca 0 jesli indeks poza zakresem
    pub fn get(&self, idx: u8) -> i64 {
        if (idx as usize) < self.points.len() {
            self.points[idx as usize]
        } else {
            0
        }
    }

    // Ustawia wartosc na danym indeksie, ignoruje jesli indeks poza zakresem
    pub fn set(&mut self, idx: u8, val: i64) {
        if (idx as usize) < self.points.len() {
            self.points[idx as usize] = val;
        }
    }
}

// Globalny rejestr punktow postaci (uzywany przez FFI w przykladzie)
static GLOBAL_REGISTRY: RwLock<Option<PointsRegistry>> = RwLock::new(None);

// Inicjalizuje globalny rejestr punktow (do uzytku przez C-FFI)
#[no_mangle]
pub extern "C" fn m2_points_registry_init() {
    safe_ffi_call((), || {
        let mut reg = GLOBAL_REGISTRY.write().unwrap();
        if reg.is_none() {
            *reg = Some(PointsRegistry::new());
        }
    });
}

// Pobiera wartosc punktu (uzywa globalnego rejestru)
#[no_mangle]
pub extern "C" fn m2_player_get_stat(point_idx: u8) -> i64 {
    safe_ffi_call(0, || {
        let reg = GLOBAL_REGISTRY.read().unwrap();
        if let Some(r) = reg.as_ref() {
            r.get(point_idx)
        } else {
            0
        }
    })
}

// Ustawia wartosc punktu (uzywa globalnego rejestru)
#[no_mangle]
pub extern "C" fn m2_player_set_stat(point_idx: u8, val: i64) {
    safe_ffi_call((), || {
        let mut reg = GLOBAL_REGISTRY.write().unwrap();
        if let Some(r) = reg.as_mut() {
            r.set(point_idx, val);
        }
    });
}

// Tworzy nowa instancje PointsRegistry i zwraca wskaznik (np. gdy C zarzadza obiektem)
#[no_mangle]
pub extern "C" fn m2_points_registry_create() -> *mut PointsRegistry {
    safe_ffi_call(std::ptr::null_mut(), || {
        let reg = Box::new(PointsRegistry::new());
        Box::into_raw(reg)
    })
}

// Pobiera wartosc dla obiektu PointsRegistry przekazanego przez wskaznik
#[no_mangle]
pub extern "C" fn m2_points_registry_get(ptr: *const PointsRegistry, idx: u8) -> i64 {
    safe_ffi_call(0, || {
        if ptr.is_null() {
            return 0;
        }
        let reg = unsafe { &*ptr };
        reg.get(idx)
    })
}

// Ustawia wartosc dla obiektu PointsRegistry przekazanego przez wskaznik
#[no_mangle]
pub extern "C" fn m2_points_registry_set(ptr: *mut PointsRegistry, idx: u8, val: i64) {
    safe_ffi_call((), || {
        if ptr.is_null() {
            return;
        }
        let reg = unsafe { &mut *ptr };
        reg.set(idx, val);
    });
}

// Zwalnia pamiec alokowana przez m2_points_registry_create
#[no_mangle]
pub extern "C" fn m2_points_registry_free(ptr: *mut PointsRegistry) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(ptr);
            }
        }
    });
}
