use m2_client::ui_slot_window::UiSlotWindow;

#[test]
fn test_integration_ui_slot_window() {
    let instance = UiSlotWindow::new();
    assert!(instance.is_ready());
}
