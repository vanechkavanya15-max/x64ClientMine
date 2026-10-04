use m2_client::py_bridge_app_camera_complete::{
    get_camera_pitch_speed, get_camera_rotate_speed, get_camera_zoom_speed, is_sensor_view_enabled,
    set_camera_pitch_speed, set_camera_rotate_speed, set_camera_zoom_speed, set_sensor_view_enabled,
};

#[test]
fn test_app_camera_speed_controls() {
    set_camera_rotate_speed(2.5);
    assert_eq!(get_camera_rotate_speed(), 2.5);

    set_camera_pitch_speed(3.0);
    assert_eq!(get_camera_pitch_speed(), 3.0);

    set_camera_zoom_speed(0.12);
    assert_eq!(get_camera_zoom_speed(), 0.12);
}

#[test]
fn test_app_camera_sensor_view() {
    set_sensor_view_enabled(true);
    assert!(is_sensor_view_enabled());

    set_sensor_view_enabled(false);
    assert!(!is_sensor_view_enabled());
}
