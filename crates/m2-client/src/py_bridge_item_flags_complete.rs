//! Python Bridge Item Flags Complete Module
//!
//! Pelna obsluga atrybutow, limitow uzycia oraz operacji bitowych na flagach
//! AntiFlag i WearFlag w module `item` dla Pythona.

pub use crate::py_bridge_item_flags::*;
use std::sync::Mutex;
use lazy_static::lazy_static;

// Glowne flagi AntiFlag
pub const ITEM_ANTIFLAG_FEMALE: u32 = 1 << 0;
pub const ITEM_ANTIFLAG_MALE: u32 = 1 << 1;
pub const ITEM_ANTIFLAG_WARRIOR: u32 = 1 << 2;
pub const ITEM_ANTIFLAG_ASSASSIN: u32 = 1 << 3;
pub const ITEM_ANTIFLAG_SURA: u32 = 1 << 4;
pub const ITEM_ANTIFLAG_SHAMAN: u32 = 1 << 5;
pub const ITEM_ANTIFLAG_DROP: u32 = 1 << 7;
pub const ITEM_ANTIFLAG_SELL: u32 = 1 << 8;
pub const ITEM_ANTIFLAG_EMPIRE_A: u32 = 1 << 9;
pub const ITEM_ANTIFLAG_EMPIRE_B: u32 = 1 << 10;
pub const ITEM_ANTIFLAG_EMPIRE_C: u32 = 1 << 11;
pub const ITEM_ANTIFLAG_SAVE: u32 = 1 << 12;
pub const ITEM_ANTIFLAG_GIVE: u32 = 1 << 13;
pub const ITEM_ANTIFLAG_PKDROP: u32 = 1 << 14;
pub const ITEM_ANTIFLAG_STACK: u32 = 1 << 15;
pub const ITEM_ANTIFLAG_MYSHOP: u32 = 1 << 16;
pub const ITEM_ANTIFLAG_SAFEBOX: u32 = 1 << 17;

// Glowne flagi WearFlag
pub const WEARFLAG_BODY: u32 = 1 << 0;
pub const WEARFLAG_HEAD: u32 = 1 << 1;
pub const WEARFLAG_FOOTS: u32 = 1 << 2;
pub const WEARFLAG_WRIST: u32 = 1 << 3;
pub const WEARFLAG_WEAPON: u32 = 1 << 4;
pub const WEARFLAG_NECK: u32 = 1 << 5;
pub const WEARFLAG_EAR: u32 = 1 << 6;
pub const WEARFLAG_UNIQUE: u32 = 1 << 7;
pub const WEARFLAG_SHIELD: u32 = 1 << 8;
pub const WEARFLAG_ARROW: u32 = 1 << 9;
pub const WEARFLAG_HAIR: u32 = 1 << 10;
pub const WEARFLAG_BELT: u32 = 1 << 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemLimit {
    pub limit_type: u8,
    pub limit_value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemApply {
    pub apply_type: u8,
    pub apply_value: i32,
}

#[derive(Debug, Clone)]
pub struct ItemTableData {
    pub vnum: u32,
    pub name: String,
    pub item_type: u8,
    pub sub_type: u8,
    pub anti_flags: u32,
    pub wear_flags: u32,
    pub limits: [ItemLimit; 2],
    pub applies: [ItemApply; 3],
}

impl Default for ItemTableData {
    fn default() -> Self {
        Self {
            vnum: 0,
            name: String::new(),
            item_type: 0,
            sub_type: 0,
            anti_flags: 0,
            wear_flags: 0,
            limits: [ItemLimit { limit_type: 0, limit_value: 0 }; 2],
            applies: [ItemApply { apply_type: 0, apply_value: 0 }; 3],
        }
    }
}

lazy_static! {
    static ref SELECTED_ITEM: Mutex<ItemTableData> = Mutex::new(ItemTableData::default());
}

pub fn select_item_table(data: ItemTableData) {
    if let Ok(mut it) = SELECTED_ITEM.lock() {
        *it = data;
    }
}

pub fn is_anti_flag(flag: u32) -> bool {
    SELECTED_ITEM.lock().map(|it| (it.anti_flags & flag) != 0).unwrap_or(false)
}

pub fn is_wear_flag(flag: u32) -> bool {
    SELECTED_ITEM.lock().map(|it| (it.wear_flags & flag) != 0).unwrap_or(false)
}

pub fn get_limit(index: usize) -> (u8, i32) {
    SELECTED_ITEM.lock().map(|it| {
        if index < it.limits.len() {
            (it.limits[index].limit_type, it.limits[index].limit_value)
        } else {
            (0, 0)
        }
    }).unwrap_or((0, 0))
}

pub fn get_affect(index: usize) -> (u8, i32) {
    SELECTED_ITEM.lock().map(|it| {
        if index < it.applies.len() {
            (it.applies[index].apply_type, it.applies[index].apply_value)
        } else {
            (0, 0)
        }
    }).unwrap_or((0, 0))
}

// C-FFI
#[no_mangle]
pub extern "C" fn item_is_anti_flag(flag: u32) -> bool {
    is_anti_flag(flag)
}

#[no_mangle]
pub extern "C" fn item_is_wear_flag(flag: u32) -> bool {
    is_wear_flag(flag)
}
