// test_slot_window_widget.rs

#[path = "../src/ui_wndmgr_widget_slot_window.rs"]
mod ui_wndmgr_widget_slot_window;

use ui_wndmgr_widget_slot_window::{SlotWindow};

#[test]
fn test_slot_definition_and_properties() {
    let mut window = SlotWindow::new();
    
    // Append slots
    window.append_slot(0, 0, 0, 32, 32);
    window.append_slot(1, 32, 0, 32, 32);
    
    assert_eq!(window.slots.len(), 2);
    
    // Test slot properties
    let slot = window.get_slot(1).unwrap();
    assert_eq!(slot.index, 1);
    assert_eq!(slot.x, 32);
    assert_eq!(slot.y, 0);
    assert_eq!(slot.width, 32);
    assert_eq!(slot.height, 32);
    assert_eq!(slot.vnum, 0);
    assert_eq!(slot.count, 0);
    
    // Test set_item
    assert!(window.set_item(1, 100, 5).is_ok());
    let slot_updated = window.get_slot(1).unwrap();
    assert_eq!(slot_updated.vnum, 100);
    assert_eq!(slot_updated.count, 5);
}

#[test]
fn test_hit_testing_coordinates() {
    let mut window = SlotWindow::new();
    
    window.append_slot(0, 0, 0, 32, 32);
    window.append_slot(1, 32, 0, 32, 32);
    
    // Set an item in slot 1 (will use placed_item_width/height if set)
    assert!(window.set_item(1, 100, 1).is_ok());
    
    // Test picking slot 0
    let picked = window.get_picked_slot(16, 16);
    assert!(picked.is_some());
    assert_eq!(picked.unwrap().index, 0);
    
    // Test picking slot 1
    let picked2 = window.get_picked_slot(40, 16);
    assert!(picked2.is_some());
    assert_eq!(picked2.unwrap().index, 1);
    
    // Out of bounds
    let picked3 = window.get_picked_slot(-5, -5);
    assert!(picked3.is_none());
}

#[test]
fn test_cooltime_logic_and_math() {
    let mut window = SlotWindow::new();
    window.append_slot(0, 0, 0, 32, 32);
    
    // Set cooltime
    assert!(window.set_cool_time(0, 10.0, 0.0).is_ok());
    
    let slot = window.get_slot(0).unwrap();
    assert_eq!(slot.cool_time, 10.0);
    assert_eq!(slot.start_cool_time, 0.0);
    
    // Test cooltime vertices calculation
    // Calculate for halfway (50%) done
    let current_time = 5.0;
    let vertices = window.calculate_cooltime_vertices(16.0, 16.0, 16.0, current_time, 0.0, 10.0);
    
    // For 50% done, there should be vertices calculated representing the darkened slice
    // 0.5 completion means tri_count should be (8.0 - 8.0*0.5) = 4
    // Vertices len should be 2 (center + top) + 4 (triangles) + 0 (last percentage) = 6
    assert_eq!(vertices.len(), 6);
    assert_eq!(vertices[0].x, 16.0); // center
    assert_eq!(vertices[0].y, 16.0);
}

#[test]
fn test_highlight_states_and_selection() {
    let mut window = SlotWindow::new();
    window.pos_x = 100;
    window.pos_y = 100;
    
    window.append_slot(0, 0, 0, 32, 32);
    window.append_slot(1, 32, 0, 32, 32);
    
    // Empty slots shouldn't be selected
    window.select_slot(0);
    assert!(!window.is_selected(0));
    
    // Add item and select
    assert!(window.set_item(1, 1234, 1).is_ok());
    window.select_slot(1);
    
    assert!(window.is_selected(1));
    assert_eq!(window.selected_slots.len(), 1);
    
    // Test highlight rect calculation
    let rect = window.get_highlight_rect(1);
    assert!(rect.is_some());
    let (left, top, right, bottom) = rect.unwrap();
    assert_eq!(left, 132); // 100 + 32
    assert_eq!(top, 100);  // 100 + 0
    assert_eq!(right, 164); // 132 + 32
    assert_eq!(bottom, 132); // 100 + 32
    
    window.clear_selected();
    assert!(!window.is_selected(1));
    assert_eq!(window.selected_slots.len(), 0);
}

#[test]
fn test_slot_locking() {
    let mut window = SlotWindow::new();
    window.append_slot(0, 0, 0, 32, 32);
    
    assert!(!window.is_locked(0));
    window.lock_slot(0);
    assert!(window.is_locked(0));
    window.unlock_slot(0);
    assert!(!window.is_locked(0));
}

#[test]
fn test_complex_hit_testing() {
    let mut window = SlotWindow::new();
    
    window.append_slot(0, 0, 0, 32, 32);
    
    // Set a 2x2 item in slot 0
    let slot = window.get_slot_mut(0).unwrap();
    slot.vnum = 123;
    slot.placed_item_width = 2;
    slot.placed_item_height = 2;
    
    // Test picking within the extended bounds (64x64)
    let picked1 = window.get_picked_slot(16, 16);
    assert!(picked1.is_some());
    assert_eq!(picked1.unwrap().index, 0);
    
    let picked2 = window.get_picked_slot(48, 48);
    assert!(picked2.is_some());
    assert_eq!(picked2.unwrap().index, 0);
    
    // Out of extended bounds
    let picked3 = window.get_picked_slot(70, 70);
    assert!(picked3.is_none());
}

#[test]
fn test_cooltime_full_coverage() {
    let mut window = SlotWindow::new();
    window.append_slot(0, 0, 0, 32, 32);
    
    // 0% completion -> Should have 10 vertices (center + top + 8 outer rim)
    let v_0 = window.calculate_cooltime_vertices(16.0, 16.0, 16.0, 0.0, 0.0, 10.0);
    assert_eq!(v_0.len(), 10);
    
    // 100% completion -> Should have 0 vertices (box is empty/invisible)
    let v_100 = window.calculate_cooltime_vertices(16.0, 16.0, 16.0, 10.0, 0.0, 10.0);
    assert_eq!(v_100.len(), 0);
    
    // 75% completion -> tri_count = 8 - 8*0.75 = 2. last percentage = 0.
    // 2 (center + top) + 2 triangles = 4 vertices.
    let v_75 = window.calculate_cooltime_vertices(16.0, 16.0, 16.0, 7.5, 0.0, 10.0);
    assert_eq!(v_75.len(), 4);
    
    // Test that time exceeding 1.0 also yields 0
    let v_over = window.calculate_cooltime_vertices(16.0, 16.0, 16.0, 15.0, 0.0, 10.0);
    assert_eq!(v_over.len(), 0);
    
    // Test negative times yield full box
    let v_under = window.calculate_cooltime_vertices(16.0, 16.0, 16.0, -5.0, 0.0, 10.0);
    assert_eq!(v_under.len(), 10);
}
