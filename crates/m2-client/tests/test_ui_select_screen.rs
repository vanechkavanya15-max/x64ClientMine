use m2_client::ui_select_screen::UiSelectScreen;

#[test]
fn test_integration_ui_select_screen() {
    let instance = UiSelectScreen::new();
    assert!(instance.is_ready());
}
