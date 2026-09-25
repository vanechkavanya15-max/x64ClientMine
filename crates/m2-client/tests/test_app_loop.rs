use m2_client::app_loop::AppLoop;

#[test]
fn test_integration_app_loop() {
    let instance = AppLoop::new();
    assert!(instance.is_ready());
}
