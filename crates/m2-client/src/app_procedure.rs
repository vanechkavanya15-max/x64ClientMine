//! Modul `app_procedure` — Procedura komunikatow okna i dyspozytor zdarzen
//! Wzorzec referencyjny C++: `source/UserInterface/PythonApplicationProcedure.cpp`
//! Opis: Odwzorowanie CPythonApplication::WindowProcedure: obsluga komunikatow klawiatury (WM_KEYDOWN, WM_KEYUP, WM_CHAR) oraz myszy (WM_LBUTTONDOWN, WM_RBUTTONDOWN, WM_MOUSEMOVE, WM_MOUSEWHEEL).

pub const WM_ACTIVATEAPP: u32 = 0x001C;
pub const WM_INPUTLANGCHANGE: u32 = 0x0051;
pub const WM_IME_STARTCOMPOSITION: u32 = 0x010D;
pub const WM_IME_COMPOSITION: u32 = 0x010F;
pub const WM_IME_ENDCOMPOSITION: u32 = 0x010E;
pub const WM_IME_NOTIFY: u32 = 0x0282;
pub const WM_IME_SETCONTEXT: u32 = 0x0281;
pub const WM_CHAR: u32 = 0x0102;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_LBUTTONDOWN: u32 = 0x0201;
pub const WM_LBUTTONUP: u32 = 0x0202;
pub const WM_MBUTTONDOWN: u32 = 0x0207;
pub const WM_MBUTTONUP: u32 = 0x0208;
pub const WM_RBUTTONDOWN: u32 = 0x0204;
pub const WM_RBUTTONUP: u32 = 0x0205;
pub const WM_MOUSEWHEEL: u32 = 0x020A;
pub const WM_SIZE: u32 = 0x0005;
pub const WM_EXITSIZEMOVE: u32 = 0x0232;
pub const WM_SYSKEYDOWN: u32 = 0x0104;
pub const WM_SYSKEYUP: u32 = 0x0105;
pub const WM_SETCURSOR: u32 = 0x0020;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;

pub const WA_ACTIVE: usize = 1;
pub const WA_CLICKACTIVE: usize = 2;

pub const SIZE_RESTORED: usize = 0;
pub const SIZE_MINIMIZED: usize = 1;
pub const SIZE_MAXIMIZED: usize = 2;

pub const VK_F10: usize = 0x79;

pub const CURSOR_MODE_HARDWARE: i32 = 1;

pub const DOUBLE_CLICK_TIME: u32 = 300;
pub const DOUBLE_CLICK_BOX: i16 = 5;

pub const ISC_SHOWUICOMPOSITIONWINDOW: isize = 0x80000000u32 as isize;
pub const ISC_SHOWUIALLCANDIDATEWINDOW: isize = 0x0000000f;

#[derive(Debug, Clone, Default)]
pub struct AppProcedure {
    pub initialized: bool,
    pub mouse_capture_ref: i32,
    pub x_down_position: i16,
    pub y_down_position: i16,
    pub l_button_down_time: u32,
    pub l_button_up_time: u32,
    pub is_activate_wnd: bool,
    pub is_window_full_screen_enable: bool,
    pub is_minimized_wnd: bool,
    pub dw_width: u32,
    pub dw_height: u32,
    pub bpp: u32,
    pub cursor_visible: bool,
    pub cursor_mode: i32,
    pub current_cursor: usize,
    pub is_active: bool,
    pub current_time_ms: u32,
    pub captured_hwnd: usize,
    pub current_hwnd: usize,
    
    // Lista zdarzen (do testow)
    pub events: Vec<String>,
}

