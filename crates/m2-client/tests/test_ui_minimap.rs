use m2_client::ui_minimap::UiMinimap;

#[test]
fn test_integration_ui_minimap() {
    let instance = UiMinimap::new();
    assert!(instance.is_ready());
}
