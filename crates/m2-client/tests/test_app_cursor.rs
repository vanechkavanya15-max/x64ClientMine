use m2_client::app_cursor::AppCursor;

#[test]
fn test_integration_app_cursor() {
    let instance = AppCursor::new();
    assert!(instance.is_ready());
}
