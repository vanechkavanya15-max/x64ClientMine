use std::time::Duration;
use m2_client_audio::audio_types::{AudioParams, SoundCategory};
use m2_client_audio::sound_manager::SoundManager;

#[test]
fn test_integration_sound_manager() {
    let mut instance = SoundManager::new();
    assert!(instance.is_ready());

    let params = AudioParams::default();
    let duration = Duration::from_millis(200);

    let id = instance.play_sound("monster_groan.wav", SoundCategory::EFFECT, params, duration, false);
    assert!(id.is_ok());
    assert_eq!(instance.active_count(), 1);

    instance.stop_category(SoundCategory::EFFECT);
    assert_eq!(instance.active_count(), 0);
}
