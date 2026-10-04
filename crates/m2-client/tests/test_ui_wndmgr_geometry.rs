use m2_client::ui_wndmgr_geometry::WindowGeometry;

#[test]
fn test_child_global_position() {
    let mut parent = WindowGeometry::new();
    parent.set_position(100.0, 50.0);
    parent.set_size(800.0, 600.0);
    
    let mut child = WindowGeometry::new();
    child.set_position(10.0, 20.0);
    
    let parent_global = parent.get_global_position(None);
    assert_eq!(parent_global, (100.0, 50.0));
    
    let child_global = child.get_global_position(Some(parent_global));
    assert_eq!(child_global, (110.0, 70.0));
}

#[test]
fn test_parent_clipping_rect_inheritance() {
    let mut parent = WindowGeometry::new();
    parent.set_clip_rect(50.0, 50.0, 200.0, 200.0);
    
    let mut child = WindowGeometry::new();
    // Child has no clip rect, inherits parent's
    let effective = child.get_effective_clip_rect(parent.clip_rect);
    assert_eq!(effective, Some([50.0, 50.0, 200.0, 200.0]));
    
    // Child has its own clip rect, intersection is computed
    child.set_clip_rect(100.0, 100.0, 300.0, 300.0);
    let effective2 = child.get_effective_clip_rect(parent.clip_rect);
    assert_eq!(effective2, Some([100.0, 100.0, 200.0, 200.0]));
    
    // Non-overlapping clip rects
    child.set_clip_rect(300.0, 300.0, 400.0, 400.0);
    let effective3 = child.get_effective_clip_rect(parent.clip_rect);
    // Should be an empty rectangle at (300.0, 300.0)
    assert_eq!(effective3, Some([300.0, 300.0, 300.0, 300.0]));
}

#[test]
fn test_is_in_rect_boundary_logic() {
    let mut win = WindowGeometry::new();
    win.set_position(100.0, 100.0);
    win.set_size(100.0, 100.0);
    
    let global_pos = win.get_global_position(None);
    
    // Inside bounds, no clip rect
    assert!(win.is_in_rect(150.0, 150.0, global_pos, None));
    
    // Outside bounds, no clip rect
    assert!(!win.is_in_rect(50.0, 150.0, global_pos, None));
    assert!(!win.is_in_rect(250.0, 150.0, global_pos, None));
    
    // Add clip rect that restricts the bounds
    let effective_clip = Some([125.0, 125.0, 175.0, 175.0]);
    
    // Inside bounds and clip rect
    assert!(win.is_in_rect(150.0, 150.0, global_pos, effective_clip));
    
    // Inside bounds but outside clip rect
    assert!(!win.is_in_rect(110.0, 150.0, global_pos, effective_clip));
    assert!(!win.is_in_rect(190.0, 150.0, global_pos, effective_clip));
}

#[test]
fn test_empty_intersection_clip_rect() {
    let mut win = WindowGeometry::new();
    win.set_position(0.0, 0.0);
    win.set_size(500.0, 500.0);
    win.set_clip_rect(200.0, 200.0, 300.0, 300.0);

    let parent_clip = Some([0.0, 0.0, 100.0, 100.0]); // No intersection

    let effective = win.get_effective_clip_rect(parent_clip);

    // Assuming our logic returns an empty rect
    assert!(effective.is_some());
    let rect = effective.unwrap();
    // width or height should be <= 0
    assert!(rect[2] <= rect[0] || rect[3] <= rect[1]);

    let global_pos = win.get_global_position(None);
    // Even if it's inside the original window, it should be outside the effective empty clip rect
    assert!(!win.is_in_rect(250.0, 250.0, global_pos, effective));
}
