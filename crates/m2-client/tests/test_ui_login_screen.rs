use m2_client::ui_login_screen::UiLoginScreen;

#[test]
fn test_integration_ui_login_screen() {
    let instance = UiLoginScreen::new();
    assert!(instance.is_ready());
}
