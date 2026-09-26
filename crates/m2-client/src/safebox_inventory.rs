pub const SAFEBOX_SLOT_X_COUNT: usize = 5;
pub const SAFEBOX_SLOT_Y_COUNT: usize = 9;
pub const SAFEBOX_PAGE_SIZE: usize = SAFEBOX_SLOT_X_COUNT * SAFEBOX_SLOT_Y_COUNT; // 45
pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3; // From C++ assumptions
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct TItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct ItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub al_sockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub a_attr: [TItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl ItemData {
    pub fn is_empty(&self) -> bool {
        self.vnum == 0
    }
}

pub struct SafeBox {
    _pages: usize, // prefixed with underscore to suppress dead_code warning
    items: Vec<ItemData>,
    safebox_gold: i64,
}

impl SafeBox {
    pub fn new(pages: usize) -> Self {
        let actual_pages = if pages == 0 { 1 } else if pages > 3 { 3 } else { pages };
        Self {
            _pages: actual_pages,
            items: vec![ItemData::default(); SAFEBOX_PAGE_SIZE * actual_pages],
            safebox_gold: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.items.len()
    }

    pub fn get_gold(&self) -> i64 {
        self.safebox_gold
    }

    pub fn deposit_gold(&mut self, amount: i64) -> bool {
        if amount <= 0 {
            return false;
        }
        
        if let Some(new_gold) = self.safebox_gold.checked_add(amount) {
            self.safebox_gold = new_gold;
            true
        } else {
            false
        }
    }

    pub fn withdraw_gold(&mut self, amount: i64) -> bool {
        if amount <= 0 {
            return false;
        }

        if self.safebox_gold >= amount {
            self.safebox_gold -= amount;
            true
        } else {
            false
        }
    }

    pub fn check_in(&mut self, slot_index: usize, item: ItemData) -> bool {
        if slot_index >= self.items.len() {
            return false;
        }

        if item.is_empty() {
            return false;
        }

        if !self.items[slot_index].is_empty() {
            return false; // slot already occupied
        }

        self.items[slot_index] = item;
        true
    }

    pub fn check_out(&mut self, slot_index: usize) -> Option<ItemData> {
        if slot_index >= self.items.len() {
            return None;
        }

        if self.items[slot_index].is_empty() {
            return None;
        }

        let item = self.items[slot_index];
        self.items[slot_index] = ItemData::default();
        Some(item)
    }

    pub fn get_item(&self, slot_index: usize) -> Option<&ItemData> {
        if slot_index >= self.items.len() {
            return None;
        }
        Some(&self.items[slot_index])
    }
}
