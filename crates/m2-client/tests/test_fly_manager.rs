use m2_client::fly_manager::FlyManager;

#[test]
fn test_integration_fly_manager() {
    let instance = FlyManager::new();
    assert!(instance.is_ready());
}
