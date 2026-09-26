use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::sync::{Mutex, OnceLock};
use crate::camera_orbit::CameraOrbit;

struct RenderState {
    width: u32,
    height: u32,
    camera: CameraOrbit,
    drawn_quads: usize,
}

static RENDER_STATE: OnceLock<Mutex<RenderState>> = OnceLock::new();

fn get_render_state() -> std::sync::MutexGuard<'static, RenderState> {
    RENDER_STATE
        .get_or_init(|| {
            Mutex::new(RenderState {
                width: 0,
                height: 0,
                camera: CameraOrbit::new([0.0, 0.0, 0.0], 10.0, 0.0, 1000.0),
                drawn_quads: 0,
            })
        })
        .lock()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn m2_render_init(width: u32, height: u32) -> bool {
    safe_ffi_call(false, || {
        if width == 0 || height == 0 {
            return false;
        }

        let mut state = get_render_state();
        state.width = width;
        state.height = height;
        state.drawn_quads = 0;
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_camera_set_target(x: f32, y: f32, z: f32) {
    safe_ffi_call((), || {
        let mut state = get_render_state();
        state.camera.set_target([x, y, z]);
    })
}

#[no_mangle]
pub extern "C" fn m2_camera_rotate(yaw: f32, pitch: f32) {
    safe_ffi_call((), || {
        let mut state = get_render_state();
        state.camera.set_yaw(yaw);
        state.camera.set_pitch(pitch);
    })
}

#[no_mangle]
pub extern "C" fn m2_camera_zoom(delta_dist: f32) {
    safe_ffi_call((), || {
        let mut state = get_render_state();
        let new_dist = state.camera.distance + delta_dist;
        state.camera.set_distance(new_dist);
    })
}

#[no_mangle]
pub extern "C" fn m2_camera_get_eye(out_x: *mut f32, out_y: *mut f32, out_z: *mut f32) -> bool {
    safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_z.is_null() {
            return false;
        }

        let state = get_render_state();
        let eye = state.camera.get_eye_position();

        unsafe {
            *out_x = eye[0];
            *out_y = eye[1];
            *out_z = eye[2];
        }

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_render_draw_ui_quad(x: f32, y: f32, w: f32, h: f32, color_argb: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_render_state();

        if state.width == 0 || state.height == 0 {
            return false; // Not initialized
        }

        if w <= 0.0 || h <= 0.0 {
            return false;
        }

        // Simulating drawing quad by incrementing count
        // and suppressing unused variable warning.
        let _ = x;
        let _ = y;
        let _ = color_argb;
        
        state.drawn_quads += 1;
        true
    })
}
