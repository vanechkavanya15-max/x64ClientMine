use std::sync::{RwLock, OnceLock};

static WIND_STATE: OnceLock<RwLock<WindState>> = OnceLock::new();

fn get_state() -> &'static RwLock<WindState> {
    WIND_STATE.get_or_init(|| RwLock::new(WindState::default()))
}

#[derive(Debug, Clone)]
pub struct WindState {
    pub wind_angle: f32,
    pub wind_strength: f32,
    pub gust_frequency: f32,
}

impl Default for WindState {
    fn default() -> Self {
        Self {
            wind_angle: 0.0,
            wind_strength: 1.0,
            gust_frequency: 1.0,
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_forest_set_wind_params(angle: f32, strength: f32, gust_freq: f32) {
    if let Ok(mut state) = get_state().write() {
        state.wind_angle = angle;
        state.wind_strength = strength;
        state.gust_frequency = gust_freq;
    }
}

#[no_mangle]
pub extern "C" fn m2_forest_get_wind_offset(x: f32, y: f32, time: f32) -> WindOffset {
    let state = get_state().read().unwrap();
    let angle = state.wind_angle;
    let strength = state.wind_strength;
    let freq = state.gust_frequency;

    let phase = x + y;
    let offset = (time * freq + phase).sin() * strength;

    WindOffset {
        x: offset * angle.cos(),
        y: offset * angle.sin(),
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindOffset {
    pub x: f32,
    pub y: f32,
}
