// py_bridge_wndmgr.rs - Complete Python bridge for wndMgr module in Metin2 Pure Rust
use crate::ui_window_core::WindowType;
use crate::ui_window_mgr::get_window_manager;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::py_bridge_app::{m2_app_set_resolution, m2_app_get_width, m2_app_get_height};
use crate::ui_font_renderer::get_global_font_atlas;

fn cstr_to_str<'a>(ptr: *const c_char) -> &'a str {
    if ptr.is_null() {
        return "";
    }
    unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") }
}

#[no_mangle]
pub extern "C" fn wndMgr_Register(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::Normal, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterImageBox(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::ImageBox, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterExpandedImageBox(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::ExpandedImageBox, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterTextLine(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::TextLine, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterButton(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::Button, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterBar(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::Bar, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterLine(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::Line, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterBox(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::Box, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterSlotWindow(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::SlotWindow, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterBar3D(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::Bar, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterGridSlotWindow(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::SlotWindow, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterMarkBox(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::ImageBox, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterAniImageBox(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::ExpandedImageBox, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterRadioButton(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::RadioButton, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterToggleButton(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::ToggleButton, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterDragButton(py_obj: usize) -> u32 {
    get_window_manager().register_with_handler(WindowType::Button, py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetParent(child: u32, parent: u32) -> bool {
    get_window_manager().set_parent(child, parent)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetPosition(handle: u32, x: i32, y: i32) -> bool {
    get_window_manager().set_position(handle, x, y)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetSize(handle: u32, w: i32, h: i32) -> bool {
    get_window_manager().set_size(handle, w, h)
}

#[no_mangle]
pub extern "C" fn wndMgr_Show(handle: u32) -> bool {
    get_window_manager().show(handle)
}

#[no_mangle]
pub extern "C" fn wndMgr_Hide(handle: u32) -> bool {
    get_window_manager().hide(handle)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetTop(handle: u32) -> bool {
    let mut wm = get_window_manager();
    if wm.windows.contains_key(&handle) {
        wm.top_window = Some(handle);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_IsShow(handle: u32) -> bool {
    get_window_manager().is_show(handle)
}

#[no_mangle]
pub extern "C" fn wndMgr_LoadImage(handle: u32, path: *const c_char) -> bool {
    let p = cstr_to_str(path);
    get_window_manager().load_image(handle, p)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetRenderingRect(handle: u32, l: f32, t: f32, r: f32, b: f32) -> bool {
    get_window_manager().set_rendering_rect(handle, l, t, r, b)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetScale(handle: u32, sx: f32, sy: f32) -> bool {
    get_window_manager().set_scale(handle, sx, sy)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetText(handle: u32, text: *const c_char) -> bool {
    let t = cstr_to_str(text);
    get_window_manager().set_text(handle, t)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetColor(handle: u32, r: f32, g: f32, b: f32, a: f32) -> bool {
    get_window_manager().set_color(handle, [r, g, b, a])
}

#[no_mangle]
pub extern "C" fn wndMgr_SetFontColor(handle: u32, r: f32, g: f32, b: f32, a: f32) -> bool {
    get_window_manager().set_font_color(handle, [r, g, b, a])
}

#[no_mangle]
pub extern "C" fn wndMgr_SetAlign(handle: u32, h_align: u8, v_align: u8) -> bool {
    get_window_manager().set_align(handle, h_align, v_align)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetUpVisual(handle: u32, path: *const c_char) -> bool {
    let p = cstr_to_str(path);
    if let Some(win) = get_window_manager().windows.get_mut(&handle) {
        win.up_visual = p.to_string();
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_SetOverVisual(handle: u32, path: *const c_char) -> bool {
    let p = cstr_to_str(path);
    if let Some(win) = get_window_manager().windows.get_mut(&handle) {
        win.over_visual = p.to_string();
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_SetDownVisual(handle: u32, path: *const c_char) -> bool {
    let p = cstr_to_str(path);
    if let Some(win) = get_window_manager().windows.get_mut(&handle) {
        win.down_visual = p.to_string();
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_Destroy(handle: u32) {
    get_window_manager().destroy_window(handle);
}

#[no_mangle]
pub extern "C" fn wndMgr_GetWidth(handle: u32) -> i32 {
    get_window_manager()
        .windows
        .get(&handle)
        .map(|w| if w.width > 0 { w.width } else { 1024 })
        .unwrap_or(1024)
}

#[no_mangle]
pub extern "C" fn wndMgr_GetHeight(handle: u32) -> i32 {
    get_window_manager()
        .windows
        .get(&handle)
        .map(|w| if w.height > 0 { w.height } else { 768 })
        .unwrap_or(768)
}

#[no_mangle]
pub extern "C" fn wndMgr_GetLocalPosition(handle: u32, out_x: *mut i32, out_y: *mut i32) {
    if let Some(win) = get_window_manager().windows.get(&handle) {
        unsafe {
            if !out_x.is_null() { *out_x = win.local_x; }
            if !out_y.is_null() { *out_y = win.local_y; }
        }
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_SetMouseHandler(po_handler: usize) {
    get_window_manager().mouse_handler = po_handler;
}

#[no_mangle]
pub extern "C" fn wndMgr_SetScreenSize(w: i32, h: i32) {
    m2_app_set_resolution(w as u32, h as u32);
}

#[no_mangle]
pub extern "C" fn wndMgr_GetScreenWidth() -> i32 {
    m2_app_get_width() as i32
}

#[no_mangle]
pub extern "C" fn wndMgr_GetScreenHeight() -> i32 {
    m2_app_get_height() as i32
}

#[no_mangle]
pub extern "C" fn wndMgr_SetWindowPosition(handle: u32, x: i32, y: i32) -> bool {
    get_window_manager().set_position(handle, x, y)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetWindowSize(handle: u32, w: i32, h: i32) -> bool {
    get_window_manager().set_size(handle, w, h)
}

#[no_mangle]
pub extern "C" fn wndMgr_GetWindowWidth(handle: u32) -> i32 {
    get_window_manager()
        .windows
        .get(&handle)
        .map(|w| if w.width > 0 { w.width } else { 1024 })
        .unwrap_or(1024)
}

#[no_mangle]
pub extern "C" fn wndMgr_GetWindowHeight(handle: u32) -> i32 {
    get_window_manager()
        .windows
        .get(&handle)
        .map(|w| if w.height > 0 { w.height } else { 768 })
        .unwrap_or(768)
}

#[no_mangle]
pub extern "C" fn wndMgr_GetWindowLocalPosition(handle: u32, out_x: *mut i32, out_y: *mut i32) {
    wndMgr_GetLocalPosition(handle, out_x, out_y);
}

#[no_mangle]
pub extern "C" fn wndMgr_GetWindowGlobalPosition(handle: u32, out_x: *mut i32, out_y: *mut i32) {
    let wm = get_window_manager();
    if let Some(win) = wm.windows.get(&handle) {
        let rect = win.calc_global_rect(None, m2_app_get_width() as i32, m2_app_get_height() as i32);
        unsafe {
            if !out_x.is_null() { *out_x = rect[0]; }
            if !out_y.is_null() { *out_y = rect[1]; }
        }
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_GetMouseLocalPosition(handle: u32, out_x: *mut i32, out_y: *mut i32) {
    let wm = get_window_manager();
    if let Some(win) = wm.windows.get(&handle) {
        let rect = win.calc_global_rect(None, m2_app_get_width() as i32, m2_app_get_height() as i32);
        unsafe {
            if !out_x.is_null() { *out_x = wm.mouse_x - rect[0]; }
            if !out_y.is_null() { *out_y = wm.mouse_y - rect[1]; }
        }
    } else {
        unsafe {
            if !out_x.is_null() { *out_x = 0; }
            if !out_y.is_null() { *out_y = 0; }
        }
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_GetWindowRect(handle: u32, out_l: *mut i32, out_t: *mut i32, out_r: *mut i32, out_b: *mut i32) {
    let wm = get_window_manager();
    if let Some(win) = wm.windows.get(&handle) {
        let rect = win.calc_global_rect(None, m2_app_get_width() as i32, m2_app_get_height() as i32);
        unsafe {
            if !out_l.is_null() { *out_l = rect[0]; }
            if !out_t.is_null() { *out_t = rect[1]; }
            if !out_r.is_null() { *out_r = rect[0] + rect[2]; }
            if !out_b.is_null() { *out_b = rect[1] + rect[3]; }
        }
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_SetWindowHorizontalAlign(handle: u32, align: u8) -> bool {
    get_window_manager().set_align(handle, align, 0)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetWindowVerticalAlign(handle: u32, align: u8) -> bool {
    get_window_manager().set_align(handle, 0, align)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetPickAlways(handle: u32) {
    if let Some(win) = get_window_manager().windows.get_mut(&handle) {
        win.is_flag_pick_always = true;
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_AddFlag(handle: u32, flag: u32) {
    if let Some(win) = get_window_manager().windows.get_mut(&handle) {
        win.flags |= flag;
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_IsRTL(_handle: u32) -> bool { false }

#[no_mangle]
pub extern "C" fn wndMgr_SetName(handle: u32, name: *const c_char) {
    let s = cstr_to_str(name);
    get_window_manager().set_name(handle, s);
}

#[no_mangle]
pub extern "C" fn wndMgr_SetFocus(handle: u32) {
    let (old_h, new_h) = get_window_manager().set_focus(handle);
    if old_h != 0 {
        crate::py_event_dispatcher::call_python_object_method0(old_h, "OnKillFocus");
    }
    if new_h != 0 {
        crate::py_event_dispatcher::call_python_object_method0(new_h, "OnSetFocus");
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_KillFocus(handle: u32) {
    let old_h = get_window_manager().kill_focus(handle);
    if old_h != 0 {
        crate::py_event_dispatcher::call_python_object_method0(old_h, "OnKillFocus");
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_IsFocus(handle: u32) -> bool {
    get_window_manager().is_focus(handle)
}

#[no_mangle]
pub extern "C" fn wndMgr_Lock(_handle: u32) {}

#[no_mangle]
pub extern "C" fn wndMgr_Unlock(_handle: u32) {}

#[no_mangle]
pub extern "C" fn wndMgr_GetName(handle: u32) -> *const c_char {
    static EMPTY: &[u8] = b"\0";
    let wm = get_window_manager();
    if let Some(win) = wm.windows.get(&handle) {
        if let Ok(c) = CString::new(win.name.as_str()) {
            return c.into_raw();
        }
    }
    EMPTY.as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn wndMgr_GetChildCount(handle: u32) -> i32 {
    let wm = get_window_manager();
    if let Some(win) = wm.windows.get(&handle) {
        win.children.len() as i32
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_IsIn(handle: u32) -> bool {
    let wm = get_window_manager();
    if let Some(win) = wm.windows.get(&handle) {
        let rect = win.calc_global_rect(None, m2_app_get_width() as i32, m2_app_get_height() as i32);
        let (mx, my) = (wm.mouse_x, wm.mouse_y);
        mx >= rect[0] && mx < rect[0] + rect[2] && my >= rect[1] && my < rect[1] + rect[3]
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_IsPickedWindow(_handle: u32) -> bool {
    false
}

// (replaced by real implementation in specialized module)





#[no_mangle]
pub extern "C" fn wndMgr_SetSlotBaseImage(_handle: u32, _path: *const c_char, _r: f32, _g: f32, _b: f32, _a: f32) -> bool { true }

#[no_mangle]
pub extern "C" fn wndMgr_ShowSlotBaseImage(_handle: u32, _slot: u32) -> bool { true }

#[no_mangle]
pub extern "C" fn wndMgr_HideSlotBaseImage(_handle: u32, _slot: u32) -> bool { true }

#[no_mangle]
pub extern "C" fn wndMgr_SetSlotStyle(_handle: u32, _style: u32) -> bool { true }

#[no_mangle]
pub extern "C" fn wndMgr_ArrangeSlot(_handle: u32) -> bool { true }

// (replaced by real implementation in specialized module)
// (replaced by real implementation in specialized module)

// (replaced by real implementation in specialized module)

// (replaced by real implementation in specialized module)

// (replaced by real implementation in specialized module)

// (replaced by real implementation in specialized module)

// (replaced by real implementation in specialized module)

#[no_mangle]
pub extern "C" fn wndMgr_GetTextSize(handle: u32, out_w: *mut i32, out_h: *mut i32) {
    if let Some(win) = get_window_manager().windows.get(&handle) {
        let font = get_global_font_atlas();
        let text_w = font.measure_text_width(&win.text) as i32;
        let text_h = font.line_height as i32;
        unsafe {
            if !out_w.is_null() { *out_w = text_w; }
            if !out_h.is_null() { *out_h = text_h; }
        }
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_GetText(handle: u32, out_buf: *mut c_char, max_len: usize) -> usize {
    if out_buf.is_null() || max_len == 0 {
        return 0;
    }
    if let Some(win) = get_window_manager().windows.get(&handle) {
        let text_to_return = if win.name == "PwdEditLine" || win.name == "Password_EditLine" {
            let pwd = crate::ui_bridge_ffi::REAL_PASSWORD.lock().unwrap();
            pwd.clone()
        } else {
            win.text.clone()
        };
        let bytes = text_to_return.as_bytes();
        let copy_len = bytes.len().min(max_len - 1);
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, copy_len);
            *out_buf.add(copy_len) = 0;
        }
        copy_len
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_GetMousePosition(out_x: *mut i32, out_y: *mut i32) {
    let wm = get_window_manager();
    if !out_x.is_null() {
        unsafe { *out_x = wm.mouse_x; }
    }
    if !out_y.is_null() {
        unsafe { *out_y = wm.mouse_y; }
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_GetCursorPosition(handle: u32) -> i32 {
    if let Some(win) = get_window_manager().windows.get(&handle) {
        win.text.len() as i32
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn wndMgr_ShowCursor(_handle: u32) {}

#[no_mangle]
pub extern "C" fn wndMgr_HideCursor(_handle: u32) {}

static AUTO_PICKUP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static FOV_LEVEL: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(30.0f32.to_bits());

/// Zwraca aktualny stan automatycznego podnoszenia przedmiotow.
#[no_mangle]
pub extern "C" fn wndMgr_GetAutoPickUp() -> bool {
    AUTO_PICKUP.load(std::sync::atomic::Ordering::SeqCst)
}

/// Ustawia stan automatycznego podnoszenia przedmiotow.
#[no_mangle]
pub extern "C" fn wndMgr_SetAutoPickUp(enable: bool) {
    AUTO_PICKUP.store(enable, std::sync::atomic::Ordering::SeqCst);
}

/// Zwraca aktualny kat widzenia kamery (FOV Level).
#[no_mangle]
pub extern "C" fn wndMgr_GetFOVLevel() -> f32 {
    f32::from_bits(FOV_LEVEL.load(std::sync::atomic::Ordering::SeqCst))
}

/// Ustawia kat widzenia kamery (FOV Level).
#[no_mangle]
pub extern "C" fn wndMgr_SetFOVLevel(fov: f32) {
    FOV_LEVEL.store(fov.to_bits(), std::sync::atomic::Ordering::SeqCst);
}
