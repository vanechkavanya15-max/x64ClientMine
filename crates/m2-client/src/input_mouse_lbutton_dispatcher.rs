use crate::ui_window_core::WindowType;
use crate::ui_window_mgr::get_window_manager;
use crate::py_event_dispatcher::{call_python_object_method0, call_python_object_method2_i32};

pub fn dispatch_wm_lbutton_down(mx: i32, my: i32, screen_w: i32, screen_h: i32) -> Option<u32> {
    let mut wm = get_window_manager();
    let hit = wm.pick_window(mx, my, screen_w, screen_h)?;
    wm.capture_window = Some(hit);
    
    let prev_active = wm.active_window;
    if prev_active != Some(hit) {
        wm.active_window = Some(hit);
        if let Some(old_h) = prev_active {
            if let Some(old_win) = wm.windows.get(&old_h) {
                if old_win.py_handler != 0 {
                    call_python_object_method0(old_win.py_handler, "OnKillFocus");
                }
            }
        }
        if let Some(new_win) = wm.windows.get(&hit) {
            if new_win.py_handler != 0 {
                call_python_object_method0(new_win.py_handler, "OnSetFocus");
            }
        }
    }
    
    if let Some(win) = wm.windows.get_mut(&hit) {
        win.is_pressed = true;
        if win.py_handler != 0 {
            let rect = win.calc_global_rect(None, screen_w, screen_h);
            let local_x = mx - rect[0];
            let local_y = my - rect[1];

            let handled = call_python_object_method2_i32(win.py_handler, "OnMouseLeftButtonDown", local_x, local_y);
            if !handled {
                call_python_object_method0(win.py_handler, "OnMouseLeftButtonDown");
            }
            
            match win.window_type {
                WindowType::Button => {
                    call_python_object_method0(win.py_handler, "DownEvent");
                }
                WindowType::RadioButton => {
                    call_python_object_method0(win.py_handler, "CallEvent");
                }
                WindowType::ToggleButton => {
                    call_python_object_method0(win.py_handler, "OnToggleDown");
                }
                _ => {}
            }
        }
    }
    Some(hit)
}

pub fn dispatch_wm_lbutton_up(mx: i32, my: i32, screen_w: i32, screen_h: i32) -> (Option<u32>, String) {
    let mut wm = get_window_manager();
    let hit = wm.pick_window(mx, my, screen_w, screen_h);
    
    for win in wm.windows.values_mut() {
        win.is_pressed = false;
    }
    
    let target = wm.capture_window.or(hit);
    let mut name = String::new();
    let mut hit_out = None;

    if let Some(t) = target {
        if let Some(win) = wm.windows.get(&t) {
            if win.py_handler != 0 {
                let rect = win.calc_global_rect(None, screen_w, screen_h);
                let local_x = mx - rect[0];
                let local_y = my - rect[1];

                let handled = call_python_object_method2_i32(win.py_handler, "OnMouseLeftButtonUp", local_x, local_y);
                if !handled {
                    call_python_object_method0(win.py_handler, "OnMouseLeftButtonUp");
                }
                
                if win.window_type == WindowType::Button {
                    call_python_object_method0(win.py_handler, "CallEvent");
                }
            }
        }
    }

    if let Some(h) = hit {
        if let Some(win) = wm.windows.get(&h) {
            name = win.name.clone();
            hit_out = Some(h);
        }
    }

    wm.capture_window = None;
    (hit_out, name)
}
