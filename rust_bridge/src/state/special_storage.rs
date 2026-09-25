use std::collections::HashMap;
use crate::state::inventory_item::InventoryItem;

pub const SPECIAL_STORAGE: u8 = 0;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageCategory {
    UpgradeItems = 0,
    BookItems = 1,
    StoneItems = 2,
}

impl StorageCategory {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(StorageCategory::UpgradeItems),
            1 => Some(StorageCategory::BookItems),
            2 => Some(StorageCategory::StoneItems),
            _ => None,
        }
    }
}

pub struct SpecialStorage {
    category_slots: [HashMap<u16, InventoryItem>; 3],
}

impl SpecialStorage {
    pub fn new() -> Self {
        Self {
            category_slots: [HashMap::new(), HashMap::new(), HashMap::new()],
        }
    }

    pub fn add_item(&mut self, cat: StorageCategory, slot: u16, item: InventoryItem) {
        let index = cat as usize;
        self.category_slots[index].insert(slot, item);
    }
    
    // Dodatkowa pomocnicza funkcja liczenia
    pub fn get_count_for_category(&self, cat: StorageCategory) -> u32 {
        let index = cat as usize;
        self.category_slots[index].len() as u32
    }
}

// Globalny stan (dummy) na potrzeby FFI m2_special_storage_count 
// Poniewaz sygnatura FFI nie ma wskaznika
static mut GLOBAL_STORAGE: Option<SpecialStorage> = None;

#[no_mangle]
pub unsafe extern "C" fn m2_special_storage_new() -> *mut SpecialStorage {
    let result = std::panic::catch_unwind(|| {
        Box::into_raw(Box::new(SpecialStorage::new()))
    });
    match result {
        Ok(ptr) => ptr,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_special_storage_free(ptr: *mut SpecialStorage) {
    let _ = std::panic::catch_unwind(|| {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr);
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn m2_special_storage_count(cat: u8) -> u32 {
    let result = std::panic::catch_unwind(|| {
        // Zwraca zero jesli kategoria nie jest wlasciwa
        let category = match StorageCategory::from_u8(cat) {
            Some(c) => c,
            None => return 0,
        };
        
        // Zwraca 0 jezeli nie jest ustawiony globalny storage, to tylko symulacja
        // w prawdziwym kodzie uzyto by wlasciwego sposobu zaleznego od silnika
        let storage_ptr = std::ptr::addr_of!(GLOBAL_STORAGE);
        if let Some(storage) = &*storage_ptr {
            storage.get_count_for_category(category)
        } else {
            0
        }
    });
    match result {
        Ok(count) => count,
        Err(_) => 0,
    }
}
