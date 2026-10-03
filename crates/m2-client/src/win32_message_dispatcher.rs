use std::sync::Mutex;
use std::collections::VecDeque;

#[path = "input_mouse_lbutton_dispatcher.rs"]
pub mod input_mouse_lbutton_dispatcher;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WinMsg {
    pub hwnd: usize,
    pub message: u32,
    pub wparam: usize,
    pub lparam: isize,
    pub time: u32,
    pub pt_x: i32,
    pub pt_y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgQueueStatus {
    Continue,
    Quit(i32),
}

// Win32 constants
pub const WM_SETFOCUS: u32 = 0x0007;
pub const WM_KILLFOCUS: u32 = 0x0008;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_QUIT: u32 = 0x0012;
pub const WM_ACTIVATE: u32 = 0x0006;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const WM_CHAR: u32 = 0x0102;
pub const WM_MOUSEMOVE: u32 = 0x0200;
pub const WM_LBUTTONDOWN: u32 = 0x0201;
pub const WM_LBUTTONUP: u32 = 0x0202;
pub const WM_RBUTTONDOWN: u32 = 0x0204;
pub const WM_RBUTTONUP: u32 = 0x0205;

pub const WA_INACTIVE: usize = 0;
pub const WA_ACTIVE: usize = 1;
pub const WA_CLICKACTIVE: usize = 2;

struct DispatcherState {
    message_queue: VecDeque<WinMsg>,
    is_active: bool,
    is_minimized: bool,
    has_focus: bool,
    quit_requested: Option<i32>,
}

impl DispatcherState {
    const fn new() -> Self {
        Self {
            message_queue: VecDeque::new(),
            is_active: true,
            is_minimized: false,
            has_focus: true,
            quit_requested: None,
        }
    }
}

static DISPATCHER_STATE: Mutex<DispatcherState> = Mutex::new(DispatcherState::new());

pub fn poll_window_messages() -> MsgQueueStatus {
    #[cfg(windows)]
    {
        #[repr(C)]
        struct MSG {
            hwnd: *mut std::ffi::c_void,
            message: u32,
            wparam: usize,
            lparam: isize,
            time: u32,
            pt_x: i32,
            pt_y: i32,
        }

        #[link(name = "user32")]
        extern "system" {
            fn PeekMessageA(
                lpMsg: *mut MSG,
                hWnd: *mut std::ffi::c_void,
                wMsgFilterMin: u32,
                wMsgFilterMax: u32,
                wRemoveMsg: u32,
            ) -> i32;
            fn TranslateMessage(lpMsg: *const MSG) -> i32;
            fn DispatchMessageA(lpMsg: *const MSG) -> isize;
        }

        const PM_REMOVE: u32 = 0x0001;

        let mut msg = MSG {
            hwnd: std::ptr::null_mut(),
            message: 0,
            wparam: 0,
            lparam: 0,
            time: 0,
            pt_x: 0,
            pt_y: 0,
        };

        while unsafe { PeekMessageA(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) } != 0 {
            if msg.message == WM_QUIT {
                post_quit_message(msg.wparam as i32);
                return MsgQueueStatus::Quit(msg.wparam as i32);
            }

            let win_msg = WinMsg {
                hwnd: msg.hwnd as usize,
                message: msg.message,
                wparam: msg.wparam,
                lparam: msg.lparam,
                time: msg.time,
                pt_x: msg.pt_x,
                pt_y: msg.pt_y,
            };

            if let Ok(mut state) = DISPATCHER_STATE.lock() {
                process_single_message_internal(&mut state, &win_msg);
            }

            unsafe {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }

    let mut state = DISPATCHER_STATE.lock().unwrap();

    if let Some(exit_code) = state.quit_requested {
        return MsgQueueStatus::Quit(exit_code);
    }

    // Processing queued messages
    while let Some(msg) = state.message_queue.pop_front() {
        process_single_message_internal(&mut state, &msg);
        if let Some(exit_code) = state.quit_requested {
            return MsgQueueStatus::Quit(exit_code);
        }
    }

    MsgQueueStatus::Continue
}

pub fn process_single_message(msg: &WinMsg) -> bool {
    let mut state = DISPATCHER_STATE.lock().unwrap();
    process_single_message_internal(&mut state, msg)
}

fn process_single_message_internal(state: &mut DispatcherState, msg: &WinMsg) -> bool {
    match msg.message {
        WM_CLOSE => {
            state.quit_requested = Some(0);
            true
        }
        WM_QUIT => {
            state.quit_requested = Some(msg.wparam as i32);
            true
        }
        WM_ACTIVATE => {
            let state_flag = msg.wparam & 0xFFFF;
            let minimized = (msg.wparam >> 16) != 0;
            
            state.is_minimized = minimized;
            state.is_active = state_flag != WA_INACTIVE && !minimized;
            true
        }
        WM_PAINT => {
            // Paint logic would go here. Usually involves BeginPaint/EndPaint
            true
        }
        WM_SETFOCUS => {
            state.has_focus = true;
            true
        }
        WM_KILLFOCUS => {
            state.has_focus = false;
            // Clean up keyboard state on focus lost is handled externally via input_focus_lost_cleanup::process_focus_lost_cleanup
            true
        }
        WM_MOUSEMOVE => {
            let mx = (msg.lparam & 0xFFFF) as i16 as i32;
            let my = ((msg.lparam >> 16) & 0xFFFF) as i16 as i32;
            let mut wm = crate::ui_window_mgr::get_window_manager();
            wm.handle_mouse_move(mx, my, 1024, 768);
            true
        }
        WM_LBUTTONDOWN => {
            let mx = (msg.lparam & 0xFFFF) as i16 as i32;
            let my = ((msg.lparam >> 16) & 0xFFFF) as i16 as i32;
            input_mouse_lbutton_dispatcher::dispatch_wm_lbutton_down(mx, my, 1024, 768);
            true
        }
        WM_LBUTTONUP => {
            let mx = (msg.lparam & 0xFFFF) as i16 as i32;
            let my = ((msg.lparam >> 16) & 0xFFFF) as i16 as i32;
            let (hit, name) = input_mouse_lbutton_dispatcher::dispatch_wm_lbutton_up(mx, my, 1024, 768);
            if let Some(h) = hit {
                let mut wm = crate::ui_window_mgr::get_window_manager();
                crate::ui_bridge_ffi::handle_ui_click(&name, h, &mut wm);
            }
            true
        }
        WM_KEYDOWN => {
            let vk = msg.wparam as u32;
            {
                let mut wm = crate::ui_window_mgr::get_window_manager();
                wm.handle_key_down(vk);
            }
            let phase = crate::ui_bridge_ffi::CURRENT_PHASE.lock().unwrap().clone();
            match vk {
                0x1B => { // VK_ESCAPE
                    let mut wm = crate::ui_window_mgr::get_window_manager();
                    if phase == "GAME" {
                        crate::ui_bridge_ffi::handle_ui_click("HudBtnSystem", 0, &mut wm);
                    } else if phase == "SELECT" {
                        crate::ui_bridge_ffi::handle_ui_click("SelectExitButton", 0, &mut wm);
                    } else {
                        let is_lb_vis = wm.windows.values().find(|w| w.name == "LoginBoard").map(|w| w.is_show).unwrap_or(false);
                        if is_lb_vis {
                            crate::ui_bridge_ffi::handle_ui_click("LoginExitButton", 0, &mut wm);
                        } else {
                            crate::ui_bridge_ffi::handle_ui_click("ServerExitButton", 0, &mut wm);
                        }
                    }
                }
                0x0D => { // VK_RETURN
                    let mut wm = crate::ui_window_mgr::get_window_manager();
                    if phase == "SELECT" {
                        crate::ui_bridge_ffi::handle_ui_click("SelectStartButton", 0, &mut wm);
                    } else if phase == "GAME" {
                        println!("[CHAT] Otwieranie pola wprowadzania wiadomosci na czacie...");
                    } else {
                        let is_lb_vis = wm.windows.values().find(|w| w.name == "LoginBoard").map(|w| w.is_show).unwrap_or(false);
                        if is_lb_vis {
                            crate::ui_bridge_ffi::handle_ui_click("LoginButton", 0, &mut wm);
                        } else {
                            crate::ui_bridge_ffi::handle_ui_click("ServerSelectButton", 0, &mut wm);
                        }
                    }
                }
                0x09 => { // VK_TAB
                    crate::ui_bridge_ffi::toggle_edit_focus();
                }
                0x49 => { // 'I' - Ekwipunek
                    if phase == "GAME" {
                        let mut wm = crate::ui_window_mgr::get_window_manager();
                        crate::ui_bridge_ffi::handle_ui_click("HudBtnInventory", 0, &mut wm);
                    }
                }
                0x43 => { // 'C' - Status postaci
                    if phase == "GAME" {
                        let mut wm = crate::ui_window_mgr::get_window_manager();
                        crate::ui_bridge_ffi::handle_ui_click("HudBtnCharacter", 0, &mut wm);
                    }
                }
                0x56 => { // 'V' - Skille
                    if phase == "GAME" {
                        let mut wm = crate::ui_window_mgr::get_window_manager();
                        crate::ui_bridge_ffi::handle_ui_click("HudBtnSkills", 0, &mut wm);
                    }
                }
                _ => {}
            }
            true
        }
        WM_KEYUP => {
            let vk = msg.wparam as u32;
            let mut wm = crate::ui_window_mgr::get_window_manager();
            wm.handle_key_up(vk);
            true
        }
        WM_RBUTTONDOWN => {
            let mx = (msg.lparam & 0xFFFF) as i16 as i32;
            let my = ((msg.lparam >> 16) & 0xFFFF) as i16 as i32;
            let mut wm = crate::ui_window_mgr::get_window_manager();
            wm.handle_mouse_right_down(mx, my, 1024, 768);
            true
        }
        WM_RBUTTONUP => {
            let mx = (msg.lparam & 0xFFFF) as i16 as i32;
            let my = ((msg.lparam >> 16) & 0xFFFF) as i16 as i32;
            let mut wm = crate::ui_window_mgr::get_window_manager();
            wm.handle_mouse_right_up(mx, my, 1024, 768);
            true
        }
        WM_CHAR => {
            let ch = (msg.wparam & 0xFFFF) as u32;
            if let Some(c) = char::from_u32(ch) {
                crate::ui_bridge_ffi::handle_char_input(c);
            }
            true
        }
        _ => false,
    }
}

pub fn post_quit_message(exit_code: i32) {
    let mut state = DISPATCHER_STATE.lock().unwrap();
    state.quit_requested = Some(exit_code);
}

// Function to push messages to the queue without blocking the main game thread
pub fn enqueue_message(msg: WinMsg) {
    if let Ok(mut state) = DISPATCHER_STATE.lock() {
        state.message_queue.push_back(msg);
    }
}

pub fn is_window_active() -> bool {
    if let Ok(state) = DISPATCHER_STATE.lock() {
        state.is_active
    } else {
        false
    }
}

pub fn is_window_minimized() -> bool {
    if let Ok(state) = DISPATCHER_STATE.lock() {
        state.is_minimized
    } else {
        false
    }
}

pub fn has_window_focus() -> bool {
    if let Ok(state) = DISPATCHER_STATE.lock() {
        state.has_focus
    } else {
        false
    }
}

pub fn reset_dispatcher_state() {
    if let Ok(mut state) = DISPATCHER_STATE.lock() {
        state.message_queue.clear();
        state.is_active = true;
        state.is_minimized = false;
        state.has_focus = true;
        state.quit_requested = None;
    }
}
