#[path = "../src/win32_keyboard_hooks.rs"]
pub mod win32_keyboard_hooks;
#[path = "../src/input_focus_lost_cleanup.rs"]
pub mod input_focus_lost_cleanup;
#[path = "../src/win32_message_dispatcher.rs"]
pub mod win32_message_dispatcher;

pub mod ui_window_mgr {
    #[derive(Clone)]
    pub struct Window {
        pub name: String,
        pub is_show: bool,
    }
    
    pub struct WindowManager {
        pub is_attaching: bool,
        pub windows: std::collections::HashMap<u32, Window>,
    }
    impl WindowManager {
        pub fn handle_mouse_move(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) {}
        pub fn handle_mouse_down(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) {}
        pub fn handle_mouse_up(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) -> Option<u32> { None }
        pub fn handle_key_down(&mut self, _vkey: u32) {}
        pub fn handle_key_up(&mut self, _vkey: u32) {}
        pub fn handle_mouse_right_down(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) {}
        pub fn handle_mouse_right_up(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) {}
        pub fn update_all_windows(&mut self) {}
    }
    pub fn get_window_manager() -> WindowManager { 
        WindowManager { 
            is_attaching: false,
            windows: std::collections::HashMap::new(),
        } 
    }
}

pub mod ui_bridge_ffi {
    pub static CURRENT_PHASE: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
    pub fn handle_ui_click(_name: &str, _hit: u32, _wm: &mut super::ui_window_mgr::WindowManager) {}
    pub fn toggle_edit_focus() {}
    pub fn handle_char_input(_c: char) {}
}

use win32_keyboard_hooks::KeyStateTable;
use win32_message_dispatcher::{WinMsg, WM_ACTIVATE, WM_KILLFOCUS, WM_MOUSEMOVE, WA_INACTIVE, WA_ACTIVE};
use input_focus_lost_cleanup::process_focus_lost_cleanup;

#[test]
fn test_wm_killfocus_clears_keys() {
    let mut key_state = KeyStateTable::new();
    key_state.on_key_down(0x41, 100); // Press 'A'
    assert!(key_state.is_pressed(0x41));

    let msg = WinMsg {
        hwnd: 0,
        message: WM_KILLFOCUS,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };

    let result = process_focus_lost_cleanup(&msg, &mut key_state);
    
    assert!(result);
    assert!(!key_state.is_pressed(0x41));
}

#[test]
fn test_wm_activate_inactive_clears_keys() {
    let mut key_state = KeyStateTable::new();
    key_state.on_key_down(0x57, 100); // Press 'W'
    assert!(key_state.is_pressed(0x57));

    let msg = WinMsg {
        hwnd: 0,
        message: WM_ACTIVATE,
        wparam: WA_INACTIVE,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };

    let result = process_focus_lost_cleanup(&msg, &mut key_state);
    
    assert!(result);
    assert!(!key_state.is_pressed(0x57));
}

#[test]
fn test_wm_activate_minimized_clears_keys() {
    let mut key_state = KeyStateTable::new();
    key_state.on_key_down(0x20, 100); // Press 'Space'
    assert!(key_state.is_pressed(0x20));

    // wparam format for WM_ACTIVATE: high word is minimized state, low word is activation state
    let wparam = (1 << 16) | WA_ACTIVE; // Minimized = true, State = Active

    let msg = WinMsg {
        hwnd: 0,
        message: WM_ACTIVATE,
        wparam,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };

    let result = process_focus_lost_cleanup(&msg, &mut key_state);
    
    assert!(result);
    assert!(!key_state.is_pressed(0x20));
}

#[test]
fn test_wm_activate_active_not_minimized_ignores() {
    let mut key_state = KeyStateTable::new();
    key_state.on_key_down(0x0D, 100); // Press 'Enter'
    assert!(key_state.is_pressed(0x0D));

    let wparam = WA_ACTIVE; // Minimized = false, State = Active

    let msg = WinMsg {
        hwnd: 0,
        message: WM_ACTIVATE,
        wparam,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };

    let result = process_focus_lost_cleanup(&msg, &mut key_state);
    
    assert!(!result);
    assert!(key_state.is_pressed(0x0D));
}

#[test]
fn test_unrelated_message_ignores() {
    let mut key_state = KeyStateTable::new();
    key_state.on_key_down(0x1B, 100); // Press 'Esc'
    assert!(key_state.is_pressed(0x1B));

    let msg = WinMsg {
        hwnd: 0,
        message: WM_MOUSEMOVE,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };

    let result = process_focus_lost_cleanup(&msg, &mut key_state);
    
    assert!(!result);
    assert!(key_state.is_pressed(0x1B));
}
