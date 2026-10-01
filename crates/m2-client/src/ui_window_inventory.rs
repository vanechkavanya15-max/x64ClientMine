use std::sync::{Mutex, OnceLock};
use crate::ui_window_core::{Window, WindowType};
use crate::item_anti_flags::{PlayerClass, PlayerSex, can_equip_by_player, Item as AntiFlagItem};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EquipmentSlot {
    Armor = 0,
    Helmet = 1,
    Shoes = 2,
    Bracelet = 3,
    Weapon = 4,
    Necklace = 5,
    Earrings = 6,
    Shield = 10,
    Belt = 23,
}

pub const INVENTORY_PAGE_COUNT: u8 = 4;
pub const INVENTORY_PAGE_SLOT_COUNT: u16 = 45; // 5 cols x 9 rows

#[derive(Debug, Clone)]
pub struct UiWindowInventory {
    pub base: Window,
    pub active_page: u8,
    pub is_visible: bool,
}

impl Default for UiWindowInventory {
    fn default() -> Self {
        let mut base = Window::new(0, WindowType::ThinBoard);
        base.width = 176;
        base.height = 375;
        base.name = "InventoryWindow".to_string();
        base.is_show = false;

        Self {
            base,
            active_page: 0,
            is_visible: false,
        }
    }
}

impl UiWindowInventory {
    pub fn get_global_slot_index(&self, page: u8, local_index: u16) -> Option<u16> {
        if page >= INVENTORY_PAGE_COUNT || local_index >= INVENTORY_PAGE_SLOT_COUNT {
            return None;
        }
        Some((page as u16) * INVENTORY_PAGE_SLOT_COUNT + local_index)
    }

    pub fn format_yang(gold: i64) -> String {
        if gold == 0 {
            return "0 Yang".to_string();
        }

        let is_negative = gold < 0;
        let mut num = if gold == i64::MIN {
            (i64::MAX as u64) + 1
        } else {
            gold.abs() as u64
        };

        let mut parts = Vec::new();
        while num > 0 {
            parts.push(num % 1000);
            num /= 1000;
        }

        let mut result = String::new();
        if is_negative {
            result.push('-');
        }

        let last = parts.pop().unwrap(); // safe because gold != 0 -> num != 0 initially
        result.push_str(&last.to_string());

        while let Some(part) = parts.pop() {
            result.push_str(&format!(",{part:03}"));
        }

        format!("{result} Yang")
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
        self.base.is_show = self.is_visible;
    }

    pub fn set_active_page(&mut self, page: u8) {
        if page < INVENTORY_PAGE_COUNT {
            self.active_page = page;
        }
    }
    
    pub fn handle_key_press(&mut self, key: char) {
        if key == 'i' || key == 'I' {
            self.toggle_visibility();
        }
    }

    pub fn can_equip_item_to_player(
        &self,
        item: &AntiFlagItem,
        player_class: PlayerClass,
        player_sex: PlayerSex,
        player_level: u32,
    ) -> bool {
        can_equip_by_player(item, player_class, player_sex, player_level)
    }
}

static UI_WINDOW_INVENTORY: OnceLock<Mutex<UiWindowInventory>> = OnceLock::new();

pub fn get_ui_window_inventory() -> &'static Mutex<UiWindowInventory> {
    UI_WINDOW_INVENTORY.get_or_init(|| Mutex::new(UiWindowInventory::default()))
}
