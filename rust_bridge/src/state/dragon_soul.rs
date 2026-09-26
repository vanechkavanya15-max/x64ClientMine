use crate::ffi_guard::safe_ffi_call;
use crate::state::inventory_item::InventoryItem;
use crate::state::game_state::GLOBAL_GAME_STATE;
use std::ptr;

pub const DRAGON_SOUL_MGR: u8 = 0;

#[derive(Debug, Clone, PartialEq)]
pub struct DragonSoulManager {
    pub active_deck: u8,
    deck_0: [Option<InventoryItem>; 6],
    deck_1: [Option<InventoryItem>; 6],
}

impl DragonSoulManager {
    pub fn new() -> Self {
        const NONE: Option<InventoryItem> = None;
        Self {
            active_deck: 0,
            deck_0: [NONE; 6],
            deck_1: [NONE; 6],
        }
    }

    pub fn set_ds_item(&mut self, deck: u8, slot: u8, item: InventoryItem) -> Result<(), &'static str> {
        if deck > 1 {
            return Err("Niepoprawny deck alchemii");
        }
        if slot >= 6 {
            return Err("Indeks slotu poza zakresem");
        }

        match deck {
            0 => self.deck_0[slot as usize] = Some(item),
            1 => self.deck_1[slot as usize] = Some(item),
            _ => unreachable!(),
        }
        Ok(())
    }

    pub fn get_ds_item(&self, deck: u8, slot: u8) -> Option<&InventoryItem> {
        if deck > 1 || slot >= 6 {
            return None;
        }
        match deck {
            0 => self.deck_0[slot as usize].as_ref(),
            1 => self.deck_1[slot as usize].as_ref(),
            _ => None,
        }
    }

    pub fn remove_ds_item(&mut self, deck: u8, slot: u8) -> Option<InventoryItem> {
        if deck > 1 || slot >= 6 {
            return None;
        }
        match deck {
            0 => self.deck_0[slot as usize].take(),
            1 => self.deck_1[slot as usize].take(),
            _ => None,
        }
    }

    pub fn switch_deck(&mut self, deck: u8) -> Result<(), &'static str> {
        if deck > 1 {
            return Err("Niepoprawny deck alchemii");
        }
        self.active_deck = deck;
        Ok(())
    }
}

// ============================================================================
// C FFI Exports - Dragon Soul Manager
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_ds_manager_new() -> *mut DragonSoulManager {
    safe_ffi_call(ptr::null_mut(), || {
        let mgr = Box::new(DragonSoulManager::new());
        Box::into_raw(mgr)
    })
}

#[no_mangle]
pub extern "C" fn m2_ds_manager_free(ptr: *mut DragonSoulManager) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(ptr);
            }
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn m2_ds_get_vnum(deck: u8, slot: u8) -> u32 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        if let Some(&vnum) = state.player.dragon_soul.get(&(deck, slot)) {
            return vnum;
        }
        state.get_item_vnum(11 + deck, slot as u16)
    })
}
