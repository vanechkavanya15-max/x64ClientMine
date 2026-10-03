#[path = "../src/rodio_bgm_fader.rs"]
mod rodio_bgm_fader;

use rodio_bgm_fader::{BgmFader, FadeState};

#[test]
fn test_play_and_stop() {
    let mut fader = BgmFader::new();
    assert_eq!(fader.state(), FadeState::Stopped);
    
    fader.play("bgm/login.mp3");
    assert_eq!(fader.state(), FadeState::Playing);
    assert_eq!(fader.current_track(), Some("bgm/login.mp3"));
    assert_eq!(fader.current_volume(), 1.0);
    
    fader.stop();
    assert_eq!(fader.state(), FadeState::Stopped);
    assert_eq!(fader.current_volume(), 0.0);
    assert_eq!(fader.current_track(), None);
}

#[test]
fn test_fade_in_out() {
    let mut fader = BgmFader::new();
    
    fader.fade_in("bgm/enter.mp3", 1.0); // 1 second duration
    assert_eq!(fader.state(), FadeState::FadeIn);
    assert_eq!(fader.current_track(), Some("bgm/enter.mp3"));
    assert_eq!(fader.current_volume(), 0.0);
    
    fader.update(0.5); // Half a second
    assert_eq!(fader.state(), FadeState::FadeIn);
    assert!((fader.current_volume() - 0.5).abs() < f32::EPSILON);
    
    fader.update(0.6); // Slightly more than remaining time
    assert_eq!(fader.state(), FadeState::Playing);
    assert_eq!(fader.current_volume(), 1.0);
    
    fader.fade_out(2.0); // 2 seconds fade out
    assert_eq!(fader.state(), FadeState::FadeOut);
    
    fader.update(1.0); // 1 second
    assert!((fader.current_volume() - 0.5).abs() < f32::EPSILON);
    
    fader.update(1.0); // Remaining 1 second
    assert_eq!(fader.state(), FadeState::Stopped);
    assert_eq!(fader.current_volume(), 0.0);
}

#[test]
fn test_crossfade_tracks() {
    let mut fader = BgmFader::new();
    
    fader.play("bgm/town.mp3");
    assert_eq!(fader.current_volume(), 1.0);
    
    fader.crossfade("bgm/field.mp3", 2.0); // 2 second crossfade
    assert_eq!(fader.state(), FadeState::Crossfading);
    assert_eq!(fader.current_track(), Some("bgm/town.mp3"));
    assert_eq!(fader.next_track(), Some("bgm/field.mp3"));
    
    fader.update(1.0); // 1 second
    assert!((fader.current_volume() - 0.5).abs() < f32::EPSILON);
    assert!((fader.next_volume() - 0.5).abs() < f32::EPSILON);
    
    fader.update(1.0); // Remaining 1 second
    assert_eq!(fader.state(), FadeState::Playing);
    assert_eq!(fader.current_track(), Some("bgm/field.mp3"));
    assert_eq!(fader.next_track(), None);
    assert_eq!(fader.current_volume(), 1.0);
    assert_eq!(fader.next_volume(), 0.0); // reset
}

#[test]
fn test_battle_zone_transitions() {
    let mut fader = BgmFader::new();
    
    // Start normal track
    fader.play("bgm/field.mp3");
    
    // Enter battle zone
    fader.enter_battle_zone("bgm/battle.mp3", 1.0);
    assert_eq!(fader.state(), FadeState::Crossfading);
    
    fader.update(1.0);
    assert_eq!(fader.state(), FadeState::Playing);
    assert_eq!(fader.current_track(), Some("bgm/battle.mp3"));
    
    // Exit battle zone
    fader.exit_battle_zone("bgm/field.mp3", 1.0);
    assert_eq!(fader.state(), FadeState::Crossfading);
    
    fader.update(1.0);
    assert_eq!(fader.state(), FadeState::Playing);
    assert_eq!(fader.current_track(), Some("bgm/field.mp3"));
}

#[test]
fn test_max_volume_clamping() {
    let mut fader = BgmFader::new();
    
    fader.set_max_volume(0.8);
    fader.play("bgm/town.mp3");
    
    assert_eq!(fader.current_volume(), 0.8);
    
    fader.crossfade("bgm/field.mp3", 1.0);
    
    fader.update(0.5); // half a second crossfade with max_vol 0.8 -> rate is 0.8/1.0 = 0.8 per sec
    assert!((fader.current_volume() - 0.4).abs() < f32::EPSILON);
    assert!((fader.next_volume() - 0.4).abs() < f32::EPSILON);
    
    fader.update(0.6); // remaining time
    assert_eq!(fader.state(), FadeState::Playing);
    assert_eq!(fader.current_volume(), 0.8);
    assert_eq!(fader.current_track(), Some("bgm/field.mp3"));
}
