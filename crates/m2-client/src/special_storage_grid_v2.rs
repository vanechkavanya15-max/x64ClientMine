use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SpecialStorageType {
    SkillBook = 0,
    Stone = 1,
    Upgrade = 2,
}

impl TryFrom<u8> for SpecialStorageType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SpecialStorageType::SkillBook),
            1 => Ok(SpecialStorageType::Stone),
            2 => Ok(SpecialStorageType::Upgrade),
            _ => Err("Invalid storage type"),
        }
    }
}

// Based on CItemData::EItemType from C++ source
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ItemType {
    None = 0,
    Weapon = 1,
    Armor = 2,
    Use = 3,
    Autouse = 4,
    Material = 5,
    Special = 6,
    Tool = 7,
    Lottery = 8,
    Elk = 9,
    Metin = 10,
    Container = 11,
    Fish = 12,
    Rod = 13,
    Resource = 14,
    Campfire = 15,
    Unique = 16,
    Skillbook = 17,
    Quest = 18,
    Polymorph = 19,
    TreasureBox = 20,
    TreasureKey = 21,
    Skillforget = 22,
}

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TPlayerItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpecialItemData {
    pub vnum: u32,
    pub count: u8,
    pub sockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub attrs: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl SpecialItemData {
    pub fn new(vnum: u32, count: u8) -> Self {
        Self {
            vnum,
            count,
            sockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
            attrs: [TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM],
        }
    }

    pub fn with_details(
        vnum: u32,
        count: u8,
        sockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
        attrs: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
    ) -> Self {
        Self {
            vnum,
            count,
            sockets,
            attrs,
        }
    }
}

pub struct SpecialStorageGridV2 {
    books: [Option<SpecialItemData>; 180],
    stones: [Option<SpecialItemData>; 180],
    upgrades: [Option<SpecialItemData>; 180],
}

impl Default for SpecialStorageGridV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SpecialStorageGridV2 {
    pub const TOTAL_SLOTS: u16 = 180;

    pub fn new() -> Self {
        Self {
            books: [None; 180],
            stones: [None; 180],
            upgrades: [None; 180],
        }
    }

    pub fn determine_storage_type_by_item_type(item_type: u8) -> Option<SpecialStorageType> {
        match item_type {
            17 => Some(SpecialStorageType::SkillBook), // ITEM_TYPE_SKILLBOOK
            10 => Some(SpecialStorageType::Stone),     // ITEM_TYPE_METIN (stones)
            5 => Some(SpecialStorageType::Upgrade),      // ITEM_TYPE_MATERIAL
            _ => None,
        }
    }

    pub fn get_storage_mut(&mut self, storage_type: SpecialStorageType) -> &mut [Option<SpecialItemData>; 180] {
        match storage_type {
            SpecialStorageType::SkillBook => &mut self.books,
            SpecialStorageType::Stone => &mut self.stones,
            SpecialStorageType::Upgrade => &mut self.upgrades,
        }
    }
    
    pub fn get_storage(&self, storage_type: SpecialStorageType) -> &[Option<SpecialItemData>; 180] {
        match storage_type {
            SpecialStorageType::SkillBook => &self.books,
            SpecialStorageType::Stone => &self.stones,
            SpecialStorageType::Upgrade => &self.upgrades,
        }
    }

    pub fn put_item(
        &mut self,
        storage_type: SpecialStorageType,
        slot: u16,
        vnum: u32,
        count: u8,
    ) -> bool {
        if slot >= Self::TOTAL_SLOTS {
            return false;
        }

        let storage = self.get_storage_mut(storage_type);
        storage[slot as usize] = Some(SpecialItemData::new(vnum, count));
        true
    }

    pub fn put_item_full(
        &mut self,
        storage_type: SpecialStorageType,
        slot: u16,
        vnum: u32,
        count: u8,
        sockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
        attrs: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
    ) -> bool {
        if slot >= Self::TOTAL_SLOTS {
            return false;
        }

        let storage = self.get_storage_mut(storage_type);
        storage[slot as usize] = Some(SpecialItemData::with_details(vnum, count, sockets, attrs));
        true
    }

    pub fn get_item(&self, storage_type: SpecialStorageType, slot: u16) -> Option<&SpecialItemData> {
        if slot >= Self::TOTAL_SLOTS {
            return None;
        }

        let storage = self.get_storage(storage_type);
        storage[slot as usize].as_ref()
    }
    
    pub fn remove_item(&mut self, storage_type: SpecialStorageType, slot: u16) -> bool {
        if slot >= Self::TOTAL_SLOTS {
            return false;
        }

        let storage = self.get_storage_mut(storage_type);
        if storage[slot as usize].is_some() {
            storage[slot as usize] = None;
            true
        } else {
            false
        }
    }
    
    pub fn swap_items(
        &mut self,
        storage_type: SpecialStorageType,
        slot1: u16,
        slot2: u16,
    ) -> bool {
        if slot1 >= Self::TOTAL_SLOTS || slot2 >= Self::TOTAL_SLOTS {
            return false;
        }

        let storage = self.get_storage_mut(storage_type);
        storage.swap(slot1 as usize, slot2 as usize);
        true
    }
}

static SPECIAL_STORAGE_FFI: OnceLock<Mutex<SpecialStorageGridV2>> = OnceLock::new();

fn get_ffi_special_storage() -> &'static Mutex<SpecialStorageGridV2> {
    SPECIAL_STORAGE_FFI.get_or_init(|| Mutex::new(SpecialStorageGridV2::new()))
}

fn safe_ffi_call<F, R>(default_val: R, f: F) -> R
where
    F: FnOnce() -> R,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(res) => res,
        Err(_) => default_val,
    }
}

#[no_mangle]
pub extern "C" fn m2_special_storage_put(
    storage_type: u8,
    slot: u16,
    vnum: u32,
    count: u8,
) -> bool {
    safe_ffi_call(false, || {
        let Ok(st_type) = SpecialStorageType::try_from(storage_type) else {
            return false;
        };
        
        let state_mutex = get_ffi_special_storage();
        if let Ok(mut state) = state_mutex.lock() {
            state.put_item(st_type, slot, vnum, count)
        } else {
            false
        }
    })
}
