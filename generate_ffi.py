import textwrap

rust_code = """
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::path::Path;
use rand::Rng;

lazy_static! {
    pub static ref HAIR_COLOR_ENABLE: Mutex<bool> = Mutex::new(false);
    pub static ref USE_ARMOR_SPECULAR: Mutex<bool> = Mutex::new(false);
    pub static ref USE_WEAPON_SPECULAR: Mutex<bool> = Mutex::new(false);
    pub static ref FRAME_SKIP: Mutex<bool> = Mutex::new(false);
    pub static ref DEFAULT_FONT_NAME: Mutex<String> = Mutex::new(String::new());
    pub static ref GUILD_MARK_PATH: Mutex<String> = Mutex::new(String::new());
    pub static ref GUILD_SYMBOL_PATH: Mutex<String> = Mutex::new(String::new());
    pub static ref CENTER_POSITION: Mutex<(f32, f32, f32)> = Mutex::new((0.0, 0.0, 0.0));
    pub static ref MOUSE_HANDLER: Mutex<i64> = Mutex::new(0);
}

#[no_mangle]
pub extern "C" fn m2_app_set_frame_skip(frame_skip: i32) {
    if let Ok(mut lock) = FRAME_SKIP.lock() {
        *lock = frame_skip != 0;
    }
}

#[no_mangle]
pub extern "C" fn m2_app_get_random(from: i32, to: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let (min, max) = if from > to { (to, from) } else { (from, to) };
    if min == max {
        min
    } else {
        rng.gen_range(min..=max)
    }
}

#[no_mangle]
pub extern "C" fn m2_app_get_global_time_stamp() -> i32 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or(std::time::Duration::from_secs(0)).as_secs() as i32
}

#[no_mangle]
pub extern "C" fn m2_app_set_mouse_handler(handler_id: i64) {
    if let Ok(mut lock) = MOUSE_HANDLER.lock() {
        *lock = handler_id;
    }
}

#[no_mangle]
pub extern "C" fn m2_app_set_hair_color_enable(enable: i32) {
    if let Ok(mut lock) = HAIR_COLOR_ENABLE.lock() {
        *lock = enable != 0;
    }
}

#[no_mangle]
pub extern "C" fn m2_app_set_armor_specular_enable(enable: i32) {
    if let Ok(mut lock) = USE_ARMOR_SPECULAR.lock() {
        *lock = enable != 0;
    }
}

#[no_mangle]
pub extern "C" fn m2_app_set_weapon_specular_enable(enable: i32) {
    if let Ok(mut lock) = USE_WEAPON_SPECULAR.lock() {
        *lock = enable != 0;
    }
}

#[no_mangle]
pub extern "C" fn m2_app_set_camera_max_distance(max_dist: f32) {
    // Modify PyBridgeApp state through py_bridge_app if needed,
    // or just set it via m2_app_set_camera max distance
    // We can use the existing m2_app_zoom_camera or something,
    // but the task asks to just provide the function. 
    // In py_bridge_app.rs, there is SystemConfig which has camera_max_distance.
    if let Ok(mut app) = crate::py_bridge_app::app_lock() {
        app.config.camera_max_distance = max_dist;
    }
}

#[no_mangle]
pub extern "C" fn m2_app_set_default_font_name(font_name: *const c_char) {
    if font_name.is_null() { return; }
    let s = unsafe { CStr::from_ptr(font_name) }.to_string_lossy().into_owned();
    if let Ok(mut lock) = DEFAULT_FONT_NAME.lock() {
        *lock = s;
    }
}

#[no_mangle]
pub extern "C" fn m2_app_set_guild_mark_path(path: *const c_char) {
    if path.is_null() { return; }
    let s = unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned();
    // C++ logic: strip .tga
    let new_path = if let Some(idx) = s.find(".tga") {
        s[..idx].to_string()
    } else {
        s.clone()
    };
    if let Ok(mut lock) = GUILD_MARK_PATH.lock() {
        *lock = new_path;
    }
}

#[no_mangle]
pub extern "C" fn m2_app_set_guild_symbol_path(path: *const c_char) {
    if path.is_null() { return; }
    let s = unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned();
    if let Ok(mut lock) = GUILD_SYMBOL_PATH.lock() {
        *lock = s;
    }
}

#[no_mangle]
pub extern "C" fn m2_app_get_locale_name() -> *const c_char {
    // Static return since it doesn't have args to free
    b"en\\0".as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn m2_app_get_locale_path() -> *const c_char {
    b"locale/en\\0".as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn m2_app_get_locale_path_common() -> *const c_char {
    b"locale/common\\0".as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn m2_app_is_rtl() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn m2_app_is_exist_file(file_name: *const c_char) -> i32 {
    if file_name.is_null() { return 0; }
    let s = unsafe { CStr::from_ptr(file_name) }.to_string_lossy();
    if Path::new(s.as_ref()).exists() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn m2_app_is_file_exist(file_name: *const c_char) -> i32 {
    m2_app_is_exist_file(file_name)
}

#[no_mangle]
pub extern "C" fn m2_app_set_center_position(x: f32, y: f32, z: f32) {
    if let Ok(mut lock) = CENTER_POSITION.lock() {
        *lock = (x, -y, z); // note: C++ uses -y
    }
}

#[no_mangle]
pub extern "C" fn m2_app_get_image_info_width(file_name: *const c_char) -> i32 {
    if file_name.is_null() { return 0; }
    let s = unsafe { CStr::from_ptr(file_name) }.to_string_lossy();
    if let Ok(dim) = image::image_dimensions(s.as_ref()) {
        dim.0 as i32
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_app_get_image_info_height(file_name: *const c_char) -> i32 {
    if file_name.is_null() { return 0; }
    let s = unsafe { CStr::from_ptr(file_name) }.to_string_lossy();
    if let Ok(dim) = image::image_dimensions(s.as_ref()) {
        dim.1 as i32
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_app_get_image_info_valid(file_name: *const c_char) -> i32 {
    if file_name.is_null() { return 0; }
    let s = unsafe { CStr::from_ptr(file_name) }.to_string_lossy();
    if image::image_dimensions(s.as_ref()).is_ok() { 1 } else { 0 }
}

"""

with open("crates/m2-client/src/py_master_runtime_wire_final.rs", "w") as f:
    f.write(rust_code)
