use m2_client::speedtree_wind_sway_v2::*;
use std::sync::Mutex;
use std::sync::OnceLock;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_mutex() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

#[test]
fn test_default_wind_state() {
    let _lock = get_test_mutex().lock().unwrap();
    m2_forest_set_wind_params(0.0, 1.0, 1.0);
    
    // offset = sin(time * 1.0 + (x + y)) * 1.0
    // At t=0, x=0, y=0 => offset = sin(0) = 0.0
    // angle = 0 => x = offset * 1.0, y = 0.0
    let offset1 = m2_forest_get_wind_offset(0.0, 0.0, 0.0);
    assert_eq!(offset1.x, 0.0);
    assert_eq!(offset1.y, 0.0);
}

#[test]
fn test_wind_offset_positive_time() {
    let _lock = get_test_mutex().lock().unwrap();
    m2_forest_set_wind_params(0.0, 1.0, 1.0);
    
    let time = std::f32::consts::PI / 2.0; // 1.5707963
    let offset = m2_forest_get_wind_offset(0.0, 0.0, time);
    
    // phase = 0, sin(PI/2) = 1.0, angle = 0 -> cos(0) = 1, sin(0) = 0
    assert!((offset.x - 1.0).abs() < 1e-5);
    assert!((offset.y - 0.0).abs() < 1e-5);
}

#[test]
fn test_wind_offset_with_phase() {
    let _lock = get_test_mutex().lock().unwrap();
    m2_forest_set_wind_params(0.0, 1.0, 1.0);
    
    // x = PI/4, y = PI/4 => phase = PI/2
    // time = 0
    let phase_x = std::f32::consts::PI / 4.0;
    let phase_y = std::f32::consts::PI / 4.0;
    let offset = m2_forest_get_wind_offset(phase_x, phase_y, 0.0);
    
    // offset = sin(PI/2) * 1.0 = 1.0
    assert!((offset.x - 1.0).abs() < 1e-5);
    assert!((offset.y - 0.0).abs() < 1e-5);
}

#[test]
fn test_wind_offset_with_angle() {
    let _lock = get_test_mutex().lock().unwrap();
    // angle = PI/2 (90 degrees), strength = 2.0
    m2_forest_set_wind_params(std::f32::consts::PI / 2.0, 2.0, 1.0);
    
    let time = std::f32::consts::PI / 2.0; // time * freq = PI/2
    let offset = m2_forest_get_wind_offset(0.0, 0.0, time);
    
    // magnitude = sin(PI/2) * 2.0 = 2.0
    // angle = PI/2 => cos(PI/2) = 0, sin(PI/2) = 1
    // expected x = 0.0, y = 2.0
    assert!((offset.x - 0.0).abs() < 1e-5);
    assert!((offset.y - 2.0).abs() < 1e-5);
}

#[test]
fn test_wind_offset_zero_strength() {
    let _lock = get_test_mutex().lock().unwrap();
    m2_forest_set_wind_params(1.23, 0.0, 5.0);
    
    let offset = m2_forest_get_wind_offset(10.0, 20.0, 50.0);
    
    // strength = 0 => offset = 0
    assert_eq!(offset.x, 0.0);
    assert_eq!(offset.y, 0.0);
}

#[test]
fn test_wind_offset_custom_frequency() {
    let _lock = get_test_mutex().lock().unwrap();
    m2_forest_set_wind_params(0.0, 1.0, 2.0); // freq = 2.0
    
    // time = PI/4 => time * freq = PI/2
    let time = std::f32::consts::PI / 4.0; 
    let offset = m2_forest_get_wind_offset(0.0, 0.0, time);
    
    assert!((offset.x - 1.0).abs() < 1e-5);
    assert!((offset.y - 0.0).abs() < 1e-5);
}

#[test]
fn test_wind_offset_arbitrary_combo() {
    let _lock = get_test_mutex().lock().unwrap();
    let angle = 1.0;
    let strength = 3.5;
    let freq = 0.5;
    
    m2_forest_set_wind_params(angle, strength, freq);
    
    let x = 1.5;
    let y = -0.5;
    let time = 4.0;
    
    let phase = x + y;
    let expected_mag = (time * freq + phase).sin() * strength;
    let expected_x = expected_mag * angle.cos();
    let expected_y = expected_mag * angle.sin();
    
    let offset = m2_forest_get_wind_offset(x, y, time);
    
    assert!((offset.x - expected_x).abs() < 1e-5);
    assert!((offset.y - expected_y).abs() < 1e-5);
}

#[test]
fn test_wind_offset_struct_equality() {
    let offset1 = WindOffset { x: 1.0, y: 2.0 };
    let offset2 = WindOffset { x: 1.0, y: 2.0 };
    assert_eq!(offset1, offset2);
}
