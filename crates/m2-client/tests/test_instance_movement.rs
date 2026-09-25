use m2_client::instance_movement::InstanceMovement;

#[test]
fn test_integration_instance_movement() {
    let instance = InstanceMovement::new();
    assert!(instance.is_ready());
}
