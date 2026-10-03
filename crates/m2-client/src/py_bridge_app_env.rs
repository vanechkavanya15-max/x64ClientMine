use std::ffi::CStr;
use std::os::raw::c_char;
use std::panic::catch_unwind;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone)]
pub struct AppEnvState {
    pub min_fog: f32,
    pub sight_range: i32,
    pub specular_speed: f32,
    pub locale_path: String,
}

impl Default for AppEnvState {
    fn default() -> Self {
        Self {
            min_fog: 2400.0,
            sight_range: -1,
            specular_speed: 0.007,
            locale_path: String::new(),
        }
    }
}

static APP_ENV_STATE: OnceLock<RwLock<AppEnvState>> = OnceLock::new();

pub fn get_env_state() -> &'static RwLock<AppEnvState> {
    APP_ENV_STATE.get_or_init(|| RwLock::new(AppEnvState::default()))
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_min_fog(min_fog: f32) {
    let _ = catch_unwind(|| {
        if let Ok(mut state) = get_env_state().write() {
            state.min_fog = min_fog;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_sight_range(range: i32) {
    let _ = catch_unwind(|| {
        if let Ok(mut state) = get_env_state().write() {
            state.sight_range = range;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_specular_speed(speed: f32) {
    let _ = catch_unwind(|| {
        if let Ok(mut state) = get_env_state().write() {
            state.specular_speed = speed;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_app_load_locale_data(locale_path: *const c_char) -> i32 {
    let result = catch_unwind(|| {
        if locale_path.is_null() {
            return 0; // Failure
        }

        let c_str = unsafe { CStr::from_ptr(locale_path) };
        if let Ok(path_str) = c_str.to_str() {
            if let Ok(mut state) = get_env_state().write() {
                state.locale_path = path_str.to_string();
                // Simulating successful load of locale data as requested
                return 1;
            }
        }
        0 // Failure
    });

    result.unwrap_or(0)
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FogParams {
    pub near_dist: f32,
    pub far_dist: f32,
}

#[no_mangle]
pub extern "C" fn m2_app_env_get_fog_params(render_time: f32) -> FogParams {
    let result = catch_unwind(|| {
        let state = get_env_state().read().unwrap();
        let near = state.min_fog;

        if state.sight_range != -1 {
            FogParams {
                near_dist: near,
                far_dist: state.sight_range as f32,
            }
        } else {
            let s_f_ave_render_time = if render_time <= 0.0 { 16.0 } else { render_time };
            let db_ave_pow = 1000.0 / s_f_ave_render_time;
            let db_max_pow = 60.0;
            let f_far = 25600.0;

            let mut f_distance = near + (f_far - near) * (db_ave_pow as f32 / db_max_pow as f32);
            if f_distance < near {
                f_distance = near;
            }

            FogParams {
                near_dist: near,
                far_dist: f_distance,
            }
        }
    });

    result.unwrap_or(FogParams {
        near_dist: 2400.0,
        far_dist: 25600.0,
    })
}
