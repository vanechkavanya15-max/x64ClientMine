use m2_client::effect_manager::EffectManager;

#[test]
fn test_integration_effect_manager() {
    let instance = EffectManager::new();
    assert!(instance.is_ready());
}
