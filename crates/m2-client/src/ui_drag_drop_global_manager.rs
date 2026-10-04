//! UI Drag and Drop Global Manager Module
//!
//! Globalny zarzadca przeciagania i upuszczania elementow UI (Drag & Drop):
//! Obsluga przeciagania przedmiotow (AttachedItem) oraz umiejetnosci (AttachedSkill),
//! sledzenie wspolrzednych kursora myszy, renderowanie ikony pod kursorem
//! oraz obsluga celow upuszczenia (Slot, QuickSlot, Ground, NPC).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotWindowType {
    Inventory,
    Equipment,
    QuickSlot,
    SafeBox,
    Exchange,
    Mall,
    DragonSoul,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraggedType {
    None,
    Item { vnum: u32, count: u8, source_slot: u32, source_window: SlotWindowType },
    Skill { skill_id: u32, skill_level: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropTarget {
    Slot { window: SlotWindowType, slot_index: u32 },
    QuickSlot { index: u32 },
    Ground,
    NpcShop { vid: u32 },
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragCursorPosition {
    pub x: i32,
    pub y: i32,
}

/// Globalny menedzer przeciagania elementow interfejsu.
#[derive(Debug, Clone)]
pub struct UiDragDropGlobalManager {
    current_drag: DraggedType,
    cursor_pos: DragCursorPosition,
    pub icon_alpha: f32,
}

impl Default for UiDragDropGlobalManager {
    fn default() -> Self {
        Self {
            current_drag: DraggedType::None,
            cursor_pos: DragCursorPosition { x: 0, y: 0 },
            icon_alpha: 0.7,
        }
    }
}

impl UiDragDropGlobalManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_dragging(&self) -> bool {
        self.current_drag != DraggedType::None
    }

    pub fn get_dragged_type(&self) -> DraggedType {
        self.current_drag
    }

    pub fn get_cursor_position(&self) -> DragCursorPosition {
        self.cursor_pos
    }

    pub fn attach_item(&mut self, vnum: u32, count: u8, source_window: SlotWindowType, source_slot: u32) {
        self.current_drag = DraggedType::Item {
            vnum,
            count,
            source_slot,
            source_window,
        };
    }

    pub fn attach_skill(&mut self, skill_id: u32, skill_level: u32) {
        self.current_drag = DraggedType::Skill {
            skill_id,
            skill_level,
        };
    }

    pub fn update_cursor(&mut self, x: i32, y: i32) {
        self.cursor_pos = DragCursorPosition { x, y };
    }

    pub fn cancel_drag(&mut self) {
        self.current_drag = DraggedType::None;
    }

    /// Przetwarza upuszczenie na docelowy element (Drop).
    pub fn execute_drop(&mut self, target: DropTarget) -> Option<DraggedType> {
        if !self.is_dragging() {
            return None;
        }

        let dragged = self.current_drag;
        self.current_drag = DraggedType::None;

        match target {
            DropTarget::Cancel => None,
            _ => Some(dragged),
        }
    }
}
