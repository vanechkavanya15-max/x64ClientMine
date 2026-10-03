#[path = "../src/py_bridge_app_env.rs"]
mod py_bridge_app_env;

use py_bridge_app_env::*;
use std::ffi::CString;

#[test]
fn test_default_values() {
    let state = get_env_state().read().unwrap();
    assert_eq!(state.min_fog, 2400.0);
    assert_eq!(state.sight_range, -1);
    assert_eq!(state.specular_speed, 0.007);
    assert_eq!(state.locale_path, "");
}

#[test]
fn test_set_min_fog() {
    m2_py_app_set_min_fog(1500.0);
    let state = get_env_state().read().unwrap();
    assert_eq!(state.min_fog, 1500.0);
}

#[test]
fn test_set_sight_range() {
    m2_py_app_set_sight_range(5000);
    let state = get_env_state().read().unwrap();
    assert_eq!(state.sight_range, 5000);
}

#[test]
fn test_set_specular_speed() {
    m2_py_app_set_specular_speed(0.015);
    let state = get_env_state().read().unwrap();
    assert_eq!(state.specular_speed, 0.015);
}

#[test]
fn test_load_locale_data_valid() {
    let locale = CString::new("locale/pl/").unwrap();
    let result = m2_py_app_load_locale_data(locale.as_ptr());
    
    assert_eq!(result, 1);
    let state = get_env_state().read().unwrap();
    assert_eq!(state.locale_path, "locale/pl/");
}

#[test]
fn test_load_locale_data_null() {
    let result = m2_py_app_load_locale_data(std::ptr::null());
    assert_eq!(result, 0);
}

#[test]
fn test_fog_params_calculation() {
    // Reset state first to ensure test isolation
    m2_py_app_set_min_fog(2400.0);
    m2_py_app_set_sight_range(-1);

    // Test case 1: sight_range == -1
    let params_dynamic = m2_app_env_get_fog_params(16.0);
    assert_eq!(params_dynamic.near_dist, 2400.0);
    // When render_time=16.0 -> ave_pow = 1000/16 = 62.5
    // db_max_pow = 60.0
    // distance = 2400 + (25600 - 2400) * (62.5 / 60.0)
    // distance = 2400 + (23200) * 1.0416666... = 2400 + 24166.666... = 26566.666...
    assert!(params_dynamic.far_dist > 25600.0);
    
    // Test case 2: sight_range != -1
    m2_py_app_set_sight_range(8000);
    let params_fixed = m2_app_env_get_fog_params(16.0);
    assert_eq!(params_fixed.near_dist, 2400.0);
    assert_eq!(params_fixed.far_dist, 8000.0);
}
