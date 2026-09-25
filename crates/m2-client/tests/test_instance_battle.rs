use m2_client::instance_battle::InstanceBattle;

#[test]
fn test_integration_instance_battle() {
    let instance = InstanceBattle::new();
    assert!(instance.is_ready());
}
