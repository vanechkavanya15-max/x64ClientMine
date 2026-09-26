use m2_client::sound_mixer_core::{SoundCategory, SoundMixerCore};

#[test]
fn test_default_volumes() {
    let mixer = SoundMixerCore::default();
    assert_eq!(mixer.get_master_volume(), 1.0);
    assert_eq!(mixer.get_sfx_volume(), 1.0);
    assert_eq!(mixer.get_music_volume(), 1.0);
    assert_eq!(mixer.get_voice_volume(), 1.0);
    assert_eq!(mixer.get_ui_volume(), 1.0);
}

#[test]
fn test_volume_clamping() {
    let mut mixer = SoundMixerCore::new();
    
    // Test upper bound
    mixer.set_master_volume(1.5);
    assert_eq!(mixer.get_master_volume(), 1.0);

    mixer.set_sfx_volume(2.0);
    assert_eq!(mixer.get_sfx_volume(), 1.0);

    // Test lower bound
    mixer.set_music_volume(-0.5);
    assert_eq!(mixer.get_music_volume(), 0.0);

    mixer.set_voice_volume(-1.0);
    assert_eq!(mixer.get_voice_volume(), 0.0);
    
    // Test valid value
    mixer.set_ui_volume(0.75);
    assert_eq!(mixer.get_ui_volume(), 0.75);
}

#[test]
fn test_compute_final_volume() {
    let mut mixer = SoundMixerCore::new();
    
    mixer.set_master_volume(0.8);
    mixer.set_sfx_volume(0.5);
    mixer.set_music_volume(1.0);
    mixer.set_voice_volume(0.0);
    
    // master (0.8) * sfx (0.5) * base (1.0) = 0.4
    assert_eq!(mixer.compute_final_volume(SoundCategory::SFX, 1.0), 0.4);

    // master (0.8) * sfx (0.5) * base (0.5) = 0.2
    assert_eq!(mixer.compute_final_volume(SoundCategory::SFX, 0.5), 0.2);

    // master (0.8) * music (1.0) * base (0.5) = 0.4
    assert_eq!(mixer.compute_final_volume(SoundCategory::Music, 0.5), 0.4);

    // master (0.8) * voice (0.0) * base (1.0) = 0.0
    assert_eq!(mixer.compute_final_volume(SoundCategory::Voice, 1.0), 0.0);
}

#[test]
fn test_compute_final_volume_clamping() {
    let mut mixer = SoundMixerCore::new();
    
    mixer.set_master_volume(1.0);
    mixer.set_sfx_volume(1.0);
    
    // Base volume > 1.0 should still result in clamped output
    assert_eq!(mixer.compute_final_volume(SoundCategory::SFX, 1.5), 1.0);

    // Base volume < 0.0 should be clamped to 0.0
    assert_eq!(mixer.compute_final_volume(SoundCategory::SFX, -0.5), 0.0);
}

#[test]
fn test_save_and_mute_restore() {
    let mut mixer = SoundMixerCore::new();
    
    // Set custom volumes
    mixer.set_master_volume(0.8);
    mixer.set_sfx_volume(0.7);
    mixer.set_music_volume(0.6);
    mixer.set_voice_volume(0.5);
    mixer.set_ui_volume(0.4);
    
    // Mute
    mixer.save_and_mute();
    
    // Verify everything is muted
    assert_eq!(mixer.get_master_volume(), 0.0);
    assert_eq!(mixer.get_sfx_volume(), 0.0);
    assert_eq!(mixer.get_music_volume(), 0.0);
    assert_eq!(mixer.get_voice_volume(), 0.0);
    assert_eq!(mixer.get_ui_volume(), 0.0);
    
    // Final volume during mute should be 0.0
    assert_eq!(mixer.compute_final_volume(SoundCategory::SFX, 1.0), 0.0);
    
    // Restore
    mixer.restore_volume();
    
    // Verify volumes are restored
    assert_eq!(mixer.get_master_volume(), 0.8);
    assert_eq!(mixer.get_sfx_volume(), 0.7);
    assert_eq!(mixer.get_music_volume(), 0.6);
    assert_eq!(mixer.get_voice_volume(), 0.5);
    assert_eq!(mixer.get_ui_volume(), 0.4);
}
