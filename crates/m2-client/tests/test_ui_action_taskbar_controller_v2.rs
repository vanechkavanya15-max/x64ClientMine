#[path = "../src/ui_action_taskbar_controller_v2.rs"]
mod ui_action_taskbar_controller_v2;

use ui_action_taskbar_controller_v2::{TaskbarController, TaskbarAction, DIK_F1, DIK_F2, DIK_F3, DIK_F4};

#[path = "../src/py_bridge_app.rs"]
mod py_bridge_app;

#[test]
fn test_taskbar_quickslots() {
    let mut controller = TaskbarController::new();
    
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_1, false, false), TaskbarAction::UseQuickSlot(0));
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_2, false, false), TaskbarAction::UseQuickSlot(1));
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_3, false, false), TaskbarAction::UseQuickSlot(2));
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_4, false, false), TaskbarAction::UseQuickSlot(3));
    
    assert_eq!(controller.handle_key_down(DIK_F1, false, false), TaskbarAction::UseFunctionSlot(0));
    assert_eq!(controller.handle_key_down(DIK_F2, false, false), TaskbarAction::UseFunctionSlot(1));
    assert_eq!(controller.handle_key_down(DIK_F3, false, false), TaskbarAction::UseFunctionSlot(2));
    assert_eq!(controller.handle_key_down(DIK_F4, false, false), TaskbarAction::UseFunctionSlot(3));
}

#[test]
fn test_taskbar_change_page() {
    let mut controller = TaskbarController::new();
    
    assert_eq!(controller.active_page, 0);
    
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_2, true, false), TaskbarAction::ChangePage(1));
    assert_eq!(controller.active_page, 1);
    
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_4, true, false), TaskbarAction::ChangePage(3));
    assert_eq!(controller.active_page, 3);
}

#[test]
fn test_taskbar_open_windows() {
    let mut controller = TaskbarController::new();
    
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_I, false, false), TaskbarAction::OpenInventory);
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_C, false, false), TaskbarAction::OpenCharacter);
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_V, false, false), TaskbarAction::OpenSkills);
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_N, false, false), TaskbarAction::OpenQuests);
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_ESCAPE, false, false), TaskbarAction::OpenOptions);
}

#[test]
fn test_taskbar_alt_modifier_ignored() {
    let mut controller = TaskbarController::new();
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_I, false, true), TaskbarAction::None);
    assert_eq!(controller.handle_key_down(py_bridge_app::DIK_1, false, true), TaskbarAction::None);
}
