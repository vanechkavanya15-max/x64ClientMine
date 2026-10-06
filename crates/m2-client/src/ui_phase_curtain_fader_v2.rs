//! Phase Curtain Fader (v2) for Metin2
//! Implements a curtain for phase transitions (Login -> Select -> Loading -> Game)
//! with Fade In and Fade Out effects.
//! Translates functionality equivalent to legacy C++ Phase Curtain.

use std::sync::{Mutex, OnceLock};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseCurtainState {
    Idle,
    FadeOut,
    FadeIn,
}

#[derive(Debug, Clone)]
pub struct PhaseCurtainFader {
    state: PhaseCurtainState,
    alpha: f32,
    fade_duration_ms: u32,
    elapsed_time_ms: f32,
    callback: Option<extern "C" fn()>,
}

impl PhaseCurtainFader {
    pub fn new() -> Self {
        Self {
            state: PhaseCurtainState::Idle,
            alpha: 0.0,
            fade_duration_ms: 1300, // Default fade duration used in legacy MovieMan is 1300ms
            elapsed_time_ms: 0.0,
            callback: None,
        }
    }

    pub fn set_fade_out(&mut self, duration_ms: u32, callback: Option<extern "C" fn()>) {
        self.state = PhaseCurtainState::FadeOut;
        self.fade_duration_ms = if duration_ms > 0 { duration_ms } else { 1300 };
        self.elapsed_time_ms = 0.0;
        self.alpha = 0.0;
        self.callback = callback;
    }

    pub fn set_fade_in(&mut self, duration_ms: u32, callback: Option<extern "C" fn()>) {
        self.state = PhaseCurtainState::FadeIn;
        self.fade_duration_ms = if duration_ms > 0 { duration_ms } else { 1300 };
        self.elapsed_time_ms = 0.0;
        self.alpha = 1.0;
        self.callback = callback;
    }

    pub fn update(&mut self, delta_time_ms: f32) {
        if self.state == PhaseCurtainState::Idle {
            return;
        }

        self.elapsed_time_ms += delta_time_ms;
        let mut progress = self.elapsed_time_ms / (self.fade_duration_ms as f32);

        if progress >= 1.0 {
            progress = 1.0;
        }

        match self.state {
            PhaseCurtainState::FadeOut => {
                self.alpha = progress;
            }
            PhaseCurtainState::FadeIn => {
                self.alpha = 1.0 - progress;
            }
            PhaseCurtainState::Idle => {}
        }

        if progress >= 1.0 {
            self.state = PhaseCurtainState::Idle;
            if let Some(cb) = self.callback.take() {
                cb();
            }
        }
    }

    pub fn get_alpha(&self) -> f32 {
        self.alpha
    }

    pub fn get_state(&self) -> PhaseCurtainState {
        self.state
    }

    pub fn clear(&mut self) {
        self.state = PhaseCurtainState::Idle;
        self.alpha = 0.0;
        self.elapsed_time_ms = 0.0;
        self.callback = None;
    }
}

// Global Singleton
static PHASE_CURTAIN_FADER: OnceLock<Mutex<PhaseCurtainFader>> = OnceLock::new();

fn get_fader() -> std::sync::MutexGuard<'static, PhaseCurtainFader> {
    PHASE_CURTAIN_FADER
        .get_or_init(|| Mutex::new(PhaseCurtainFader::new()))
        .lock()
        .unwrap()
}

// C-FFI API
#[no_mangle]
pub extern "C" fn m2_ui_phase_curtain_set_fade_out(duration_ms: u32, callback: Option<extern "C" fn()>) {
    get_fader().set_fade_out(duration_ms, callback);
}

#[no_mangle]
pub extern "C" fn m2_ui_phase_curtain_set_fade_in(duration_ms: u32, callback: Option<extern "C" fn()>) {
    get_fader().set_fade_in(duration_ms, callback);
}

#[no_mangle]
pub extern "C" fn m2_ui_phase_curtain_update(delta_time_ms: f32) {
    get_fader().update(delta_time_ms);
}

#[no_mangle]
pub extern "C" fn m2_ui_phase_curtain_get_alpha() -> f32 {
    get_fader().get_alpha()
}

#[no_mangle]
pub extern "C" fn m2_ui_phase_curtain_get_state() -> PhaseCurtainState {
    get_fader().get_state()
}

#[no_mangle]
pub extern "C" fn m2_ui_phase_curtain_clear() {
    get_fader().clear();
}
