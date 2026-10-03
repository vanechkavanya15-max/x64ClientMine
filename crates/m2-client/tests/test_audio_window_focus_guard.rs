// test_audio_window_focus_guard.rs - Testy jednostkowe dla audio_window_focus_guard
#[path = "../src/audio_window_focus_guard.rs"]
mod audio_window_focus_guard;

use audio_window_focus_guard::AudioWindowFocusGuard;

#[test]
fn test_default_focus_guard_state() {
    let guard = AudioWindowFocusGuard::new();
    assert!(guard.is_focused);
    assert!(!guard.is_minimized);
    assert!((guard.master_volume - 1.0).abs() < 1e-5);
    assert!((guard.get_effective_volume() - 1.0).abs() < 1e-5);
    assert!(!guard.should_pause_bgm());
    assert!(!guard.is_muted());
}

#[test]
fn test_focus_lost_and_fade_out() {
    let mut guard = AudioWindowFocusGuard::new();
    guard.set_master_volume(0.8);
    guard.on_focus_change(false);

    assert!(guard.should_pause_bgm());
    assert_eq!(guard.target_fade_factor, 0.0);

    // Krok w czasie: polowa czasu fade (0.125s z 0.25s)
    guard.update(0.125);
    assert!((guard.current_fade_factor - 0.5).abs() < 0.01);
    assert!((guard.get_effective_volume() - 0.4).abs() < 0.01);

    // Drugi krok: pelne wyciszenie
    guard.update(0.15);
    assert!(guard.is_muted());
    assert!((guard.get_effective_volume() - 0.0).abs() < 1e-4);
}

#[test]
fn test_focus_regained_and_fade_in() {
    let mut guard = AudioWindowFocusGuard::new();
    guard.on_focus_change(false);
    guard.update(1.0);
    assert!(guard.is_muted());

    // Przywrocenie fokusu
    guard.on_focus_change(true);
    assert_eq!(guard.target_fade_factor, 1.0);
    assert!(!guard.should_pause_bgm());

    guard.update(0.25);
    assert!(!guard.is_muted());
    assert!((guard.get_effective_volume() - 1.0).abs() < 1e-4);
}

#[test]
fn test_minimize_behavior() {
    let mut guard = AudioWindowFocusGuard::new();
    guard.set_mute_policies(false, true); // Tylko przy minimalizacji, nie przy utracie fokusu
    guard.on_focus_change(false);
    assert!(!guard.should_pause_bgm());
    assert_eq!(guard.target_fade_factor, 1.0);

    // Minimalizacja okna
    guard.on_minimize_change(true);
    assert!(guard.should_pause_bgm());
    assert_eq!(guard.target_fade_factor, 0.0);

    guard.update(1.0);
    assert!(guard.is_muted());

    // Przywrocenie okna
    guard.on_minimize_change(false);
    assert!(!guard.should_pause_bgm());
    guard.update(1.0);
    assert!(!guard.is_muted());
}

#[test]
fn test_master_volume_clamping() {
    let mut guard = AudioWindowFocusGuard::new();
    guard.set_master_volume(1.5);
    assert_eq!(guard.master_volume, 1.0);

    guard.set_master_volume(-0.5);
    assert_eq!(guard.master_volume, 0.0);

    guard.set_master_volume(0.65);
    assert!((guard.master_volume - 0.65).abs() < 1e-5);
    assert!((guard.get_effective_volume() - 0.65).abs() < 1e-5);
}
