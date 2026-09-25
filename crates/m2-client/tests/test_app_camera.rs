use m2_client::app_camera::AppCamera;

#[test]
fn test_integration_app_camera() {
    let instance = AppCamera::new();
    assert!(instance.is_ready());
}
