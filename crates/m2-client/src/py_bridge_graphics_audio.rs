use std::ffi::CStr;
use std::os::raw::c_char;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::sync::{Mutex, OnceLock};

use crate::py_bridge_app::app_lock as py_app_lock;
use crate::actor_text_tail::{TextTail, Empire, Vector3};

// textTail
// In C++, the text tail is managed globally. We can store it here for the bridge.

pub struct TextTailState {
    pub registered_tails: Vec<u32>,
    pub shown_tails: Vec<u32>,
    pub attached_titles: std::collections::HashMap<u32, String>,
}

static TEXT_TAIL_STATE: OnceLock<Mutex<TextTailState>> = OnceLock::new();

pub fn get_text_tail_state() -> std::sync::MutexGuard<'static, TextTailState> {
    TEXT_TAIL_STATE
        .get_or_init(|| Mutex::new(TextTailState {
            registered_tails: Vec::new(),
            shown_tails: Vec::new(),
            attached_titles: std::collections::HashMap::new(),
        }))
        .lock()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn m2_text_tail_register_character(
    vid: u32,
    name_ptr: *const c_char,
    _r: f32,
    _g: f32,
    _b: f32,
) -> bool {
    safe_ffi_call(false, || {
        if name_ptr.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(name_ptr) };
        if let Ok(_name) = c_str.to_str() {
            let mut state = get_text_tail_state();
            if !state.registered_tails.contains(&vid) {
                state.registered_tails.push(vid);
            }
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_text_tail_show_character(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_text_tail_state();
        if state.registered_tails.contains(&vid) {
            if !state.shown_tails.contains(&vid) {
                state.shown_tails.push(vid);
            }
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_text_tail_hide_character(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_text_tail_state();
        state.shown_tails.retain(|&x| x != vid);
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_text_tail_attach_title(
    vid: u32,
    title_ptr: *const c_char,
    _r: f32,
    _g: f32,
    _b: f32,
) -> bool {
    safe_ffi_call(false, || {
        if title_ptr.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(title_ptr) };
        if let Ok(title) = c_str.to_str() {
            let mut state = get_text_tail_state();
            state.attached_titles.insert(vid, title.to_string());
            true
        } else {
            false
        }
    })
}

// grp / grpImage

// The render functions can hook into Batch2D if needed, but since we don't have
// a globally exposed Batch2D, we will use a global GrpState to record operations
// like the C++ bridge does for basic Python grp bindings.

use crate::render_2d_batch::Batch2D;

pub struct GrpState {
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub color_a: f32,
    pub batch: Batch2D,
}

impl Default for GrpState {
    fn default() -> Self {
        Self {
            color_r: 1.0,
            color_g: 1.0,
            color_b: 1.0,
            color_a: 1.0,
            batch: Batch2D::new(),
        }
    }
}

static GRP_STATE: OnceLock<Mutex<GrpState>> = OnceLock::new();

pub fn get_grp_state() -> std::sync::MutexGuard<'static, GrpState> {
    GRP_STATE
        .get_or_init(|| Mutex::new(GrpState::default()))
        .lock()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn m2_grp_set_color(r: f32, g: f32, b: f32, a: f32) {
    safe_ffi_call((), || {
        let mut state = get_grp_state();
        state.color_r = r;
        state.color_g = g;
        state.color_b = b;
        state.color_a = a;
    })
}

#[no_mangle]
pub extern "C" fn m2_grp_render_box(x: f32, y: f32, w: f32, h: f32) -> bool {
    safe_ffi_call(false, || {
        if w <= 0.0 || h <= 0.0 {
            return false;
        }
        let mut state = get_grp_state();
        let color = [state.color_r, state.color_g, state.color_b, state.color_a];
        state.batch.add_quad(x, y, w, h, (0.0, 0.0, 1.0, 1.0), color);
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_grp_render_line(x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_grp_state();
        let color = [state.color_r, state.color_g, state.color_b, state.color_a];
        
        let dx = x2 - x1;
        let dy = y2 - y1;
        let length = (dx * dx + dy * dy).sqrt();
        if length == 0.0 {
            return false;
        }
        
        // Simulating a line with a thin quad of thickness 1.0
        let thickness = 1.0;
        let angle = dy.atan2(dx);
        
        // This is a simplified representation of a line just to add to the batch
        state.batch.add_quad(x1, y1, length, thickness, (0.0, 0.0, 1.0, 1.0), color);
        true
    })
}


// snd (Audio)
// Wrap the existing FFI functions from sound_ffi.rs

use crate::sound_ffi::{
    m2_audio_play_sound_2d,
    m2_audio_play_bgm,
    m2_audio_stop_bgm,
    m2_audio_set_master_volume,
    m2_audio_set_category_volume
};

#[no_mangle]
pub extern "C" fn m2_snd_play_sound(filename_ptr: *const c_char) -> bool {
    m2_audio_play_sound_2d(filename_ptr, 1.0)
}

#[no_mangle]
pub extern "C" fn m2_snd_play_bgm(filename_ptr: *const c_char) -> bool {
    m2_audio_play_bgm(filename_ptr, true)
}

#[no_mangle]
pub extern "C" fn m2_snd_stop_bgm() -> bool {
    m2_audio_stop_bgm(true)
}

#[no_mangle]
pub extern "C" fn m2_snd_set_sound_volume(vol: f32) {
    m2_audio_set_category_volume(0, vol);
}

#[no_mangle]
pub extern "C" fn m2_snd_set_bgm_volume(vol: f32) {
    m2_audio_set_category_volume(1, vol);
}


// systemSetting

#[no_mangle]
pub extern "C" fn m2_sys_get_resolution(out_w: *mut u32, out_h: *mut u32) -> bool {
    safe_ffi_call(false, || {
        if out_w.is_null() || out_h.is_null() {
            return false;
        }
        
        if let Ok(app) = py_app_lock() {
            unsafe {
                *out_w = app.get_width();
                *out_h = app.get_height();
            }
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_sys_set_resolution(w: u32, h: u32) -> bool {
    safe_ffi_call(false, || {
        if w == 0 || h == 0 {
            return false;
        }
        
        if let Ok(mut app) = py_app_lock() {
            app.set_resolution(w, h);
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_sys_get_bgm_volume() -> f32 {
    safe_ffi_call(0.0, || {
        let state = crate::sound_ffi::get_audio_state().lock().unwrap();
        state.category_volumes[1]
    })
}

#[no_mangle]
pub extern "C" fn m2_sys_set_bgm_volume(v: f32) {
    m2_audio_set_category_volume(1, v);
}


// dbg

use std::fs::OpenOptions;
use std::io::Write;

pub const SYSERR_FILE: &str = "syserr.txt";

pub fn log_to_syserr(msg: &str) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(SYSERR_FILE) {
        let _ = writeln!(file, "[Python DBG] {}", msg);
    }
}

pub struct DbgState {
    pub traces: Vec<String>,
    pub errors: Vec<String>,
    pub log_boxes: Vec<String>,
}

static DBG_STATE: OnceLock<Mutex<DbgState>> = OnceLock::new();

pub fn get_dbg_state() -> std::sync::MutexGuard<'static, DbgState> {
    DBG_STATE
        .get_or_init(|| Mutex::new(DbgState {
            traces: Vec::new(),
            errors: Vec::new(),
            log_boxes: Vec::new(),
        }))
        .lock()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn m2_dbg_trace(msg_ptr: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if msg_ptr.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(msg_ptr) };
        if let Ok(msg) = c_str.to_str() {
            let mut state = get_dbg_state();
            state.traces.push(msg.to_string());
            log_to_syserr(&format!("TRACE: {}", msg));
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_dbg_trace_error(msg_ptr: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if msg_ptr.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(msg_ptr) };
        if let Ok(msg) = c_str.to_str() {
            let mut state = get_dbg_state();
            state.errors.push(msg.to_string());
            log_to_syserr(&format!("ERROR: {}", msg));
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_dbg_log_box(msg_ptr: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if msg_ptr.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(msg_ptr) };
        if let Ok(msg) = c_str.to_str() {
            let mut state = get_dbg_state();
            state.log_boxes.push(msg.to_string());
            log_to_syserr(&format!("LOGBOX: {}", msg));
            true
        } else {
            false
        }
    })
}