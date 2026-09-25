use m2_client_granny::motion_blender::MotionBlender;

#[test]
fn test_integration_motion_blender() {
    let instance = MotionBlender::new();
    assert!(instance.is_ready());
}
