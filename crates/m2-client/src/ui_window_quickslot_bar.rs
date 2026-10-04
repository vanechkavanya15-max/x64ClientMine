// Modul paska szybkiego dostepu (QuickSlot Bar) dla Metin2 w Pure Rust.
// Zgodnosc z Faza D: obsluga 4 stron x 8 slotow, integracja z pakietami i wejsciem klawiatury.

use crate::send_quickslot_quest_v2::{SendQuickSlotAddPacket, SendQuickSlotDelPacket, SendQuickSlotMovePacket};

pub const QUICKSLOT_MAX_LINE: usize = 4;
pub const QUICKSLOT_MAX_COUNT_PER_LINE: usize = 8;
pub const QUICKSLOT_MAX_NUM: usize = QUICKSLOT_MAX_LINE * QUICKSLOT_MAX_COUNT_PER_LINE;
pub const QUICKSLOT_MAX_NUM_SERVER: usize = 36;

pub const SLOT_TYPE_NONE: u8 = 0;
pub const SLOT_TYPE_INVENTORY: u8 = 1;
pub const SLOT_TYPE_ITEM: u8 = 1; // Alias for inventory per the prompt specification
pub const SLOT_TYPE_SKILL: u8 = 2;
pub const SLOT_TYPE_EMOTION: u8 = 3;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct TQuickSlot {
    pub slot_type: u8,
    pub slot_position: u32,
}

pub struct QuickSlotBarManager {
    pub active_page: u8,
    pub slots: [TQuickSlot; QUICKSLOT_MAX_NUM_SERVER],
}

impl QuickSlotBarManager {
    pub fn new() -> Self {
        Self {
            active_page: 0,
            slots: [TQuickSlot::default(); QUICKSLOT_MAX_NUM_SERVER],
        }
    }

    pub fn set_page(&mut self, page: i32) {
        if page < 0 {
            self.active_page = ((QUICKSLOT_MAX_LINE as i32 + (page % QUICKSLOT_MAX_LINE as i32)) % QUICKSLOT_MAX_LINE as i32) as u8;
        } else {
            self.active_page = (page % QUICKSLOT_MAX_LINE as i32) as u8;
        }
    }

    pub fn get_global_index(&self, local_idx: u8) -> usize {
        (self.active_page as usize * QUICKSLOT_MAX_COUNT_PER_LINE) + local_idx as usize
    }

    pub fn add_quickslot(&mut self, local_idx: u8, slot_type: u8, slot_position: u32) {
        if (local_idx as usize) >= QUICKSLOT_MAX_COUNT_PER_LINE {
            return;
        }

        let global_idx = self.get_global_index(local_idx);
        
        self.slots[global_idx] = TQuickSlot {
            slot_type,
            slot_position,
        };

        SendQuickSlotAddPacket(global_idx as u8, slot_type, slot_position as u8);
    }

    pub fn remove_quickslot(&mut self, global_idx: usize) {
        if global_idx >= QUICKSLOT_MAX_NUM_SERVER {
            return;
        }

        self.slots[global_idx] = TQuickSlot::default();

        SendQuickSlotDelPacket(global_idx as u8);
    }

    pub fn swap_quickslot(&mut self, src_global_idx: usize, dst_global_idx: usize) {
        if src_global_idx >= QUICKSLOT_MAX_NUM_SERVER || dst_global_idx >= QUICKSLOT_MAX_NUM_SERVER {
            return;
        }

        self.slots.swap(src_global_idx, dst_global_idx);

        SendQuickSlotMovePacket(src_global_idx as u8, dst_global_idx as u8);
    }

    pub fn use_slot(&self, local_idx: u8) -> Option<TQuickSlot> {
        if (local_idx as usize) >= QUICKSLOT_MAX_COUNT_PER_LINE {
            return None;
        }
        let global_idx = self.get_global_index(local_idx);
        
        if global_idx >= QUICKSLOT_MAX_NUM_SERVER {
            return None;
        }

        let slot = self.slots[global_idx];
        
        if slot.slot_type == SLOT_TYPE_NONE {
            return None;
        }

        match slot.slot_type {
            SLOT_TYPE_INVENTORY => {
                let mut buf = Vec::with_capacity(5);
                buf.extend_from_slice(&0x0501u16.to_le_bytes()); // HEADER_CG_ITEM_USE
                buf.extend_from_slice(&5u16.to_le_bytes());
                buf.push(1); // window_type = INVENTORY
                buf.extend_from_slice(&(slot.slot_position as u16).to_le_bytes());
                m2_client_bridge::net::send_queue::enqueue_outgoing_packet(buf);
            }
            SLOT_TYPE_SKILL => {
                crate::send_combat::SendUseSkillPacket(slot.slot_position, 0);
            }
            SLOT_TYPE_EMOTION => {
                unsafe { crate::py_bridge_player::m2_py_player_act_emotion(slot.slot_position); }
            }
            _ => {}
        }

        Some(slot)
    }

    pub fn on_key_pressed(&mut self, key_code: u32, is_shift_pressed: bool) -> Option<TQuickSlot> {
        match key_code {
            0x31 /* 1 */ => if is_shift_pressed { self.set_page(0); None } else { self.use_slot(0) },
            0x32 /* 2 */ => if is_shift_pressed { self.set_page(1); None } else { self.use_slot(1) },
            0x33 /* 3 */ => if is_shift_pressed { self.set_page(2); None } else { self.use_slot(2) },
            0x34 /* 4 */ => if is_shift_pressed { self.set_page(3); None } else { self.use_slot(3) },
            0x70 /* F1 */ => self.use_slot(4),
            0x71 /* F2 */ => self.use_slot(5),
            0x72 /* F3 */ => self.use_slot(6),
            0x73 /* F4 */ => self.use_slot(7),
            _ => None,
        }
    }
}
