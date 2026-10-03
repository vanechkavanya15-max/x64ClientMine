
use m2_client::full_input_win32_10;

use full_input_win32_10::win32_cursor_manager::{CursorController, CursorType};
use full_input_win32_10::{MouseCursorVisualManager, MouseTarget};

#[test]
fn test_mouse_target_ground_sets_normal_cursor() {
    let mut controller = CursorController::new();
    let manager = MouseCursorVisualManager::new();

    manager.update_cursor(MouseTarget::Ground, &mut controller);
    assert_eq!(controller.get_current_cursor(), CursorType::Normal);
}

#[test]
fn test_mouse_target_enemy_sets_attack_cursor() {
    let mut controller = CursorController::new();
    let manager = MouseCursorVisualManager::new();

    manager.update_cursor(MouseTarget::Enemy, &mut controller);
    assert_eq!(controller.get_current_cursor(), CursorType::Attack);
}

#[test]
fn test_mouse_target_npc_sets_talk_cursor() {
    let mut controller = CursorController::new();
    let manager = MouseCursorVisualManager::new();

    manager.update_cursor(MouseTarget::Npc, &mut controller);
    assert_eq!(controller.get_current_cursor(), CursorType::Talk);
}

#[test]
fn test_mouse_target_item_sets_pick_cursor() {
    let mut controller = CursorController::new();
    let manager = MouseCursorVisualManager::new();

    manager.update_cursor(MouseTarget::Item, &mut controller);
    assert_eq!(controller.get_current_cursor(), CursorType::Pick);
}

#[test]
fn test_mouse_target_blocked_sets_cant_go_cursor() {
    let mut controller = CursorController::new();
    let manager = MouseCursorVisualManager::new();

    manager.update_cursor(MouseTarget::Blocked, &mut controller);
    assert_eq!(controller.get_current_cursor(), CursorType::CantGo);
}

#[test]
fn test_mouse_target_transitions() {
    let mut controller = CursorController::new();
    let manager = MouseCursorVisualManager::new();

    // Start with Enemy
    manager.update_cursor(MouseTarget::Enemy, &mut controller);
    assert_eq!(controller.get_current_cursor(), CursorType::Attack);

    // Transition to Ground
    manager.update_cursor(MouseTarget::Ground, &mut controller);
    assert_eq!(controller.get_current_cursor(), CursorType::Normal);

    // Transition to Npc
    manager.update_cursor(MouseTarget::Npc, &mut controller);
    assert_eq!(controller.get_current_cursor(), CursorType::Talk);
}
