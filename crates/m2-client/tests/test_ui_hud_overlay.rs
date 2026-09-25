use m2_client::ui_hud_overlay::UiHudOverlay;

#[test]
fn test_integration_ui_hud_overlay() {
    let instance = UiHudOverlay::new();
    assert!(instance.is_ready());
}
