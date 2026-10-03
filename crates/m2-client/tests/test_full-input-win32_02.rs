use m2_client::input_mouse_rbutton_dispatcher::RButtonDispatcher;
use m2_client::target_selection::TargetManager;
use m2_client::win32_message_dispatcher::{WinMsg, WM_MOUSEMOVE, WM_RBUTTONDOWN, WM_RBUTTONUP};

#[test]
fn test_start_camera_rotation() {
    let mut dispatcher = RButtonDispatcher::new();
    let mut target_manager = TargetManager::new();

    let msg = WinMsg {
        hwnd: 0,
        message: WM_RBUTTONDOWN,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 100,
        pt_y: 200,
    };

    let result = dispatcher.handle_message(&msg, None, &mut target_manager);

    assert_eq!(result, Some(()));
    assert!(dispatcher.is_rotating_camera);
    assert_eq!(dispatcher.last_mouse_x, 100);
    assert_eq!(dispatcher.last_mouse_y, 200);
    assert!(target_manager.selected_target_vid.is_none());
}

#[test]
fn test_activate_target_instead_of_rotation() {
    let mut dispatcher = RButtonDispatcher::new();
    let mut target_manager = TargetManager::new();

    let msg = WinMsg {
        hwnd: 0,
        message: WM_RBUTTONDOWN,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 100,
        pt_y: 200,
    };

    let target_vid = 12345;
    let result = dispatcher.handle_message(&msg, Some(target_vid), &mut target_manager);

    assert_eq!(result, Some(()));
    assert!(!dispatcher.is_rotating_camera);
    assert_eq!(target_manager.selected_target_vid, Some(target_vid));
}

#[test]
fn test_update_rotation_via_mouse_move() {
    let mut dispatcher = RButtonDispatcher::new();
    let mut target_manager = TargetManager::new();

    // Start rotation
    let msg_down = WinMsg {
        hwnd: 0,
        message: WM_RBUTTONDOWN,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 100,
        pt_y: 200,
    };
    dispatcher.handle_message(&msg_down, None, &mut target_manager);

    // Move mouse
    let msg_move = WinMsg {
        hwnd: 0,
        message: WM_MOUSEMOVE,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 150,
        pt_y: 180,
    };
    let result = dispatcher.handle_message(&msg_move, None, &mut target_manager);

    assert_eq!(result, Some(()));
    assert_eq!(dispatcher.camera_yaw, 50.0);
    assert_eq!(dispatcher.camera_pitch, 20.0); // 180 - 200 = -20, inverted -> +20
    assert_eq!(dispatcher.last_mouse_x, 150);
    assert_eq!(dispatcher.last_mouse_y, 180);
}

#[test]
fn test_stop_rotation_via_mouse_up() {
    let mut dispatcher = RButtonDispatcher::new();
    let mut target_manager = TargetManager::new();

    // Start rotation
    dispatcher.is_rotating_camera = true;

    // Stop rotation
    let msg_up = WinMsg {
        hwnd: 0,
        message: WM_RBUTTONUP,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 150,
        pt_y: 180,
    };
    let result = dispatcher.handle_message(&msg_up, None, &mut target_manager);

    assert_eq!(result, Some(()));
    assert!(!dispatcher.is_rotating_camera);
}

#[test]
fn test_unhandled_messages() {
    let mut dispatcher = RButtonDispatcher::new();
    let mut target_manager = TargetManager::new();

    // Ignore RBUTTONUP if not rotating
    let msg_up = WinMsg {
        hwnd: 0,
        message: WM_RBUTTONUP,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 100,
        pt_y: 200,
    };
    let result = dispatcher.handle_message(&msg_up, None, &mut target_manager);
    assert_eq!(result, None);

    // Ignore MOUSEMOVE if not rotating
    let msg_move = WinMsg {
        hwnd: 0,
        message: WM_MOUSEMOVE,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 150,
        pt_y: 180,
    };
    let result2 = dispatcher.handle_message(&msg_move, None, &mut target_manager);
    assert_eq!(result2, None);

    // Ignore completely random message
    let msg_random = WinMsg {
        hwnd: 0,
        message: 0x9999,
        wparam: 0,
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    let result3 = dispatcher.handle_message(&msg_random, None, &mut target_manager);
    assert_eq!(result3, None);
}
