#![allow(non_snake_case)]
#![allow(dead_code)]

pub const SAFEBOX_SLOT_X_COUNT: usize = 5;
pub const SAFEBOX_SLOT_Y_COUNT: usize = 9;
pub const SAFEBOX_PAGE_SIZE: usize = SAFEBOX_SLOT_X_COUNT * SAFEBOX_SLOT_Y_COUNT;

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

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

pub struct SafeboxStorageManager {
    safebox_items: Vec<TItemData>,
    mall_items: Vec<TItemData>,
    money: i32,
    is_open: bool,
    expected_password_hash: String,
}

impl Default for SafeboxStorageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SafeboxStorageManager {
    pub fn new() -> Self {
        Self {
            safebox_items: Vec::new(),
            mall_items: Vec::new(),
            money: 0,
            is_open: false,
            expected_password_hash: String::from("000000"), // Default dummy password
        }
    }

    pub fn set_expected_password(&mut self, hash: &str) {
        self.expected_password_hash = hash.to_string();
    }

    pub fn checkin_password(&mut self, password_hash: &str) -> bool {
        if password_hash == self.expected_password_hash {
            self.is_open = true;
            true
        } else {
            false
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn open_safebox(&mut self, size: usize) {
        self.money = 0;
        self.safebox_items.clear();
        let max_size = size.max(1).min(3);
        let actual_size = max_size * SAFEBOX_PAGE_SIZE;
        self.safebox_items.resize(actual_size, TItemData::default());
    }

    pub fn get_current_safebox_size(&self) -> usize {
        self.safebox_items.len()
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

    // Money
    pub fn set_money(&mut self, money: i32) {
        self.money = money;
    }

    pub fn get_money(&self) -> i32 {
        self.money
    }

    pub fn deposit_yang(&mut self, amount: i32) -> bool {
        if amount <= 0 {
            return false;
        }
        if let Some(new_money) = self.money.checked_add(amount) {
            self.money = new_money;
            true
        } else {
            false
        }
    }

    pub fn withdraw_yang(&mut self, amount: i32) -> bool {
        if amount <= 0 {
            return false;
        }
        if self.money >= amount {
            self.money -= amount;
            true
        } else {
            false
        }
    }

    // Safebox move logic
    pub fn checkin_item(&mut self, slot_index: usize, item: TItemData) -> bool {
        if slot_index >= self.safebox_items.len() {
            return false;
        }
        if item.vnum == 0 {
            return false;
        }
        if self.safebox_items[slot_index].vnum != 0 {
            return false;
        }
        self.safebox_items[slot_index] = item;
        true
    }

    pub fn checkout_item(&mut self, slot_index: usize) -> Option<TItemData> {
        if slot_index >= self.safebox_items.len() {
            return None;
        }
        if self.safebox_items[slot_index].vnum == 0 {
            return None;
        }
        let item = self.safebox_items[slot_index].clone();
        self.safebox_items[slot_index] = TItemData::default();
        Some(item)
    }

    pub fn move_item(&mut self, source_slot: usize, dest_slot: usize) -> bool {
        if source_slot >= self.safebox_items.len() || dest_slot >= self.safebox_items.len() {
            return false;
        }
        if self.safebox_items[source_slot].vnum == 0 {
            return false;
        }
        if self.safebox_items[dest_slot].vnum != 0 {
            // Swap not supported in standard move, must be empty
            return false;
        }
        
        self.safebox_items.swap(source_slot, dest_slot);
        true
    }

    // MALL
    pub fn open_mall(&mut self, size: usize) {
        self.mall_items.clear();
        let max_size = size.max(1).min(3);
        let actual_size = max_size * SAFEBOX_PAGE_SIZE;
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
