#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemSize {
    Size1x1 = 1,
    Size1x2 = 2,
    Size1x3 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
}

#[derive(Debug, Clone)]
pub struct InventoryGrid {
    slots: [bool; 180],
    items: [Option<ItemData>; 180],
}

impl Default for InventoryGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl InventoryGrid {
    pub const TOTAL_SLOTS: u16 = 180;
    pub const PAGE_SLOTS: u16 = 45; // 5 kolumn * 9 wierszy = 45 slotow na strone (4 strony = 180)
    pub const COLUMNS: u16 = 5;
    pub const ROWS_PER_PAGE: u16 = 9; // W oknie ekwipunku strona ma dokladnie 9 wierszy
    pub const PAGE_COUNT: u16 = 4;

    pub fn new() -> Self {
        const INIT_ITEM: Option<ItemData> = None;
        Self {
            slots: [false; 180],
            items: [INIT_ITEM; 180],
        }
    }

    pub fn set_item(&mut self, cell: u16, item: ItemData) -> bool {
        if cell >= Self::TOTAL_SLOTS {
            return false;
        }
        self.items[cell as usize] = Some(item);
        true
    }

    pub fn get_item(&self, cell: u16) -> Option<&ItemData> {
        if cell >= Self::TOTAL_SLOTS {
            return None;
        }
        self.items[cell as usize].as_ref()
    }

    pub fn del_item(&mut self, cell: u16) -> Option<ItemData> {
        if cell >= Self::TOTAL_SLOTS {
            return None;
        }
        self.items[cell as usize].take()
    }

    pub fn swap_items(&mut self, cell_a: u16, cell_b: u16) -> bool {
        if cell_a >= Self::TOTAL_SLOTS || cell_b >= Self::TOTAL_SLOTS {
            return false;
        }
        self.items.swap(cell_a as usize, cell_b as usize);
        true
    }

    pub fn split_stack(&mut self, cell_from: u16, cell_to: u16, count: u8) -> bool {
        if cell_from >= Self::TOTAL_SLOTS || cell_to >= Self::TOTAL_SLOTS || count == 0 {
            return false;
        }
        if cell_from == cell_to {
            return false;
        }
        if self.items[cell_to as usize].is_some() {
            return false;
        }

        if let Some(ref mut item_from) = self.items[cell_from as usize] {
            if item_from.count <= count {
                return false;
            }
            item_from.count -= count;

            let mut new_item = item_from.clone();
            new_item.count = count;
            self.items[cell_to as usize] = Some(new_item);
            true
        } else {
            false
        }
    }

    pub fn find_empty_slot(&self) -> Option<u16> {
        self.find_free_slot(1)
    }

    /// Sprawdza, czy mozna umiescic przedmiot w danym slocie
    pub fn can_put_item(&self, slot: u16, size: u8) -> bool {
        if slot >= Self::TOTAL_SLOTS {
            return false;
        }

        if size < 1 || size > 3 {
            return false;
        }

        let size_u16 = size as u16;

        // Okreslamy strone (0..3)
        let _page = slot / Self::PAGE_SLOTS;
        // Pozycja slotu wewnatrz aktualnej strony (0..44)
        let slot_in_page = slot % Self::PAGE_SLOTS;
        // Wiersz wewnatrz aktualnej strony (0..8)
        let row = slot_in_page / Self::COLUMNS;

        // Przedmiot nie moze wychodzic poza dolna krawedz aktualnej strony (max 9 wierszy)
        if row + size_u16 > Self::ROWS_PER_PAGE {
            return false;
        }

        // Sprawdzanie kolizji ze wszystkimi fragmentami przedmiotu w pionie
        for i in 0..size_u16 {
            let check_slot = slot + i * Self::COLUMNS;
            
            if check_slot >= Self::TOTAL_SLOTS {
                return false;
            }

            if self.slots[check_slot as usize] {
                return false;
            }
        }

        true
    }

    /// Wklada przedmiot do inwentarza zajmujac odpowiednie kafelki
    pub fn put_item(&mut self, slot: u16, size: u8) -> Result<(), &'static str> {
        if !self.can_put_item(slot, size) {
            return Err("Kolizja lub brak miejsca w inwentarzu.");
        }

        let size_u16 = size as u16;
        for i in 0..size_u16 {
            let s = slot + i * Self::COLUMNS;
            self.slots[s as usize] = true;
        }

        self.set_item(slot, ItemData {
            vnum: 0,
            count: 1,
            flags: 0,
            anti_flags: 0,
        });

        Ok(())
    }

    /// Usuwa przedmiot z inwentarza zwalniajac odpowiednie kafelki
    pub fn remove_item(&mut self, slot: u16, size: u8) {
        let size_u16 = size as u16;
        for i in 0..size_u16 {
            let s = slot + i * Self::COLUMNS;
            if s < Self::TOTAL_SLOTS {
                self.slots[s as usize] = false;
            }
        }
        self.del_item(slot);
    }

    /// Wyszukuje pierwszy wolny slot, w ktorym zmiesci sie przedmiot
    pub fn find_free_slot(&self, size: u8) -> Option<u16> {
        for slot in 0..Self::TOTAL_SLOTS {
            if self.can_put_item(slot, size) {
                return Some(slot);
            }
        }
        None
    }
}

static INVENTORY_FFI: std::sync::OnceLock<std::sync::Mutex<InventoryGrid>> = std::sync::OnceLock::new();

fn get_ffi_inventory() -> &'static std::sync::Mutex<InventoryGrid> {
    INVENTORY_FFI.get_or_init(|| std::sync::Mutex::new(InventoryGrid::new()))
}

fn safe_ffi_call<F, R>(default_val: R, f: F) -> R
where
    F: FnOnce() -> R,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(res) => res,
        Err(_) => default_val,
    }
}

#[no_mangle]
pub extern "C" fn m2_inv_set_item(cell: u16, vnum: u32, count: u8, flags: u32, anti_flags: u32) -> bool {
    safe_ffi_call(false, || {
        let state_mutex = get_ffi_inventory();
        if let Ok(mut state) = state_mutex.lock() {
            state.set_item(cell, ItemData { vnum, count, flags, anti_flags })
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_inv_del_item(cell: u16) -> bool {
    safe_ffi_call(false, || {
        let state_mutex = get_ffi_inventory();
        if let Ok(mut state) = state_mutex.lock() {
            state.del_item(cell).is_some()
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_inv_swap(cell_a: u16, cell_b: u16) -> bool {
    safe_ffi_call(false, || {
        let state_mutex = get_ffi_inventory();
        if let Ok(mut state) = state_mutex.lock() {
            state.swap_items(cell_a, cell_b)
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_inv_find_empty() -> i32 {
    safe_ffi_call(-1, || {
        let state_mutex = get_ffi_inventory();
        if let Ok(state) = state_mutex.lock() {
            if let Some(slot) = state.find_empty_slot() {
                slot as i32
            } else {
                -1
            }
        } else {
            -1
        }
    })
}
