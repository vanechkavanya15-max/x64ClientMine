#[path = "../src/ui_widget_slotwindow.rs"]
mod ui_widget_slotwindow;

use ui_widget_slotwindow::{SlotWindow, SlotItem};

#[test]
fn test_slot_window_initialization() {
    let window = SlotWindow::new(5, 9, (32, 32));
    assert_eq!(window.columns, 5);
    assert_eq!(window.rows, 9);
    assert_eq!(window.cells.len(), 45);
    
    // Check first and last cells position
    assert_eq!(window.cells[0].x, 0);
    assert_eq!(window.cells[0].y, 0);
    assert_eq!(window.cells[44].x, 4 * 32);
    assert_eq!(window.cells[44].y, 8 * 32);
}

#[test]
fn test_get_cell_index_at_position() {
    let window = SlotWindow::new(5, 9, (32, 32));
    
    // Inside first cell
    assert_eq!(window.get_cell_index_at_position(0, 0), Some(0));
    assert_eq!(window.get_cell_index_at_position(16, 16), Some(0));
    assert_eq!(window.get_cell_index_at_position(31, 31), Some(0));
    
    // Inside second cell on first row
    assert_eq!(window.get_cell_index_at_position(32, 16), Some(1));
    
    // Inside first cell on second row
    assert_eq!(window.get_cell_index_at_position(16, 32), Some(5));
    
    // Out of bounds
    assert_eq!(window.get_cell_index_at_position(-1, 16), None);
    assert_eq!(window.get_cell_index_at_position(16, -1), None);
    assert_eq!(window.get_cell_index_at_position(5 * 32, 16), None);
    assert_eq!(window.get_cell_index_at_position(16, 9 * 32), None);
}

#[test]
fn test_on_mouse_over() {
    let mut window = SlotWindow::new(5, 9, (32, 32));
    
    let index = window.on_mouse_over(16, 16);
    assert_eq!(index, Some(0));
    assert_eq!(window.selected_slot, Some(0));
    
    let index = window.on_mouse_over(-10, -10);
    assert_eq!(index, None);
    assert_eq!(window.selected_slot, None);
}

#[test]
fn test_drag_and_drop_swap() {
    let mut window = SlotWindow::new(5, 9, (32, 32));
    
    let item1 = SlotItem { vnum: 100, count: 1, icon_path: "item1.png".to_string(), is_locked: false };
    let item2 = SlotItem { vnum: 200, count: 2, icon_path: "item2.png".to_string(), is_locked: false };
    
    assert!(window.set_item(0, item1.clone()).is_ok());
    assert!(window.set_item(1, item2.clone()).is_ok());
    
    // Start drag from cell 0
    let drag_start = window.on_mouse_left_down(16, 16);
    assert_eq!(drag_start, Some(0));
    assert_eq!(window.dragged_slot, Some(0));
    
    // Drop on cell 1 (swap)
    let drop_result = window.on_mouse_left_up(32 + 16, 16);
    assert_eq!(drop_result.unwrap(), Some((0, 1)));
    assert_eq!(window.dragged_slot, None);
    
    // Verify swap
    assert_eq!(window.cells[0].item.as_ref().unwrap().vnum, 200);
    assert_eq!(window.cells[1].item.as_ref().unwrap().vnum, 100);
}

#[test]
fn test_drag_and_drop_empty_slot() {
    let mut window = SlotWindow::new(5, 9, (32, 32));
    
    let item = SlotItem { vnum: 100, count: 1, icon_path: "item1.png".to_string(), is_locked: false };
    assert!(window.set_item(0, item.clone()).is_ok());
    
    // Start drag from cell 0
    window.on_mouse_left_down(16, 16);
    
    // Drop on empty cell 1
    let drop_result = window.on_mouse_left_up(32 + 16, 16);
    assert_eq!(drop_result.unwrap(), Some((0, 1)));
    
    // Verify move
    assert!(window.cells[0].item.is_none());
    assert_eq!(window.cells[1].item.as_ref().unwrap().vnum, 100);
}

#[test]
fn test_drag_and_drop_out_of_bounds() {
    let mut window = SlotWindow::new(5, 9, (32, 32));
    
    let item = SlotItem { vnum: 100, count: 1, icon_path: "item1.png".to_string(), is_locked: false };
    assert!(window.set_item(0, item.clone()).is_ok());
    
    // Start drag from cell 0
    window.on_mouse_left_down(16, 16);
    
    // Drop out of bounds
    let drop_result = window.on_mouse_left_up(-10, -10);
    assert_eq!(drop_result.unwrap(), None); // None means it was dropped out of bounds
    
    // Item should remain in original slot
    assert_eq!(window.cells[0].item.as_ref().unwrap().vnum, 100);
    assert_eq!(window.dragged_slot, None);
}

#[test]
fn test_drag_empty_slot() {
    let mut window = SlotWindow::new(5, 9, (32, 32));
    
    // Start drag from empty cell 0
    let drag_start = window.on_mouse_left_down(16, 16);
    assert_eq!(drag_start, None);
    assert_eq!(window.dragged_slot, None);
}

#[test]
fn test_on_mouse_right_click() {
    let mut window = SlotWindow::new(5, 9, (32, 32));
    
    let click_res = window.on_mouse_right_click(16, 16);
    assert_eq!(click_res, Some(0));
    
    let click_res = window.on_mouse_right_click(-10, -10);
    assert_eq!(click_res, None);
}
