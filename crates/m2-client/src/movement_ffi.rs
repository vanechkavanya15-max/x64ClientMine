use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::sync::Mutex;

pub struct MovementState {
    pub target_x: f32,
    pub target_y: f32,
    pub is_moving: bool,
}

static MOVEMENT_STATE: Mutex<MovementState> = Mutex::new(MovementState {
    target_x: 0.0,
    target_y: 0.0,
    is_moving: false,
});

/// Pomocnicza funkcja do testow, nie eksportowana do C.
pub fn get_movement_state() -> (f32, f32, bool) {
    let state = MOVEMENT_STATE.lock().unwrap();
    (state.target_x, state.target_y, state.is_moving)
}

#[no_mangle]
pub extern "C" fn m2_move_to(target_x: f32, target_y: f32) -> bool {
    safe_ffi_call(false, || {
        if !m2_is_walkable(target_x, target_y) {
            return false;
        }
        
        let mut state = MOVEMENT_STATE.lock().unwrap();
        state.target_x = target_x;
        state.target_y = target_y;
        state.is_moving = true;
        
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_get_height_at(x: f32, y: f32) -> f32 {
    safe_ffi_call(0.0, || {
        let height = (x * 0.01).sin() * 100.0 + (y * 0.01).cos() * 100.0;
        height
    })
}

#[no_mangle]
pub extern "C" fn m2_is_walkable(x: f32, y: f32) -> bool {
    safe_ffi_call(false, || {
        if !x.is_finite() || !y.is_finite() {
            return false;
        }
        if x < 0.0 || y < 0.0 {
            return false;
        }
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_move_to_direction(dir_rot: f32) -> bool {
    safe_ffi_call(false, || {
        if !dir_rot.is_finite() {
            return false;
        }
        let mut state = MOVEMENT_STATE.lock().unwrap();
        state.is_moving = true;
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_stop_movement() {
    safe_ffi_call((), || {
        let mut state = MOVEMENT_STATE.lock().unwrap();
        state.is_moving = false;
    })
}
