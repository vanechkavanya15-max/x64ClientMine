#[path = "../src/win32_keyboard_hooks.rs"]
mod win32_keyboard_hooks;

use win32_keyboard_hooks::{ActionType, KeyStateTable, get_default_action_bindings, get_current_time_ms};

#[test]
fn test_key_state_registration() {
    let mut table = KeyStateTable::new();
    let ts = get_current_time_ms();

    assert!(!table.is_pressed(0x57)); // W key

    let evt1 = table.on_key_down(0x57, ts).unwrap();
    assert!(evt1.is_down);
    assert!(!evt1.is_repeat);
    assert_eq!(evt1.key_code, 0x57);
    assert!(table.is_pressed(0x57));

    let evt2 = table.on_key_up(0x57, ts).unwrap();
    assert!(!evt2.is_down);
    assert!(!evt2.is_repeat);
    assert_eq!(evt2.key_code, 0x57);
    assert!(!table.is_pressed(0x57));
}

#[test]
fn test_event_queuing() {
    let mut table = KeyStateTable::new();
    let ts1 = 1000;
    
    // First press
    let evt1 = table.on_key_down(0x20, ts1).unwrap();
    assert!(!evt1.is_repeat);
    assert!(evt1.is_down);
    assert_eq!(evt1.key_code, 0x20);

    // Holding key down (repetition)
    let ts2 = 1050;
    let evt2 = table.on_key_down(0x20, ts2).unwrap();
    assert!(evt2.is_repeat);
    assert!(evt2.is_down);
    assert_eq!(evt2.key_code, 0x20);

    // Releasing key
    let ts3 = 1100;
    let evt3 = table.on_key_up(0x20, ts3).unwrap();
    assert!(!evt3.is_down);
    assert!(!evt3.is_repeat);
    assert_eq!(evt3.key_code, 0x20);

    // Pressing again
    let ts4 = 1150;
    let evt4 = table.on_key_down(0x20, ts4).unwrap();
    assert!(!evt4.is_repeat);
    assert!(evt4.is_down);
}

#[test]
fn test_on_kill_focus_cleanup() {
    let mut table = KeyStateTable::new();
    
    table.on_key_down(0x57, 100);
    table.on_key_down(0x41, 100);
    table.on_key_down(0x20, 100);

    assert!(table.is_pressed(0x57));
    assert!(table.is_pressed(0x41));
    assert!(table.is_pressed(0x20));

    table.on_kill_focus();

    assert!(!table.is_pressed(0x57));
    assert!(!table.is_pressed(0x41));
    assert!(!table.is_pressed(0x20));
}

#[test]
fn test_action_bindings_movement() {
    let bindings = get_default_action_bindings();

    let wasd_forward = bindings.iter().find(|b| b.action_type == ActionType::MoveForward && b.primary_key == 0x57);
    assert!(wasd_forward.is_some());

    let wasd_backward = bindings.iter().find(|b| b.action_type == ActionType::MoveBackward && b.primary_key == 0x53);
    assert!(wasd_backward.is_some());
    
    let wasd_left = bindings.iter().find(|b| b.action_type == ActionType::MoveLeft && b.primary_key == 0x41);
    assert!(wasd_left.is_some());

    let wasd_right = bindings.iter().find(|b| b.action_type == ActionType::MoveRight && b.primary_key == 0x44);
    assert!(wasd_right.is_some());

    let arrows_forward = bindings.iter().find(|b| b.action_type == ActionType::MoveForward && b.primary_key == 0x26);
    assert!(arrows_forward.is_some());

    let arrows_backward = bindings.iter().find(|b| b.action_type == ActionType::MoveBackward && b.primary_key == 0x28);
    assert!(arrows_backward.is_some());

    let arrows_left = bindings.iter().find(|b| b.action_type == ActionType::MoveLeft && b.primary_key == 0x25);
    assert!(arrows_left.is_some());

    let arrows_right = bindings.iter().find(|b| b.action_type == ActionType::MoveRight && b.primary_key == 0x27);
    assert!(arrows_right.is_some());
}

#[test]
fn test_action_bindings_combat_and_ui() {
    let bindings = get_default_action_bindings();

    let attack = bindings.iter().find(|b| b.action_type == ActionType::Attack && b.primary_key == 0x20); // Space
    assert!(attack.is_some());

    let chat = bindings.iter().find(|b| b.action_type == ActionType::OpenChat && b.primary_key == 0x0D); // Enter
    assert!(chat.is_some());

    let menu = bindings.iter().find(|b| b.action_type == ActionType::GameMenu && b.primary_key == 0x1B); // Escape
    assert!(menu.is_some());
    
    let pickup1 = bindings.iter().find(|b| b.action_type == ActionType::PickupDrop1 && b.primary_key == 0x5A); // Z
    assert!(pickup1.is_some());
    
    let pickup2 = bindings.iter().find(|b| b.action_type == ActionType::PickupDrop2 && b.primary_key == 0xC0); // `
    assert!(pickup2.is_some());
}

#[test]
fn test_action_bindings_quickslots() {
    let bindings = get_default_action_bindings();

    let q1 = bindings.iter().find(|b| b.action_type == ActionType::QuickSlot1 && b.primary_key == 0x31);
    assert!(q1.is_some());

    let q4 = bindings.iter().find(|b| b.action_type == ActionType::QuickSlot4 && b.primary_key == 0x34);
    assert!(q4.is_some());

    let qf1 = bindings.iter().find(|b| b.action_type == ActionType::QuickSlotF1 && b.primary_key == 0x70);
    assert!(qf1.is_some());

    let qf4 = bindings.iter().find(|b| b.action_type == ActionType::QuickSlotF4 && b.primary_key == 0x73);
    assert!(qf4.is_some());
}

#[test]
fn test_invalid_key_codes() {
    let mut table = KeyStateTable::new();
    let ts = get_current_time_ms();

    let evt = table.on_key_down(256, ts);
    assert!(evt.is_none());

    let evt2 = table.on_key_up(256, ts);
    assert!(evt2.is_none());

    assert!(!table.is_pressed(256));
}
