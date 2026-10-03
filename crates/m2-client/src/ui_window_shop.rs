// ui_window_shop.rs - Metin2 NPC Shop UI Window (Okno sklepu NPC 5x8)
use crate::ui_window_core::{WindowType, HORIZONTAL_ALIGN_CENTER};
use crate::ui_window_mgr::get_window_manager;

pub const SHOP_COLS: usize = 5;
pub const SHOP_ROWS: usize = 8;
pub const SHOP_SLOTS_COUNT: usize = SHOP_COLS * SHOP_ROWS; // 40 przedmiotow

/// Pojedynczy przedmiot wystawiony w sklepie NPC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShopItem {
    pub slot_idx: u8,
    pub vnum: u32,
    pub count: u8,
    pub price: u32,
}

/// Bledy transakcji handlowych w sklepie NPC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopError {
    NotEnoughGold,
    InventoryFull,
    InvalidSlot,
    EmptySlot,
    ShopClosed,
}

/// Okno sklepu NPC z siatka 5x8 (40 slotow), cenami Yang i mechanizmem kupowania
pub struct UiWindowShop {
    pub root_handle: u32,
    pub title_text: u32,
    pub slot_handles: [u32; SHOP_SLOTS_COUNT],
    pub info_text: u32,
    pub price_text: u32,
    pub buy_button: u32,
    pub close_button: u32,
    pub items: [Option<ShopItem>; SHOP_SLOTS_COUNT],
    pub selected_slot: Option<u8>,
    pub npc_vid: u32,
    pub is_visible: bool,
}

impl Default for UiWindowShop {
    fn default() -> Self {
        Self::new()
    }
}

impl UiWindowShop {
    pub const WINDOW_WIDTH: i32 = 200;
    pub const WINDOW_HEIGHT: i32 = 380;
    pub const SLOT_SIZE: i32 = 32;
    pub const GRID_START_X: i32 = 18;
    pub const GRID_START_Y: i32 = 36;
    pub const SLOT_SPACING: i32 = 33;

    /// Tworzy instancje okna sklepu i buduje strukture okien w WindowManagerze
    pub fn new() -> Self {
        let mut wm = get_window_manager();

        // 1. Glowne okno (Board z podwojna ramka)
        let root_handle = wm.register(WindowType::Board);
        wm.set_size(root_handle, Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT);
        wm.set_position(root_handle, 80, 100);
        wm.set_name(root_handle, "ShopWindow");
        wm.hide(root_handle);

        // 2. Tytul okna sklepu ("Sklep")
        let title_text = wm.register(WindowType::TextLine);
        wm.set_parent(title_text, root_handle);
        wm.set_position(title_text, 0, 10);
        wm.set_size(title_text, Self::WINDOW_WIDTH, 16);
        wm.set_align(title_text, HORIZONTAL_ALIGN_CENTER, 0);
        wm.set_font_color(title_text, [1.0, 0.85, 0.3, 1.0]); // Zlocisty
        wm.set_text(title_text, "Sklep");
        wm.set_name(title_text, "ShopTitle");

        // 3. Przycisk zamkniecia okna (x)
        let close_button = wm.register(WindowType::Button);
        wm.set_parent(close_button, root_handle);
        wm.set_position(close_button, Self::WINDOW_WIDTH - 20, 6);
        wm.set_size(close_button, 14, 14);
        wm.set_text(close_button, "x");
        wm.set_name(close_button, "ShopCloseBtn");

        // 4. Siatka slotow 5x8 (40 slotow Bar3D)
        let mut slot_handles = [0u32; SHOP_SLOTS_COUNT];
        for row in 0..SHOP_ROWS {
            for col in 0..SHOP_COLS {
                let idx = row * SHOP_COLS + col;
                let sx = Self::GRID_START_X + (col as i32) * Self::SLOT_SPACING;
                let sy = Self::GRID_START_Y + (row as i32) * Self::SLOT_SPACING;

                let s_handle = wm.register(WindowType::Bar3D);
                wm.set_parent(s_handle, root_handle);
                wm.set_position(s_handle, sx, sy);
                wm.set_size(s_handle, Self::SLOT_SIZE, Self::SLOT_SIZE);
                wm.set_color(s_handle, [0.08, 0.08, 0.1, 0.9]);
                wm.set_name(s_handle, &format!("ShopSlot_{}", idx));

                slot_handles[idx] = s_handle;
            }
        }

        // 5. Informacja o wybranym przedmiocie
        let info_y = Self::GRID_START_Y + (SHOP_ROWS as i32) * Self::SLOT_SPACING + 6;
        let info_text = wm.register(WindowType::TextLine);
        wm.set_parent(info_text, root_handle);
        wm.set_position(info_text, 14, info_y);
        wm.set_size(info_text, Self::WINDOW_WIDTH - 28, 14);
        wm.set_font_color(info_text, [0.85, 0.85, 0.85, 1.0]);
        wm.set_text(info_text, "Wybierz przedmiot");
        wm.set_name(info_text, "ShopItemInfo");

        // 6. Cena w Yang wybranego przedmiotu
        let price_text = wm.register(WindowType::TextLine);
        wm.set_parent(price_text, root_handle);
        wm.set_position(price_text, 14, info_y + 16);
        wm.set_size(price_text, Self::WINDOW_WIDTH - 28, 14);
        wm.set_font_color(price_text, [1.0, 0.85, 0.2, 1.0]);
        wm.set_text(price_text, "Cena: -");
        wm.set_name(price_text, "ShopPriceText");

        // 7. Przycisk zakupu ("Kup")
        let buy_button = wm.register(WindowType::Button);
        wm.set_parent(buy_button, root_handle);
        wm.set_position(buy_button, 14, info_y + 36);
        wm.set_size(buy_button, Self::WINDOW_WIDTH - 28, 22);
        wm.set_text(buy_button, "Kup przedmiot");
        wm.set_name(buy_button, "ShopBuyBtn");

        Self {
            root_handle,
            title_text,
            slot_handles,
            info_text,
            price_text,
            buy_button,
            close_button,
            items: [None; SHOP_SLOTS_COUNT],
            selected_slot: None,
            npc_vid: 0,
            is_visible: false,
        }
    }

