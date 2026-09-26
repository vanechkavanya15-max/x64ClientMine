use m2_client::sound_types::{SoundCategory, EMusicState, SoundInstance};

#[test]
fn test_sound_category() {
    assert_eq!(SoundCategory::Effect as u8, 0);
    assert_eq!(SoundCategory::Music as u8, 1);
    assert_eq!(SoundCategory::Voice as u8, 2);
    assert_eq!(SoundCategory::Ui as u8, 3);
}

#[test]
fn test_music_state() {
    assert_eq!(EMusicState::Off as u8, 0);
    assert_eq!(EMusicState::Play as u8, 1);
    assert_eq!(EMusicState::FadeIn as u8, 2);
    assert_eq!(EMusicState::FadeOut as u8, 3);
    assert_eq!(EMusicState::FadeLimitOut as u8, 4);
}

#[test]
fn test_sound_instance_new_and_defaults() {
    let instance = SoundInstance::new(
        1,
        "test_sound.wav".to_string(),
        SoundCategory::Effect,
        0.5,
        0.0,
        1.0,
        false,
        1,
    );

    assert_eq!(instance.id, 1);
    assert_eq!(instance.file_name, "test_sound.wav");
    assert_eq!(instance.category, SoundCategory::Effect);
    assert_eq!(instance.volume(), 0.5);
    assert_eq!(instance.pan(), 0.0);
    assert_eq!(instance.pitch, 1.0);
    assert_eq!(instance.is_looping, false);
    assert_eq!(instance.play_count, 1);
}

#[test]
fn test_sound_instance_volume_clamping() {
    let mut instance = SoundInstance::new(
        2,
        "test_volume.wav".to_string(),
        SoundCategory::Ui,
        1.5, // Should clamp to 1.0
        0.0,
        1.0,
        false,
        1,
    );

    assert_eq!(instance.volume(), 1.0);

    instance.set_volume(-0.5); // Should clamp to 0.0
    assert_eq!(instance.volume(), 0.0);

    instance.set_volume(0.8);
    assert_eq!(instance.volume(), 0.8);
}

#[test]
fn test_sound_instance_pan_clamping() {
    let mut instance = SoundInstance::new(
        3,
        "test_pan.wav".to_string(),
        SoundCategory::Music,
        1.0,
        2.0, // Should clamp to 1.0
        1.0,
        true,
        0,
    );

    assert_eq!(instance.pan(), 1.0);

    instance.set_pan(-1.5); // Should clamp to -1.0
    assert_eq!(instance.pan(), -1.0);

    instance.set_pan(0.5);
    assert_eq!(instance.pan(), 0.5);
}

#[test]
fn test_sound_instance_clone_and_eq() {
    let instance1 = SoundInstance::new(
        4,
        "clone.wav".to_string(),
        SoundCategory::Voice,
        0.5,
        0.5,
        1.5,
        true,
        2,
    );
    let instance2 = instance1.clone();
    assert_eq!(instance1, instance2);
}
