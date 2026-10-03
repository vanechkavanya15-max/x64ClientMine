#[path = "../src/ui_drag_drop_slot_controller.rs"]
mod ui_drag_drop_slot_controller;

use ui_drag_drop_slot_controller::{DragDropSlotController, DraggedItem, SlotWindowType};

#[test]
fn test_pick_up_item_success() {
    let mut controller = DragDropSlotController::new();
    assert_eq!(controller.get_dragged_item(), None);

    let result = controller.pick_up_item(SlotWindowType::Inventory, 5, 1234, 1);
    assert_eq!(result, Ok(()));

    let dragged = controller.get_dragged_item().unwrap();
    assert_eq!(dragged.window_type, SlotWindowType::Inventory);
    assert_eq!(dragged.slot_index, 5);
    assert_eq!(dragged.item_vnum, 1234);
    assert_eq!(dragged.count, 1);
}

#[test]
fn test_drop_on_slot_success() {
    let mut controller = DragDropSlotController::new();
    controller.pick_up_item(SlotWindowType::Inventory, 5, 1234, 1).unwrap();

    let result = controller.drop_on_slot(SlotWindowType::Inventory, 10);
    let expected = DraggedItem {
        window_type: SlotWindowType::Inventory,
        slot_index: 5,
        item_vnum: 1234,
        count: 1,
    };
    
    assert_eq!(result, Ok(expected));
    assert_eq!(controller.get_dragged_item(), None);
}

#[test]
fn test_drop_on_same_slot_fails() {
    let mut controller = DragDropSlotController::new();
    controller.pick_up_item(SlotWindowType::Inventory, 5, 1234, 1).unwrap();

    let result = controller.drop_on_slot(SlotWindowType::Inventory, 5);
    assert_eq!(result, Err("Cannot drop on the same slot"));
    assert_eq!(controller.get_dragged_item(), None);
}

#[test]
fn test_drop_on_ground_success() {
    let mut controller = DragDropSlotController::new();
    controller.pick_up_item(SlotWindowType::Equipment, 2, 9999, 1).unwrap();

    let result = controller.drop_on_ground();
    let expected = DraggedItem {
        window_type: SlotWindowType::Equipment,
        slot_index: 2,
        item_vnum: 9999,
        count: 1,
    };

    assert_eq!(result, Ok(expected));
    assert_eq!(controller.get_dragged_item(), None);
}

#[test]
fn test_cancel_drag() {
    let mut controller = DragDropSlotController::new();
    controller.pick_up_item(SlotWindowType::Inventory, 1, 100, 10).unwrap();
    
    controller.cancel();
    assert_eq!(controller.get_dragged_item(), None);
}

#[test]
fn test_pick_up_when_already_dragging_fails() {
    let mut controller = DragDropSlotController::new();
    controller.pick_up_item(SlotWindowType::Inventory, 1, 100, 1).unwrap();
    
    let result = controller.pick_up_item(SlotWindowType::Inventory, 2, 200, 1);
    assert_eq!(result, Err("An item is already being dragged"));
}

#[test]
fn test_drop_without_picking_up_fails() {
    let mut controller = DragDropSlotController::new();
    
    let result_slot = controller.drop_on_slot(SlotWindowType::Inventory, 1);
    assert_eq!(result_slot, Err("No item is currently being dragged"));
    
    let result_ground = controller.drop_on_ground();
    assert_eq!(result_ground, Err("No item is currently being dragged"));
}