    /// Otwiera okno sklepu dla danego NPC
    pub fn open(&mut self, npc_vid: u32, title: &str) {
        self.npc_vid = npc_vid;
        self.selected_slot = None;
        self.is_visible = true;

        let mut wm = get_window_manager();
        wm.set_text(self.title_text, title);
        wm.set_text(self.info_text, "Wybierz przedmiot");
        wm.set_text(self.price_text, "Cena: -");
        wm.show(self.root_handle);
    }

    /// Zamyka okno sklepu
    pub fn close(&mut self) {
        self.is_visible = false;
        self.selected_slot = None;
        let mut wm = get_window_manager();
        wm.hide(self.root_handle);
    }

    /// Umieszcza przedmiot w wybranym slocie sklepu
    pub fn set_item(&mut self, slot_idx: u8, vnum: u32, count: u8, price: u32) -> Result<(), ShopError> {
        let idx = slot_idx as usize;
        if idx >= SHOP_SLOTS_COUNT {
            return Err(ShopError::InvalidSlot);
        }

        self.items[idx] = Some(ShopItem {
            slot_idx,
            vnum,
            count: count.max(1),
            price,
        });

        // Wizualna aktualizacja slotu
        let mut wm = get_window_manager();
        let slot_h = self.slot_handles[idx];
        wm.set_color(slot_h, [0.15, 0.18, 0.25, 0.95]); // Podswietl zajety slot
        Ok(())
    }

    /// Czysci wszystkie przedmioty w sklepie
    pub fn clear_items(&mut self) {
        self.items = [None; SHOP_SLOTS_COUNT];
        self.selected_slot = None;

        let mut wm = get_window_manager();
        for &s_handle in &self.slot_handles {
            wm.set_color(s_handle, [0.08, 0.08, 0.1, 0.9]);
        }
        wm.set_text(self.info_text, "Wybierz przedmiot");
        wm.set_text(self.price_text, "Cena: -");
    }

    /// Pobiera informacje o przedmiocie w danym slocie
    pub fn get_item(&self, slot_idx: u8) -> Option<&ShopItem> {
        let idx = slot_idx as usize;
        if idx >= SHOP_SLOTS_COUNT {
            return None;
        }
        self.items[idx].as_ref()
    }

    /// Zaznacza slot w sklepie
    pub fn select_slot(&mut self, slot_idx: u8) -> Result<(), ShopError> {
        let idx = slot_idx as usize;
        if idx >= SHOP_SLOTS_COUNT {
            return Err(ShopError::InvalidSlot);
        }

        let item = self.items[idx].ok_or(ShopError::EmptySlot)?;
        self.selected_slot = Some(slot_idx);

        let mut wm = get_window_manager();
        // Zresetuj kolory innych slotow
        for (i, &s_h) in self.slot_handles.iter().enumerate() {
            if i == idx {
                wm.set_color(s_h, [0.25, 0.40, 0.65, 1.0]); // Zaznaczony slot
            } else if self.items[i].is_some() {
                wm.set_color(s_h, [0.15, 0.18, 0.25, 0.95]);
            } else {
                wm.set_color(s_h, [0.08, 0.08, 0.1, 0.9]);
            }
        }

        wm.set_text(self.info_text, &format!("VNUM: {} x{}", item.vnum, item.count));
        wm.set_text(self.price_text, &format!("Cena: {}", Self::format_yang(item.price)));
        Ok(())
    }

