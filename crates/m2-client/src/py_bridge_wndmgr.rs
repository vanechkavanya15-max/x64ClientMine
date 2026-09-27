use crate::ui_window_mgr::get_window_manager;

#[no_mangle]
pub extern "C" fn wndMgr_Register(py_obj: usize) -> u32 {
    let mut mgr = get_window_manager();
    mgr.register(py_obj)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetPosition(handle: u32, x: i32, y: i32) -> bool {
    let mut mgr = get_window_manager();
    mgr.set_position(handle, x, y)
}

#[no_mangle]
pub extern "C" fn wndMgr_Show(handle: u32) -> bool {
    let mut mgr = get_window_manager();
    mgr.show(handle)
}

#[no_mangle]
pub extern "C" fn wndMgr_Hide(handle: u32) -> bool {
    let mut mgr = get_window_manager();
    mgr.hide(handle)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetTop(handle: u32) -> bool {
    let mut mgr = get_window_manager();
    mgr.set_top(handle)
}

#[no_mangle]
pub extern "C" fn wndMgr_SetSize(handle: u32, w: i32, h: i32) -> bool {
    let mut mgr = get_window_manager();
    mgr.set_size(handle, w, h)
}

#[no_mangle]
pub extern "C" fn wndMgr_IsShow(handle: u32) -> bool {
    let mgr = get_window_manager();
    mgr.is_show(handle)
}
