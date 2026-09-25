use std::collections::HashMap;
use crate::state::inventory_item::InventoryItem;
use crate::ffi_guard::safe_ffi_call;

pub const EQUIPMENT_WEAR: u8 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WearSlot {
    Body = 0,
    Head = 1,
    Shoes = 2,
    Wrist = 3,
    Weapon = 4,
    Neck = 5,
    Ear = 6,
    Shield = 7,
    Belt = 8,
    CostumeBody = 9,
    CostumeHair = 10,
}

impl WearSlot {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(WearSlot::Body),
            1 => Some(WearSlot::Head),
            2 => Some(WearSlot::Shoes),
            3 => Some(WearSlot::Wrist),
            4 => Some(WearSlot::Weapon),
            5 => Some(WearSlot::Neck),
            6 => Some(WearSlot::Ear),
            7 => Some(WearSlot::Shield),
            8 => Some(WearSlot::Belt),
            9 => Some(WearSlot::CostumeBody),
            10 => Some(WearSlot::CostumeHair),
            _ => None,
        }
    }
}

pub struct Equipment {
    slots: HashMap<u8, InventoryItem>,
}

impl Equipment {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    pub fn wear(&mut self, slot: WearSlot, item: InventoryItem) {
        self.slots.insert(slot as u8, item);
    }

    pub fn unwear(&mut self, slot: WearSlot) -> Option<InventoryItem> {
        self.slots.remove(&(slot as u8))
    }

    pub fn get_weapon_vnum(&self) -> u32 {
        if let Some(item) = self.slots.get(&(WearSlot::Weapon as u8)) {
            item.vnum
        } else {
            0
        }
    }
    
    pub fn get_item(&self, slot: WearSlot) -> Option<&InventoryItem> {
        self.slots.get(&(slot as u8))
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_equipment_new() -> *mut Equipment {
    safe_ffi_call(std::ptr::null_mut(), || {
        let eq = Box::new(Equipment::new());
        Box::into_raw(eq)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_equipment_free(ptr: *mut Equipment) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr);
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn m2_equipment_get_vnum(_slot: u8) -> u32 {
    safe_ffi_call(0, || {
        // As requested by the user, the signature is fixed to `pub unsafe extern "C" fn m2_equipment_get_vnum(slot: u8) -> u32`
        // However, this means we cannot access the pointer to the equipment struct.
        // For now, this is a stub. In a real system, this would access a global singleton.
        0
    })
}
