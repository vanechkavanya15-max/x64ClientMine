use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::panic::{catch_unwind, AssertUnwindSafe};

// Struktura przedmiotu w ekwipunku
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item {
    pub vnum: u32,
    pub count: u8,
}

// Stan ekwipunku i zlota
pub struct InventoryState {
    pub items: HashMap<(u8, u16), Item>,
    pub gold: i64,
}

impl InventoryState {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
            gold: 0,
        }
    }
}

// Globalny stan ekwipunku
static INVENTORY_STATE: OnceLock<Mutex<InventoryState>> = OnceLock::new();

pub fn get_inventory_state() -> &'static Mutex<InventoryState> {
    INVENTORY_STATE.get_or_init(|| Mutex::new(InventoryState::new()))
}

// Ochrona przed panika (FFI Guard)
fn safe_ffi_call<F, R>(default_val: R, f: F) -> R
where
    F: FnOnce() -> R,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(res) => res,
        Err(_) => default_val,
    }
}

/// Dodaje lub ustawia przedmiot w celu umozliwienia testowania logiki (wewnetrzna funkcja pomocnicza).
pub fn set_item_for_test(window: u8, slot: u16, vnum: u32, count: u8) {
    let state_mutex = get_inventory_state();
    if let Ok(mut state) = state_mutex.lock() {
        state.items.insert((window, slot), Item { vnum, count });
    }
}

/// Cysci ekwipunek i ustawia zloto (dla testow).
pub fn reset_inventory_for_test() {
    let state_mutex = get_inventory_state();
    if let Ok(mut state) = state_mutex.lock() {
        state.items.clear();
        state.gold = 0;
    }
}

/// Ustawia wartosc zlota (dla testow).
pub fn set_gold_for_test(gold: i64) {
    let state_mutex = get_inventory_state();
    if let Ok(mut state) = state_mutex.lock() {
        state.gold = gold;
    }
}

// ============================================================================
// Eksporty FFI
// ============================================================================

/// Pobiera informacje o przedmiocie w danym oknie i slocie.
#[no_mangle]
pub extern "C" fn m2_inv_get_item(window: u8, slot: u16, out_vnum: *mut u32, out_count: *mut u8) -> bool {
    safe_ffi_call(false, || {
        if out_vnum.is_null() || out_count.is_null() {
            return false;
        }

        let state_mutex = get_inventory_state();
        if let Ok(state) = state_mutex.lock() {
            if let Some(item) = state.items.get(&(window, slot)) {
                unsafe {
                    *out_vnum = item.vnum;
                    *out_count = item.count;
                }
                return true;
            }
        }
        false
    })
}

/// Przenosi przedmiot miedzy slotami ekwipunku.
#[no_mangle]
pub extern "C" fn m2_inv_move_item(src_window: u8, src_cell: u16, dst_window: u8, dst_cell: u16, count: u8) -> bool {
    safe_ffi_call(false, || {
        if src_window == dst_window && src_cell == dst_cell {
            return false;
        }

        if count == 0 {
            return false;
        }

        let state_mutex = get_inventory_state();
        if let Ok(mut state) = state_mutex.lock() {
            // Sprawdz czy przedmiot zrodlowy istnieje
            if let Some(src_item) = state.items.get(&(src_window, src_cell)).copied() {
                if src_item.count < count {
                    return false; // Proba przeniesienia wiecej niz posiadano
                }

                // Sprawdz co jest w docelowym
                if let Some(dst_item) = state.items.get_mut(&(dst_window, dst_cell)) {
                    if dst_item.vnum == src_item.vnum {
                        // Ten sam typ - dodaj (pomijamy limit staku dla uproszczenia w tym tescie)
                        let new_count = dst_item.count.saturating_add(count);
                        dst_item.count = new_count;
                        
                        // Zaktualizuj zrodlowy
                        if src_item.count == count {
                            state.items.remove(&(src_window, src_cell));
                        } else {
                            if let Some(item) = state.items.get_mut(&(src_window, src_cell)) {
                                item.count -= count;
                            }
                        }
                        return true;
                    } else {
                        // Inny przedmiot - swap jezeli calosc (nie implementujemy swapa czesciowego w tej iteracji)
                        if src_item.count == count {
                            let dst_item_copy = *dst_item;
                            *dst_item = src_item;
                            state.items.insert((src_window, src_cell), dst_item_copy);
                            return true;
                        } else {
                            return false;
                        }
                    }
                } else {
                    // Puste docelowe
                    state.items.insert((dst_window, dst_cell), Item {
                        vnum: src_item.vnum,
                        count,
                    });
                    
                    if src_item.count == count {
                        state.items.remove(&(src_window, src_cell));
                    } else {
                        if let Some(item) = state.items.get_mut(&(src_window, src_cell)) {
                            item.count -= count;
                        }
                    }
                    return true;
                }
            }
        }
        false
    })
}

/// Uzywa przedmiotu w danym slocie (dla uproszczenia tylko zwracamy prawde jesli istnieje).
#[no_mangle]
pub extern "C" fn m2_inv_use_item(window: u8, cell: u16) -> bool {
    safe_ffi_call(false, || {
        let state_mutex = get_inventory_state();
        if let Ok(mut state) = state_mutex.lock() {
            if let Some(item) = state.items.get_mut(&(window, cell)) {
                // Symulacja uzycia: zmniejszenie licznika o 1
                if item.count > 1 {
                    item.count -= 1;
                } else {
                    state.items.remove(&(window, cell));
                }
                return true;
            }
        }
        false
    })
}

/// Zwraca ilosc posiadanego zlota.
#[no_mangle]
pub extern "C" fn m2_inv_get_gold() -> i64 {
    safe_ffi_call(0, || {
        let state_mutex = get_inventory_state();
        if let Ok(state) = state_mutex.lock() {
            state.gold
        } else {
            0
        }
    })
}
