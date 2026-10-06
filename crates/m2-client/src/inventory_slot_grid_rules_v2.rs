#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemData {
    pub vnum: u32,
    pub count: u8,
    pub size: u8, // 1, 2, or 3
    pub flags: u32,
    pub anti_flags: u32,
}

pub const ITEM_FLAG_STACKABLE: u32 = 1 << 2;
pub const ITEM_ANTIFLAG_STACK: u32 = 1 << 15;

pub const INVENTORY_PAGE_SIZE: u16 = 45; // 5 x 9
pub const INVENTORY_PAGE_COUNT: u16 = 2;
pub const INVENTORY_MAX_NUM: u16 = INVENTORY_PAGE_SIZE * INVENTORY_PAGE_COUNT;
pub const INVENTORY_PAGE_COLUMN: u16 = 5;
pub const INVENTORY_PAGE_ROW: u16 = 9;

#[derive(Debug, Clone)]
pub struct InventoryGridRules {
    slots: Vec<Option<ItemData>>,
}

impl InventoryGridRules {
    pub fn new() -> Self {
        Self {
            slots: vec![None; INVENTORY_MAX_NUM as usize],
        }
    }

    pub fn get_item(&self, slot: u16) -> Option<ItemData> {
        if slot >= INVENTORY_MAX_NUM {
            return None;
        }
        self.slots[slot as usize]
    }

    pub fn set_item(&mut self, slot: u16, item: Option<ItemData>) {
        if slot >= INVENTORY_MAX_NUM {
            return;
        }
        self.slots[slot as usize] = item;
    }

    pub fn is_empty(&self, slot: u16) -> bool {
        self.get_item(slot).is_none()
    }

    pub fn check_collision(&self, start_slot: u16, size: u8) -> bool {
        if size == 0 || size > 3 {
            return true; // Invalid size causes collision
        }
        if start_slot >= INVENTORY_MAX_NUM {
            return true;
        }

        let _page = start_slot / INVENTORY_PAGE_SIZE;
        let slot_in_page = start_slot % INVENTORY_PAGE_SIZE;
        let row_in_page = slot_in_page / INVENTORY_PAGE_COLUMN;

        // Check if item crosses bottom boundary of page
        if row_in_page + (size as u16) - 1 >= INVENTORY_PAGE_ROW {
            return true;
        }

        for i in 0..size {
            let target_slot = start_slot + (i as u16) * INVENTORY_PAGE_COLUMN;
            if target_slot >= INVENTORY_MAX_NUM {
                return true;
            }
            if !self.is_empty(target_slot) {
                return true; // Collision detected
            }
        }

        false
    }

    pub fn find_blank_slot(&self, size: u8) -> Option<u16> {
        if size == 0 || size > 3 {
            return None;
        }

        for slot in 0..INVENTORY_MAX_NUM {
            if !self.check_collision(slot, size) {
                return Some(slot);
            }
        }

        None
    }

    pub fn can_stack(&self, dst_slot: u16, src_item: &ItemData) -> bool {
        if dst_slot >= INVENTORY_MAX_NUM {
            return false;
        }

        if let Some(dst_item) = self.get_item(dst_slot) {
            if dst_item.vnum != src_item.vnum {
                return false;
            }

            // Check flags (C++ ITEM_FLAG_STACKABLE and ITEM_ANTIFLAG_STACK)
            if (dst_item.flags & ITEM_FLAG_STACKABLE) == 0 {
                return false;
            }
            if (dst_item.anti_flags & ITEM_ANTIFLAG_STACK) != 0 {
                return false;
            }

            // Stack split logic: max count is usually 200 in Metin2
            if dst_item.count as u32 + src_item.count as u32 > 200 {
                return false; // Cannot fully stack, would exceed 200 (though partial stacking is a thing, we'll return false for now or handle it depending on specific rule)
            }

            return true;
        }

        false
    }
}
