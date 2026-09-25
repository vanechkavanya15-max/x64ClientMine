use m2_client_render::camera_3d::Camera3d;

#[test]
fn test_integration_camera_3d() {
    let instance = Camera3d::new();
    assert!(instance.is_ready());
}
