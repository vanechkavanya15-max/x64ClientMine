pub use crate::win32_cursor_manager::*;
use crate::win32_cursor_manager::{CursorController, CursorType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseTarget {
    Ground,
    Enemy,
    Npc,
    Item,
    Blocked,
}

pub struct MouseCursorVisualManager;

impl MouseCursorVisualManager {
    pub fn new() -> Self {
        Self
    }

    pub fn update_cursor(&self, target: MouseTarget, controller: &mut CursorController) {
        let cursor_type = match target {
            MouseTarget::Ground => CursorType::Normal,
            MouseTarget::Enemy => CursorType::Attack,
            MouseTarget::Npc => CursorType::Talk,
            MouseTarget::Item => CursorType::Pick,
            MouseTarget::Blocked => CursorType::CantGo,
        };
        controller.set_cursor(cursor_type);
    }
}
