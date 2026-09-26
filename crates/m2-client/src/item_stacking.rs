pub const ITEM_MAX_COUNT: u16 = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item {
    pub vnum: u32,
    pub count: u16,
    pub has_unique_attributes: bool,
}

pub struct Inventory {
    pub slots: Vec<Option<Item>>,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
        }
    }

    /// Merges item from `src_slot` into `dst_slot`.
    /// Returns true if successful (fully or partially merged).
    pub fn merge_slots(&mut self, src_slot: usize, dst_slot: usize) -> Result<(), &'static str> {
        if src_slot == dst_slot {
            return Err("Cannot merge a slot into itself");
        }

        if src_slot >= self.slots.len() || dst_slot >= self.slots.len() {
            return Err("Slot index out of bounds");
        }

        let src_item = self.slots[src_slot].ok_or("Source slot is empty")?;
        let dst_item = self.slots[dst_slot].ok_or("Destination slot is empty")?;

        if src_item.vnum != dst_item.vnum {
            return Err("Items have different VNUMs");
        }

        if src_item.has_unique_attributes || dst_item.has_unique_attributes {
            return Err("Cannot merge items with unique attributes");
        }

        let total_count = src_item.count as u32 + dst_item.count as u32;

        if total_count > ITEM_MAX_COUNT as u32 {
            // Target gets ITEM_MAX_COUNT, source gets the remainder
            let mut new_dst = dst_item;
            new_dst.count = ITEM_MAX_COUNT;
            
            let mut new_src = src_item;
            new_src.count = (total_count - ITEM_MAX_COUNT as u32) as u16;

            self.slots[dst_slot] = Some(new_dst);
            self.slots[src_slot] = Some(new_src);
        } else {
            // Target gets total_count, source is cleared
            let mut new_dst = dst_item;
            new_dst.count = total_count as u16;
            
            self.slots[dst_slot] = Some(new_dst);
            self.slots[src_slot] = None;
        }

        Ok(())
    }

    /// Splits `count` items from `src_slot` into `dst_slot`.
    pub fn split_slot(&mut self, src_slot: usize, dst_slot: usize, count: u16) -> Result<(), &'static str> {
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

        if count > src_item.count {
            return Err("Not enough items to split");
        }

        let remaining_count = src_item.count - count;
        
        let mut new_dst = src_item;
        new_dst.count = count;

        self.slots[dst_slot] = Some(new_dst);

        if remaining_count == 0 {
            self.slots[src_slot] = None;
        } else {
            let mut new_src = src_item;
            new_src.count = remaining_count;
            self.slots[src_slot] = Some(new_src);
        }

        Ok(())
    }
}
