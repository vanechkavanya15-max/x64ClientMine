use m2_client::instance_transform::InstanceTransform;

#[test]
fn test_integration_instance_transform() {
    let instance = InstanceTransform::new();
    assert!(instance.is_ready());
}
