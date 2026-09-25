use std::ptr;
use crate::ffi_guard::safe_ffi_call;

pub const QUICKSLOT_MANAGER: u8 = 0;
pub const SLOT_TYPE_NONE: u8 = 0;
pub const SLOT_TYPE_ITEM: u8 = 1;
pub const SLOT_TYPE_SKILL: u8 = 2;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuickslotEntry {
    pub slot_type: u8, // SLOT_TYPE_NONE, ITEM, SKILL
    pub pos: u32,      // vnum skilla lub slot itemu
}

impl Default for QuickslotEntry {
    fn default() -> Self {
        Self {
            slot_type: SLOT_TYPE_NONE,
            pos: 0,
        }
    }
}

pub struct QuickslotManager {
    pub slots: [QuickslotEntry; 32],
}

impl QuickslotManager {
    // Tworzy nowa, pusta instancje menedzera paska szybkiego dostepu
    pub fn new() -> Self {
        Self {
            slots: [QuickslotEntry::default(); 32],
        }
    }

    // Ustawia slot na danym indeksie
    pub fn set_slot(&mut self, idx: u8, slot_type: u8, pos: u32) {
        if (idx as usize) < self.slots.len() {
            self.slots[idx as usize] = QuickslotEntry { slot_type, pos };
        }
    }

    // Czysci dany slot
    pub fn clear_slot(&mut self, idx: u8) {
        if (idx as usize) < self.slots.len() {
            self.slots[idx as usize] = QuickslotEntry::default();
        }
    }

    // Pobiera zawartosc slotu
    pub fn get_slot(&self, idx: u8) -> QuickslotEntry {
        if (idx as usize) < self.slots.len() {
            self.slots[idx as usize]
        } else {
            QuickslotEntry::default()
        }
    }

    // Zamienia dwa sloty miejscami
    pub fn swap_slots(&mut self, idx1: u8, idx2: u8) {
        if (idx1 as usize) < self.slots.len() && (idx2 as usize) < self.slots.len() {
            self.slots.swap(idx1 as usize, idx2 as usize);
        }
    }
}

// Zgodnie ze specyfikacja z opisu, `m2_quickslot_get` nie pobiera referencji na managera,
// zatem musimy utrzymac globalny stan, albo potraktowac to jako funkcje globalna.
// Poniewaz w specyfikacji bylo:
// pub unsafe extern "C" fn m2_quickslot_get(idx: u8, out_type: *mut u8, out_pos: *mut u32) -> bool
// dodamy statyczna instancje paska.
use once_cell::sync::Lazy;
use std::sync::Mutex;

static GLOBAL_QUICKSLOT: Lazy<Mutex<QuickslotManager>> = Lazy::new(|| Mutex::new(QuickslotManager::new()));

// Funkcje globalne do modyfikacji, choc mozemy tez uzyc C-FFI API jesli bedzie potrzebne 
// ale glowne to: set_slot, clear_slot, swap_slots.

#[no_mangle]
pub unsafe extern "C" fn m2_quickslot_get(idx: u8, out_type: *mut u8, out_pos: *mut u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = GLOBAL_QUICKSLOT.lock().unwrap();
        
        if (idx as usize) >= mgr.slots.len() {
            return false;
        }

        let entry = mgr.get_slot(idx);
        if !out_type.is_null() {
            *out_type = entry.slot_type;
        }
        if !out_pos.is_null() {
            *out_pos = entry.pos;
        }
        true
    })
}

// Dodatkowe C-FFI do testowania C++ i operacji 
#[no_mangle]
pub unsafe extern "C" fn m2_quickslot_set(idx: u8, slot_type: u8, pos: u32) {
    safe_ffi_call((), || {
        let mut mgr = GLOBAL_QUICKSLOT.lock().unwrap();
        mgr.set_slot(idx, slot_type, pos);
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_quickslot_clear(idx: u8) {
    safe_ffi_call((), || {
        let mut mgr = GLOBAL_QUICKSLOT.lock().unwrap();
        mgr.clear_slot(idx);
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_quickslot_swap(idx1: u8, idx2: u8) {
    safe_ffi_call((), || {
        let mut mgr = GLOBAL_QUICKSLOT.lock().unwrap();
        mgr.swap_slots(idx1, idx2);
    })
}

// Metody na tworzenie lokalnej kopii paska w pamieci uzytkownika C (gdyby nie chciano globala)
#[no_mangle]
pub unsafe extern "C" fn m2_quickslot_manager_new() -> *mut QuickslotManager {
    safe_ffi_call(ptr::null_mut(), || {
        Box::into_raw(Box::new(QuickslotManager::new()))
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_quickslot_manager_free(ptr: *mut QuickslotManager) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr);
        }
    });
}
