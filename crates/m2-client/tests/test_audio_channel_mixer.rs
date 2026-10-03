#[path = "../src/audio_channel_mixer.rs"]
mod audio_channel_mixer;

use audio_channel_mixer::{AudioChannel, AudioChannelMixer};

#[test]
fn test_default_volumes() {
    let mixer = AudioChannelMixer::new();
    assert_eq!(mixer.get_volume(AudioChannel::Master), 1.0);
    assert_eq!(mixer.get_volume(AudioChannel::Bgm), 1.0);
    assert_eq!(mixer.get_volume(AudioChannel::Sfx), 1.0);
    assert_eq!(mixer.get_volume(AudioChannel::Voice), 1.0);

    assert_eq!(mixer.get_effective_volume(AudioChannel::Master), 1.0);
    assert_eq!(mixer.get_effective_volume(AudioChannel::Bgm), 1.0);
}

#[test]
fn test_set_and_get_volume_with_clamp() {
    let mut mixer = AudioChannelMixer::new();
    
    mixer.set_volume(AudioChannel::Bgm, 0.5);
    assert_eq!(mixer.get_volume(AudioChannel::Bgm), 0.5);
    assert_eq!(mixer.get_effective_volume(AudioChannel::Bgm), 0.5);

    mixer.set_volume(AudioChannel::Sfx, 1.5); // Should clamp to 1.0
    assert_eq!(mixer.get_volume(AudioChannel::Sfx), 1.0);

    mixer.set_volume(AudioChannel::Voice, -0.5); // Should clamp to 0.0
    assert_eq!(mixer.get_volume(AudioChannel::Voice), 0.0);
}

#[test]
fn test_master_volume_affects_channels() {
    let mut mixer = AudioChannelMixer::new();
    
    mixer.set_volume(AudioChannel::Master, 0.5);
    mixer.set_volume(AudioChannel::Bgm, 0.8);
    
    assert_eq!(mixer.get_volume(AudioChannel::Bgm), 0.8);
    assert_eq!(mixer.get_effective_volume(AudioChannel::Master), 0.5);
    assert_eq!(mixer.get_effective_volume(AudioChannel::Bgm), 0.4); // 0.8 * 0.5 = 0.4
}

#[test]
fn test_channel_mute() {
    let mut mixer = AudioChannelMixer::new();
    
    mixer.set_volume(AudioChannel::Sfx, 0.7);
    mixer.set_muted(AudioChannel::Sfx, true);
    
    assert!(mixer.is_muted(AudioChannel::Sfx));
    assert_eq!(mixer.get_volume(AudioChannel::Sfx), 0.7); // Base volume remains unchanged
    assert_eq!(mixer.get_effective_volume(AudioChannel::Sfx), 0.0); // Effective volume is 0
    
    // Unmute
    mixer.set_muted(AudioChannel::Sfx, false);
    assert_eq!(mixer.get_effective_volume(AudioChannel::Sfx), 0.7);
}

#[test]
fn test_master_mute_affects_all() {
    let mut mixer = AudioChannelMixer::new();
    
    mixer.set_volume(AudioChannel::Bgm, 0.6);
    mixer.set_volume(AudioChannel::Voice, 0.9);
    
    mixer.set_muted(AudioChannel::Master, true);
    
    assert_eq!(mixer.get_effective_volume(AudioChannel::Master), 0.0);
    assert_eq!(mixer.get_effective_volume(AudioChannel::Bgm), 0.0);
    assert_eq!(mixer.get_effective_volume(AudioChannel::Voice), 0.0);
    
    // Unmuting master restores effective volumes
    mixer.set_muted(AudioChannel::Master, false);
    assert_eq!(mixer.get_effective_volume(AudioChannel::Bgm), 0.6);
    assert_eq!(mixer.get_effective_volume(AudioChannel::Voice), 0.9);
}
