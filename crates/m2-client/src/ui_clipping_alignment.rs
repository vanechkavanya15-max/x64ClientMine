// ui_clipping_alignment.rs - UI Scissor Rects and Alignment Module

use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use crate::ui_window_mgr::{get_window_manager, WindowManager};
use crate::ui_window_core::{
    HORIZONTAL_ALIGN_CENTER, HORIZONTAL_ALIGN_LEFT, HORIZONTAL_ALIGN_RIGHT, VERTICAL_ALIGN_BOTTOM,
    VERTICAL_ALIGN_CENTER, VERTICAL_ALIGN_TOP,
};

static SCISSOR_RECT_STATE: OnceLock<Mutex<HashMap<u32, bool>>> = OnceLock::new();

fn get_scissor_state() -> std::sync::MutexGuard<'static, HashMap<u32, bool>> {
    SCISSOR_RECT_STATE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn wndMgr_EnableScissorRect(handle: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_scissor_state();
        state.insert(handle, true);
        true
    })
}

#[no_mangle]
pub extern "C" fn wndMgr_DisableScissorRect(handle: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_scissor_state();
        state.insert(handle, false);
        true
    })
}

#[no_mangle]
pub extern "C" fn wndMgr_IsScissorRectEnabled(handle: u32) -> bool {
    safe_ffi_call(false, || {
        let state = get_scissor_state();
        *state.get(&handle).unwrap_or(&false)
    })
}

#[no_mangle]
pub extern "C" fn wndMgr_SetHorizontalAlign(handle: u32, align: u32) -> bool {
    safe_ffi_call(false, || {
        let mut wm = get_window_manager();
        let mut v_align = VERTICAL_ALIGN_TOP;
        if let Some(win) = wm.windows.get(&handle) {
            v_align = win.vertical_align;
        } else {
            return false;
        }
        let res = wm.set_align(handle, align as u8, v_align);
        drop(wm);
        wndMgr_UpdateRect(handle);
        res
    })
}

#[no_mangle]
pub extern "C" fn wndMgr_SetVerticalAlign(handle: u32, align: u32) -> bool {
    safe_ffi_call(false, || {
        let mut wm = get_window_manager();
        let mut h_align = HORIZONTAL_ALIGN_LEFT;
        if let Some(win) = wm.windows.get(&handle) {
            h_align = win.horizontal_align;
        } else {
            return false;
        }
        let res = wm.set_align(handle, h_align, align as u8);
        drop(wm);
        wndMgr_UpdateRect(handle);
        res
    })
}

