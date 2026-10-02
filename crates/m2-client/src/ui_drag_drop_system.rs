use std::sync::{Mutex, OnceLock};

use crate::send_item_storage_packets::{m2_net_send_item_move_v2, m2_net_send_item_drop_v2};
use crate::send_shop_exchange_v2::m2_send_exchange_item_add;
use crate::send_quickslot_quest_v2::SendQuickSlotAddPacket;
use crate::belt_inventory_gate_v2::m2_belt_can_store;
use crate::inventory_grid_v2::WindowType;
use crate::ui_window_quickslot::SLOT_TYPE_INVENTORY;
use crate::ui_window_mgr::get_window_manager;
use crate::wgpu_engine_context::Quad2D;

pub const DRAG_ITEM_ICON_TYPE_ITEM: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DragState {
    pub is_dragging: bool,
    pub dragged_window_type: u8,
    pub dragged_cell: u16,
    pub dragged_vnum: u32,
    pub dragged_count: u32,
    pub is_drop_dialog_open: bool,
}

pub struct DragDropSystem {
    pub state: DragState,
}

impl DragDropSystem {
    pub fn new() -> Self {
        Self {
            state: DragState::default(),
        }
    }

    pub fn on_drag_start(&mut self, window_type: u8, cell: u16, vnum: u32, count: u32, width: u8, height: u8) {
        self.state.is_dragging = true;
        self.state.dragged_window_type = window_type;
        self.state.dragged_cell = cell;
        self.state.dragged_vnum = vnum;
        self.state.dragged_count = count;
        
        let mut wm = get_window_manager();
        wm.attach_icon(DRAG_ITEM_ICON_TYPE_ITEM, vnum, cell as u32, width, height);
    }

    pub fn cancel_drag(&mut self) {
        self.state = DragState::default();
        let mut wm = get_window_manager();
        wm.deattach_icon();
    }

    pub fn handle_key_press(&mut self, key_code: u32) -> bool {
        // VK_ESCAPE = 27
        if key_code == 27 && self.state.is_dragging {
            self.cancel_drag();
            return true;
        }
        false
    }
    
    pub fn handle_mouse_click(&mut self, button: u8) -> bool {
        // 1 = Right click
        if button == 1 && self.state.is_dragging {
            self.cancel_drag();
            return true;
        }
        false
    }

    pub fn on_drop_on_inventory(&mut self, target_window_type: u8, target_cell: u16, out_buf: *mut u8) -> usize {
        if !self.state.is_dragging {
            return 0;
        }

        let written = unsafe {
            m2_net_send_item_move_v2(
                self.state.dragged_window_type,
                self.state.dragged_cell,
                target_window_type,
                target_cell,
                self.state.dragged_count as u8,
                out_buf,
            )
        };

        self.cancel_drag();
        written
    }

    pub fn on_drop_on_belt(&mut self, target_cell: u16, item_type: u8, belt_level: u8, out_buf: *mut u8) -> usize {
        if !self.state.is_dragging {
            return 0;
        }

        if !m2_belt_can_store(belt_level, target_cell as u8, item_type) {
            self.cancel_drag();
            return 0;
        }

        let target_window = WindowType::BELT_INVENTORY as u8;
        let written = unsafe {
            m2_net_send_item_move_v2(
                self.state.dragged_window_type,
                self.state.dragged_cell,
                target_window,
                target_cell,
                self.state.dragged_count as u8,
                out_buf,
            )
        };

        self.cancel_drag();
        written
    }

    pub fn on_drop_on_exchange(&mut self, display_pos: u8, out_buf: *mut u8) -> usize {
        if !self.state.is_dragging {
            return 0;
        }

        let written = unsafe {
            m2_send_exchange_item_add(
                self.state.dragged_window_type,
                self.state.dragged_cell,
                display_pos,
                out_buf,
            )
        };

        self.cancel_drag();
        written
    }

    pub fn on_drop_on_quickslot(&mut self, slot_position: u8) -> bool {
        if !self.state.is_dragging {
            return false;
        }

        let success = SendQuickSlotAddPacket(
            slot_position,
            SLOT_TYPE_INVENTORY,
            self.state.dragged_cell as u8,
        );

        self.cancel_drag();
        success
    }

    pub fn try_drop_on_ground(&mut self) -> bool {
        if !self.state.is_dragging {
            return false;
        }
        
        self.state.is_drop_dialog_open = true;
        let mut wm = get_window_manager();
        wm.deattach_icon(); // Hide icon while dialog is open
        
        true
    }

    pub fn confirm_drop_on_ground(&mut self, gold: u32, out_buf: *mut u8) -> usize {
        if !self.state.is_dragging || !self.state.is_drop_dialog_open {
            return 0;
        }

        let written = unsafe {
            m2_net_send_item_drop_v2(
                self.state.dragged_window_type,
                self.state.dragged_cell,
                gold,
                self.state.dragged_count as u8,
                out_buf,
            )
        };

        self.cancel_drag();
        written
    }

    /// Appends the quad representing the dragged item to the rendering batches
    /// with 0.7 alpha if currently dragging.
    pub fn render_dragged_item(&self, mouse_x: f32, mouse_y: f32) -> Option<Quad2D> {
        let wm = get_window_manager();
        if self.state.is_dragging && wm.is_attaching && !self.state.is_drop_dialog_open {
            let (w, h) = wm.get_attaching_icon_size();
            let mut quad = Quad2D::new(mouse_x, mouse_y, (w as f32) * 32.0, (h as f32) * 32.0); // usually 32x32 per grid slot
            quad.color = [1.0, 1.0, 1.0, 0.7]; // Top layer alpha 0.7
            return Some(quad);
        }
        None
    }
}

static DRAG_DROP_SYSTEM: OnceLock<Mutex<DragDropSystem>> = OnceLock::new();

pub fn get_drag_drop_system() -> &'static Mutex<DragDropSystem> {
    DRAG_DROP_SYSTEM.get_or_init(|| Mutex::new(DragDropSystem::new()))
}
