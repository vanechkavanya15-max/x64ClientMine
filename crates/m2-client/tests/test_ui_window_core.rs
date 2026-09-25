use m2_client::ui_window_core::UiWindowCore;

#[test]
fn test_integration_ui_window_core() {
    let instance = UiWindowCore::new();
    assert!(instance.is_ready());
}
