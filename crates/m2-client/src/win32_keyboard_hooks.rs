use std::time::{SystemTime, UNIX_EPOCH};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Attack,
    QuickSlot1,
    QuickSlot2,
    QuickSlot3,
    QuickSlot4,
    QuickSlotF1,
    QuickSlotF2,
    QuickSlotF3,
    QuickSlotF4,
    PickupDrop1,
    PickupDrop2,
    OpenChat,
    GameMenu,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionBinding {
    pub action_type: ActionType,
    pub primary_key: u32,
    pub modifier: Option<u32>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub key_code: u32,
    pub is_down: bool,
    pub is_repeat: bool,
    pub timestamp_ms: u64,
}

impl KeyEvent {
    pub fn new(key_code: u32, is_down: bool, is_repeat: bool, timestamp_ms: u64) -> Self {
        Self {
            key_code,
            is_down,
            is_repeat,
            timestamp_ms,
        }
    }
}

pub struct KeyStateTable {
    states: [bool; 256],
}

impl KeyStateTable {
    pub fn new() -> Self {
        Self {
            states: [false; 256],
        }
    }

    pub fn is_pressed(&self, key_code: u32) -> bool {
        if key_code < 256 {
            self.states[key_code as usize]
        } else {
            false
        }
    }

    pub fn on_key_down(&mut self, key_code: u32, timestamp_ms: u64) -> Option<KeyEvent> {
        if key_code >= 256 {
            return None;
        }

        let is_repeat = self.states[key_code as usize];
        self.states[key_code as usize] = true;

        Some(KeyEvent::new(key_code, true, is_repeat, timestamp_ms))
    }

    pub fn on_key_up(&mut self, key_code: u32, timestamp_ms: u64) -> Option<KeyEvent> {
        if key_code >= 256 {
            return None;
        }

        self.states[key_code as usize] = false;

        Some(KeyEvent::new(key_code, false, false, timestamp_ms))
    }

    pub fn on_kill_focus(&mut self) {
        self.states.fill(false);
    }
}

impl Default for KeyStateTable {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_default_action_bindings() -> Vec<ActionBinding> {
    vec![
        // WASD
        ActionBinding { action_type: ActionType::MoveForward, primary_key: 0x57, modifier: None }, // W
        ActionBinding { action_type: ActionType::MoveBackward, primary_key: 0x53, modifier: None }, // S
        ActionBinding { action_type: ActionType::MoveLeft, primary_key: 0x41, modifier: None }, // A
        ActionBinding { action_type: ActionType::MoveRight, primary_key: 0x44, modifier: None }, // D

        // Arrows
        ActionBinding { action_type: ActionType::MoveForward, primary_key: 0x26, modifier: None }, // Up
        ActionBinding { action_type: ActionType::MoveBackward, primary_key: 0x28, modifier: None }, // Down
        ActionBinding { action_type: ActionType::MoveLeft, primary_key: 0x25, modifier: None }, // Left
        ActionBinding { action_type: ActionType::MoveRight, primary_key: 0x27, modifier: None }, // Right

        // Combat / General
        ActionBinding { action_type: ActionType::Attack, primary_key: 0x20, modifier: None }, // Space
        ActionBinding { action_type: ActionType::OpenChat, primary_key: 0x0D, modifier: None }, // Enter
        ActionBinding { action_type: ActionType::GameMenu, primary_key: 0x1B, modifier: None }, // Escape

        // QuickSlots 1-4
        ActionBinding { action_type: ActionType::QuickSlot1, primary_key: 0x31, modifier: None }, // 1
        ActionBinding { action_type: ActionType::QuickSlot2, primary_key: 0x32, modifier: None }, // 2
        ActionBinding { action_type: ActionType::QuickSlot3, primary_key: 0x33, modifier: None }, // 3
        ActionBinding { action_type: ActionType::QuickSlot4, primary_key: 0x34, modifier: None }, // 4

        // QuickSlots F1-F4
        ActionBinding { action_type: ActionType::QuickSlotF1, primary_key: 0x70, modifier: None }, // F1
        ActionBinding { action_type: ActionType::QuickSlotF2, primary_key: 0x71, modifier: None }, // F2
        ActionBinding { action_type: ActionType::QuickSlotF3, primary_key: 0x72, modifier: None }, // F3
        ActionBinding { action_type: ActionType::QuickSlotF4, primary_key: 0x73, modifier: None }, // F4

        // Pickup
        ActionBinding { action_type: ActionType::PickupDrop1, primary_key: 0x5A, modifier: None }, // Z
        ActionBinding { action_type: ActionType::PickupDrop2, primary_key: 0xC0, modifier: None }, // `
    ]
}

pub fn get_current_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(std::time::Duration::ZERO)
        .as_millis() as u64
}
