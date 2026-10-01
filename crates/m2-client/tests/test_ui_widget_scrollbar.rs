use m2_client::ui_widget_scrollbar::{ScrollBarState, ListBoxState, ScrollBarOrientation};

#[test]
fn test_scrollbar_initial_state() {
    let sb = ScrollBarState::new();
    assert_eq!(sb.orientation, ScrollBarOrientation::Vertical);
    assert_eq!(sb.get_pos(), 0.0);
    assert_eq!(sb.is_dragging_thumb, false);
}

#[test]
fn test_scrollbar_clamping() {
    let mut sb = ScrollBarState::new();
    sb.set_pos(1.5);
    assert_eq!(sb.get_pos(), 1.0);
    sb.set_pos(-0.5);
    assert_eq!(sb.get_pos(), 0.0);
}

#[test]
fn test_scrollbar_scroll_step() {
    let mut sb = ScrollBarState::new();
    sb.scroll_step = 0.1;
    sb.scroll_down();
    assert_eq!(sb.get_pos(), 0.1);
    sb.scroll_up();
    assert_eq!(sb.get_pos(), 0.0);
}

#[test]
fn test_scrollbar_mouse_wheel() {
    let mut sb = ScrollBarState::new();
    sb.scroll_step = 0.2;
    sb.handle_mouse_wheel(-120); // scroll down
    assert_eq!(sb.get_pos(), 0.2);
    sb.handle_mouse_wheel(120); // scroll up
    assert_eq!(sb.get_pos(), 0.0);
}

#[test]
fn test_scrollbar_drag_thumb() {
    let mut sb = ScrollBarState::new();
    sb.track_length = 100.0;
    sb.thumb_length = 20.0;
    
    // Movable track is 80.0
    sb.begin_drag_thumb(10.0);
    assert_eq!(sb.is_dragging_thumb, true);
    
    sb.handle_drag_thumb(50.0); // moved 40 pixels down
    // 40 / 80 = 0.5
    assert_eq!(sb.get_pos(), 0.5);
    
    sb.end_drag_thumb();
    assert_eq!(sb.is_dragging_thumb, false);
}

#[test]
fn test_scrollbar_track_click() {
    let mut sb = ScrollBarState::new();
    sb.track_length = 100.0;
    sb.thumb_length = 20.0;
    sb.page_size = 0.2;
    
    // Initial pos 0.0, thumb is at 0.0 - 20.0
    // Click below thumb (e.g. at 50.0)
    sb.handle_mouse_down_track(50.0);
    assert_eq!(sb.get_pos(), 0.2);
}

#[test]
fn test_listbox_operations() {
    let mut lb = ListBoxState::new();
    lb.append_item("Line 1".to_string());
    lb.append_item("Line 2".to_string());
    lb.append_item("Line 3".to_string());
    
    assert_eq!(lb.get_item_count(), 3);
    assert_eq!(lb.get_item_text(1).unwrap(), "Line 2");
    
    lb.insert_item(1, "Inserted Line".to_string());
    assert_eq!(lb.get_item_count(), 4);
    assert_eq!(lb.get_item_text(1).unwrap(), "Inserted Line");
    
    lb.remove_item(1);
    assert_eq!(lb.get_item_count(), 3);
    assert_eq!(lb.get_item_text(1).unwrap(), "Line 2");
}

#[test]
fn test_listbox_scrolling() {
    let mut lb = ListBoxState::new();
    lb.visible_lines = 2;
    lb.append_item("1".to_string());
    lb.append_item("2".to_string());
    lb.append_item("3".to_string());
    lb.append_item("4".to_string());
    
    // max offset should be 4 - 2 = 2
    lb.set_scroll_offset(5); // should clamp to 2
    assert_eq!(lb.scroll_offset, 2);
    
    lb.set_scroll_offset(0);
    lb.handle_mouse_wheel(-120); // scroll down
    assert_eq!(lb.scroll_offset, 1);
    lb.handle_mouse_wheel(120); // scroll up
    assert_eq!(lb.scroll_offset, 0);
}
