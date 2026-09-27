use std::sync::Mutex;
use lazy_static::lazy_static;

pub const DIK_ESCAPE: u32 = 0x01;
pub const DIK_1: u32 = 0x02;
pub const DIK_2: u32 = 0x03;
pub const DIK_3: u32 = 0x04;
pub const DIK_4: u32 = 0x05;
pub const DIK_5: u32 = 0x06;
pub const DIK_6: u32 = 0x07;
pub const DIK_7: u32 = 0x08;
pub const DIK_8: u32 = 0x09;
pub const DIK_9: u32 = 0x0A;
pub const DIK_0: u32 = 0x0B;
pub const DIK_MINUS: u32 = 0x0C;
pub const DIK_EQUALS: u32 = 0x0D;
pub const DIK_BACK: u32 = 0x0E;
pub const DIK_TAB: u32 = 0x0F;
pub const DIK_Q: u32 = 0x10;
pub const DIK_W: u32 = 0x11;
pub const DIK_E: u32 = 0x12;
pub const DIK_R: u32 = 0x13;
pub const DIK_T: u32 = 0x14;
pub const DIK_Y: u32 = 0x15;
pub const DIK_U: u32 = 0x16;
pub const DIK_I: u32 = 0x17;
pub const DIK_O: u32 = 0x18;
pub const DIK_P: u32 = 0x19;
pub const DIK_LBRACKET: u32 = 0x1A;
pub const DIK_RBRACKET: u32 = 0x1B;
pub const DIK_RETURN: u32 = 0x1C;
pub const DIK_LCONTROL: u32 = 0x1D;
pub const DIK_A: u32 = 0x1E;
pub const DIK_S: u32 = 0x1F;
pub const DIK_D: u32 = 0x20;
pub const DIK_F: u32 = 0x21;
pub const DIK_G: u32 = 0x22;
pub const DIK_H: u32 = 0x23;
pub const DIK_J: u32 = 0x24;
pub const DIK_K: u32 = 0x25;
pub const DIK_L: u32 = 0x26;
pub const DIK_SEMICOLON: u32 = 0x27;
pub const DIK_APOSTROPHE: u32 = 0x28;
pub const DIK_GRAVE: u32 = 0x29;
pub const DIK_LSHIFT: u32 = 0x2A;
pub const DIK_BACKSLASH: u32 = 0x2B;
pub const DIK_Z: u32 = 0x2C;
pub const DIK_X: u32 = 0x2D;
pub const DIK_C: u32 = 0x2E;
pub const DIK_V: u32 = 0x2F;
pub const DIK_B: u32 = 0x30;
pub const DIK_N: u32 = 0x31;
pub const DIK_M: u32 = 0x32;
pub const DIK_COMMA: u32 = 0x33;
pub const DIK_PERIOD: u32 = 0x34;
pub const DIK_SLASH: u32 = 0x35;
pub const DIK_RSHIFT: u32 = 0x36;
pub const DIK_MULTIPLY: u32 = 0x37;
pub const DIK_LALT: u32 = 0x38;
pub const DIK_SPACE: u32 = 0x39;

pub const INFO_ITEM: u32 = 1;
pub const INFO_ACTOR: u32 = 2;
pub const INFO_EFFECT: u32 = 3;
pub const INFO_TEXTTAIL: u32 = 4;

pub const CAMERA_MAX_DISTANCE: f32 = 2500.0;