#[no_mangle]
pub extern "C" fn wndMgr_SetDiffuseColor(handle: u32, r: f32, g: f32, b: f32, a: f32) -> bool {
    safe_ffi_call(false, || {
        let mut wm = get_window_manager();
        if let Some(win) = wm.windows.get_mut(&handle) {
            win.color = [r, g, b, a];
            wm.render_dirty = true;
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn wndMgr_UpdateRect(handle: u32) {
    safe_ffi_call((), || {
        let mut wm = get_window_manager();
        update_rect_internal(&mut wm, handle);
    });
}

fn update_rect_internal(wm: &mut std::sync::MutexGuard<'static, WindowManager>, handle: u32) {
    let mut child_handles = Vec::new();

    if let Some(win) = wm.windows.get(&handle) {
        child_handles = win.children.clone();
        
        let mut top = win.local_y;
        let mut left = win.local_x;
        let mut right = left + win.width;
        let mut bottom = top + win.height;
        let width = win.width;
        let height = win.height;
        let v_align = win.vertical_align;
        let h_align = win.horizontal_align;
        
        let mut parent_rect: Option<[f32; 4]> = None;
        let mut parent_w: i32 = 0;
        let mut parent_h: i32 = 0;

        if let Some(parent_handle) = win.parent {
            if let Some(parent) = wm.windows.get(&parent_handle) {
                parent_rect = Some(parent.rendering_rect);
                parent_w = parent.width;
                parent_h = parent.height;
            }
        }
        
        if let Some(pr) = parent_rect {
            match v_align {
                VERTICAL_ALIGN_BOTTOM => {
                    top = parent_h - height - top;
                }
                VERTICAL_ALIGN_CENTER => {
                    top = (parent_h - height) / 2 + top;
                }
                _ => {}
            }
            top += pr[1] as i32; // parent_rect.top
            
            // RTL is not natively in our ui_window_core.rs flag system, so we use the C++ standard logic 
            // for non-RTL flip in CWindow::UpdateRect
            match h_align {
                HORIZONTAL_ALIGN_RIGHT => {
                    left = parent_w.abs() - width - left;
                }
                HORIZONTAL_ALIGN_CENTER => {
                    left = parent_w / 2 - width / 2 + left;
                }
                _ => {}
            }
            let p_left = pr[0] as i32;
            let p_right = pr[2] as i32;
            
            left += if parent_w > 0 { p_left } else { p_right + parent_w.abs() };
        }
        
        bottom = top + height;
        right = left + width;

        if let Some(win_mut) = wm.windows.get_mut(&handle) {
            win_mut.rendering_rect = [left as f32, top as f32, right as f32, bottom as f32];
        }
    } else {
        return;
    }

    wm.render_dirty = true;

    for child in child_handles {
        update_rect_internal(wm, child);
    }
}


pub fn calculate_wgpu_scissor_rect(
    wm: &std::sync::MutexGuard<'static, WindowManager>,
    handle: u32,
    screen_w: u32,
    screen_h: u32,
) -> Option<[u32; 4]> {
    let win = wm.windows.get(&handle)?;
    
    let mut scissor_left = std::cmp::max(0, win.rendering_rect[0] as i32);
    let mut scissor_top = std::cmp::max(0, win.rendering_rect[1] as i32);
    let mut scissor_right = win.rendering_rect[0] as i32 + win.width;
    let mut scissor_bottom = win.rendering_rect[1] as i32 + win.height;

    // Find parent scissor window
    let mut parent_scissor_win = None;
    let mut current_parent = win.parent;
    let state = get_scissor_state();
    
    while let Some(p_handle) = current_parent {
        if let Some(p_win) = wm.windows.get(&p_handle) {
            if *state.get(&p_handle).unwrap_or(&false) {
                parent_scissor_win = Some(p_win);
                break;
            }
            current_parent = p_win.parent;
        } else {
            break;
        }
    }

    if let Some(p_win) = parent_scissor_win {
        let parent_left = std::cmp::max(0, p_win.rendering_rect[0] as i32);
        let parent_top = std::cmp::max(0, p_win.rendering_rect[1] as i32);
        let parent_right = p_win.rendering_rect[0] as i32 + p_win.width;
        let parent_bottom = p_win.rendering_rect[1] as i32 + p_win.height;

        scissor_left = std::cmp::max(scissor_left, parent_left);
        scissor_top = std::cmp::max(scissor_top, parent_top);
        scissor_right = std::cmp::min(scissor_right, parent_right);
        scissor_bottom = std::cmp::min(scissor_bottom, parent_bottom);
    }

    if scissor_right <= scissor_left || scissor_bottom <= scissor_top {
        return None;
    }

    let w = scissor_right - scissor_left;
    let h = scissor_bottom - scissor_top;
    
    // Clamp to screen bounds to prevent WGPU crash
    let x = std::cmp::min(scissor_left as u32, screen_w);
    let y = std::cmp::min(scissor_top as u32, screen_h);
    let final_w = std::cmp::min(w as u32, screen_w.saturating_sub(x));
    let final_h = std::cmp::min(h as u32, screen_h.saturating_sub(y));

    if final_w == 0 || final_h == 0 {
        return None;
    }

    Some([x, y, final_w, final_h])
}
