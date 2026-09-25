use m2_client::ui_widgets::UiWidgets;

#[test]
fn test_integration_ui_widgets() {
    let instance = UiWidgets::new();
    assert!(instance.is_ready());
}
