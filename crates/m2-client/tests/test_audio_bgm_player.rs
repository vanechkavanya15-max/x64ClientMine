#[path = "../src/audio_bgm_player.rs"]
mod audio_bgm_player;

use audio_bgm_player::{AudioBgmPlayer, BgmTrack};

#[test]
fn test_initial_state() {
    let player = AudioBgmPlayer::new();
    assert!(!player.state.is_playing);
    assert_eq!(player.state.current_track, None);
}

#[test]
fn test_play_bgm() {
    let mut player = AudioBgmPlayer::new();
    player.play_bgm("BGM/login_window.mp3", true);

    assert!(player.state.is_playing);
    let track = player.state.current_track.unwrap();
    assert_eq!(track.title, "login_window.mp3");
    assert_eq!(track.file_path, "BGM/login_window.mp3");
    assert_eq!(track.volume, 1.0);
    assert!(track.is_looping);
}

#[test]
fn test_play_bgm_with_windows_path() {
    let mut player = AudioBgmPlayer::new();
    player.play_bgm("BGM\\login_window.mp3", false);

    assert!(player.state.is_playing);
    let track = player.state.current_track.unwrap();
    assert_eq!(track.title, "login_window.mp3");
    assert_eq!(track.file_path, "BGM\\login_window.mp3");
    assert_eq!(track.volume, 1.0);
    assert!(!track.is_looping);
}

#[test]
fn test_stop_bgm() {
    let mut player = AudioBgmPlayer::new();
    player.play_bgm("BGM/test.mp3", true);
    assert!(player.state.is_playing);

    player.stop_bgm();
    assert!(!player.state.is_playing);
    
    // Stop should set volume to 0.0 but keep the track info for resume or record
    let track = player.state.current_track.unwrap();
    assert_eq!(track.volume, 0.0);
}

#[test]
fn test_set_bgm_volume() {
    let mut player = AudioBgmPlayer::new();
    player.play_bgm("BGM/test.mp3", true);

    player.set_bgm_volume(0.5);
    let track = player.state.current_track.as_ref().unwrap();
    assert_eq!(track.volume, 0.5);

    // Test clamping
    player.set_bgm_volume(1.5);
    let track2 = player.state.current_track.as_ref().unwrap();
    assert_eq!(track2.volume, 1.0);

    player.set_bgm_volume(-0.5);
    let track3 = player.state.current_track.as_ref().unwrap();
    assert_eq!(track3.volume, 0.0);
}

#[test]
fn test_fade_out_bgm_starts() {
    let mut player = AudioBgmPlayer::new();
    player.play_bgm("BGM/test.mp3", true);
    
    player.fade_out_bgm(1000); // 1 second fade out

    // Should still be playing until update finishes the fade
    assert!(player.state.is_playing);
    
    // Test fade out with 0 duration acts like immediate stop
    let mut player2 = AudioBgmPlayer::new();
    player2.play_bgm("BGM/test2.mp3", true);
    player2.fade_out_bgm(0);
    assert!(!player2.state.is_playing);
}

#[test]
fn test_fade_out_update() {
    let mut player = AudioBgmPlayer::new();
    player.play_bgm("BGM/test.mp3", true);
    
    player.set_bgm_volume(1.0); // Explicitly set to 1.0
    player.fade_out_bgm(1000);

    // Update halfway
    player.update(500);

    assert!(player.state.is_playing);
    let track = player.state.current_track.as_ref().unwrap();
    // 500ms out of 1000ms means volume should be 50% of 1.0
    assert!((track.volume - 0.5).abs() < f32::EPSILON);

    // Update another 250ms (total 750ms)
    player.update(250);
    assert!(player.state.is_playing);
    let track2 = player.state.current_track.as_ref().unwrap();
    // 750ms out of 1000ms means volume should be 25% of 1.0
    assert!((track2.volume - 0.25).abs() < f32::EPSILON);
}

#[test]
fn test_fade_out_completes() {
    let mut player = AudioBgmPlayer::new();
    player.play_bgm("BGM/test.mp3", true);
    
    player.fade_out_bgm(1000);
    
    // Update past the duration
    player.update(1500);

    // Playback should have stopped
    assert!(!player.state.is_playing);
    let track = player.state.current_track.as_ref().unwrap();
    assert_eq!(track.volume, 0.0);
}

#[test]
fn test_fade_out_preserves_initial_volume() {
    let mut player = AudioBgmPlayer::new();
    player.play_bgm("BGM/test.mp3", true);
    
    // Set initial volume lower
    player.set_bgm_volume(0.8);
    
    player.fade_out_bgm(1000);
    
    // Update halfway
    player.update(500);
    
    // Volume should be 50% of the initial 0.8 -> 0.4
    let track = player.state.current_track.as_ref().unwrap();
    assert!((track.volume - 0.4).abs() < f32::EPSILON);
}
