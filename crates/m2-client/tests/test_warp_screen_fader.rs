#[path = "../src/game_warp_screen_fader.rs"]
mod game_warp_screen_fader;

// Hack for standalone module test
#[allow(dead_code)]
mod character_manager_core {
    use std::sync::RwLock;
    pub struct CharacterManagerCore;
    impl CharacterManagerCore {
        pub fn new() -> Self { Self }
        pub fn clear(&mut self) {}
    }
    static MGR: std::sync::OnceLock<RwLock<CharacterManagerCore>> = std::sync::OnceLock::new();
    pub fn get_character_manager() -> &'static RwLock<CharacterManagerCore> {
        MGR.get_or_init(|| RwLock::new(CharacterManagerCore::new()))
    }
}

#[allow(dead_code)]
mod weather_particle_sim_v2 {
    use std::sync::Mutex;
    pub struct WeatherSimulation {
        pub particles: Vec<()>,
    }
    impl WeatherSimulation {
        pub fn new() -> Self { Self { particles: Vec::new() } }
    }
    static SIM: std::sync::OnceLock<Mutex<WeatherSimulation>> = std::sync::OnceLock::new();
    pub fn get_weather_sim() -> &'static Mutex<WeatherSimulation> {
        SIM.get_or_init(|| Mutex::new(WeatherSimulation::new()))
    }
}

#[allow(dead_code)]
mod terrain_sector_cache {
    use std::sync::RwLock;
    pub struct TerrainSectorCache;
    impl TerrainSectorCache {
        pub fn new() -> Self { Self }
        pub fn clear(&mut self) {}
    }
    static CACHE: std::sync::OnceLock<RwLock<TerrainSectorCache>> = std::sync::OnceLock::new();
    pub fn get_sector_cache() -> &'static RwLock<TerrainSectorCache> {
        CACHE.get_or_init(|| RwLock::new(TerrainSectorCache::new()))
    }
}

use game_warp_screen_fader::{GameWarpScreenFader, WarpEvent, WarpFadeState};

#[test]
fn test_idle_to_fade_out() {
    let mut fader = GameWarpScreenFader::new(1.0);
    assert_eq!(fader.get_state(), WarpFadeState::Idle);
    
    fader.start_warp_fade_out();
    assert_eq!(fader.get_state(), WarpFadeState::FadeOut);
    assert_eq!(fader.get_alpha(), 0.0);
}

#[test]
fn test_fade_out_to_loading() {
    let mut fader = GameWarpScreenFader::new(0.5); // 0.5 per second
    fader.start_warp_fade_out();
    
    let event1 = fader.update(1.0); // alpha = 0.5
    assert_eq!(event1, WarpEvent::None);
    assert_eq!(fader.get_state(), WarpFadeState::FadeOut);
    assert_eq!(fader.get_alpha(), 0.5);
    
    let event2 = fader.update(1.0); // alpha = 1.0 -> Complete
    assert_eq!(event2, WarpEvent::FadeOutComplete);
    assert_eq!(fader.get_state(), WarpFadeState::Loading);
    assert_eq!(fader.get_alpha(), 1.0);
}

#[test]
fn test_loading_blocks_fade_in_without_flags() {
    let mut fader = GameWarpScreenFader::new(2.0);
    fader.start_warp_fade_out();
    fader.update(1.0); // fast forward to loading
    
    assert_eq!(fader.get_state(), WarpFadeState::Loading);
    
    let event = fader.update(0.1);
    assert_eq!(event, WarpEvent::ReadyToWarp); // Should stay in loading, generating ReadyToWarp event
    
    // Clear only some flags
    fader.clear_actors();
    fader.clear_particle_effects();
    
    let event2 = fader.update(0.1);
    assert_eq!(event2, WarpEvent::ReadyToWarp);
    assert_eq!(fader.get_state(), WarpFadeState::Loading);
}

#[test]
fn test_full_warp_transition_cycle() {
    let mut fader = GameWarpScreenFader::new(1.0);
    
    // 1. Idle -> FadeOut
    fader.start_warp_fade_out();
    
    // 2. FadeOut -> Loading
    let event = fader.update(1.0);
    assert_eq!(event, WarpEvent::FadeOutComplete);
    assert_eq!(fader.get_state(), WarpFadeState::Loading);
    
    // 3. Clear flags and transition to FadeIn
    fader.clear_actors();
    fader.clear_particle_effects();
    fader.clear_previous_map_buffers();
    fader.initialize_new_map();
    
    assert_eq!(fader.get_state(), WarpFadeState::FadeIn);
    
    // 4. FadeIn -> Idle
    let event_fade_in = fader.update(1.0);
    assert_eq!(event_fade_in, WarpEvent::FadeInComplete);
    assert_eq!(fader.get_state(), WarpFadeState::Idle);
    assert_eq!(fader.get_alpha(), 0.0);
}
