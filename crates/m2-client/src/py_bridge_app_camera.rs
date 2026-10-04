use std::fs::File;
use std::io::{Read, Write};
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::path::Path;

use crate::py_bridge_app::{app_lock, CAMERA_MAX_DISTANCE};

const CAMERA_ROTATE_SPEED: f32 = 1.5;
const CAMERA_PITCH_SPEED: f32 = 1.5;
const CAMERA_ZOOM_SPEED: f32 = 0.05;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CameraPos {
    pub up_dir: f32,
    pub view_dir: f32,
    pub cross_dir: f32,
}

impl Default for CameraPos {
    fn default() -> Self {
        Self {
            up_dir: 0.0,
            view_dir: 0.0,
            cross_dir: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CameraSetting {
    pub center_position: Vector3,
    pub zoom: f32,
    pub pitch: f32,
    pub rotation: f32,
    pub cmr_pos: CameraPos,
}

impl Default for CameraSetting {
    fn default() -> Self {
        Self {
            center_position: Vector3::default(),
            zoom: 1000.0,
            pitch: 30.0,
            rotation: 0.0,
            cmr_pos: CameraPos::default(),
        }
    }
}

#[derive(Debug)]
pub struct AppCameraState {
    pub rotation_speed: f32,
    pub base_rotate_speed: f32,
    pub base_pitch_speed: f32,
    pub base_zoom_speed: f32,
    pub pitch_speed: f32,
    pub zoom_speed: f32,

    pub center_position: Vector3,
    pub cmr_pos: CameraPos,
    pub cmr_spd: CameraPos,

    pub default_camera_setting: CameraSetting,
    pub event_camera_setting: CameraSetting,
    
    pub is_locked: bool,
    pub is_special_camera_mode: bool,
}

impl Default for AppCameraState {
    fn default() -> Self {
        Self {
            rotation_speed: 0.0,
            base_rotate_speed: 1.5,
            base_pitch_speed: 1.5,
            base_zoom_speed: 0.05,
            pitch_speed: 0.0,
            zoom_speed: 0.0,
            center_position: Vector3::default(),
            cmr_pos: CameraPos::default(),
            cmr_spd: CameraPos::default(),
            default_camera_setting: CameraSetting::default(),
            event_camera_setting: CameraSetting::default(),
            is_locked: false,
            is_special_camera_mode: false,
        }
    }
}

lazy_static! {
    pub static ref PY_APP_CAMERA: Mutex<AppCameraState> = Mutex::new(AppCameraState::default());
}

pub fn app_camera_lock() -> std::sync::MutexGuard<'static, AppCameraState> {
    PY_APP_CAMERA.lock().unwrap()
}

#[no_mangle]
pub extern "C" fn m2_py_app_pitch_camera(direction: i32) {
    let mut cam = app_camera_lock();
    if cam.is_locked {
        return;
    }
    
    let degree = cam.base_pitch_speed * (direction as f32);
    cam.pitch_speed = degree;
}

#[no_mangle]
pub extern "C" fn m2_py_app_rotate_camera(direction: i32) {
    let mut cam = app_camera_lock();
    if cam.is_locked {
        return;
    }
    
    let degree = cam.base_rotate_speed * (direction as f32);
    cam.rotation_speed = degree;
}

#[no_mangle]
pub extern "C" fn m2_py_app_zoom_camera(direction: i32) {
    let mut cam = app_camera_lock();
    if cam.is_locked {
        return;
    }
    
    let ratio = 1.0 + cam.base_zoom_speed * (direction as f32);
    cam.zoom_speed = ratio;
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_camera(distance: f32, pitch: f32, rotation: f32, destination_height: f32) {
    let cam = app_camera_lock();
    if cam.is_locked {
        return;
    }
    
    if let Ok(mut app) = app_lock() {
        app.set_camera(distance, pitch, rotation, destination_height);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_get_camera_pitch() -> f32 {
    if let Ok(app) = app_lock() {
        app.camera.pitch
    } else {
        30.0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_get_camera_rotation() -> f32 {
    if let Ok(app) = app_lock() {
        app.camera.rotation
    } else {
        0.0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_load_camera_setting(file_name: *const std::ffi::c_char) -> bool {
    if file_name.is_null() {
        return false;
    }
    let c_str = unsafe { std::ffi::CStr::from_ptr(file_name) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    load_camera_setting_internal(path_str)
}

pub fn load_camera_setting_internal(path_str: &str) -> bool {
    let path = Path::new(path_str);
    if !path.exists() {
        return false;
    }

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return false;
    }

    let mut center_pos = Vector3::default();
    let mut camera_setting = (1000.0, 30.0, 0.0);
    let mut cmr_pos = CameraPos::default();

    let mut found_center = false;
    let mut found_setting = false;
    let mut found_cmr = false;

    for line in contents.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0].to_lowercase().as_str() {
            "centerpos" => {
                if parts.len() >= 4 {
                    center_pos.x = parts[1].parse().unwrap_or(0.0);
                    center_pos.y = parts[2].parse().unwrap_or(0.0);
                    center_pos.z = parts[3].parse().unwrap_or(0.0);
                    found_center = true;
                }
            }
            "camerasetting" => {
                if parts.len() >= 4 {
                    camera_setting.0 = parts[1].parse().unwrap_or(1000.0);
                    camera_setting.1 = parts[2].parse().unwrap_or(30.0);
                    camera_setting.2 = parts[3].parse().unwrap_or(0.0);
                    found_setting = true;
                }
            }
            "cmrpos" => {
                if parts.len() >= 4 {
                    cmr_pos.up_dir = parts[1].parse().unwrap_or(0.0);
                    cmr_pos.view_dir = parts[2].parse().unwrap_or(0.0);
                    cmr_pos.cross_dir = parts[3].parse().unwrap_or(0.0);
                    found_cmr = true;
                }
            }
            _ => {}
        }
    }

    if found_center && found_setting && found_cmr {
        let mut cam = app_camera_lock();
        let mut setting = CameraSetting::default();
        setting.center_position = center_pos;
        setting.zoom = camera_setting.0;
        setting.pitch = camera_setting.1;
        setting.rotation = camera_setting.2;
        setting.cmr_pos = cmr_pos;
        
        cam.event_camera_setting = setting.clone();
        
        // Also update the core PyBridgeApp camera
        if let Ok(mut app) = app_lock() {
            app.set_camera(setting.zoom, setting.pitch, setting.rotation, setting.center_position.z);
        }
        
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_save_camera_setting(file_name: *const std::ffi::c_char) {
    if file_name.is_null() {
        return;
    }
    let c_str = unsafe { std::ffi::CStr::from_ptr(file_name) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return,
    };
    
    save_camera_setting_internal(path_str);
}

pub fn save_camera_setting_internal(path_str: &str) {
    let cam = app_camera_lock();
    
    // Retrieve current core camera settings
    let mut zoom = 1000.0;
    let mut pitch = 30.0;
    let mut rotation = 0.0;
    let mut target_height = 0.0;
    
    if let Ok(app) = app_lock() {
        zoom = app.camera.distance;
        pitch = app.camera.pitch;
        rotation = app.camera.rotation;
        target_height = app.camera.target_height;
    }
    
    let mut center_pos = cam.center_position.clone();
    center_pos.z += target_height;

    let mut file = match File::create(path_str) {
        Ok(f) => f,
        Err(_) => return,
    };

    let _ = writeln!(
        file,
        "CenterPos {:.6} {:.6} {:.6}",
        center_pos.x, center_pos.y, center_pos.z
    );
    let _ = writeln!(
        file,
        "CameraSetting {:.6} {:.6} {:.6}",
        zoom, pitch, rotation
    );
    let _ = writeln!(
        file,
        "CmrPos {:.6} {:.6} {:.6}",
        cam.cmr_pos.up_dir, cam.cmr_pos.view_dir, cam.cmr_pos.cross_dir
    );
    let _ = writeln!(
        file,
        "Line \"x;{}|y;{}|z;{}|distance;{}|pitch;{}|rot;{}|up;{}|view;{}|cross;{}\"",
        center_pos.x as i32,
        center_pos.y as i32,
        center_pos.z as i32,
        zoom as i32,
        pitch as i32,
        rotation as i32,
        cam.cmr_pos.up_dir as i32,
        cam.cmr_pos.view_dir as i32,
        cam.cmr_pos.cross_dir as i32
    );
}

#[no_mangle]
pub extern "C" fn m2_py_app_camera_update() {
    let mut cam = app_camera_lock();
    if cam.is_locked {
        return;
    }

    if let Ok(mut app) = app_lock() {
        if cam.rotation_speed != 0.0 {
            app.camera.rotation += cam.rotation_speed;
            while app.camera.rotation >= 360.0 {
                app.camera.rotation -= 360.0;
            }
            while app.camera.rotation < 0.0 {
                app.camera.rotation += 360.0;
            }
        }

        if cam.pitch_speed != 0.0 {
            let mut new_pitch = app.camera.pitch + cam.pitch_speed;
            if new_pitch > 80.0 {
                new_pitch = 80.0;
            } else if new_pitch < 10.0 {
                new_pitch = 10.0;
            }
            app.camera.pitch = new_pitch;
        }

        if cam.zoom_speed != 0.0 {
            let mut new_distance = app.camera.distance * cam.zoom_speed;
            // CAMERA_MAX_DISTANCE = 2500.0, min distance from original = 400.0.
            if new_distance > CAMERA_MAX_DISTANCE {
                new_distance = CAMERA_MAX_DISTANCE;
            } else if new_distance < 400.0 {
                new_distance = 400.0;
            }
            app.camera.distance = new_distance;
        }
        
        if cam.cmr_spd.view_dir != 0.0 {
            cam.cmr_pos.view_dir += cam.cmr_spd.view_dir;
        }
        if cam.cmr_spd.cross_dir != 0.0 {
            cam.cmr_pos.cross_dir += cam.cmr_spd.cross_dir;
        }
        if cam.cmr_spd.up_dir != 0.0 {
            cam.cmr_pos.up_dir += cam.cmr_spd.up_dir;
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_camera_rotate_speed(speed: f32) {
    let mut cam = app_camera_lock();
    cam.base_rotate_speed = speed;
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_camera_pitch_speed(speed: f32) {
    let mut cam = app_camera_lock();
    cam.base_pitch_speed = speed;
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_camera_zoom_speed(speed: f32) {
    let mut cam = app_camera_lock();
    cam.base_zoom_speed = speed;
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_default_camera() {
    let mut cam = app_camera_lock();
    cam.is_special_camera_mode = false;
    
    // Copy default to current
    let default_setting = cam.default_camera_setting.clone();
    cam.event_camera_setting = default_setting.clone();
    
    // Set actual camera via core
    if let Ok(mut app) = app_lock() {
        app.set_camera(
            default_setting.zoom,
            default_setting.pitch,
            default_setting.rotation,
            default_setting.center_position.z
        );
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_camera_setting(setting: *const CameraSetting) {
    if setting.is_null() {
        return;
    }
    let setting_ref = unsafe { &*setting };
    
    let mut cam = app_camera_lock();
    cam.event_camera_setting = setting_ref.clone();
    
    // Reset camera movement speeds
    cam.cmr_pos.up_dir = 0.0;
    cam.cmr_pos.view_dir = 0.0;
    cam.cmr_pos.cross_dir = 0.0;
    cam.cmr_spd.up_dir = 0.0;
    cam.cmr_spd.view_dir = 0.0;
    cam.cmr_spd.cross_dir = 0.0;
    cam.zoom_speed = 0.0;
    cam.pitch_speed = 0.0;
    cam.rotation_speed = 0.0;
    
    if let Ok(mut app) = app_lock() {
        app.set_camera(
            setting_ref.zoom,
            setting_ref.pitch,
            setting_ref.rotation,
            setting_ref.center_position.z
        );
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_get_camera_setting(out_setting: *mut CameraSetting) {
    if out_setting.is_null() {
        return;
    }
    
    let cam = app_camera_lock();
    
    // Retrieve current core camera settings
    let mut zoom = 1000.0;
    let mut pitch = 30.0;
    let mut rotation = 0.0;
    let mut target_height = 0.0;
    
    if let Ok(app) = app_lock() {
        zoom = app.camera.distance;
        pitch = app.camera.pitch;
        rotation = app.camera.rotation;
        target_height = app.camera.target_height;
    }
    
    let mut current_setting = CameraSetting {
        center_position: cam.center_position.clone(),
        zoom,
        pitch,
        rotation,
        cmr_pos: cam.cmr_pos.clone(),
    };
    current_setting.center_position.z += target_height;
    
    unsafe {
        *out_setting = current_setting;
    }
}
