use m2_client::py_bridge_app::*;
use serial_test::serial;

#[test]
#[serial]
fn test_default_config() {
    // Reset global state if any previous test modified it unexpectedly
    m2_app_set_resolution(800, 600);
    
    let width = m2_app_get_width();
    let height = m2_app_get_height();
    assert_eq!(width, 800);
    assert_eq!(height, 600);
}

#[test]
#[serial]
fn test_set_resolution() {
    m2_app_set_resolution(1920, 1080);
    let width = m2_app_get_width();
    let height = m2_app_get_height();
    assert_eq!(width, 1920);
    assert_eq!(height, 1080);
}

#[test]
#[serial]
fn test_time_updates() {
    m2_app_set_time(123.45, 1234567890);
    let current_time = m2_app_get_time();
    let global_time = m2_app_get_global_time();
    assert_eq!(current_time, 123.45);
    assert_eq!(global_time, 1234567890);
}

#[test]
#[serial]
fn test_camera_state() {
    m2_app_set_camera(1500.0, 45.0, 90.0, 120.0);
    let app = app_lock().unwrap();
    assert_eq!(app.camera.distance, 1500.0);
    assert_eq!(app.camera.pitch, 45.0);
    assert_eq!(app.camera.rotation, 90.0);
    assert_eq!(app.camera.target_height, 120.0);
}

#[test]
#[serial]
fn test_enable_special_camera_mode() {
    m2_app_enable_special_camera_mode();
    let app = app_lock().unwrap();
    assert_eq!(app.camera.is_special_mode, true);
}

#[test]
#[serial]
fn test_keys_pressed() {
    m2_app_set_key_state(DIK_ESCAPE, true);
    m2_app_set_key_state(DIK_SPACE, true);
    m2_app_set_key_state(DIK_A, false);

    assert_eq!(m2_app_is_pressed(DIK_ESCAPE), true);
    assert_eq!(m2_app_is_pressed(DIK_SPACE), true);
    assert_eq!(m2_app_is_pressed(DIK_A), false);

    // Out of bounds
    assert_eq!(m2_app_is_pressed(300), false);
    
    // Internal struct validation
    let mut app = app_lock().unwrap();
    assert_eq!(app.is_pressed(DIK_SPACE), Ok(true));
    assert_eq!(app.is_pressed(300), Err(AppBridgeError::InvalidKeyCode(300)));
}

#[test]
#[serial]
fn test_exit_request() {
    {
        let mut app = app_lock().unwrap();
        app.is_exit_requested = false;
    }

    m2_app_exit();

    let app = app_lock().unwrap();
    assert_eq!(app.is_exit_requested, true);
}

#[test]
#[serial]
fn test_constants_exposed() {
    assert_eq!(DIK_ESCAPE, 0x01);
    assert_eq!(DIK_RETURN, 0x1C);
    assert_eq!(DIK_SPACE, 0x39);
    assert_eq!(INFO_ITEM, 1);
    assert_eq!(INFO_ACTOR, 2);
    assert_eq!(CAMERA_MAX_DISTANCE, 2500.0);
}
