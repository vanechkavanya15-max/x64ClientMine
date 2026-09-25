use crate::state::inventory_item::InventoryItem;
use std::sync::RwLock;

pub struct BeltInventory {
    pub unlocked_slots: u8,
    slots: [Option<InventoryItem>; 16],
}

impl BeltInventory {
    pub fn new() -> Self {
        Self {
            unlocked_slots: 0,
            slots: Default::default(),
        }
    }

    pub fn set_unlocked(&mut self, count: u8) {
        self.unlocked_slots = count.min(16);
    }

    pub fn put_item(&mut self, slot: u8, item: InventoryItem) -> Result<(), &'static str> {
        if slot >= self.unlocked_slots {
            return Err("Slot jest zablokowany lub poza zakresem");
        }
        self.slots[slot as usize] = Some(item);
        Ok(())
    }

    pub fn get_item(&self, slot: u8) -> Option<&InventoryItem> {
        if slot >= 16 {
            return None;
        }
        self.slots[slot as usize].as_ref()
    }
}

// C-FFI
use std::panic::catch_unwind;

// Zmienna globalna symulujaca instancje w C-FFI
// Uzywamy once_cell z crate aby uniknac mut static problemow
use once_cell::sync::Lazy;

static BELT_INVENTORY_INSTANCE: Lazy<RwLock<Option<BeltInventory>>> =
    Lazy::new(|| RwLock::new(None));

#[no_mangle]
pub unsafe extern "C" fn m2_belt_init() {
    let _ = catch_unwind(|| {
        if let Ok(mut belt) = BELT_INVENTORY_INSTANCE.write() {
            *belt = Some(BeltInventory::new());
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn m2_belt_cleanup() {
    let _ = catch_unwind(|| {
        if let Ok(mut belt) = BELT_INVENTORY_INSTANCE.write() {
            *belt = None;
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn m2_belt_set_unlocked(count: u8) {
    let _ = catch_unwind(|| {
        if let Ok(mut belt_lock) = BELT_INVENTORY_INSTANCE.write() {
            if let Some(belt) = &mut *belt_lock {
                belt.set_unlocked(count);
            }
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn m2_belt_get_vnum(slot: u8) -> u32 {
    catch_unwind(|| {
        if let Ok(belt_lock) = BELT_INVENTORY_INSTANCE.read() {
            if let Some(belt) = &*belt_lock {
                if let Some(item) = belt.get_item(slot) {
                    return item.vnum;
                }
            }
        }
        0
    })
    .unwrap_or(0)
}

// Dodano dla testow integracyjnych w Rust (ffi bridge usage from inside Rust)
pub fn m2_belt_put_item_rust(slot: u8, item: InventoryItem) -> Result<(), &'static str> {
    if let Ok(mut belt_lock) = BELT_INVENTORY_INSTANCE.write() {
        if let Some(belt) = &mut *belt_lock {
            return belt.put_item(slot, item);
        }
    }
    Err("BeltInventory not initialized")
}
