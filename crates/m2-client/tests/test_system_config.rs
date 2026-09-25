use m2_client::system_config::SystemConfig;

#[test]
fn test_integration_system_config() {
    let instance = SystemConfig::new();
    assert!(instance.is_ready());
}
