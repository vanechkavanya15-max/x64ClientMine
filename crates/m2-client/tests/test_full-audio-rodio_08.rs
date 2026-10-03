// test_full-audio-rodio_08.rs - Jules Swarm Test Suite for full-audio-rodio-08
#[path = "../src/audio_window_focus_guard.rs"]
mod audio_window_focus_guard;

use audio_window_focus_guard::*;

#[test]
fn test_audio_rodio_08_lifecycle() {
    let mut guard = AudioWindowFocusGuard::new();
    guard.set_master_volume(0.9);
    assert!(!guard.is_muted());

    guard.on_focus_change(false);
    guard.update(0.5);
    assert!(guard.is_muted());

    guard.on_focus_change(true);
    guard.update(0.5);
    assert!(!guard.is_muted());
    assert!((guard.get_effective_volume() - 0.9).abs() < 1e-4);
}

#[test]
fn test_audio_rodio_08_policies() {
    let mut guard = AudioWindowFocusGuard::new();
    guard.set_mute_policies(false, false);
    guard.on_focus_change(false);
    guard.on_minimize_change(true);
    guard.update(1.0);
    assert_eq!(guard.target_fade_factor, 1.0);
    assert!(!guard.is_muted());
}

#[test]
fn test_audio_rodio_08_fade_speed() {
    let mut guard = AudioWindowFocusGuard::new();
    guard.fade_duration_sec = 0.5;
    guard.on_focus_change(false);
    guard.update(0.25);
    assert!((guard.current_fade_factor - 0.5).abs() < 0.05);
}

#[test]
fn test_audio_rodio_08_zero_duration() {
    let mut guard = AudioWindowFocusGuard::new();
    guard.fade_duration_sec = 0.0;
    guard.on_focus_change(false);
    guard.update(0.01);
    assert_eq!(guard.current_fade_factor, 0.0);
    assert!(guard.is_muted());
}

#[test]
fn test_audio_rodio_08_helper_functions() {
    on_window_focus_event(true);
    on_window_minimize_event(false);
    let vol = get_master_audio_volume_attenuated();
    assert!(vol > 0.0);
}
