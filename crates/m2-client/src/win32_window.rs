use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(windows))]
use std::collections::HashMap;
#[cfg(not(windows))]
use std::sync::Mutex;
#[cfg(not(windows))]
use std::sync::OnceLock;

pub type WindowHandle = *mut c_void;

pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_POPUP: u32 = 0x80000000;
pub const WS_VISIBLE: u32 = 0x10000000;

pub const GWL_STYLE: i32 = -16;

pub const SWP_NOMOVE: u32 = 0x0002;
pub const SWP_NOSIZE: u32 = 0x0001;
pub const SWP_NOZORDER: u32 = 0x0004;
pub const SWP_FRAMECHANGED: u32 = 0x0020;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: *const c_char,
    pub width: i32,
    pub height: i32,
    pub is_fullscreen: bool,
    pub is_borderless: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: ptr::null(),
            width: 800,
            height: 600,
            is_fullscreen: false,
            is_borderless: false,
        }
    }
}

pub type WndProc = extern "system" fn(WindowHandle, u32, usize, isize) -> isize;

#[repr(C)]
pub struct WndClassA {
    pub style: u32,
    pub lpfn_wnd_proc: WndProc,
    pub cb_cls_extra: i32,
    pub cb_wnd_extra: i32,
    pub h_instance: *mut c_void,
    pub h_icon: *mut c_void,
    pub h_cursor: *mut c_void,
    pub hbr_background: *mut c_void,
    pub lpsz_menu_name: *const c_char,
    pub lpsz_class_name: *const c_char,
}

#[cfg(not(windows))]
fn mock_windows() -> &'static Mutex<HashMap<usize, MockWindow>> {
    static MOCK_WINDOWS: OnceLock<Mutex<HashMap<usize, MockWindow>>> = OnceLock::new();
    MOCK_WINDOWS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(not(windows))]
static NEXT_HWND: AtomicUsize = AtomicUsize::new(1);

#[cfg(not(windows))]
#[derive(Debug, Clone)]
struct MockWindow {
    hwnd: usize,
    title: String,
    style: u32,
    rect: Rect,
}

#[cfg(not(windows))]
mod sys {
    use super::*;

    pub unsafe fn AdjustWindowRect(rect: *mut Rect, style: u32, _menu: bool) -> i32 {
        if rect.is_null() {
            return 0;
        }
        let r = &mut *rect;
        
        // C++ Metin2 AdjustWindowRect logic typically expands by frame sizes
        if (style & WS_OVERLAPPEDWINDOW) == WS_OVERLAPPEDWINDOW {
            r.left -= 8;
            r.top -= 31;
            r.right += 8;
            r.bottom += 8;
        }
        1
    }

    pub unsafe fn RegisterClassA(_class: *const WndClassA) -> u16 {
        1 // Mock success
    }

    pub unsafe fn DefWindowProcA(
        _hwnd: WindowHandle,
        _msg: u32,
        _w_param: usize,
        _l_param: isize,
    ) -> isize {
        1 // Return 1 to simulate successful handling, especially for WM_NCCREATE
    }

    pub unsafe fn CreateWindowExA(
        _ex_style: u32,
        _class_name: *const c_char,
        window_name: *const c_char,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        _parent: WindowHandle,
        _menu: WindowHandle,
        _instance: WindowHandle,
        _param: *mut c_void,
    ) -> WindowHandle {
        let hwnd = NEXT_HWND.fetch_add(1, Ordering::SeqCst);
        let title = if !window_name.is_null() {
            CStr::from_ptr(window_name).to_string_lossy().into_owned()
        } else {
            String::new()
        };

        let mock_win = MockWindow {
            hwnd,
            title,
            style,
            rect: Rect { left: x, top: y, right: x + width, bottom: y + height },
        };

        let mut windows = mock_windows().lock().unwrap();
        windows.insert(hwnd, mock_win);

        hwnd as WindowHandle
    }

    pub unsafe fn SetWindowTextA(hwnd: WindowHandle, string: *const c_char) -> i32 {
        let hwnd_val = hwnd as usize;
        let title = if !string.is_null() {
            CStr::from_ptr(string).to_string_lossy().into_owned()
        } else {
            String::new()
        };

        let mut windows = mock_windows().lock().unwrap();
        if let Some(win) = windows.get_mut(&hwnd_val) {
            win.title = title;
            1
        } else {
            0
        }
    }