impl AppProcedure {
    pub fn new() -> Self {
        Self {
            initialized: true,
            cursor_mode: CURSOR_MODE_HARDWARE,
            cursor_visible: true,
            is_active: true,
            ..Default::default()
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    fn loword(lparam: isize) -> i16 {
        (lparam & 0xFFFF) as i16
    }

    fn hiword(lparam: isize) -> i16 {
        ((lparam >> 16) & 0xFFFF) as i16
    }
    
    fn loword_w(wparam: usize) -> u16 {
        (wparam & 0xFFFF) as u16
    }

    fn hiword_w(wparam: usize) -> u16 {
        ((wparam >> 16) & 0xFFFF) as u16
    }

    pub fn eltimer_get_msec(&self) -> u32 {
        self.current_time_ms
    }

    pub fn safe_set_capture(&mut self, hwnd: usize) {
        self.events.push(format!("SetCapture({})", hwnd));
        self.mouse_capture_ref += 1;
        self.captured_hwnd = hwnd;
    }

    pub fn safe_release_capture(&mut self) {
        self.mouse_capture_ref -= 1;
        if self.mouse_capture_ref == 0 {
            self.events.push("ReleaseCapture".to_string());
            self.captured_hwnd = 0;
        }
    }

    fn set_full_screen_window(&mut self, hwnd: usize, width: u32, height: u32, bpp: u32) {
        self.events.push(format!("__SetFullScreenWindow({}, {}, {}, {})", hwnd, width, height, bpp));
    }

    fn minimize_full_screen_window(&mut self, hwnd: usize, width: u32, height: u32) {
        self.events.push(format!("__MinimizeFullScreenWindow({}, {}, {})", hwnd, width, height));
    }

    pub fn window_procedure(&mut self, hwnd: usize, ui_msg: u32, wparam: usize, mut lparam: isize) -> isize {
        self.current_hwnd = hwnd;

        let result = match ui_msg {
            WM_ACTIVATEAPP => {
                self.is_activate_wnd = wparam == WA_ACTIVE || wparam == WA_CLICKACTIVE;

                if self.is_activate_wnd {
                    self.events.push("m_SoundManager.RestoreVolume".to_string());
                    if self.is_window_full_screen_enable {
                        self.set_full_screen_window(hwnd, self.dw_width, self.dw_height, self.bpp);
                    }
                } else {
                    self.events.push("m_SoundManager.SaveVolume".to_string());
                    if self.is_window_full_screen_enable {
                        self.minimize_full_screen_window(hwnd, self.dw_width, self.dw_height);
                    }
                }
                None
            }
            WM_INPUTLANGCHANGE => {
                self.events.push("CPythonIME::WMInputLanguage".to_string());
                Some(1)
            }
            WM_IME_STARTCOMPOSITION => {
                self.events.push("CPythonIME::WMStartComposition".to_string());
                Some(1)
            }
            WM_IME_COMPOSITION => {
                self.events.push("CPythonIME::WMComposition".to_string());
                Some(1)
            }
            WM_IME_ENDCOMPOSITION => {
                self.events.push("CPythonIME::WMEndComposition".to_string());
                Some(1)
            }
            WM_IME_NOTIFY => {
                self.events.push("CPythonIME::WMNotify".to_string());
                Some(1)
            }
            WM_IME_SETCONTEXT => {
                lparam &= !(ISC_SHOWUICOMPOSITIONWINDOW | ISC_SHOWUIALLCANDIDATEWINDOW);
                self.events.push(format!("WM_IME_SETCONTEXT(lParam={})", lparam));
                None
            }
            WM_CHAR => {
                self.events.push(format!("CPythonIME::WMChar({})", wparam));
                Some(1)
            }
            WM_KEYDOWN => {
                self.events.push(format!("OnIMEKeyDown({})", Self::loword_w(wparam)));
                None
            }
            WM_LBUTTONDOWN => {
                self.safe_set_capture(hwnd);

                let x = Self::loword(lparam);
                let y = Self::hiword(lparam);
                let current_time = self.eltimer_get_msec();

                if current_time >= self.l_button_down_time && current_time - self.l_button_down_time < DOUBLE_CLICK_TIME &&
                    (x - self.x_down_position).abs() < DOUBLE_CLICK_BOX &&
                    (y - self.y_down_position).abs() < DOUBLE_CLICK_BOX
                {
                    self.l_button_down_time = 0;
                    self.events.push(format!("OnMouseLeftButtonDoubleClick({}, {})", x, y));
                } else {
                    self.l_button_down_time = current_time;
                    self.events.push(format!("OnMouseLeftButtonDown({}, {})", x, y));
                }

                self.x_down_position = x;
                self.y_down_position = y;
                Some(0)
            }
            WM_LBUTTONUP => {
                self.l_button_up_time = self.eltimer_get_msec();
                if hwnd == self.captured_hwnd {
                    self.safe_release_capture();
                    self.events.push(format!("OnMouseLeftButtonUp({}, {})", Self::loword(lparam), Self::hiword(lparam)));
                }
                Some(0)
            }
            WM_MBUTTONDOWN => {
                self.safe_set_capture(hwnd);
                self.events.push(format!("RunMouseMiddleButtonDown({}, {})", Self::loword(lparam), Self::hiword(lparam)));
                None
            }
            WM_MBUTTONUP => {
                if self.captured_hwnd == hwnd {
                    self.safe_release_capture();
                    self.events.push(format!("RunMouseMiddleButtonUp({}, {})", Self::loword(lparam), Self::hiword(lparam)));
                }
                None
            }
            WM_RBUTTONDOWN => {
                self.safe_set_capture(hwnd);
                self.events.push(format!("OnMouseRightButtonDown({}, {})", Self::loword(lparam), Self::hiword(lparam)));
                Some(0)
            }
            WM_RBUTTONUP => {
                if hwnd == self.captured_hwnd {
                    self.safe_release_capture();
                    self.events.push(format!("OnMouseRightButtonUp({}, {})", Self::loword(lparam), Self::hiword(lparam)));
                }
                Some(0)
            }
            WM_MOUSEWHEEL => {
                self.events.push(format!("OnMouseWheel({})", Self::hiword_w(wparam) as i16));
                None
            }
            WM_SIZE => {
                match wparam {
                    SIZE_RESTORED | SIZE_MAXIMIZED => {
                        self.events.push("ResizeBackBuffer".to_string());
                    }
                    _ => {}
                }

                if wparam == SIZE_MINIMIZED {
                    self.is_minimized_wnd = true;
                } else {
                    self.is_minimized_wnd = false;
                }

                self.events.push(format!("OnSizeChange({}, {})", Self::loword(lparam), Self::hiword(lparam)));
                None
            }
            WM_EXITSIZEMOVE => {
                self.events.push("ResizeBackBuffer".to_string());
                self.events.push(format!("OnSizeChange({}, {})", Self::loword(lparam), Self::hiword(lparam)));
                None
            }
            WM_SYSKEYDOWN => {
                if Self::loword_w(wparam) == VK_F10 as u16 {
                    self.events.push("VK_F10_DOWN".to_string());
                }
                None
            }
            WM_SYSKEYUP => {
                match Self::loword_w(wparam) {
                    18 => return 0,
                    vk if vk == VK_F10 as u16 => {
                        self.events.push("VK_F10_UP".to_string());
                    }
                    _ => {}
                }
                None
            }
            WM_SETCURSOR => {
                if self.is_active {
                    if self.cursor_visible && CURSOR_MODE_HARDWARE == self.cursor_mode {
                        self.events.push(format!("SetCursor({})", self.current_cursor));
                        return 0;
                    } else {
                        self.events.push("SetCursor(NULL)".to_string());
                        return 0;
                    }
                }
                None
            }
            WM_CLOSE => {
                self.events.push("RunPressExitKey".to_string());
                return 0;
            }
            WM_DESTROY => {
                return 0;
            }
            _ => {
                None
            }
        };

        if let Some(ret) = result {
            return ret;
        }

        self.events.push(format!("CMSApplication::WindowProcedure({}, {}, {}, {})", hwnd, ui_msg, wparam, lparam));
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_app_procedure() {
        let instance = AppProcedure::new();
        assert!(instance.is_ready());
    }
}
