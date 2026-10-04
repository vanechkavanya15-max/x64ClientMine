use crate::character_manager_core::get_character_manager;
use crate::weather_particle_sim_v2::get_weather_sim;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarpFadeState {
    Idle,
    FadeOut,
    Loading,
    FadeIn,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WarpEvent {
    None,
    FadeOutComplete,
    ReadyToWarp,
    FadeInComplete,
}

#[derive(Debug, Clone, Default)]
pub struct WarpLoadingFlags {
    pub actors_cleared: bool,
    pub particles_cleared: bool,
    pub buffers_cleared: bool,
    pub map_initialized: bool,
}

impl WarpLoadingFlags {
    pub fn all_ready(&self) -> bool {
        self.actors_cleared && self.particles_cleared && self.buffers_cleared && self.map_initialized
    }
}

pub struct GameWarpScreenFader {
    state: WarpFadeState,
    alpha: f32,
    fade_speed: f32,
    flags: WarpLoadingFlags,
}

impl GameWarpScreenFader {
    pub fn new(fade_speed: f32) -> Self {
        Self {
            state: WarpFadeState::Idle,
            alpha: 0.0,
            fade_speed,
            flags: WarpLoadingFlags::default(),
        }
    }

    pub fn start_warp_fade_out(&mut self) {
        if self.state == WarpFadeState::Idle {
            self.state = WarpFadeState::FadeOut;
            self.alpha = 0.0;
            self.flags = WarpLoadingFlags::default();
        }
    }

    pub fn clear_actors(&mut self) {
        if let Ok(mut mgr) = get_character_manager().write() {
            mgr.clear();
        }
        self.flags.actors_cleared = true;
        self.try_finish_loading();
    }

    pub fn clear_particle_effects(&mut self) {
        if let Ok(mut sim) = get_weather_sim().lock() {
            sim.particles.clear();
        }
        self.flags.particles_cleared = true;
        self.try_finish_loading();
    }

    pub fn clear_previous_map_buffers(&mut self) {
        self.flags.buffers_cleared = true;
        self.try_finish_loading();
    }

    pub fn initialize_new_map(&mut self) {
        self.flags.map_initialized = true;
        self.try_finish_loading();
    }

    fn try_finish_loading(&mut self) {
        if self.state == WarpFadeState::Loading && self.flags.all_ready() {
            self.state = WarpFadeState::FadeIn;
        }
    }

    pub fn update(&mut self, delta_time: f32) -> WarpEvent {
        match self.state {
            WarpFadeState::Idle => WarpEvent::None,
            WarpFadeState::FadeOut => {
                self.alpha += self.fade_speed * delta_time;
                if self.alpha >= 1.0 {
                    self.alpha = 1.0;
                    self.state = WarpFadeState::Loading;
                    WarpEvent::FadeOutComplete
                } else {
                    WarpEvent::None
                }
            }
            WarpFadeState::Loading => WarpEvent::ReadyToWarp,
            WarpFadeState::FadeIn => {
                self.alpha -= self.fade_speed * delta_time;
                if self.alpha <= 0.0 {
                    self.alpha = 0.0;
                    self.state = WarpFadeState::Idle;
                    WarpEvent::FadeInComplete
                } else {
                    WarpEvent::None
                }
            }
        }
    }

    pub fn get_state(&self) -> WarpFadeState {
        self.state
    }

    pub fn get_alpha(&self) -> f32 {
        self.alpha
    }
}
