#[path = "../src/full-input-win32_08.rs"]
mod full_input_win32_08;

use full_input_win32_08::{DragDropSlotController, DraggedItem, SlotWindowType};

#[test]
fn test_reexport_pick_up_item() {
    let mut controller = DragDropSlotController::new();
    let res = controller.pick_up_item(SlotWindowType::Equipment, 1, 555, 2);
    assert_eq!(res, Ok(()));
    
    let item = controller.get_dragged_item().unwrap();
    assert_eq!(item.window_type, SlotWindowType::Equipment);
    assert_eq!(item.slot_index, 1);
    assert_eq!(item.item_vnum, 555);
    assert_eq!(item.count, 2);
}

#[test]
fn test_reexport_drop_on_slot() {
    let mut controller = DragDropSlotController::new();
    controller.pick_up_item(SlotWindowType::Exchange, 0, 111, 1).unwrap();
    
    let dropped = controller.drop_on_slot(SlotWindowType::Inventory, 5).unwrap();
    assert_eq!(dropped.item_vnum, 111);
    assert_eq!(controller.get_dragged_item(), None);
}

#[test]
fn test_reexport_drop_on_same_slot() {
    let mut controller = DragDropSlotController::new();
    controller.pick_up_item(SlotWindowType::SafeBox, 10, 777, 10).unwrap();
    
    let result = controller.drop_on_slot(SlotWindowType::SafeBox, 10);
    assert!(result.is_err());
    assert_eq!(controller.get_dragged_item(), None); // State is reset
}

#[test]
fn test_reexport_drop_on_ground() {
    let mut controller = DragDropSlotController::new();
    controller.pick_up_item(SlotWindowType::QuickSlot, 3, 222, 5).unwrap();
    
    let dropped = controller.drop_on_ground().unwrap();
    assert_eq!(dropped.item_vnum, 222);
    assert_eq!(controller.get_dragged_item(), None);
}

#[test]
fn test_reexport_cancel() {
    let mut controller = DragDropSlotController::new();
    controller.pick_up_item(SlotWindowType::Inventory, 20, 333, 1).unwrap();
    
    controller.cancel();
    assert_eq!(controller.get_dragged_item(), None);
}
