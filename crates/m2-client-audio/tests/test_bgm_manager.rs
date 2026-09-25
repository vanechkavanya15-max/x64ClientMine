use m2_client_audio::bgm_manager::BgmManager;

#[test]
fn test_integration_bgm_manager() {
    let instance = BgmManager::new();
    assert!(instance.is_ready());
}
