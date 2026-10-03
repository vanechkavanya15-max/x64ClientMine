use std::fs::File;
use std::io::Read;
use std::path::Path;

use m2_client::py_bridge_app_camera::*;
use m2_client::py_bridge_app::{app_lock, CAMERA_MAX_DISTANCE};

#[test]
fn test_camera_pitch_speed() {
    // Reset state
    {
        let mut cam = app_camera_lock();
        cam.pitch_speed = 0.0;
        cam.is_locked = false;
        
        let mut app = app_lock().unwrap();
        app.camera.pitch = 30.0;
    }
    
    // Call function
    m2_py_app_pitch_camera(1);
    
    // Verify speed updated
    {
        let cam = app_camera_lock();
        assert_eq!(cam.pitch_speed, 1.5);
    }
    
    // Call update
    m2_py_app_camera_update();
    
    // Verify app camera pitch changed
    let pitch = m2_py_app_get_camera_pitch();
    assert_eq!(pitch, 31.5);
}

#[test]
fn test_camera_rotation_speed() {
    // Reset state
    {
        let mut cam = app_camera_lock();
        cam.rotation_speed = 0.0;
        cam.is_locked = false;
        
        let mut app = app_lock().unwrap();
        app.camera.rotation = 0.0;
    }
    
    m2_py_app_rotate_camera(2);
    
    {
        let cam = app_camera_lock();
        assert_eq!(cam.rotation_speed, 3.0); // 1.5 * 2
    }
    
    m2_py_app_camera_update();
    
    let rotation = m2_py_app_get_camera_rotation();
    assert_eq!(rotation, 3.0);
}

#[test]
fn test_camera_zoom_speed_limits() {
    // Reset state
    {
        let mut cam = app_camera_lock();
        cam.zoom_speed = 0.0;
        cam.is_locked = false;
        
        let mut app = app_lock().unwrap();
        app.camera.distance = 1000.0;
    }
    
    m2_py_app_zoom_camera(10); // ratio = 1.0 + 0.05 * 10 = 1.5
    
    m2_py_app_camera_update();
    
    {
        let app = app_lock().unwrap();
        assert_eq!(app.camera.distance, 1500.0);
    }
    
    // Zoom way out, test limit
    m2_py_app_zoom_camera(100); // huge ratio -> > MAX_DISTANCE
    m2_py_app_camera_update();
    
    {
        let app = app_lock().unwrap();
        assert_eq!(app.camera.distance, CAMERA_MAX_DISTANCE);
    }
    
    // Zoom way in, test limit
    m2_py_app_zoom_camera(-100); // negative ratio
    m2_py_app_camera_update();
    
    {
        let app = app_lock().unwrap();
        assert_eq!(app.camera.distance, 400.0);
    }
}

#[test]
fn test_camera_save_load_setting() {
    let test_file = "test_camera_setting.txt";
    
    // Setup initial state
    {
        let mut cam = app_camera_lock();
        cam.center_position.x = 100.0;
        cam.center_position.y = 200.0;
        cam.center_position.z = 300.0;
        
        cam.cmr_pos.up_dir = 1.0;
        cam.cmr_pos.view_dir = 2.0;
        cam.cmr_pos.cross_dir = 3.0;
        
        let mut app = app_lock().unwrap();
        app.camera.distance = 800.0;
        app.camera.pitch = 45.0;
        app.camera.rotation = 90.0;
        app.camera.target_height = 50.0;
    }
    
    let c_str = std::ffi::CString::new(test_file).unwrap();
    m2_py_app_save_camera_setting(c_str.as_ptr());
    
    // Read directly to verify format
    let mut file = File::open(test_file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    // Due to Z height addition (300.0 center + 50.0 target_height) = 350.0
    assert!(content.contains("CenterPos 100.000000 200.000000 350.000000"));
    assert!(content.contains("CameraSetting 800.000000 45.000000 90.000000"));
    assert!(content.contains("CmrPos 1.000000 2.000000 3.000000"));
    assert!(content.contains("Line \"x;100|y;200|z;350|distance;800|pitch;45|rot;90|up;1|view;2|cross;3\""));
    
    // Clear state
    {
        let mut cam = app_camera_lock();
        cam.center_position = Vector3::default();
        let mut app = app_lock().unwrap();
        app.camera.distance = 0.0;
    }
    
    // Load it back
    let success = m2_py_app_load_camera_setting(c_str.as_ptr());
    assert!(success);
    
    {
        let app = app_lock().unwrap();
        assert_eq!(app.camera.distance, 800.0);
        assert_eq!(app.camera.pitch, 45.0);
        assert_eq!(app.camera.rotation, 90.0);
    }
    
    {
        let cam = app_camera_lock();
        assert_eq!(cam.event_camera_setting.center_position.x, 100.0);
        assert_eq!(cam.event_camera_setting.center_position.y, 200.0);
        assert_eq!(cam.event_camera_setting.center_position.z, 350.0); // loaded as 350
        assert_eq!(cam.event_camera_setting.cmr_pos.up_dir, 1.0);
    }
    
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_camera_is_locked() {
    {
        let mut cam = app_camera_lock();
        cam.is_locked = true;
        cam.pitch_speed = 0.0;
    }
    
    m2_py_app_pitch_camera(1);
    
    {
        let cam = app_camera_lock();
        assert_eq!(cam.pitch_speed, 0.0, "Should not update pitch speed when locked");
    }
    
    {
        let mut cam = app_camera_lock();
        cam.is_locked = false;
    }
}
