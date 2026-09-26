use std::ffi::CStr;
use std::os::raw::c_char;

pub const SAFEBOX_SLOT_X_COUNT: usize = 5;
pub const SAFEBOX_SLOT_Y_COUNT: usize = 9;
pub const SAFEBOX_PAGE_SIZE: usize = SAFEBOX_SLOT_X_COUNT * SAFEBOX_SLOT_Y_COUNT;

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl Default for TItemData {
    fn default() -> Self {
        Self {
            vnum: 0,
            count: 0,
            flags: 0,
            anti_flags: 0,
            alSockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
            aAttr: [TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM],
        }
    }
}

pub struct SafeboxMallManagerV2 {
    safebox_items: Vec<TItemData>,
    mall_items: Vec<TItemData>,
    money: u32,
    is_open: bool,
    wrong_attempts: u32,
    expected_password_hash: String,
}

impl Default for SafeboxMallManagerV2 {
    fn default() -> Self {
        Self::new("000000") // Default dummy hash
    }
}

impl SafeboxMallManagerV2 {
    pub fn new(expected_hash: &str) -> Self {
        Self {
            safebox_items: Vec::new(),
            mall_items: Vec::new(),
            money: 0,
            is_open: false,
            wrong_attempts: 0,
            expected_password_hash: expected_hash.to_string(),
        }
    }

    pub fn set_expected_password(&mut self, hash: &str) {
        self.expected_password_hash = hash.to_string();
    }

    pub fn try_open_with_password(&mut self, password_hash: &str) -> bool {
        if password_hash == self.expected_password_hash {
            self.is_open = true;
            self.wrong_attempts = 0;
            true
        } else {
            self.wrong_attempts += 1;
            false
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn get_wrong_attempts(&self) -> u32 {
        self.wrong_attempts
    }

    pub fn open_safebox(&mut self, size: usize) {
        self.money = 0;
        self.safebox_items.clear();
        let actual_size = size * SAFEBOX_SLOT_X_COUNT;
        self.safebox_items.resize(actual_size, TItemData::default());
    }

    pub fn set_item_data(&mut self, slot_index: usize, item_data: TItemData) {
        if slot_index < self.safebox_items.len() {
            self.safebox_items[slot_index] = item_data;
        }
    }

    pub fn del_item_data(&mut self, slot_index: usize) {
        if slot_index < self.safebox_items.len() {
            self.safebox_items[slot_index] = TItemData::default();
        }
    }

    pub fn get_item_data(&self, slot_index: usize) -> Option<&TItemData> {
        self.safebox_items.get(slot_index)
    }

    pub fn set_money(&mut self, money: u32) {
        self.money = money;
    }

    pub fn get_money(&self) -> u32 {
        self.money
    }

    pub fn get_current_safebox_size(&self) -> usize {
        self.safebox_items.len()
    }

    // MALL
    pub fn open_mall(&mut self, size: usize) {
        self.mall_items.clear();
        let actual_size = size * SAFEBOX_SLOT_X_COUNT;
        self.mall_items.resize(actual_size, TItemData::default());
    }

    pub fn set_mall_item_data(&mut self, slot_index: usize, item_data: TItemData) {
        if slot_index < self.mall_items.len() {
            self.mall_items[slot_index] = item_data;
        }
    }

    pub fn del_mall_item_data(&mut self, slot_index: usize) {
        if slot_index < self.mall_items.len() {
            self.mall_items[slot_index] = TItemData::default();
        }
    }

    pub fn get_mall_item_data(&self, slot_index: usize) -> Option<&TItemData> {
        self.mall_items.get(slot_index)
    }

    pub fn get_mall_size(&self) -> usize {
        self.mall_items.len()
    }
}

use std::sync::{RwLock, LazyLock};

// Global thread-safe instance
pub static G_SAFEBOX_MALL_MANAGER: LazyLock<RwLock<SafeboxMallManagerV2>> =
    LazyLock::new(|| RwLock::new(SafeboxMallManagerV2::default()));

#[no_mangle]
pub extern "C" fn m2_safebox_open(password_hash: *const u8) -> bool {
    if password_hash.is_null() {
        return false;
    }
    
    // We expect a null-terminated string for the hash
    let c_str = unsafe { CStr::from_ptr(password_hash as *const c_char) };
    if let Ok(hash_str) = c_str.to_str() {
        if let Ok(mut manager) = G_SAFEBOX_MALL_MANAGER.write() {
            manager.try_open_with_password(hash_str)
        } else {
            false
        }
    } else {
        false
    }
}
