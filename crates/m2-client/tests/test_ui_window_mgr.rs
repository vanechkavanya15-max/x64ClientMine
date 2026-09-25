use m2_client::ui_window_mgr::UiWindowMgr;

#[test]
fn test_integration_ui_window_mgr() {
    let instance = UiWindowMgr::new();
    assert!(instance.is_ready());
}
