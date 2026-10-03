use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::panic::{catch_unwind, AssertUnwindSafe};
use crate::inventory_grid_v2::{get_inventory_sys, WindowType, ItemData as ItemDataV2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item {
    pub vnum: u32,
    pub count: u8,
}

fn safe_ffi_call<F, R>(default_val: R, f: F) -> R
where
    F: FnOnce() -> R,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(res) => res,
        Err(_) => default_val,
    }
}

pub fn set_item_for_test(window: u8, slot: u16, vnum: u32, count: u8) {
    if let Some(win) = WindowType::from_u8(window) {
        if let Ok(mut sys) = get_inventory_sys().lock() {
            let mut item = ItemDataV2::default();
            item.vnum = vnum;
            item.count = count;
            sys.set_item(win, slot, item);
        }
    }
}

pub fn reset_inventory_for_test() {
    if let Ok(mut sys) = get_inventory_sys().lock() {
        sys.gold = 0;
        // simplistic clear
        for w in [WindowType::INVENTORY, WindowType::EQUIPMENT, WindowType::SAFEBOX] {
            for slot in 0..180 { sys.del_item(w, slot); }
        }
    }
}

pub fn set_gold_for_test(gold: i64) {
    if let Ok(mut sys) = get_inventory_sys().lock() {
        sys.gold = gold;
    }
}

#[no_mangle]
pub extern "C" fn m2_inv_get_item(window: u8, slot: u16, out_vnum: *mut u32, out_count: *mut u8) -> bool {
    safe_ffi_call(false, || {
        if out_vnum.is_null() || out_count.is_null() { return false; }
        if let Some(win) = WindowType::from_u8(window) {
            if let Ok(sys) = get_inventory_sys().lock() {
                if let Some(item) = sys.get_item(win, slot) {
                    unsafe {
                        *out_vnum = item.vnum;
                        *out_count = item.count;
                    }
                    return true;
                }
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_inv_move_item(src_window: u8, src_cell: u16, dst_window: u8, dst_cell: u16, count: u8) -> bool {
    safe_ffi_call(false, || {
        if src_window == dst_window && src_cell == dst_cell { return false; }
        if count == 0 { return false; }
        let swin = match WindowType::from_u8(src_window) { Some(w) => w, None => return false };
        let dwin = match WindowType::from_u8(dst_window) { Some(w) => w, None => return false };
        if let Ok(mut sys) = get_inventory_sys().lock() {
            if let Some(src_item) = sys.get_item(swin, src_cell).copied() {
                if src_item.count < count { return false; }
                if let Some(dst_item) = sys.get_item(dwin, dst_cell).copied() {
                    if dst_item.vnum == src_item.vnum {
                        let mut new_dst = dst_item;
                        new_dst.count = dst_item.count.saturating_add(count);
                        sys.set_item(dwin, dst_cell, new_dst);
                        if src_item.count == count {
                            sys.del_item(swin, src_cell);
                        } else {
                            let mut new_src = src_item;
                            new_src.count -= count;
                            sys.set_item(swin, src_cell, new_src);
                        }
                        return true;
                    } else if src_item.count == count {
                        sys.set_item(dwin, dst_cell, src_item);
                        sys.set_item(swin, src_cell, dst_item);
                        return true;
                    }
                } else {
                    let mut new_dst = src_item;
                    new_dst.count = count;
                    sys.set_item(dwin, dst_cell, new_dst);
                    if src_item.count == count {
                        sys.del_item(swin, src_cell);
                    } else {
                        let mut new_src = src_item;
                        new_src.count -= count;
                        sys.set_item(swin, src_cell, new_src);
                    }
                    return true;
                }
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_inv_use_item(window: u8, cell: u16) -> bool {
    safe_ffi_call(false, || {
        let win = match WindowType::from_u8(window) { Some(w) => w, None => return false };
        if let Ok(mut sys) = get_inventory_sys().lock() {
            if let Some(item) = sys.get_item(win, cell).copied() {
                if item.count > 1 {
                    let mut new_item = item;
                    new_item.count -= 1;
                    sys.set_item(win, cell, new_item);
                } else {
                    sys.del_item(win, cell);
                }
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_inv_get_gold() -> i64 {
    safe_ffi_call(0, || {
        if let Ok(sys) = get_inventory_sys().lock() {
            sys.gold
        } else {
            0
        }
    })
}
