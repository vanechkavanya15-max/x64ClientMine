use m2_client::instance_base::InstanceBase;

#[test]
fn test_integration_instance_base() {
    let instance = InstanceBase::new();
    assert!(instance.is_ready());
}
