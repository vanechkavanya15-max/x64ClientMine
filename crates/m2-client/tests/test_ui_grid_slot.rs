use m2_client::ui_grid_slot::{UiGridSlot, SlotStyle};

#[test]
fn test_integration_ui_grid_slot() {
    let instance = UiGridSlot::new();
    assert!(instance.is_ready());
}

#[test]
fn test_arrange_grid_slot() {
    let mut grid = UiGridSlot::new();
    // 5x9 grid (like standard Metin2 inventory page)
    grid.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    assert_eq!(grid.x_count, 5);
    assert_eq!(grid.y_count, 9);
    assert_eq!(grid.width, 5 * 32);
    assert_eq!(grid.height, 9 * 32);
    assert_eq!(grid.slot_vector.len(), 45);

    let slot = grid.get_grid_slot_pointer(0, 0).unwrap();
    assert_eq!(slot.dw_slot_number, 0);
    assert_eq!(slot.ix_position, 0);
    assert_eq!(slot.iy_position, 0);

    let slot_end = grid.get_grid_slot_pointer(4, 8).unwrap();
    assert_eq!(slot_end.dw_slot_number, 44);
    assert_eq!(slot_end.ix_position, 4 * 32);
    assert_eq!(slot_end.iy_position, 8 * 32);
}

#[test]
fn test_get_picked_grid_slot_position() {
    let mut grid = UiGridSlot::new();
    grid.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    // Pick top-left slot
    assert_eq!(grid.get_picked_grid_slot_position(10, 10), Some((0, 0)));
    // Pick next slot
    assert_eq!(grid.get_picked_grid_slot_position(40, 10), Some((1, 0)));
    // Pick bottom-right slot
    assert_eq!(grid.get_picked_grid_slot_position(150, 270), Some((4, 8)));
    
    // Out of bounds
    assert_eq!(grid.get_picked_grid_slot_position(-10, -10), None);
    assert_eq!(grid.get_picked_grid_slot_position(200, 300), None);
}

#[test]
fn test_get_picked_slot_list_single_item() {
    let mut grid = UiGridSlot::new();
    grid.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    // 1x1 item at (10, 10) which corresponds to (0, 0)
    let slots = grid.get_picked_slot_list(1, 1, 10, 10).unwrap();
    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0].dw_slot_number, 0);
}

#[test]
fn test_get_picked_slot_list_multi_item() {
    let mut grid = UiGridSlot::new();
    grid.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    // 1x2 item at (10, 40) which corresponds to (0, 1) -> picks (0,1) and (0,2)
    let slots = grid.get_picked_slot_list(1, 2, 10, 40).unwrap();
    assert_eq!(slots.len(), 2);
    assert_eq!(slots[0].dw_slot_number, 5); // 0 + 1 * 5
    assert_eq!(slots[1].dw_slot_number, 10); // 0 + 2 * 5
}

#[test]
fn test_get_picked_slot_list_edge_cases() {
    let mut grid = UiGridSlot::new();
    grid.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    // Test bound correction logic for 1x3 item placed at bottom
    // Place at (0, 8) with height 3 -> (1, 3). Will push it up to fit
    let slots = grid.get_picked_slot_list(1, 3, 10, 8 * 32 + 10).unwrap();
    assert_eq!(slots.len(), 3);
    assert_eq!(slots[0].dw_slot_number, 30); // 0 + 6 * 5
    assert_eq!(slots[1].dw_slot_number, 35); // 0 + 7 * 5
    assert_eq!(slots[2].dw_slot_number, 40); // 0 + 8 * 5
}

#[test]
fn test_on_refresh_slot() {
    let mut grid = UiGridSlot::new();
    grid.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    // Set item 1x3 at (0,0)
    if let Some(slot) = grid.get_grid_slot_pointer_mut(0, 0) {
        slot.is_item = true;
        slot.by_x_placed_item_size = 1;
        slot.by_y_placed_item_size = 3;
        slot.dw_item_index = 5001;
    }

    grid.on_refresh_slot();

    // The center slot for (0,0), (0,1), (0,2) should be 0
    let s0 = grid.get_grid_slot_pointer(0, 0).unwrap();
    assert_eq!(s0.dw_center_slot_number, 0);
    assert_eq!(s0.dw_item_index, 5001);

    let s1 = grid.get_grid_slot_pointer(0, 1).unwrap();
    assert_eq!(s1.dw_center_slot_number, 0);
    assert_eq!(s1.dw_item_index, 5001);

    let s2 = grid.get_grid_slot_pointer(0, 2).unwrap();
    assert_eq!(s2.dw_center_slot_number, 0);
    assert_eq!(s2.dw_item_index, 5001);

    // Unrelated slot should not have item
    let s3 = grid.get_grid_slot_pointer(0, 3).unwrap();
    assert_eq!(s3.dw_center_slot_number, 15);
    assert_eq!(s3.dw_item_index, 0);
}

#[test]
fn test_check_moving() {
    let mut grid = UiGridSlot::new();
    grid.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);
    
    // Create an item taking up 2 slots
    if let Some(slot) = grid.get_grid_slot_pointer_mut(0, 0) {
        slot.is_item = true;
        slot.by_x_placed_item_size = 1;
        slot.by_y_placed_item_size = 2;
        slot.dw_item_index = 1010;
    }
    grid.on_refresh_slot();

    // Default style is PickUp. Trying to put another item in an occupied slot should fail,
    // unless dw_item_index matches (same item picked up again).

    // Getting slot list for 1x1 at (0,0)
    let slot_list = grid.get_picked_slot_list(1, 1, 10, 10).unwrap();
    
    // Trying to move a different item into occupied slot (should fail)
    assert_eq!(grid.check_moving(999, 2020, &slot_list), false);
    
    // Trying to move the SAME item into occupied slot (should pass)
    assert_eq!(grid.check_moving(999, 1010, &slot_list), true);

    // If style is Select, check_moving should always return true
    grid.slot_style = SlotStyle::Select;
    assert_eq!(grid.check_moving(999, 2020, &slot_list), true);
}

#[test]
fn test_get_picked_slot_pointer() {
    let mut grid = UiGridSlot::new();
    grid.arrange_grid_slot(0, 5, 9, 32, 32, 0, 0);

    // Make an item at (0,0) that is 1x2
    if let Some(slot) = grid.get_grid_slot_pointer_mut(0, 0) {
        slot.is_item = true;
        slot.by_x_placed_item_size = 1;
        slot.by_y_placed_item_size = 2;
        slot.dw_item_index = 123;
    }
    grid.on_refresh_slot();

    // Click on (0,1) with a 1x1 size cursor
    let picked_slot = grid.get_picked_slot_pointer(1, 1, 10, 40, None).unwrap();
    
    // It should return the center slot which is (0,0)
    assert_eq!(picked_slot.dw_slot_number, 0);
    assert_eq!(picked_slot.dw_center_slot_number, 0);
}
