use m2_client::app_event::AppEvent;

#[test]
fn test_integration_app_event() {
    let instance = AppEvent::new();
    assert!(instance.is_ready());
}
