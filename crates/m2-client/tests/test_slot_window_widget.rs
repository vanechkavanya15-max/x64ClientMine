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
