// py_bridge_wndmgr.rs - Complete Python bridge for wndMgr module in Metin2 Pure Rust
use crate::ui_window_core::WindowType;
use crate::ui_window_mgr::get_window_manager;
use std::ffi::CStr;
use std::os::raw::c_char;

fn cstr_to_str<'a>(ptr: *const c_char) -> &'a str {
    if ptr.is_null() {
        return "";
    }
    unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") }
}

#[no_mangle]
pub extern "C" fn wndMgr_Register(_py_obj: usize) -> u32 {
    get_window_manager().register(WindowType::Normal)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterImageBox(_py_obj: usize) -> u32 {
    get_window_manager().register(WindowType::ImageBox)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterExpandedImageBox(_py_obj: usize) -> u32 {
    get_window_manager().register(WindowType::ExpandedImageBox)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterTextLine(_py_obj: usize) -> u32 {
    get_window_manager().register(WindowType::TextLine)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterButton(_py_obj: usize) -> u32 {
    get_window_manager().register(WindowType::Button)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterBar(_py_obj: usize) -> u32 {
    get_window_manager().register(WindowType::Bar)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterLine(_py_obj: usize) -> u32 {
    get_window_manager().register(WindowType::Line)
}

#[no_mangle]
pub extern "C" fn wndMgr_RegisterBox(_py_obj: usize) -> u32 {
    get_window_manager().register(WindowType::Box)
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
    wm.top_window = Some(handle);
    true
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
    get_window_manager().windows.get(&handle).map(|w| w.width).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn wndMgr_GetHeight(handle: u32) -> i32 {
    get_window_manager().windows.get(&handle).map(|w| w.height).unwrap_or(0)
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