#[derive(Debug, Clone, PartialEq)]
pub enum AppBridgeError {
    MutexLockError,
    InvalidKeyCode(u32),
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct SystemConfig {
    pub width: u32,
    pub height: u32,
    pub is_windowed: bool,
    pub camera_max_distance: f32,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            is_windowed: true,
            camera_max_distance: CAMERA_MAX_DISTANCE,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct AppTimingClock {
    pub current_time: f32,
    pub global_time: u32,
}

impl Default for AppTimingClock {
    fn default() -> Self {
        Self {
            current_time: 0.0,
            global_time: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CameraState {
    pub distance: f32,
    pub pitch: f32,
    pub rotation: f32,
    pub target_height: f32,
    pub is_special_mode: bool,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            distance: 1000.0,
            pitch: 30.0,
            rotation: 0.0,
            target_height: 100.0,
            is_special_mode: false,
        }
    }
}

#[derive(Debug)]
pub struct PyBridgeApp {
    pub config: SystemConfig,
    pub clock: AppTimingClock,
    pub camera: CameraState,
    pub keys_pressed: [bool; 256],
    pub is_exit_requested: bool,
}

impl Default for PyBridgeApp {
    fn default() -> Self {
        Self {
            config: SystemConfig::default(),
            clock: AppTimingClock::default(),
            camera: CameraState::default(),
            keys_pressed: [false; 256],
            is_exit_requested: false,
        }
    }
}

impl PyBridgeApp {
    pub fn get_time(&self) -> f32 {
        self.clock.current_time
    }

    pub fn get_global_time(&self) -> u32 {
        self.clock.global_time
    }

    pub fn get_width(&self) -> u32 {
        self.config.width
    }

    pub fn get_height(&self) -> u32 {
        self.config.height
    }

    pub fn exit(&mut self) {
        self.is_exit_requested = true;
    }

    pub fn set_camera(&mut self, distance: f32, pitch: f32, rotation: f32, target_height: f32) {
        self.camera.distance = distance;
        self.camera.pitch = pitch;
        self.camera.rotation = rotation;
        self.camera.target_height = target_height;
    }

    pub fn is_pressed(&self, key_code: u32) -> Result<bool, AppBridgeError> {
        if key_code < 256 {
            Ok(self.keys_pressed[key_code as usize])
        } else {
            Err(AppBridgeError::InvalidKeyCode(key_code))
        }
    }

    pub fn enable_special_camera_mode(&mut self) {
        self.camera.is_special_mode = true;
    }

    // internal testing and state updates
    pub fn set_key_state(&mut self, key_code: u32, pressed: bool) -> Result<(), AppBridgeError> {
        if key_code < 256 {
            self.keys_pressed[key_code as usize] = pressed;
            Ok(())
        } else {
            Err(AppBridgeError::InvalidKeyCode(key_code))
        }
    }

    pub fn set_time(&mut self, current_time: f32, global_time: u32) {
        self.clock.current_time = current_time;
        self.clock.global_time = global_time;
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
    }
}

lazy_static! {
    pub static ref PY_APP: Mutex<PyBridgeApp> = Mutex::new(PyBridgeApp::default());
}

pub fn app_lock() -> Result<std::sync::MutexGuard<'static, PyBridgeApp>, AppBridgeError> {
    PY_APP.lock().map_err(|_| AppBridgeError::MutexLockError)
}

// C-FFI Exports

#[no_mangle]
pub extern "C" fn m2_app_get_time() -> f32 {
    app_lock().map(|app| app.get_time()).unwrap_or(0.0)
}

#[no_mangle]
pub extern "C" fn m2_app_get_global_time() -> u32 {
    app_lock().map(|app| app.get_global_time()).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn m2_app_get_width() -> u32 {
    app_lock().map(|app| app.get_width()).unwrap_or(800)
}

#[no_mangle]
pub extern "C" fn m2_app_get_height() -> u32 {
    app_lock().map(|app| app.get_height()).unwrap_or(600)
}

#[no_mangle]
pub extern "C" fn m2_app_exit() {
    if let Ok(mut app) = app_lock() {
        app.exit();
    }
}

#[no_mangle]
pub extern "C" fn m2_app_set_camera(distance: f32, pitch: f32, rotation: f32, target_height: f32) {
    if let Ok(mut app) = app_lock() {
        app.set_camera(distance, pitch, rotation, target_height);
    }
}

#[no_mangle]
pub extern "C" fn m2_app_is_pressed(key_code: u32) -> bool {
    app_lock()
        .and_then(|app| app.is_pressed(key_code))
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_app_enable_special_camera_mode() {
    if let Ok(mut app) = app_lock() {
        app.enable_special_camera_mode();
    }
}

// Setters for testability & game loop state updates

#[no_mangle]
pub extern "C" fn m2_app_set_time(current_time: f32, global_time: u32) {
    if let Ok(mut app) = app_lock() {
        app.set_time(current_time, global_time);
    }
}

#[no_mangle]
pub extern "C" fn m2_app_set_resolution(width: u32, height: u32) {
    if let Ok(mut app) = app_lock() {
        app.set_resolution(width, height);
    }
}

#[no_mangle]
pub extern "C" fn m2_app_set_key_state(key_code: u32, pressed: bool) {
    if let Ok(mut app) = app_lock() {
        let _ = app.set_key_state(key_code, pressed);
    }
}
