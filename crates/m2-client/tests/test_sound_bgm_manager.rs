use m2_client::sound_bgm_manager::{BgmState, SoundBgmManager};

const DELTA_60FPS: f32 = 1.0 / 60.0;

#[test]
fn test_play_music_immediate() {
    let mut manager = SoundBgmManager::new();
    
    assert_eq!(manager.state(), BgmState::Off);
    assert_eq!(manager.current_volume(), 0.0);
    
    manager.play_music("bgm/login.mp3", true);
    
    assert_eq!(manager.state(), BgmState::Play);
    assert_eq!(manager.current_file(), "bgm/login.mp3");
    assert_eq!(manager.current_volume(), 1.0);
    
    // Updating should not change anything when state is Play
    manager.update(DELTA_60FPS);
    assert_eq!(manager.state(), BgmState::Play);
    assert_eq!(manager.current_volume(), 1.0);
}

#[test]
fn test_fade_in_to_limit() {
    let mut manager = SoundBgmManager::new();
    
    // Target volume is 0.5, speed per frame is 0.016
    manager.fade_in("bgm/enter.mp3", 0.5, 0.016);
    
    assert_eq!(manager.state(), BgmState::FadeIn);
    assert_eq!(manager.current_file(), "bgm/enter.mp3");
    assert_eq!(manager.current_volume(), 0.0);
    
    // 1 frame
    manager.update(DELTA_60FPS);
    assert_eq!(manager.state(), BgmState::FadeIn);
    let vol_1_frame = 0.016;
    assert!((manager.current_volume() - vol_1_frame).abs() < f32::EPSILON, "Volume mismatch. Expected approx {}, got {}", vol_1_frame, manager.current_volume());
    
    // Many frames to hit limit
    for _ in 0..100 {
        manager.update(DELTA_60FPS);
    }
    
    assert_eq!(manager.state(), BgmState::Play);
    assert_eq!(manager.current_volume(), 0.5);
    
    // Update more shouldn't change
    manager.update(DELTA_60FPS);
    assert_eq!(manager.state(), BgmState::Play);
    assert_eq!(manager.current_volume(), 0.5);
}

#[test]
fn test_fade_out_to_zero() {
    let mut manager = SoundBgmManager::new();
    
    manager.play_music("bgm/battle.mp3", true);
    assert_eq!(manager.state(), BgmState::Play);
    assert_eq!(manager.current_volume(), 1.0);
    
    manager.fade_out(0.016);
    assert_eq!(manager.state(), BgmState::FadeOut);
    
    // 1 frame
    manager.update(DELTA_60FPS);
    assert_eq!(manager.state(), BgmState::FadeOut);
    let vol_1_frame = 1.0 - 0.016;
    assert!((manager.current_volume() - vol_1_frame).abs() < f32::EPSILON, "Volume mismatch. Expected approx {}, got {}", vol_1_frame, manager.current_volume());
    
    // Many frames to hit 0
    for _ in 0..100 {
        manager.update(DELTA_60FPS);
    }
    
    assert_eq!(manager.state(), BgmState::Off);
    assert_eq!(manager.current_volume(), 0.0);
    assert_eq!(manager.current_file(), "");
}

#[test]
fn test_cross_fade() {
    let mut manager = SoundBgmManager::new();
    
    manager.play_music("bgm/town.mp3", true);
    assert_eq!(manager.current_volume(), 1.0);
    
    manager.fade_out(0.1);
    
    for _ in 0..15 {
        manager.update(DELTA_60FPS);
    }
    
    assert_eq!(manager.state(), BgmState::Off);
    assert_eq!(manager.current_volume(), 0.0);
    
    manager.fade_in("bgm/field.mp3", 0.8, 0.1);
    assert_eq!(manager.state(), BgmState::FadeIn);
    
    for _ in 0..10 {
        manager.update(DELTA_60FPS);
    }
    
    assert_eq!(manager.state(), BgmState::Play);
    assert_eq!(manager.current_volume(), 0.8);
    assert_eq!(manager.current_file(), "bgm/field.mp3");
}

#[test]
fn test_edge_cases() {
    let mut manager = SoundBgmManager::new();
    
    // 0 speed fade in
    manager.fade_in("bgm/silence.mp3", 1.0, 0.0);
    for _ in 0..10 {
        manager.update(DELTA_60FPS);
    }
    assert_eq!(manager.state(), BgmState::FadeIn);
    assert_eq!(manager.current_volume(), 0.0);
    
    // Exceed target vol handling with large delta
    manager.fade_in("bgm/silence.mp3", 0.5, 1.0); // very fast speed
    manager.update(1.0); // 1 sec at 60 FPS equivalent -> 1.0 * (1.0 * 60) = 60.0
    assert_eq!(manager.state(), BgmState::Play);
    assert_eq!(manager.current_volume(), 0.5); // clamped to target
}
