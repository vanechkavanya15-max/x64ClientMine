use lazy_static::lazy_static;
use std::sync::Mutex;

#[cfg(windows)]
#[link(name = "user32")]
extern "system" {
    fn ShowCursor(bShow: i32) -> i32;
    fn SetCursor(hCursor: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    Normal = 0,
    Attack,
    Target,
    Talk,
    CantGo,
    Pick,
    Door,
    Chair,
    Magic,
    Buy,
    Sell,
    CameraRotate,
    HSize,
    VSize,
    HVSize,
    Count,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorMode {
    Hardware = 0,
    Software = 1,
}

pub struct AppCursorState {
    pub cursor_visible: bool,
    pub liar_cursor_on: bool,
    pub cursor_mode: CursorMode,
    pub cursor_num: i32,
    pub continuous_cursor_num: i32,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

impl Default for AppCursorState {
    fn default() -> Self {
        Self {
            cursor_visible: true,
            liar_cursor_on: false,
            cursor_mode: CursorMode::Hardware,
            cursor_num: CursorShape::Normal as i32,
            continuous_cursor_num: CursorShape::Normal as i32,
            mouse_x: 0,
            mouse_y: 0,
        }
    }
}

impl AppCursorState {
    fn is_continuous_change_type_cursor(cursor_num: i32) -> bool {
        match cursor_num {
            x if x == CursorShape::Normal as i32 => true,
            x if x == CursorShape::Attack as i32 => true,
            x if x == CursorShape::Target as i32 => true,
            x if x == CursorShape::Magic as i32 => true,
            x if x == CursorShape::Buy as i32 => true,
            x if x == CursorShape::Sell as i32 => true,
            _ => false,
        }
    }

    pub fn set_cursor_num(&mut self, mut cursor_num: i32) -> bool {
        if cursor_num == CursorShape::Normal as i32 {
            if !Self::is_continuous_change_type_cursor(self.cursor_num) {
                cursor_num = self.continuous_cursor_num;
            }
        } else {
            if Self::is_continuous_change_type_cursor(self.cursor_num) {
                self.continuous_cursor_num = self.cursor_num;
            }
        }

        // Simulating the hardware handle lookup failure if not found.
        // We know shapes up to Count - 1 exist.
        if cursor_num < 0 || cursor_num >= CursorShape::Count as i32 {
            return false;
        }

        if self.cursor_mode == CursorMode::Hardware {
            // Note: Since this is an FFI boundary that will link with CWindowManager/Win32CursorManager later,
            // we simulate setting the handle if we were on Windows.
            // In a full environment with handles, we'd map cursor_num -> HCURSOR here.
            // For now, this struct just maintains the state.
        }

        self.cursor_num = cursor_num;
        true
    }

    pub fn set_cursor_visible(&mut self, flag: bool, liar_cursor_on: bool) {
        self.cursor_visible = flag;
        self.liar_cursor_on = liar_cursor_on;
        
        if self.cursor_mode == CursorMode::Hardware {
            if !self.cursor_visible {
                let mut show_num;
                loop {
                    #[cfg(windows)]
                    unsafe { show_num = ShowCursor(0); }
                    #[cfg(not(windows))]
                    { show_num = -1; }
                    
                    if show_num < 0 { break; }
                }
            } else {
                let mut show_num;
                loop {
                    #[cfg(windows)]
                    unsafe { show_num = ShowCursor(1); }
                    #[cfg(not(windows))]
                    { show_num = 0; }
                    
                    if show_num >= 0 { break; }
                }
            }
        }
    }

    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        self.cursor_mode = mode;
        match mode {
            CursorMode::Hardware => {
                #[cfg(windows)]
                unsafe { ShowCursor(1); }
            }
            CursorMode::Software => {
                #[cfg(windows)]
                unsafe { 
                    SetCursor(std::ptr::null_mut()); 
                    ShowCursor(0);
                }
            }
        }
    }

    pub fn get_cursor_position(&self, lx: *mut i32, ly: *mut i32) {
        unsafe {
            if !lx.is_null() {
                *lx = self.mouse_x;
            }
            if !ly.is_null() {
                *ly = self.mouse_y;
            }
        }
    }
}

lazy_static! {
    pub static ref PY_APP_CURSOR: Mutex<AppCursorState> = Mutex::new(AppCursorState::default());
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_cursor(cursor_num: i32) -> bool {
    if let Ok(mut state) = PY_APP_CURSOR.lock() {
        state.set_cursor_num(cursor_num)
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_show_cursor() {
    if let Ok(mut state) = PY_APP_CURSOR.lock() {
        let liar = state.liar_cursor_on;
        state.set_cursor_visible(true, liar);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_hide_cursor() {
    if let Ok(mut state) = PY_APP_CURSOR.lock() {
        let liar = state.liar_cursor_on;
        state.set_cursor_visible(false, liar);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_get_cursor_position(lx: *mut i32, ly: *mut i32) {
    if let Ok(state) = PY_APP_CURSOR.lock() {
        state.get_cursor_position(lx, ly);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_hardware_cursor() {
    if let Ok(mut state) = PY_APP_CURSOR.lock() {
        state.set_cursor_mode(CursorMode::Hardware);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_app_set_software_cursor() {
    if let Ok(mut state) = PY_APP_CURSOR.lock() {
        state.set_cursor_mode(CursorMode::Software);
    }
}
