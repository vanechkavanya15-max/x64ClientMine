use m2_client::instance_effect::InstanceEffect;

#[test]
fn test_integration_instance_effect() {
    let instance = InstanceEffect::new();
    assert!(instance.is_ready());
}
