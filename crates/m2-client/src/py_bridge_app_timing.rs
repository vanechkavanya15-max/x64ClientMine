use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;

#[cfg(not(test))]
use crate::app_timing_clock::m2_get_time;

#[cfg(test)]
use m2_client::app_timing_clock::m2_get_time;

#[derive(Debug)]
pub struct AppTimingState {
    pub render_fps: u32,
    pub update_fps: u32,
    pub render_frames: u32,
    pub update_frames: u32,
    pub last_time: u64,
    pub global_time: u64,
}

impl Default for AppTimingState {
    fn default() -> Self {
        Self {
            render_fps: 0,
            update_fps: 0,
            render_frames: 0,
            update_frames: 0,
            last_time: m2_get_time(),
            global_time: m2_get_time(),
        }
    }
}

impl AppTimingState {
    pub fn process(&mut self) {
        let current_time = m2_get_time();
        self.global_time = current_time;

        if current_time >= self.last_time + 1000 {
            self.render_fps = self.render_frames;
            self.update_fps = self.update_frames;
            
            self.render_frames = 0;
            self.update_frames = 0;
            
            self.last_time = current_time;
        }
    }

    pub fn reset_last_time_for_test(&mut self, time: u64) {
        self.last_time = time;
    }

    pub fn force_fps_update_for_test(&mut self, render_fps: u32, update_fps: u32) {
        self.render_fps = render_fps;
        self.update_fps = update_fps;
        self.render_frames = 0;
        self.update_frames = 0;
    }
}

lazy_static! {
    pub static ref APP_TIMING: Mutex<AppTimingState> = Mutex::new(AppTimingState::default());
}

pub fn timing_lock() -> MutexGuard<'static, AppTimingState> {
    APP_TIMING.lock().unwrap()
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_fps(fps: u32) {
    safe_ffi_call((), || {
        let _ = fps;
    });
}

#[no_mangle]
pub extern "C" fn m2_py_app_get_render_fps() -> u32 {
    safe_ffi_call(0, || {
        timing_lock().render_fps
    })
}

#[no_mangle]
pub extern "C" fn m2_py_app_get_update_fps() -> u32 {
    safe_ffi_call(0, || {
        timing_lock().update_fps
    })
}

#[no_mangle]
pub extern "C" fn m2_py_app_render_game() {
    safe_ffi_call((), || {
        timing_lock().render_frames += 1;
    });
}

#[no_mangle]
pub extern "C" fn m2_py_app_update_game() {
    safe_ffi_call((), || {
        timing_lock().update_frames += 1;
    });
}

#[no_mangle]
pub extern "C" fn m2_py_app_process() {
    safe_ffi_call((), || {
        timing_lock().process();
    });
}
