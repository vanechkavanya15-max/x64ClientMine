use crate::inventory_grid_v2::{get_inventory_sys, WindowType, ItemData as ItemDataV2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemSize {
    Size1x1 = 1,
    Size1x2 = 2,
    Size1x3 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
}

pub struct InventoryGrid {
    slots: [bool; 180],
    items: [Option<ItemData>; 180],
}

impl InventoryGrid {
    pub fn new() -> Self {
        Self {
            slots: [false; 180],
            items: [None; 180],
        }
    }

    pub fn can_put_item(&self, slot: u16, size: u8) -> bool {
        if size == 0 || size > 3 || slot >= 180 {
            return false;
        }

        let row = (slot % 45) / 5;
        if row + (size as u16) > 9 {
            return false;
        }

        for i in 0..size {
            let s = slot + (i as u16) * 5;
            if s >= 180 || self.slots[s as usize] {
                return false;
            }
        }

        true
    }

    pub fn put_item(&mut self, slot: u16, size: u8) -> Result<(), &'static str> {
        if !self.can_put_item(slot, size) {
            return Err("Cannot put item");
        }

        for i in 0..size {
            let s = slot + (i as u16) * 5;
            self.slots[s as usize] = true;
        }

        Ok(())
    }

    pub fn remove_item(&mut self, slot: u16, size: u8) {
        if size == 0 || size > 3 || slot >= 180 {
            return;
        }
        for i in 0..size {
            let s = (slot + (i as u16) * 5) as usize;
            if s < 180 {
                self.slots[s] = false;
            }
        }
        self.items[slot as usize] = None;
    }

    pub fn find_free_slot(&self, size: u8) -> Option<u16> {
        for slot in 0..180 {
            if self.can_put_item(slot, size) {
                return Some(slot);
            }
        }
        None
    }

    pub fn set_item(&mut self, slot: u16, item: ItemData) -> bool {
        if slot >= 180 {
            return false;
        }
        self.items[slot as usize] = Some(item);
        self.slots[slot as usize] = true;
        true
    }

    pub fn get_item(&self, slot: u16) -> Option<&ItemData> {
        if slot >= 180 {
            return None;
        }
        self.items[slot as usize].as_ref()
    }

    pub fn swap_items(&mut self, slot_a: u16, slot_b: u16) -> bool {
        if slot_a >= 180 || slot_b >= 180 {
            return false;
        }
        let a = slot_a as usize;
        let b = slot_b as usize;
        self.items.swap(a, b);
        self.slots.swap(a, b);
        true
    }

    pub fn split_stack(&mut self, src_slot: u16, dst_slot: u16, count: u8) -> bool {
        if src_slot >= 180 || dst_slot >= 180 || count == 0 {
            return false;
        }
        if self.items[dst_slot as usize].is_some() {
            return false;
        }
        let src_item = match self.items[src_slot as usize].as_mut() {
            Some(it) => it,
            None => return false,
        };
        if count >= src_item.count {
            return false;
        }
        src_item.count -= count;
        let mut new_item = *src_item;
        new_item.count = count;
        self.items[dst_slot as usize] = Some(new_item);
        self.slots[dst_slot as usize] = true;
        true
    }
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
pub extern "C" fn m2_inv_set_item(cell: u16, vnum: u32, count: u8, flags: u32, anti_flags: u32) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut sys) = get_inventory_sys().lock() {
            let mut item = ItemDataV2::default();
            item.vnum = vnum;
            item.count = count;
            item.flags = flags;
            item.anti_flags = anti_flags;
            sys.set_item(WindowType::INVENTORY, cell, item)
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_inv_del_item(cell: u16) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut sys) = get_inventory_sys().lock() {
            sys.del_item(WindowType::INVENTORY, cell)
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_inv_swap(cell_a: u16, cell_b: u16) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut sys) = get_inventory_sys().lock() {
            let item_a = sys.get_item(WindowType::INVENTORY, cell_a).copied();
            let item_b = sys.get_item(WindowType::INVENTORY, cell_b).copied();
            
            if let Some(ia) = item_a { sys.del_item(WindowType::INVENTORY, cell_a); }
            if let Some(ib) = item_b { sys.del_item(WindowType::INVENTORY, cell_b); }
            
            if let Some(ia) = item_a { sys.set_item(WindowType::INVENTORY, cell_b, ia); }
            if let Some(ib) = item_b { sys.set_item(WindowType::INVENTORY, cell_a, ib); }
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_inv_find_empty() -> i32 {
    safe_ffi_call(-1, || {
        if let Ok(sys) = get_inventory_sys().lock() {
            // Simplified find_empty for 1x1 size
            for slot in 0..180 {
                if sys.get_item(WindowType::INVENTORY, slot).is_none() {
                    return slot as i32;
                }
            }
        }
        -1
    })
}
