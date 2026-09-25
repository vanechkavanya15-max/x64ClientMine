use m2_client::ui_chat_window::UiChatWindow;

#[test]
fn test_integration_ui_chat_window() {
    let instance = UiChatWindow::new();
    assert!(instance.is_ready());
}
