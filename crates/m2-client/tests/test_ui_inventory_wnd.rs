use m2_client::ui_inventory_wnd::{UiInventoryWnd, SlotStyle};

#[test]
fn test_integration_ui_inventory_wnd_initialization() {
    let instance = UiInventoryWnd::new();
    assert!(instance.is_ready());
    assert_eq!(instance.x_count, 0);
    assert_eq!(instance.y_count, 0);
    assert_eq!(instance.slot_style, SlotStyle::PickUp);
}

#[test]
fn test_arrange_grid_slot() {
    let mut instance = UiInventoryWnd::new();
    // Ekwipunek 5x9 (45 slotow)
    instance.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);
    
    assert_eq!(instance.x_count, 5);
    assert_eq!(instance.y_count, 9);
    assert_eq!(instance.width, 5 * 32);
    assert_eq!(instance.height, 9 * 32);
    
    let slot_ptr = instance.get_grid_slot_pointer(0, 0).expect("Should have a slot at 0,0");
    assert_eq!(slot_ptr.x_position, 0);
    assert_eq!(slot_ptr.y_position, 0);
    assert_eq!(slot_ptr.slot_number, 0);

    let slot_ptr = instance.get_grid_slot_pointer(4, 8).expect("Should have a slot at 4,8");
    assert_eq!(slot_ptr.x_position, 4 * 32);
    assert_eq!(slot_ptr.y_position, 8 * 32);
    assert_eq!(slot_ptr.slot_number, 44);
}

#[test]
fn test_get_picked_grid_slot_position() {
    let mut instance = UiInventoryWnd::new();
    instance.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    // Clicking top-left
    assert_eq!(instance.get_picked_grid_slot_position(5, 5), Some((0, 0)));
    // Clicking slot (1, 1) -> local position (33, 33)
    assert_eq!(instance.get_picked_grid_slot_position(40, 40), Some((1, 1)));
    // Clicking outside boundaries
    assert_eq!(instance.get_picked_grid_slot_position(-5, -5), None);
    assert_eq!(instance.get_picked_grid_slot_position(200, 300), None);
}

#[test]
fn test_get_picked_slot_list_1x1() {
    let mut instance = UiInventoryWnd::new();
    instance.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    // 1x1 item in slot (1, 1) -> local_x ~40, local_y ~40
    let slots = instance.get_picked_slot_list(1, 1, 40, 40).expect("Should pick slot");
    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0].slot_number, 6); // 1 + 1 * 5 = 6
}

#[test]
fn test_get_picked_slot_list_1x2() {
    let mut instance = UiInventoryWnd::new();
    instance.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    // 1x2 item in slot (2, 2)
    let slots = instance.get_picked_slot_list(1, 2, 70, 70).expect("Should pick slot");
    assert_eq!(slots.len(), 2);
    assert_eq!(slots[0].slot_number, 12); // (2, 2) -> 2 + 2 * 5 = 12
    assert_eq!(slots[1].slot_number, 17); // (2, 3) -> 2 + 3 * 5 = 17
}

#[test]
fn test_get_picked_slot_list_1x3() {
    let mut instance = UiInventoryWnd::new();
    instance.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    // 1x3 item in slot (2, 2)
    // C++ behavior: iyStart = iy - 1, iyEnd = iy + 1
    // iy = 2, so iyStart = 1, iyEnd = 3
    let slots = instance.get_picked_slot_list(1, 3, 70, 70).expect("Should pick slot");
    assert_eq!(slots.len(), 3);
    assert_eq!(slots[0].slot_number, 7);  // (2, 1)
    assert_eq!(slots[1].slot_number, 12); // (2, 2)
    assert_eq!(slots[2].slot_number, 17); // (2, 3)
}

#[test]
fn test_check_moving() {
    let mut instance = UiInventoryWnd::new();
    instance.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);
    
    // Default style is PickUp. Set up a slot that is occupied by an item.
    let slot = instance.get_grid_slot_pointer_mut(0, 0).unwrap();
    slot.center_slot_number = 0;
    slot.item_index = 100;
    
    let slot_ptr = instance.get_grid_slot_pointer(0, 0).unwrap();
    let slot_list = vec![slot_ptr];
    
    // Trying to move item 100 to slot 0
    assert!(instance.check_moving(0, 100, &slot_list));
    
    // Trying to move item 200 to slot 1 but it checks slot 0 which has item 100
    // Because center_slot_number != slot_number being passed
    assert!(!instance.check_moving(1, 200, &slot_list));
}

#[test]
fn test_on_refresh_slot() {
    let mut instance = UiInventoryWnd::new();
    instance.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);
    
    // Make slot (0,0) a 1x2 item
    {
        let slot = instance.get_grid_slot_pointer_mut(0, 0).unwrap();
        slot.is_item = true;
        slot.item_index = 500;
        slot.x_placed_item_size = 1;
        slot.y_placed_item_size = 2;
    }
    
    instance.on_refresh_slot();
    
    // Sub-slot (0,1) should now have center_slot_number = 0 and item_index = 500
    let sub_slot = instance.get_grid_slot_pointer(0, 1).unwrap();
    assert_eq!(sub_slot.center_slot_number, 0);
    assert_eq!(sub_slot.item_index, 500);
}

use m2_client::ui_inventory_wnd::EquipmentSlot;

#[test]
fn test_tab_switching() {
    let mut instance = UiInventoryWnd::new();
    assert_eq!(instance.get_tab(), 0);
    
    instance.set_tab(1);
    assert_eq!(instance.get_tab(), 1);
    
    // Test boundary (only 0 and 1 allowed)
    instance.set_tab(2);
    assert_eq!(instance.get_tab(), 1); // should not change
}

#[test]
fn test_equipment_slots() {
    let mut instance = UiInventoryWnd::new();
    assert_eq!(instance.get_equipment_item(EquipmentSlot::Weapon), None);
    
    instance.set_equipment_item(EquipmentSlot::Weapon, 100);
    instance.set_equipment_item(EquipmentSlot::Armor, 200);
    
    assert_eq!(instance.get_equipment_item(EquipmentSlot::Weapon), Some(100));
    assert_eq!(instance.get_equipment_item(EquipmentSlot::Armor), Some(200));
    assert_eq!(instance.get_equipment_item(EquipmentSlot::Helmet), None);
}

#[test]
fn test_yang_counter() {
    let mut instance = UiInventoryWnd::new();
    assert_eq!(instance.get_yang(), 0);
    
    instance.set_yang(999_999_999);
    assert_eq!(instance.get_yang(), 999_999_999);
}
