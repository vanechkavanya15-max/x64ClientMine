use m2_client::ui_slot_window::*;

#[test]
fn test_integration_ui_slot_window() {
    let mut window = UiSlotWindow::new();
    assert!(window.is_ready());
    
    // Set properties
    window.set_slot_type(1);
    assert_eq!(window.slot_type, 1);
    
    window.set_slot_style(SlotStyle::Select);
    assert_eq!(window.slot_style, SlotStyle::Select);
    
    // Manage Slot
    window.append_slot(1, 10, 10, 32, 32);
    assert!(window.has_slot(1));
    assert_eq!(window.get_slot_count(), 1);
    
    // Check initial slot state
    let slot = window.get_slot(1).unwrap();
    assert_eq!(slot.slot_number, 1);
    assert_eq!(slot.x_position, 10);
    assert_eq!(slot.y_position, 10);
    assert_eq!(slot.x_cell_size, 32);
    assert_eq!(slot.y_cell_size, 32);
    assert!(!slot.is_item);
    assert_eq!(slot.state, 0);
    
    // Set slot
    window.set_slot(1, 1000, 1, 1);
    let slot = window.get_slot(1).unwrap();
    assert!(slot.is_item);
    assert_eq!(slot.item_index, 1000);
    assert_eq!(slot.x_placed_item_size, 1);
    assert_eq!(slot.y_placed_item_size, 1);
    
    // Set slot count
    window.set_slot_count(1, 5);
    let slot = window.get_slot(1).unwrap();
    assert_eq!(slot.count, 5);
    assert_eq!(slot.count_grade, None);
    
    // Set slot count new
    window.set_slot_count_new(1, 2, 3);
    let slot = window.get_slot(1).unwrap();
    assert_eq!(slot.count, 3);
    assert_eq!(slot.count_grade, Some(2));
    
    // Enable / Disable cover button
    window.set_always_render_cover_button(1, true);
    assert_eq!(window.get_slot(1).unwrap().state & SLOT_STATE_ALWAYS_RENDER_COVER, SLOT_STATE_ALWAYS_RENDER_COVER);
    window.set_always_render_cover_button(1, false);
    assert_eq!(window.get_slot(1).unwrap().state & SLOT_STATE_ALWAYS_RENDER_COVER, 0);
    
    // Lock / Unlock slot
    window.lock_slot(1);
    assert!(window.is_lock_slot(1));
    window.unlock_slot(1);
    assert!(!window.is_lock_slot(1));
    
    // Enable / Disable slot
    window.disable_slot(1);
    assert!(!window.is_enable_slot(1));
    window.enable_slot(1);
    assert!(window.is_enable_slot(1));
    
    // Use / Cant Use slot
    window.set_cant_use_slot(1);
    assert!(window.is_cant_use_slot(1));
    window.set_use_slot(1);
    assert!(!window.is_cant_use_slot(1));
    
    // Base image
    window.hide_slot_base_image(1);
    assert!(!window.get_slot(1).unwrap().render_base_slot_image);
    window.show_slot_base_image(1);
    assert!(window.get_slot(1).unwrap().render_base_slot_image);
    
    // Selection
    window.select_slot(1);
    assert!(window.is_selected_slot(1));
    assert_eq!(window.get_selected_slot_count(), 1);
    assert_eq!(window.get_selected_slot_number(0), 1);
    
    window.select_slot(1); // Deselect
    assert!(!window.is_selected_slot(1));
    assert_eq!(window.get_selected_slot_count(), 0);
    
    window.clear_selected();
    assert_eq!(window.get_selected_slot_count(), 0);
    
    // Clear slot
    window.clear_slot(1);
    let slot = window.get_slot(1).unwrap();
    assert!(!slot.is_item);
    assert_eq!(slot.item_index, 0);
    assert_eq!(slot.count, 0);
    assert_eq!(slot.state, 0);
    
    // Clear all
    window.clear_all_slot();
    assert_eq!(window.get_slot_count(), 0);
}