    pub unsafe fn GetWindowLongA(hwnd: WindowHandle, index: i32) -> i32 {
        if index != GWL_STYLE {
            return 0;
        }
        let hwnd_val = hwnd as usize;
        let windows = mock_windows().lock().unwrap();
        if let Some(win) = windows.get(&hwnd_val) {
            win.style as i32
        } else {
            0
        }
    }

    pub unsafe fn SetWindowLongA(hwnd: WindowHandle, index: i32, new_long: i32) -> i32 {
        if index != GWL_STYLE {
            return 0;
        }
        let hwnd_val = hwnd as usize;
        let mut windows = mock_windows().lock().unwrap();
        if let Some(win) = windows.get_mut(&hwnd_val) {
            let old_style = win.style;
            win.style = new_long as u32;
            old_style as i32
        } else {
            0
        }
    }

    pub unsafe fn SetWindowPos(
        hwnd: WindowHandle,
        _hwnd_insert_after: WindowHandle,
        _x: i32,
        _y: i32,
        _cx: i32,
        _cy: i32,
        _flags: u32,
    ) -> i32 {
        let hwnd_val = hwnd as usize;
        let windows = mock_windows().lock().unwrap();
        if windows.contains_key(&hwnd_val) {
            1
        } else {
            0
        }
    }

    pub unsafe fn ShowWindow(_hwnd: WindowHandle, _n_cmd_show: i32) -> i32 {
        1
    }

    pub unsafe fn UpdateWindow(_hwnd: WindowHandle) -> i32 {
        1
    }

    pub unsafe fn DestroyWindow(_hwnd: WindowHandle) -> i32 {
        1
    }

    pub unsafe fn PostQuitMessage(_n_exit_code: i32) {}

    pub unsafe fn GetLastError() -> u32 {
        0
    }

    pub unsafe fn GetModuleHandleA(_lp_module_name: *const c_char) -> *mut c_void {
        ptr::null_mut()
    }

    pub unsafe fn GetStockObject(_i: i32) -> *mut c_void {
        ptr::null_mut()
    }
}

#[cfg(windows)]
mod sys {
    use super::*;

    #[link(name = "user32")]
    extern "system" {
        pub fn AdjustWindowRect(lpRect: *mut Rect, dwStyle: u32, bMenu: i32) -> i32;
        pub fn RegisterClassA(lpWndClass: *const WndClassA) -> u16;
        pub fn CreateWindowExA(
            dwExStyle: u32,
            lpClassName: *const c_char,
            lpWindowName: *const c_char,
            dwStyle: u32,
            X: i32,
            Y: i32,
            nWidth: i32,
            nHeight: i32,
            hWndParent: WindowHandle,
            hMenu: WindowHandle,
            hInstance: WindowHandle,
            lpParam: *mut c_void,
        ) -> WindowHandle;
        pub fn ShowWindow(hWnd: WindowHandle, nCmdShow: i32) -> i32;
        pub fn UpdateWindow(hWnd: WindowHandle) -> i32;
        pub fn DestroyWindow(hWnd: WindowHandle) -> i32;
        pub fn PostQuitMessage(nExitCode: i32);
        pub fn SetWindowTextA(hWnd: WindowHandle, lpString: *const c_char) -> i32;
        pub fn GetWindowLongA(hWnd: WindowHandle, nIndex: i32) -> i32;
        pub fn SetWindowLongA(hWnd: WindowHandle, nIndex: i32, dwNewLong: i32) -> i32;
        pub fn SetWindowPos(
            hWnd: WindowHandle,
            hWndInsertAfter: WindowHandle,
            X: i32,
            Y: i32,
            cx: i32,
            cy: i32,
            uFlags: u32,
        ) -> i32;
        pub fn DefWindowProcA(
            hWnd: WindowHandle,
            Msg: u32,
            wParam: usize,
            lParam: isize,
        ) -> isize;
    }

    #[link(name = "gdi32")]
    extern "system" {
        pub fn GetStockObject(i: i32) -> *mut c_void;
    }

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetModuleHandleA(lpModuleName: *const c_char) -> *mut c_void;
        pub fn GetLastError() -> u32;
    }
}

