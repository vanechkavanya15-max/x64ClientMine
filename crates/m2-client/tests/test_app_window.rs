use m2_client::app_window::AppWindow;

#[test]
fn test_integration_app_window() {
    let instance = AppWindow::new();
    assert!(instance.is_ready());
}
