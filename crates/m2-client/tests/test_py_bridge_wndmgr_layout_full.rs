#[path = "../src/py_bridge_wndmgr_layout_full.rs"]
mod py_bridge_wndmgr_layout_full;

use py_bridge_wndmgr_layout_full::*;
use std::ffi::CString;

#[test]
fn test_wndmgr_register_and_destroy() {
    let layer = CString::new("UI").unwrap();
    let id1 = m2_py_wndmgr_register(WT_NORMAL, layer.as_ptr());
    assert!(id1 > 0);

    let id2 = m2_py_wndmgr_register(WT_BUTTON, layer.as_ptr());
    assert!(id2 > 0);
    assert_ne!(id1, id2);

    let mut mgr = get_wnd_mgr().lock().unwrap();
    assert!(mgr.windows.contains_key(&id1));
    assert!(mgr.windows.contains_key(&id2));
    
    // Test destroy
    mgr.destroy_window(id1);
    assert!(!mgr.windows.contains_key(&id1));
}

#[test]
fn test_wndmgr_set_position_and_size() {
    let layer = CString::new("UI").unwrap();
    let win_id = m2_py_wndmgr_register(WT_NORMAL, layer.as_ptr());

    m2_py_wndmgr_set_position(win_id, 100, 200);
    m2_py_wndmgr_set_size(win_id, 300, 400);

    let mgr = get_wnd_mgr().lock().unwrap();
    let win = mgr.windows.get(&win_id).unwrap();
    
    assert_eq!(win.x, 100);
    assert_eq!(win.y, 200);
    assert_eq!(win.width, 300);
    assert_eq!(win.height, 400);
    
    // Rect should be updated
    assert_eq!(win.rect.left, 100);
    assert_eq!(win.rect.top, 200);
    assert_eq!(win.rect.right, 400);
    assert_eq!(win.rect.bottom, 600);
}

#[test]
fn test_wndmgr_show_hide() {
    let layer = CString::new("UI").unwrap();
    let win_id = m2_py_wndmgr_register(WT_NORMAL, layer.as_ptr());

    assert!(!m2_py_wndmgr_is_show(win_id));

    m2_py_wndmgr_show(win_id);
    assert!(m2_py_wndmgr_is_show(win_id));

    m2_py_wndmgr_hide(win_id);
    assert!(!m2_py_wndmgr_is_show(win_id));
}

#[test]
fn test_wndmgr_focus_kill_focus() {
    let layer = CString::new("UI").unwrap();
    let win1 = m2_py_wndmgr_register(WT_NORMAL, layer.as_ptr());
    let win2 = m2_py_wndmgr_register(WT_NORMAL, layer.as_ptr());

    m2_py_wndmgr_set_focus(win1);
    assert!(m2_py_wndmgr_is_focus(win1));
    assert!(!m2_py_wndmgr_is_focus(win2));

    m2_py_wndmgr_set_focus(win2);
    assert!(!m2_py_wndmgr_is_focus(win1));
    assert!(m2_py_wndmgr_is_focus(win2));

    // Test specific kill focus
    m2_py_wndmgr_kill_focus(win2);
    assert!(!m2_py_wndmgr_is_focus(win2));
    // When specific kill focus happens, it should pop the previous active window
    assert!(m2_py_wndmgr_is_focus(win1));
    
    // Test general kill focus (window_id = 0)
    m2_py_wndmgr_kill_focus(0);
    assert!(!m2_py_wndmgr_is_focus(win1));
}

#[test]
fn test_wndmgr_set_top() {
    let layer = CString::new("UI").unwrap();
    let parent = m2_py_wndmgr_register(WT_NORMAL, layer.as_ptr());
    let child1 = m2_py_wndmgr_register(WT_NORMAL, layer.as_ptr());
    let child2 = m2_py_wndmgr_register(WT_NORMAL, layer.as_ptr());

    m2_py_wndmgr_add_flag(child1, FLAG_FLOAT);
    m2_py_wndmgr_add_flag(child2, FLAG_FLOAT);

    m2_py_wndmgr_set_parent(child1, parent);
    m2_py_wndmgr_set_parent(child2, parent);

    {
        let mgr = get_wnd_mgr().lock().unwrap();
        let p_win = mgr.windows.get(&parent).unwrap();
        assert_eq!(p_win.child_ids, vec![child1, child2]);
    }

    // Call set_top on child1, it should move to the end of parent's child list
    m2_py_wndmgr_set_top(child1);

    {
        let mgr = get_wnd_mgr().lock().unwrap();
        let p_win = mgr.windows.get(&parent).unwrap();
        assert_eq!(p_win.child_ids, vec![child2, child1]);
    }
}
