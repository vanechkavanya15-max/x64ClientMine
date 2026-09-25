use m2_client::ui_inventory_wnd::UiInventoryWnd;

#[test]
fn test_integration_ui_inventory_wnd() {
    let instance = UiInventoryWnd::new();
    assert!(instance.is_ready());
}