extern "system" fn def_window_proc(hwnd: WindowHandle, msg: u32, w_param: usize, l_param: isize) -> isize {
    match msg {
        0x0010 => { // WM_CLOSE
            unsafe {
                sys::DestroyWindow(hwnd);
            }
            crate::win32_message_dispatcher::post_quit_message(0);
            0
        }
        0x0002 => { // WM_DESTROY
            unsafe {
                sys::PostQuitMessage(0);
            }
            crate::win32_message_dispatcher::post_quit_message(0);
            0
        }
        0x000F => { // WM_PAINT
            #[cfg(windows)]
            unsafe {
                #[repr(C)]
                struct PAINTSTRUCT {
                    hdc: usize,
                    f_erase: i32,
                    rc_paint: Rect,
                    f_restore: i32,
                    f_inc_update: i32,
                    rgb_reserved: [u8; 32],
                }
                extern "system" {
                    fn BeginPaint(hwnd: WindowHandle, lpPaint: *mut PAINTSTRUCT) -> usize;
                    fn EndPaint(hwnd: WindowHandle, lpPaint: *const PAINTSTRUCT) -> i32;
                }
                let mut ps = std::mem::zeroed();
                BeginPaint(hwnd, &mut ps);
                EndPaint(hwnd, &ps);
            }
            0
        }
        _ => unsafe {
            sys::DefWindowProcA(hwnd, msg, w_param, l_param)
        }
    }
}

pub fn create_game_window(config: &WindowConfig) -> Result<WindowHandle, String> {
    unsafe {
        let class_name = CString::new("Metin2WindowClass").unwrap();
        
        let h_instance = sys::GetModuleHandleA(ptr::null());

        let wnd_class = WndClassA {
            style: 0x0003, // CS_HREDRAW | CS_VREDRAW
            lpfn_wnd_proc: def_window_proc,
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance,
            h_icon: ptr::null_mut(),
            h_cursor: ptr::null_mut(),
            hbr_background: sys::GetStockObject(4), // BLACK_BRUSH (4)
            lpsz_menu_name: ptr::null(),
            lpsz_class_name: class_name.as_ptr(),
        };

        sys::RegisterClassA(&wnd_class);

        let mut style = WS_OVERLAPPEDWINDOW | WS_VISIBLE;
        if config.is_fullscreen || config.is_borderless {
            style = WS_POPUP | WS_VISIBLE;
        }

        let mut rect = Rect {
            left: 0,
            top: 0,
            right: config.width,
            bottom: config.height,
        };

        if !config.is_fullscreen && !config.is_borderless {
            #[cfg(windows)]
            sys::AdjustWindowRect(&mut rect, style, 0);
            
            #[cfg(not(windows))]
            sys::AdjustWindowRect(&mut rect, style, false);
        }

        let actual_width = rect.right - rect.left;
        let actual_height = rect.bottom - rect.top;

        let hwnd = sys::CreateWindowExA(
            0,
            class_name.as_ptr(),
            config.title,
            style,
            0,
            0,
            actual_width,
            actual_height,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut(),
        );

        if hwnd.is_null() {
            let err = sys::GetLastError();
            Err(format!("Failed to create window (GetLastError={})", err))
        } else {
            sys::ShowWindow(hwnd, 1); // SW_SHOWNORMAL
            sys::UpdateWindow(hwnd);
            Ok(hwnd)
        }
    }
}

pub fn set_window_fullscreen(hwnd: WindowHandle, fullscreen: bool) {
    if hwnd.is_null() {
        return;
    }

    unsafe {
        let current_style = sys::GetWindowLongA(hwnd, GWL_STYLE) as u32;
        let mut new_style = current_style;

        if fullscreen {
            new_style &= !WS_OVERLAPPEDWINDOW;
            new_style |= WS_POPUP | WS_VISIBLE;
        } else {
            new_style &= !(WS_POPUP | WS_VISIBLE);
            new_style |= WS_OVERLAPPEDWINDOW;
        }

        if new_style != current_style {
            sys::SetWindowLongA(hwnd, GWL_STYLE, new_style as i32);
            sys::SetWindowPos(
                hwnd,
                ptr::null_mut(),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
            );
        }
    }
}

pub fn set_window_title(hwnd: WindowHandle, title: &str) {
    if hwnd.is_null() {
        return;
    }

    if let Ok(c_title) = CString::new(title) {
        unsafe {
            sys::SetWindowTextA(hwnd, c_title.as_ptr());
        }
    }
}

#[cfg(not(windows))]
pub fn _get_mock_window_title(hwnd: WindowHandle) -> Option<String> {
    let hwnd_val = hwnd as usize;
    let windows = mock_windows().lock().unwrap();
    windows.get(&hwnd_val).map(|w| w.title.clone())
}

#[cfg(not(windows))]
pub fn _get_mock_window_style(hwnd: WindowHandle) -> Option<u32> {
    let hwnd_val = hwnd as usize;
    let windows = mock_windows().lock().unwrap();
    windows.get(&hwnd_val).map(|w| w.style)
}
