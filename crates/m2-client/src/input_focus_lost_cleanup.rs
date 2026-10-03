use crate::win32_keyboard_hooks::KeyStateTable;
use crate::win32_message_dispatcher::{WinMsg, WM_ACTIVATE, WM_KILLFOCUS, WA_INACTIVE};

pub fn process_focus_lost_cleanup(msg: &WinMsg, key_state: &mut KeyStateTable) -> bool {
    let mut should_cleanup = false;
    
    match msg.message {
        WM_KILLFOCUS => {
            should_cleanup = true;
        }
        WM_ACTIVATE => {
            let state_flag = msg.wparam & 0xFFFF;
            let minimized = (msg.wparam >> 16) != 0;
            
            if state_flag == WA_INACTIVE || minimized {
                should_cleanup = true;
            }
        }
        _ => {}
    }
    
    if should_cleanup {
        key_state.on_kill_focus();
        true
    } else {
        false
    }
}
