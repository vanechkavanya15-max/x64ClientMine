use m2_client::py_bridge_wndmgr::*;

#[test]
fn test_register_window() {
    let handle = wndMgr_Register(1001);
    assert!(handle > 0);
}

#[test]
fn test_set_position() {
    let handle = wndMgr_Register(1002);
    assert!(wndMgr_SetPosition(handle, 100, 200));
}

#[test]
fn test_visibility() {
    let handle = wndMgr_Register(1003);
    assert!(!wndMgr_IsShow(handle));

    assert!(wndMgr_Show(handle));
    assert!(wndMgr_IsShow(handle));

    assert!(wndMgr_Hide(handle));
    assert!(!wndMgr_IsShow(handle));
}

#[test]
fn test_set_size() {
    let handle = wndMgr_Register(1004);
    assert!(wndMgr_SetSize(handle, 800, 600));
}

#[test]
fn test_set_top() {
    let handle1 = wndMgr_Register(1005);
    let handle2 = wndMgr_Register(1006);

    assert!(wndMgr_SetTop(handle1));
    assert!(wndMgr_SetTop(handle2));
}

#[test]
fn test_invalid_handle() {
    assert!(!wndMgr_SetPosition(99999, 100, 100));
    assert!(!wndMgr_Show(99999));
    assert!(!wndMgr_Hide(99999));
    assert!(!wndMgr_SetTop(99999));
    assert!(!wndMgr_SetSize(99999, 100, 100));
    assert!(!wndMgr_IsShow(99999));
}
