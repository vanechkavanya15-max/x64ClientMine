use m2_client::win32_message_dispatcher::*;
use serial_test::serial;

#[test]
#[serial]
fn test_initial_state() {
    reset_dispatcher_state();
    assert_eq!(is_window_active(), true);
    assert_eq!(is_window_minimized(), false);
    assert_eq!(has_window_focus(), true);
    assert_eq!(poll_window_messages(), MsgQueueStatus::Continue);
}

#[test]
#[serial]
fn test_wm_close_requests_quit() {
    reset_dispatcher_state();
    
    let msg = WinMsg {
        hwnd: 0,
        message: WM_CLOSE,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    
    enqueue_message(msg);
    let status = poll_window_messages();
    assert_eq!(status, MsgQueueStatus::Quit(0));
}

#[test]
#[serial]
fn test_wm_quit_requests_quit() {
    reset_dispatcher_state();
    
    let exit_code = 42;
    let msg = WinMsg {
        hwnd: 0,
        message: WM_QUIT,
        wparam: exit_code as usize,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    
    enqueue_message(msg);
    let status = poll_window_messages();
    assert_eq!(status, MsgQueueStatus::Quit(exit_code));
}

#[test]
#[serial]
fn test_post_quit_message() {
    reset_dispatcher_state();
    
    post_quit_message(101);
    let status = poll_window_messages();
    assert_eq!(status, MsgQueueStatus::Quit(101));
}

#[test]
#[serial]
fn test_wm_activate_states() {
    reset_dispatcher_state();
    
    // Test Inactive
    let msg = WinMsg {
        hwnd: 0,
        message: WM_ACTIVATE,
        wparam: WA_INACTIVE,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    enqueue_message(msg);
    poll_window_messages();
    
    assert_eq!(is_window_active(), false);
    assert_eq!(is_window_minimized(), false);
    
    // Test Active
    let msg2 = WinMsg {
        hwnd: 0,
        message: WM_ACTIVATE,
        wparam: WA_ACTIVE,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    enqueue_message(msg2);
    poll_window_messages();
    
    assert_eq!(is_window_active(), true);
    assert_eq!(is_window_minimized(), false);
    
    // Test Minimized (high word of wparam != 0)
    let wparam_minimized = (1 << 16) | WA_ACTIVE;
    let msg3 = WinMsg {
        hwnd: 0,
        message: WM_ACTIVATE,
        wparam: wparam_minimized,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    enqueue_message(msg3);
    poll_window_messages();
    
    assert_eq!(is_window_active(), false);
    assert_eq!(is_window_minimized(), true);
}

#[test]
#[serial]
fn test_wm_setfocus_killfocus() {
    reset_dispatcher_state();
    
    let msg_kill = WinMsg {
        hwnd: 0,
        message: WM_KILLFOCUS,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    enqueue_message(msg_kill);
    poll_window_messages();
    assert_eq!(has_window_focus(), false);
    
    let msg_set = WinMsg {
        hwnd: 0,
        message: WM_SETFOCUS,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    enqueue_message(msg_set);
    poll_window_messages();
    assert_eq!(has_window_focus(), true);
}

#[test]
#[serial]
fn test_multiple_messages_queued() {
    reset_dispatcher_state();
    
    // Enqueue a kill focus, an activate inactive, then a close
    enqueue_message(WinMsg {
        hwnd: 0,
        message: WM_KILLFOCUS,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    });
    
    enqueue_message(WinMsg {
        hwnd: 0,
        message: WM_ACTIVATE,
        wparam: WA_INACTIVE,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    });
    
    enqueue_message(WinMsg {
        hwnd: 0,
        message: WM_QUIT,
        wparam: 99,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    });
    
    let status = poll_window_messages();
    
    assert_eq!(has_window_focus(), false);
    assert_eq!(is_window_active(), false);
    assert_eq!(status, MsgQueueStatus::Quit(99));
}

#[test]
#[serial]
fn test_process_single_message() {
    reset_dispatcher_state();
    
    let msg = WinMsg {
        hwnd: 0,
        message: WM_KILLFOCUS,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    
    let handled = process_single_message(&msg);
    assert_eq!(handled, true);
    assert_eq!(has_window_focus(), false);
    
    let unhandled_msg = WinMsg {
        hwnd: 0,
        message: 0x9999, // Some unknown message
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    
    let handled2 = process_single_message(&unhandled_msg);
    assert_eq!(handled2, false);
}
