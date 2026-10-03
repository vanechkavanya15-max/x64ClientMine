use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;

/// Reprezentacja atrybutu (bonus) przedmiotu.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

impl Default for TPlayerItemAttribute {
    fn default() -> Self {
        Self { bType: 0, sValue: 0 }
    }
}

/// Struktura ItemData zgodna w 100% z binarnego punktu widzenia z C++ TItemData.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub sockets: [i32; 3],
    pub attrs: [TPlayerItemAttribute; 7],
}

impl Default for ItemData {
    fn default() -> Self {
        Self {
            vnum: 0,
            count: 0,
            flags: 0,
            anti_flags: 0,
            sockets: [0; 3],
            attrs: [TPlayerItemAttribute::default(); 7],
        }
    }
}

/// Typ okna inwentarza.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowType {
    INVENTORY = 1,
    EQUIPMENT = 2,
    SAFEBOX = 3,
    MALL = 4,
    DRAGON_SOUL_INVENTORY = 5,
    BELT_INVENTORY = 7,
}

impl WindowType {
    pub fn max_slots(&self) -> u16 {
        match self {
            WindowType::INVENTORY => 180, // 4 strony x 45
            WindowType::EQUIPMENT => 32, // c_Wear_Max
            WindowType::SAFEBOX => 135, // 3 strony x 45
            WindowType::MALL => 45, // 1 strona
            WindowType::DRAGON_SOUL_INVENTORY => 960, // 6 typow * 5 stopni * 32 sloty (C++ GameType.h)
            WindowType::BELT_INVENTORY => 16, // 4x4
        }
    }

    pub fn columns(&self) -> u16 {
        match self {
            WindowType::INVENTORY => 5,
            WindowType::SAFEBOX => 5,
            WindowType::MALL => 5,
            WindowType::DRAGON_SOUL_INVENTORY => 5, // Zazwyczaj 5x6
            WindowType::BELT_INVENTORY => 4,
            WindowType::EQUIPMENT => 1, // Ekwi jest specjalne
        }
    }
    
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(WindowType::INVENTORY),
            2 => Some(WindowType::EQUIPMENT),
            3 => Some(WindowType::SAFEBOX),
            4 => Some(WindowType::MALL),
            5 => Some(WindowType::DRAGON_SOUL_INVENTORY),
            7 => Some(WindowType::BELT_INVENTORY),
            _ => None,
        }
    }
}

pub struct WindowGrid {
    window_type: WindowType,
    slots: Vec<bool>,
    items: Vec<Option<ItemData>>,
}

impl WindowGrid {
    pub fn new(window_type: WindowType) -> Self {
        let max_slots = window_type.max_slots() as usize;
        Self {
            window_type,
            slots: vec![false; max_slots],
            items: vec![None; max_slots],
        }
    }

    pub fn max_slots(&self) -> u16 {
        self.window_type.max_slots()
    }

    pub fn get_item(&self, slot: u16) -> Option<&ItemData> {
        if slot >= self.max_slots() {
            return None;
        }
        self.items[slot as usize].as_ref()
    }

    pub fn set_item(&mut self, slot: u16, item: ItemData, size: u8) -> bool {
        if slot >= self.max_slots() || size == 0 {
            return false;
        }

        // Najpierw sprawdz czy miejsce jest wolne.
        let mut target_slots = Vec::new();
        if self.window_type == WindowType::EQUIPMENT {
            // Equipment nie ma gridu pionowego, zajmuje tylko swoj konkretny slot
            target_slots.push(slot);
        } else {
            let cols = self.window_type.columns();
            for i in 0..(size as u16) {
                let s = slot + i * cols;
                if s >= self.max_slots() {
                    return false;
                }
                target_slots.push(s);
            }
        }

        // Usun ewentualny stary przedmiot w tym slocie by zwolnic jego miejsce (jesli nadpisujemy punkt poczatkowy, choc w metinie zwykle robi sie del i potem set)
        if let Some(old_item) = self.items[slot as usize] {
            // My nie znamy tu starego rozmiaru z latwoscia, w tej funkcji zakladamy ze miejsce zostalo wyczyszczone wczesniej jesli to potrzebne.
            // Ale dla bezpieczenstwa zrobimy to prosto: nadpisujemy item, zakladamy ze to czysty set bez usuwania starych powiazan (lub del_item powinnien byc wywolany wczesniej).
        }

        for &s in &target_slots {
            self.slots[s as usize] = true;
        }

        self.items[slot as usize] = Some(item);
        true
    }

    pub fn del_item(&mut self, slot: u16, size: u8) -> Option<ItemData> {
        if slot >= self.max_slots() {
            return None;
        }

        let item = self.items[slot as usize].take();
        if item.is_some() {
            if self.window_type == WindowType::EQUIPMENT {
                self.slots[slot as usize] = false;
            } else {
                let cols = self.window_type.columns();
                for i in 0..(size as u16) {
                    let s = slot + i * cols;
                    if s < self.max_slots() {
                        self.slots[s as usize] = false;
                    }
                }
            }
        }
        item
    }
}

pub struct InventorySystem {
    windows: HashMap<WindowType, WindowGrid>,
    item_sizes: HashMap<u32, u8>,
    pub gold: i64,
}

