use crate::item_attributes::{ItemAttribute, ITEM_ATTRIBUTE_SLOT_MAX_NUM};

pub const ITEM_FLAG_STACKABLE: u32 = 1 << 2;
pub const ITEM_MAX_COUNT: u16 = 200;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ItemV2 {
    pub vnum: u32,
    pub count: u16,
    pub flags: u32,
    pub sockets: [i32; 3],
    pub attributes: [ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl Eq for ItemV2 {}

impl ItemV2 {
    pub fn can_stack_with(&self, other: &ItemV2) -> bool {
        if self.vnum != other.vnum {
            return false;
        }

        if (self.flags & ITEM_FLAG_STACKABLE) == 0 || (other.flags & ITEM_FLAG_STACKABLE) == 0 {
            return false;
        }

        if self.sockets != other.sockets {
            return false;
        }

        if self.attributes != other.attributes {
            return false;
        }
        
        let total = (self.count as u32) + (other.count as u32);
        if total > ITEM_MAX_COUNT as u32 {
            return false;
        }

        true
    }
}

use m2_client_bridge::ffi_guard::safe_ffi_call;

#[no_mangle]
pub extern "C" fn m2_item_can_stack(item_a: *const ItemV2, item_b: *const ItemV2) -> bool {
    safe_ffi_call(false, || {
        if item_a.is_null() || item_b.is_null() {
            return false;
        }

        let a = unsafe { &*item_a };
        let b = unsafe { &*item_b };

        a.can_stack_with(b)
    })
}

pub struct InventoryV2 {
    pub slots: Vec<Option<ItemV2>>,
}

impl InventoryV2 {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
        }
    }

    pub fn split_stack(&mut self, src_slot: usize, dst_slot: usize, count: u16) -> Result<(), &'static str> {
        if src_slot == dst_slot {
            return Err("Cannot split into the same slot");
        }

        if src_slot >= self.slots.len() || dst_slot >= self.slots.len() {
            return Err("Slot index out of bounds");
        }

        if self.slots[dst_slot].is_some() {
            return Err("Destination slot is not empty");
        }

        if count == 0 {
            return Err("Cannot split 0 items");
        }

        let src_item = self.slots[src_slot].ok_or("Source slot is empty")?;

        if count >= src_item.count {
            return Err("Not enough items to split");
        }

        let remaining_count = src_item.count - count;
        
        let mut new_dst = src_item;
        new_dst.count = count;

        self.slots[dst_slot] = Some(new_dst);

        let mut new_src = src_item;
        new_src.count = remaining_count;
        self.slots[src_slot] = Some(new_src);

        Ok(())
    }
}
