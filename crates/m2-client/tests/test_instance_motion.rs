use m2_client::instance_motion::InstanceMotion;

#[test]
fn test_integration_instance_motion() {
    let instance = InstanceMotion::new();
    assert!(instance.is_ready());
}
