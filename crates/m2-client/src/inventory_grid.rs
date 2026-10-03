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
