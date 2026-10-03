// audio_window_focus_guard.rs - Automatyczne zarzadzanie glosnoscia i pauzowaniem audio przy minimalizacji okna
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, PartialEq)]
pub struct AudioWindowFocusGuard {
    pub is_focused: bool,
    pub is_minimized: bool,
    pub master_volume: f32,
    pub mute_on_focus_lost: bool,
    pub mute_on_minimize: bool,
    pub fade_duration_sec: f32,
    pub current_fade_factor: f32,
    pub target_fade_factor: f32,
}

impl Default for AudioWindowFocusGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioWindowFocusGuard {
    pub fn new() -> Self {
        Self {
            is_focused: true,
            is_minimized: false,
            master_volume: 1.0,
            mute_on_focus_lost: true,
            mute_on_minimize: true,
            fade_duration_sec: 0.25,
            current_fade_factor: 1.0,
            target_fade_factor: 1.0,
        }
    }

    pub fn on_focus_change(&mut self, has_focus: bool) {
        self.is_focused = has_focus;
        self.recompute_target();
    }

    pub fn on_minimize_change(&mut self, is_minimized: bool) {
        self.is_minimized = is_minimized;
        self.recompute_target();
    }

    pub fn set_master_volume(&mut self, vol: f32) {
        self.master_volume = vol.clamp(0.0, 1.0);
    }

    pub fn set_mute_policies(&mut self, on_focus_lost: bool, on_minimize: bool) {
        self.mute_on_focus_lost = on_focus_lost;
        self.mute_on_minimize = on_minimize;
        self.recompute_target();
    }

    fn recompute_target(&mut self) {
        let should_mute = (self.mute_on_minimize && self.is_minimized)
            || (self.mute_on_focus_lost && !self.is_focused);
        self.target_fade_factor = if should_mute { 0.0 } else { 1.0 };
    }

    pub fn update(&mut self, delta_seconds: f32) {
        if (self.current_fade_factor - self.target_fade_factor).abs() < 1e-4 {
            self.current_fade_factor = self.target_fade_factor;
            return;
        }

        let step = if self.fade_duration_sec > 1e-4 {
            delta_seconds / self.fade_duration_sec
        } else {
            1.0
        };

        if self.current_fade_factor < self.target_fade_factor {
            self.current_fade_factor = (self.current_fade_factor + step).min(self.target_fade_factor);
        } else {
            self.current_fade_factor = (self.current_fade_factor - step).max(self.target_fade_factor);
        }
    }

    pub fn get_effective_volume(&self) -> f32 {
        self.master_volume * self.current_fade_factor
    }

    pub fn should_pause_bgm(&self) -> bool {
        self.is_minimized || (!self.is_focused && self.mute_on_focus_lost)
    }

    pub fn is_muted(&self) -> bool {
        self.current_fade_factor <= 1e-4
    }
}

static GLOBAL_AUDIO_FOCUS_GUARD: OnceLock<Mutex<AudioWindowFocusGuard>> = OnceLock::new();

pub fn get_audio_focus_guard() -> std::sync::MutexGuard<'static, AudioWindowFocusGuard> {
    GLOBAL_AUDIO_FOCUS_GUARD
        .get_or_init(|| Mutex::new(AudioWindowFocusGuard::new()))
        .lock()
        .unwrap()
}

pub fn on_window_focus_event(has_focus: bool) {
    let mut guard = get_audio_focus_guard();
    guard.on_focus_change(has_focus);
}

pub fn on_window_minimize_event(is_minimized: bool) {
    let mut guard = get_audio_focus_guard();
    guard.on_minimize_change(is_minimized);
}

pub fn get_master_audio_volume_attenuated() -> f32 {
    let guard = get_audio_focus_guard();
    guard.get_effective_volume()
}
