use std::sync::Mutex;
use std::collections::VecDeque;

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
