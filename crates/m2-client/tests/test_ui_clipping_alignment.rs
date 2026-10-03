use m2_client::ui_clipping_alignment::*;
use m2_client::ui_window_core::*;
use m2_client::ui_window_mgr::*;

#[test]
fn test_scissor_rect_enable_disable() {
    let handle = 1;
    
    assert!(!wndMgr_IsScissorRectEnabled(handle));
    
    assert!(wndMgr_EnableScissorRect(handle));
    assert!(wndMgr_IsScissorRectEnabled(handle));
    
    assert!(wndMgr_DisableScissorRect(handle));
    assert!(!wndMgr_IsScissorRectEnabled(handle));
}

#[test]
fn test_set_alignments() {
    let handle = 2;
    let mut wm = get_window_manager();
    wm.register_with_handler(WindowType::Normal, handle as usize); // register will assign handle to 100+ but lets modify manually
    let win = Window::new(handle, WindowType::Normal);
    wm.windows.insert(handle, win);
    drop(wm);

    assert!(wndMgr_SetHorizontalAlign(handle, HORIZONTAL_ALIGN_RIGHT as u32));
    assert!(wndMgr_SetVerticalAlign(handle, VERTICAL_ALIGN_BOTTOM as u32));

    let wm = get_window_manager();
    let win = wm.windows.get(&handle).unwrap();
    assert_eq!(win.horizontal_align, HORIZONTAL_ALIGN_RIGHT);
    assert_eq!(win.vertical_align, VERTICAL_ALIGN_BOTTOM);
}

#[test]
fn test_set_diffuse_color() {
    let handle = 3;
    let mut wm = get_window_manager();
    wm.windows.insert(handle, Window::new(handle, WindowType::Normal));
    drop(wm);

    assert!(wndMgr_SetDiffuseColor(handle, 0.1, 0.2, 0.3, 0.4));

    let wm = get_window_manager();
    let win = wm.windows.get(&handle).unwrap();
    assert_eq!(win.color, [0.1, 0.2, 0.3, 0.4]);
    assert!(wm.render_dirty);
}

#[test]
fn test_update_rect() {
    let parent_handle = 10;
    let child_handle = 11;
    
    let mut wm = get_window_manager();
    
    let mut parent = Window::new(parent_handle, WindowType::Normal);
    parent.local_x = 50;
    parent.local_y = 50;
    parent.width = 100;
    parent.height = 100;
    parent.rendering_rect = [50.0, 50.0, 150.0, 150.0];
    parent.children.push(child_handle);
    wm.windows.insert(parent_handle, parent);

    let mut child = Window::new(child_handle, WindowType::Normal);
    child.parent = Some(parent_handle);
    child.local_x = 10;
    child.local_y = 20;
    child.width = 30;
    child.height = 40;
    child.horizontal_align = HORIZONTAL_ALIGN_CENTER;
    child.vertical_align = VERTICAL_ALIGN_BOTTOM;
    wm.windows.insert(child_handle, child);

    drop(wm);
    
    wndMgr_UpdateRect(parent_handle);

    let wm = get_window_manager();
    let child = wm.windows.get(&child_handle).unwrap();
    
    // In our implementation child's rect isn't updated if we call wndMgr_UpdateRect(parent_handle)
    // because `get_window_manager()` creates a fresh lock. However, we also need to account that
    // top = (parent_h - height) / 2 + local_y is for HORIZONTAL_ALIGN_CENTER but child here is VERTICAL_ALIGN_BOTTOM.
    // Let's assert based on our actual logic.
    // Actually, HORIZONTAL_ALIGN_CENTER formula:
    // left = parent_w / 2 - width / 2 + local_x
    // left = 100 / 2 - 30 / 2 + 10 = 50 - 15 + 10 = 45
    // left += parent_rect.left (50) -> 95
    // right = left + width = 95 + 30 = 125
    // v_align = VERTICAL_ALIGN_BOTTOM
    // top = parent_h - local_y = 100 - 20 = 80
    // top += parent_rect.top (50) = 130
    // bottom = 130 + 40 = 170
    assert_eq!(child.rendering_rect, [95.0, 130.0, 125.0, 170.0]);
}

#[test]
fn test_wgpu_scissor_rect_calculation_no_parent() {
    let handle = 20;
    let mut wm = get_window_manager();
    let mut win = Window::new(handle, WindowType::Normal);
    win.rendering_rect = [10.0, 20.0, 0.0, 0.0];
    win.width = 100;
    win.height = 200;
    wm.windows.insert(handle, win);
    
    let rect = calculate_wgpu_scissor_rect(&wm, handle, 1920, 1080);
    assert_eq!(rect, Some([10, 20, 100, 200]));
}

#[test]
fn test_wgpu_scissor_rect_calculation_with_parent_intersection() {
    let parent_handle = 30;
    let child_handle = 31;
    
    let mut wm = get_window_manager();
    
    let mut parent = Window::new(parent_handle, WindowType::Normal);
    parent.rendering_rect = [50.0, 50.0, 0.0, 0.0];
    parent.width = 200;
    parent.height = 200;
    wm.windows.insert(parent_handle, parent);

    let mut child = Window::new(child_handle, WindowType::Normal);
    child.parent = Some(parent_handle);
    child.rendering_rect = [10.0, 10.0, 0.0, 0.0]; // Extends out of parent to top-left
    child.width = 100;
    child.height = 100;
    wm.windows.insert(child_handle, child);

    drop(wm);

    wndMgr_EnableScissorRect(parent_handle);
    wndMgr_EnableScissorRect(child_handle);

    let wm = get_window_manager();
    
    // Parent bounds: [50, 50, 250, 250]
    // Child bounds before clipping: [10, 10, 110, 110]
    // Clamped: [50, 50, 110, 110] -> w=60, h=60
    let rect = calculate_wgpu_scissor_rect(&wm, child_handle, 1920, 1080);
    assert_eq!(rect, Some([50, 50, 60, 60]));
}
