// Modul paska szybkiego wyboru (QuickSlot) dla Metin2 w Pure Rust.
// Implementacja w pelni zgodna z zachowaniem z C++ (Faza D).

use crate::send_quickslot_quest_v2::{SendQuickSlotAddPacket, SendQuickSlotDelPacket, SendQuickSlotMovePacket};

// Stale zgodnie z uiquickslot.py / PythonPlayer.cpp / GameType.h
pub const QUICKSLOT_MAX_LINE: usize = 4;
pub const QUICKSLOT_MAX_COUNT_PER_LINE: usize = 8;
pub const QUICKSLOT_MAX_NUM: usize = QUICKSLOT_MAX_LINE * QUICKSLOT_MAX_COUNT_PER_LINE; // 32, but in recv_quickslot_sync_v2 it's 36
pub const QUICKSLOT_MAX_NUM_SERVER: usize = 36; // 36 to the server

pub const SLOT_TYPE_NONE: u8 = 0;
pub const SLOT_TYPE_INVENTORY: u8 = 1;
pub const SLOT_TYPE_SKILL: u8 = 2;
pub const SLOT_TYPE_EMOTION: u8 = 3;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct TQuickSlot {
    pub slot_type: u8,
    pub slot_position: u32,
}

pub struct QuickSlotWindow {
    pub active_page: u8,
    pub slots: [TQuickSlot; QUICKSLOT_MAX_NUM_SERVER],
}

impl QuickSlotWindow {
    pub fn new() -> Self {
        Self {
            active_page: 0,
            slots: [TQuickSlot::default(); QUICKSLOT_MAX_NUM_SERVER],
        }
    }

    /// Przelacza aktywna strone paska szybkiego wyboru (od 0 do QUICKSLOT_MAX_LINE - 1).
    pub fn set_page(&mut self, page: i32) {
        if page < 0 {
            self.active_page = ((QUICKSLOT_MAX_LINE as i32 + page) % QUICKSLOT_MAX_LINE as i32) as u8;
        } else if page >= QUICKSLOT_MAX_LINE as i32 {
            self.active_page = (page % QUICKSLOT_MAX_LINE as i32) as u8;
        } else {
            self.active_page = page as u8;
        }
    }

    /// Zwraca globalny index slotu dla danej lokalnej pozycji (0..7) na podstawie aktywnej strony.
    pub fn get_global_index(&self, local_idx: u8) -> usize {
        (self.active_page as usize * QUICKSLOT_MAX_COUNT_PER_LINE) + local_idx as usize
    }

    /// Przypisuje element do paska szybkiego dostepu (wywoluje wyslanie pakietu).
    pub fn add_quickslot(&mut self, local_idx: u8, slot_type: u8, slot_position: u32) {
        if (local_idx as usize) >= QUICKSLOT_MAX_COUNT_PER_LINE {
            return;
        }
        
        let global_idx = self.get_global_index(local_idx);
        
        // Zapis do lokalnego stanu w oczekiwaniu na odpowiedz serwera / dla optymalizacji
        self.slots[global_idx] = TQuickSlot {
            slot_type,
            slot_position,
        };

        // Wyslij zadanie dodania do serwera
        SendQuickSlotAddPacket(global_idx as u8, slot_type, slot_position as u8);
    }

    /// Usuwa element z paska szybkiego dostepu.
    pub fn remove_quickslot(&mut self, global_idx: usize) {
        if global_idx >= QUICKSLOT_MAX_NUM_SERVER {
            return;
        }

        self.slots[global_idx] = TQuickSlot::default();

        SendQuickSlotDelPacket(global_idx as u8);
    }

    /// Zamienia dwa sloty miejscami w pasku.
    pub fn swap_quickslot(&mut self, src_global_idx: usize, dst_global_idx: usize) {
        if src_global_idx >= QUICKSLOT_MAX_NUM_SERVER || dst_global_idx >= QUICKSLOT_MAX_NUM_SERVER {
            return;
        }

        self.slots.swap(src_global_idx, dst_global_idx);

        SendQuickSlotMovePacket(src_global_idx as u8, dst_global_idx as u8);
    }

    /// Wykonuje akcje przypisana do danego lokalnego slota.
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

        // Zwracamy slot. Wlasciwe uzycie przedmiotu/skilla/emocji
        // powinno zostac wywolane przez system nasluchujacy okien np. wysylajac CPythonPlayer::UseItem(pos)
        Some(slot)
    }

    /// Obsluguje zdarzenie wcisniecia klawisza przypisanego do quickslota.
    /// Klawisze: '1', '2', '3', '4' -> local_idx: 0, 1, 2, 3
    /// Klawisze: 'F1', 'F2', 'F3', 'F4' -> local_idx: 4, 5, 6, 7
    pub fn on_key_pressed(&mut self, key_code: u32, is_shift_pressed: bool) -> Option<TQuickSlot> {
        match key_code {
            // Zakladamy tu jakies kody. Dla przykladu 0x31 to '1'. 
            // Jesli wcisniety shift - to jest to zmiana strony (Shift + 1 -> strona 0)
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
