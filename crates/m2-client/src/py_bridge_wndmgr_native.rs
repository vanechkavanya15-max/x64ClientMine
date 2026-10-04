// py_bridge_wndmgr_native.rs - Drag&Drop Icon and SlotWindow implementation for Metin2 Pure Rust

use crate::ui_window_mgr::get_window_manager;
use crate::ui_window_core::WindowType;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn wndMgr_AttachIcon(
    _py_obj: usize,
    icon_type: c_int,
    icon_index: c_int,
    slot_number: c_int,
    width: c_int,
    height: c_int,
) -> bool {
    let mut wm = get_window_manager();
    wm.attach_icon(
        icon_type as u32,
        icon_index as u32,
        slot_number as u32,
        width as u8,
        height as u8,
    );
    true
}

#[no_mangle]
pub extern "C" fn wndMgr_DeattachIcon(_py_obj: usize) -> bool {
    get_window_manager().deattach_icon();
    true
}

#[no_mangle]
pub extern "C" fn wndMgr_IsAttaching(_py_obj: usize) -> bool {
    get_window_manager().is_attaching
}

#[no_mangle]
pub extern "C" fn wndMgr_GetAttachedSlotNumber(_py_obj: usize) -> u32 {
    get_window_manager().get_attaching_slot_number()
}

#[no_mangle]
pub extern "C" fn wndMgr_GetAttachedItemIndex(_py_obj: usize) -> u32 {
    get_window_manager().get_attaching_index()
}

#[no_mangle]
pub extern "C" fn wndMgr_AppendSlot(
    _py_obj: usize,
    handle: u32,
    slot_idx: c_int,
    x: c_int,
    y: c_int,
    w: c_int,
    h: c_int,
) -> bool {
    let mut wm = get_window_manager();
    if let Some(win) = wm.windows.get_mut(&handle) {
        if win.window_type == WindowType::SlotWindow {
            if let Some(ref mut slot_win) = win.slot_window {
                slot_win.append_slot(slot_idx as usize, x as i32, y as i32, w as i32, h as i32);
                return true;
            }
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn wndMgr_SetSlot(
    _py_obj: usize,
    handle: u32,
    slot_idx: c_int,
    item_vnum: c_int,
    count: c_int,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) -> bool {
    let mut wm = get_window_manager();
    if let Some(win) = wm.windows.get_mut(&handle) {
        if win.window_type == WindowType::SlotWindow {
            if let Some(ref mut slot_win) = win.slot_window {
                return slot_win
                    .set_slot(slot_idx as usize, item_vnum as u32, count as u32, [r, g, b, a])
                    .is_ok();
            }
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn wndMgr_ClearSlot(_py_obj: usize, handle: u32, slot_idx: c_int) -> bool {
    let mut wm = get_window_manager();
    if let Some(win) = wm.windows.get_mut(&handle) {
        if win.window_type == WindowType::SlotWindow {
            if let Some(ref mut slot_win) = win.slot_window {
                return slot_win.clear_slot(slot_idx as usize).is_ok();
            }
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn wndMgr_SetSlotCount(_py_obj: usize, handle: u32, slot_idx: c_int, count: c_int) -> bool {
    let mut wm = get_window_manager();
    if let Some(win) = wm.windows.get_mut(&handle) {
        if win.window_type == WindowType::SlotWindow {
            if let Some(ref mut slot_win) = win.slot_window {
                return slot_win.set_slot_count(slot_idx as usize, count as u32).is_ok();
            }
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn wndMgr_SetOverInItemEvent(_py_obj: usize, handle: u32, event_id: u32) -> bool {
    let mut wm = get_window_manager();
    if let Some(win) = wm.windows.get_mut(&handle) {
        if win.window_type == WindowType::SlotWindow {
            if let Some(ref mut slot_win) = win.slot_window {
                slot_win.over_in_event = Some(event_id);
                return true;
            }
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn wndMgr_SetOverOutItemEvent(_py_obj: usize, handle: u32, event_id: u32) -> bool {
    let mut wm = get_window_manager();
    if let Some(win) = wm.windows.get_mut(&handle) {
        if win.window_type == WindowType::SlotWindow {
            if let Some(ref mut slot_win) = win.slot_window {
                slot_win.over_out_event = Some(event_id);
                return true;
            }
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn wndMgr_SetSelectItemSlotEvent(_py_obj: usize, handle: u32, event_id: u32) -> bool {
    let mut wm = get_window_manager();
    if let Some(win) = wm.windows.get_mut(&handle) {
        if win.window_type == WindowType::SlotWindow {
            if let Some(ref mut slot_win) = win.slot_window {
                slot_win.select_item_event = Some(event_id);
                return true;
            }
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn wndMgr_SetSlotCoolTime(
    _py_obj: usize,
    handle: u32,
    slot_idx: c_int,
    cool_time: f32,
    elapsed_time: f32,
) -> bool {
    let mut wm = get_window_manager();
    wm.set_slot_cool_time(handle, slot_idx as usize, cool_time, elapsed_time)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetClipRect(
    _py_obj: usize,
    handle: u32,
    l: f32,
    t: f32,
    r: f32,
    b: f32,
) -> bool {
    let mut wm = get_window_manager();
    wm.set_clip_rect(handle, l, t, r, b)
}
