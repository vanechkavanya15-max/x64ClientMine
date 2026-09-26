use std::sync::{Mutex, OnceLock};
use glam::{Mat4, Quat, Vec3};
use m2_client_bridge::ffi_guard::safe_ffi_call;

struct SashPhysicsState {
    last_pos: Vec3,
    last_vel: Vec3,
    bend_pitch: f32,
    bend_roll: f32,
}

static SASH_STATE: OnceLock<Mutex<SashPhysicsState>> = OnceLock::new();

fn get_sash_state() -> &'static Mutex<SashPhysicsState> {
    SASH_STATE.get_or_init(|| {
        Mutex::new(SashPhysicsState {
            last_pos: Vec3::ZERO,
            last_vel: Vec3::ZERO,
            bend_pitch: 0.0,
            bend_roll: 0.0,
        })
    })
}

#[no_mangle]
pub extern "C" fn m2_sash_update(x: f32, y: f32, z: f32, yaw_deg: f32, delta_time_ms: u32) -> bool {
    safe_ffi_call(false, || {
        if delta_time_ms == 0 {
            return true;
        }

        let dt = delta_time_ms as f32 / 1000.0;
        let current_pos = Vec3::new(x, y, z);
        
        let mut state = get_sash_state().lock().unwrap();
        
        let velocity = (current_pos - state.last_pos) / dt;
        let acceleration = (velocity - state.last_vel) / dt;
        
        let yaw_rad = yaw_deg.to_radians();
        let cos_y = yaw_rad.cos();
        let sin_y = yaw_rad.sin();
        
        let local_accel_x = acceleration.x * cos_y - acceleration.y * sin_y;
        let local_accel_y = acceleration.x * sin_y + acceleration.y * cos_y;
        
        let target_pitch = (local_accel_y * -0.05).clamp(-45.0, 45.0);
        let target_roll = (local_accel_x * -0.05).clamp(-30.0, 30.0);
        
        let interp_speed = 5.0 * dt;
        state.bend_pitch += (target_pitch - state.bend_pitch) * interp_speed;
        state.bend_roll += (target_roll - state.bend_roll) * interp_speed;
        
        state.last_pos = current_pos;
        state.last_vel = velocity;

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_sash_get_transform(out_mat: *mut f32) -> bool {
    safe_ffi_call(false, || {
        if out_mat.is_null() {
            return false;
        }
        
        let state = get_sash_state().lock().unwrap();
        
        let pitch_rad = state.bend_pitch.to_radians();
        let roll_rad = state.bend_roll.to_radians();
        
        let quat = Quat::from_euler(glam::EulerRot::XYZ, pitch_rad, roll_rad, 0.0);
        let mat = Mat4::from_quat(quat);
        
        unsafe {
            let out_slice = std::slice::from_raw_parts_mut(out_mat, 16);
            out_slice.copy_from_slice(&mat.to_cols_array());
        }
        
        true
    })
}

pub fn test_reset_sash_state() {
    let mut state = get_sash_state().lock().unwrap();
    state.last_pos = Vec3::ZERO;
    state.last_vel = Vec3::ZERO;
    state.bend_pitch = 0.0;
    state.bend_roll = 0.0;
}
