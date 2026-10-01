use m2_client::py_bridge_wndmgr_native::*;
use m2_client::ui_window_mgr::{get_window_manager, WindowManager};
use m2_client::ui_window_core::WindowType;
use m2_client::ui_widget_slotwindow::SlotWindow;

fn create_test_slot_window() -> u32 {
    let mut wm = get_window_manager();
    let handle = wm.register(WindowType::SlotWindow);
    if let Some(win) = wm.windows.get_mut(&handle) {
        win.slot_window = Some(SlotWindow::new(5, 5, (32, 32)));
    }
    handle
}

#[test]
fn test_attach_icon() {
    wndMgr_AttachIcon(0, 1, 100, 2, 32, 32);
    assert!(wndMgr_IsAttaching(0));
    assert_eq!(wndMgr_GetAttachedItemIndex(0), 100);
    assert_eq!(wndMgr_GetAttachedSlotNumber(0), 2);
    
    wndMgr_DeattachIcon(0);
    assert!(!wndMgr_IsAttaching(0));
}

#[test]
fn test_append_slot() {
    let handle = create_test_slot_window();
    let res = wndMgr_AppendSlot(0, handle, 10, 0, 0, 32, 32);
    assert!(res);
    
    let wm = get_window_manager();
    let win = wm.windows.get(&handle).unwrap();
    let slot_win = win.slot_window.as_ref().unwrap();
    assert!(slot_win.has_slot(10));
}

#[test]
fn test_set_slot() {
    let handle = create_test_slot_window();
    wndMgr_AppendSlot(0, handle, 1, 0, 0, 32, 32);
    
    let res = wndMgr_SetSlot(0, handle, 1, 50200, 200, 1.0, 1.0, 1.0, 1.0);
    assert!(res);
    
    let wm = get_window_manager();
    let win = wm.windows.get(&handle).unwrap();
    let slot_win = win.slot_window.as_ref().unwrap();
    let cell = slot_win.cells.iter().find(|c| c.id == 1).unwrap();
    let item = cell.item.as_ref().unwrap();
    assert_eq!(item.vnum, 50200);
    assert_eq!(item.count, 200);
}

#[test]
fn test_clear_slot() {
    let handle = create_test_slot_window();
    wndMgr_AppendSlot(0, handle, 1, 0, 0, 32, 32);
    wndMgr_SetSlot(0, handle, 1, 50200, 200, 1.0, 1.0, 1.0, 1.0);
    
    let res = wndMgr_ClearSlot(0, handle, 1);
    assert!(res);
    
    let wm = get_window_manager();
    let win = wm.windows.get(&handle).unwrap();
    let slot_win = win.slot_window.as_ref().unwrap();
    let cell = slot_win.cells.iter().find(|c| c.id == 1).unwrap();
    assert!(cell.item.is_none());
}

#[test]
fn test_set_slot_count() {
    let handle = create_test_slot_window();
    wndMgr_AppendSlot(0, handle, 1, 0, 0, 32, 32);
    wndMgr_SetSlot(0, handle, 1, 50200, 200, 1.0, 1.0, 1.0, 1.0);
    
    let res = wndMgr_SetSlotCount(0, handle, 1, 150);
    assert!(res);
    
    let wm = get_window_manager();
    let win = wm.windows.get(&handle).unwrap();
    let slot_win = win.slot_window.as_ref().unwrap();
    let cell = slot_win.cells.iter().find(|c| c.id == 1).unwrap();
    let item = cell.item.as_ref().unwrap();
    assert_eq!(item.count, 150);
}

#[test]
fn test_set_events() {
    let handle = create_test_slot_window();
    assert!(wndMgr_SetOverInItemEvent(0, handle, 1234));
    assert!(wndMgr_SetOverOutItemEvent(0, handle, 5678));
    assert!(wndMgr_SetSelectItemSlotEvent(0, handle, 9012));
    
    let wm = get_window_manager();
    let win = wm.windows.get(&handle).unwrap();
    let slot_win = win.slot_window.as_ref().unwrap();
    assert_eq!(slot_win.over_in_event, Some(1234));
    assert_eq!(slot_win.over_out_event, Some(5678));
    assert_eq!(slot_win.select_item_event, Some(9012));
}

#[test]
fn test_invalid_handle() {
    let res = wndMgr_SetSlot(0, 999999, 1, 10, 1, 1.0, 1.0, 1.0, 1.0);
    assert!(!res);
}

#[test]
fn test_invalid_slot_id() {
    let handle = create_test_slot_window();
    let res = wndMgr_SetSlot(0, handle, 999, 10, 1, 1.0, 1.0, 1.0, 1.0);
    assert!(!res);
}