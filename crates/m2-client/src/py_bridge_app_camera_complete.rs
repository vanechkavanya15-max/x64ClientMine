//! Python Bridge App Camera Complete Module
//!
//! Rozbudowany interfejs kontroli kamery z poziomu Pythona (`app` module):
//! Obsluga predkosci obrotu, pochylenia (pitch), przyblizenia (zoom), trybu sensorycznego (SensorView)
//! oraz konfiguracji limitow widoku.

pub use crate::py_bridge_app_camera::*;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraAdvancedLimits {
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub sensor_view_enabled: bool,
    pub rotate_speed: f32,
    pub pitch_speed: f32,
    pub zoom_speed: f32,
}

impl Default for CameraAdvancedLimits {
    fn default() -> Self {
        Self {
            min_pitch: 5.0,
            max_pitch: 80.0,
            min_distance: 100.0,
            max_distance: 5000.0,
            sensor_view_enabled: false,
            rotate_speed: 1.5,
            pitch_speed: 1.5,
            zoom_speed: 0.05,
        }
    }
}

lazy_static! {
    static ref GLOBAL_CAMERA_LIMITS: Mutex<CameraAdvancedLimits> = Mutex::new(CameraAdvancedLimits::default());
}

pub fn set_camera_rotate_speed(speed: f32) {
    if let Ok(mut l) = GLOBAL_CAMERA_LIMITS.lock() {
        l.rotate_speed = speed;
    }
}

pub fn get_camera_rotate_speed() -> f32 {
    GLOBAL_CAMERA_LIMITS.lock().map(|l| l.rotate_speed).unwrap_or(1.5)
}

pub fn set_camera_pitch_speed(speed: f32) {
    if let Ok(mut l) = GLOBAL_CAMERA_LIMITS.lock() {
        l.pitch_speed = speed;
    }
}

pub fn get_camera_pitch_speed() -> f32 {
    GLOBAL_CAMERA_LIMITS.lock().map(|l| l.pitch_speed).unwrap_or(1.5)
}

pub fn set_camera_zoom_speed(speed: f32) {
    if let Ok(mut l) = GLOBAL_CAMERA_LIMITS.lock() {
        l.zoom_speed = speed;
    }
}

pub fn get_camera_zoom_speed() -> f32 {
    GLOBAL_CAMERA_LIMITS.lock().map(|l| l.zoom_speed).unwrap_or(0.05)
}

pub fn set_sensor_view_enabled(enabled: bool) {
    if let Ok(mut l) = GLOBAL_CAMERA_LIMITS.lock() {
        l.sensor_view_enabled = enabled;
    }
}

pub fn is_sensor_view_enabled() -> bool {
    GLOBAL_CAMERA_LIMITS.lock().map(|l| l.sensor_view_enabled).unwrap_or(false)
}

// C-FFI dla silnika Pythona
#[no_mangle]
pub extern "C" fn app_set_camera_rotate_speed(speed: f32) {
    set_camera_rotate_speed(speed);
}

#[no_mangle]
pub extern "C" fn app_get_camera_rotate_speed() -> f32 {
    get_camera_rotate_speed()
}

#[no_mangle]
pub extern "C" fn app_set_camera_pitch_speed(speed: f32) {
    set_camera_pitch_speed(speed);
}

#[no_mangle]
pub extern "C" fn app_get_camera_pitch_speed() -> f32 {
    get_camera_pitch_speed()
}

#[no_mangle]
pub extern "C" fn app_set_sensor_view(enabled: bool) {
    set_sensor_view_enabled(enabled);
}

#[no_mangle]
pub extern "C" fn app_is_sensor_view() -> bool {
    is_sensor_view_enabled()
}
