use m2_client::race_manager::RaceManager;

#[test]
fn test_integration_race_manager() {
    let instance = RaceManager::new();
    assert!(instance.is_ready());
}