    /// Zwraca biezaco wybrany przedmiot
    pub fn selected_item(&self) -> Option<&ShopItem> {
        self.selected_slot.and_then(|idx| self.get_item(idx))
    }

    /// Kupuje aktualnie wybrany przedmiot, odejmujac Yang od gracza
    pub fn buy_selected(&mut self, player_yang: &mut u32) -> Result<ShopItem, ShopError> {
        let slot = self.selected_slot.ok_or(ShopError::EmptySlot)?;
        self.buy_item(slot, player_yang)
    }

    /// Kupuje przedmiot z podanego slotu
    pub fn buy_item(&mut self, slot_idx: u8, player_yang: &mut u32) -> Result<ShopItem, ShopError> {
        if !self.is_visible {
            return Err(ShopError::ShopClosed);
        }

        let idx = slot_idx as usize;
        if idx >= SHOP_SLOTS_COUNT {
            return Err(ShopError::InvalidSlot);
        }

        let item = self.items[idx].ok_or(ShopError::EmptySlot)?;

        if *player_yang < item.price {
            return Err(ShopError::NotEnoughGold);
        }

        // Pobierz Yang
        *player_yang -= item.price;
        Ok(item)
    }

    /// Formatuje ilosc waluty Yang z czytelnymi separatorami
    pub fn format_yang(yang: u32) -> String {
        if yang == 0 {
            return "0 Yang".to_string();
        }

        let mut num = yang;
        let mut parts = Vec::new();
        while num > 0 {
            parts.push(num % 1000);
            num /= 1000;
        }

        let mut result = String::new();
        let last = parts.pop().unwrap();
        result.push_str(&last.to_string());

        while let Some(part) = parts.pop() {
            result.push_str(&format!(",{part:03}"));
        }

        format!("{result} Yang")
    }

    /// Czy okno sklepu jest widoczne
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    /// VID NPC prowadzacego sklep
    pub fn npc_vid(&self) -> u32 {
        self.npc_vid
    }

    /// Indeks aktualnie wybranego slotu
    pub fn selected_slot(&self) -> Option<u8> {
        self.selected_slot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shop_window_init_and_grid() {
        let mut shop = UiWindowShop::new();
        assert!(!shop.is_visible());
        assert_eq!(shop.selected_slot(), None);

        shop.open(5001, "Handlarka Rozmaitosci");
        assert!(shop.is_visible());
        assert_eq!(shop.npc_vid(), 5001);

        // Dodaj przedmioty w siatce 5x8
        assert!(shop.set_item(0, 10, 1, 500).is_ok()); // Miecz +0
        assert!(shop.set_item(39, 27001, 200, 10000).is_ok()); // Czerwone Mikstury
        assert_eq!(shop.set_item(40, 10, 1, 100), Err(ShopError::InvalidSlot));

        assert_eq!(shop.get_item(0).unwrap().vnum, 10);
        assert_eq!(shop.get_item(39).unwrap().vnum, 27001);
    }

    #[test]
    fn test_shop_buy_item_and_gold_check() {
        let mut shop = UiWindowShop::new();
        shop.open(5002, "Zbrojmistrz");
        shop.set_item(5, 11200, 1, 2000).unwrap(); // Mnisia Zbroja +0

        let mut player_gold = 5000u32;

        // Wybierz slot
        assert!(shop.select_slot(5).is_ok());
        assert_eq!(shop.selected_slot(), Some(5));

        // Kupno z sukcesem
        let bought = shop.buy_selected(&mut player_gold).unwrap();
        assert_eq!(bought.vnum, 11200);
        assert_eq!(player_gold, 3000);

        // Proba kupna bez wystarczajacej ilosci Yang
        let mut poor_gold = 500u32;
        let err = shop.buy_item(5, &mut poor_gold);
        assert_eq!(err, Err(ShopError::NotEnoughGold));
        assert_eq!(poor_gold, 500); // Zloto nienaruszone
    }

    #[test]
    fn test_shop_format_yang() {
        assert_eq!(UiWindowShop::format_yang(0), "0 Yang");
        assert_eq!(UiWindowShop::format_yang(100), "100 Yang");
        assert_eq!(UiWindowShop::format_yang(1000), "1,000 Yang");
        assert_eq!(UiWindowShop::format_yang(1234567), "1,234,567 Yang");
    }
}