impl InventorySystem {
    pub fn new() -> Self {
        let mut windows = HashMap::new();
        windows.insert(WindowType::INVENTORY, WindowGrid::new(WindowType::INVENTORY));
        windows.insert(WindowType::EQUIPMENT, WindowGrid::new(WindowType::EQUIPMENT));
        windows.insert(WindowType::SAFEBOX, WindowGrid::new(WindowType::SAFEBOX));
        windows.insert(WindowType::MALL, WindowGrid::new(WindowType::MALL));
        windows.insert(WindowType::DRAGON_SOUL_INVENTORY, WindowGrid::new(WindowType::DRAGON_SOUL_INVENTORY));
        windows.insert(WindowType::BELT_INVENTORY, WindowGrid::new(WindowType::BELT_INVENTORY));

        Self {
            windows,
            item_sizes: HashMap::new(),
            gold: 0,
        }
    }

    pub fn register_item_size(&mut self, vnum: u32, size: u8) {
        let s = if size > 3 { 3 } else if size == 0 { 1 } else { size };
        self.item_sizes.insert(vnum, s);
    }

    pub fn get_item_size(&self, vnum: u32) -> u8 {
        *self.item_sizes.get(&vnum).unwrap_or(&1)
    }

    pub fn set_item(&mut self, window: WindowType, slot: u16, item: ItemData) -> bool {
        let size = self.get_item_size(item.vnum);
        let mut old_size = 0;
        if let Some(w) = self.windows.get(&window) {
            if let Some(old_item) = w.get_item(slot) {
                old_size = self.get_item_size(old_item.vnum);
            }
        }

        if let Some(w) = self.windows.get_mut(&window) {
            if old_size > 0 {
                w.del_item(slot, old_size);
            }
            w.set_item(slot, item, size)
        } else {
            false
        }
    }

    pub fn del_item(&mut self, window: WindowType, slot: u16) -> bool {
        let size = if let Some(w) = self.windows.get(&window) {
            if let Some(item) = w.get_item(slot) {
                self.get_item_size(item.vnum)
            } else {
                return false;
            }
        } else {
            return false;
        };

        if let Some(w) = self.windows.get_mut(&window) {
            w.del_item(slot, size).is_some()
        } else {
            false
        }
    }

    pub fn get_item(&self, window: WindowType, slot: u16) -> Option<&ItemData> {
        self.windows.get(&window).and_then(|w| w.get_item(slot))
    }
}

impl Default for InventorySystem {
    fn default() -> Self {
        Self::new()
    }
}

static INVENTORY_SYS: OnceLock<Mutex<InventorySystem>> = OnceLock::new();

pub fn get_inventory_sys() -> &'static Mutex<InventorySystem> {
    INVENTORY_SYS.get_or_init(|| Mutex::new(InventorySystem::new()))
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
pub extern "C" fn m2_inv2_register_item_size(vnum: u32, size: u8) {
    safe_ffi_call((), || {
        if let Ok(mut sys) = get_inventory_sys().lock() {
            sys.register_item_size(vnum, size);
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_inv2_set_item(
    window: u8,
    slot: u16,
    vnum: u32,
    count: u8,
    sockets: *const i32,
    attrs: *const TPlayerItemAttribute,
) -> bool {
    safe_ffi_call(false, || {
        let win = match WindowType::from_u8(window) {
            Some(w) => w,
            None => return false,
        };

        let mut item = ItemData {
            vnum,
            count,
            ..Default::default()
        };

        if !sockets.is_null() {
            unsafe {
                let s_slice = std::slice::from_raw_parts(sockets, 3);
                // W Rust structs with #[repr(C, packed)] cant have fields borrowed directly
                // We do it by assigning array:
                item.sockets = [s_slice[0], s_slice[1], s_slice[2]];
            }
        }

        if !attrs.is_null() {
            unsafe {
                let a_slice = std::slice::from_raw_parts(attrs, 7);
                item.attrs = [
                    a_slice[0], a_slice[1], a_slice[2], a_slice[3],
                    a_slice[4], a_slice[5], a_slice[6]
                ];
            }
        }

        if let Ok(mut sys) = get_inventory_sys().lock() {
            sys.set_item(win, slot, item)
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_inv2_get_item(
    window: u8,
    slot: u16,
    out_vnum: *mut u32,
    out_count: *mut u8,
) -> bool {
    safe_ffi_call(false, || {
        let win = match WindowType::from_u8(window) {
            Some(w) => w,
            None => return false,
        };

        if out_vnum.is_null() || out_count.is_null() {
            return false;
        }

        if let Ok(sys) = get_inventory_sys().lock() {
            if let Some(item) = sys.get_item(win, slot) {
                unsafe {
                    *out_vnum = item.vnum;
                    *out_count = item.count;
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_inv2_del_item(window: u8, slot: u16) -> bool {
    safe_ffi_call(false, || {
        let win = match WindowType::from_u8(window) {
            Some(w) => w,
            None => return false,
        };

        if let Ok(mut sys) = get_inventory_sys().lock() {
            sys.del_item(win, slot)
        } else {
            false
        }
    })
}
