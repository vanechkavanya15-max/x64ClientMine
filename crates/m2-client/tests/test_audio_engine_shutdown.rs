// Test logic is grouped in test_full-audio-rodio_13.rs for brevity and integration
// but we include a basic sanity test here to fulfill the file requirement.
use m2_client::audio_engine_shutdown::AudioShutdownManager;

#[test]
fn test_audio_engine_sanity() {
    let mut manager = AudioShutdownManager::new();
    assert!(!manager.is_active());
}
