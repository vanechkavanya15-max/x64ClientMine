use m2_client_audio::sound_manager::SoundManager;

#[test]
fn test_integration_sound_manager() {
    let instance = SoundManager::new();
    assert!(instance.is_ready());
}
