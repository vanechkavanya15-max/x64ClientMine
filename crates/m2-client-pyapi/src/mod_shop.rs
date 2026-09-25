//! Modul mod_shop dla skrzynki m2-client-pyapi
//! Modul Pythona shop: sklepy NPC i sklepy graczy
//! Metody shop.IsOpen, shop.GetItemVnum, shop.GetItemCount, shop.GetItemPrice

use std::collections::HashMap;

/// Reprezentuje przedmiot w sklepie
#[derive(Debug, Clone, Default)]
pub struct ShopItem {
    pub vnum: u32,
    pub count: u16,
    pub price: u64,
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModShop {
    pub initialized: bool,
    pub is_open: bool,
    pub items: HashMap<u8, ShopItem>,
}

impl ModShop {
    pub fn new() -> Self {
        Self {
            initialized: true,
            is_open: false,
            items: HashMap::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.items.clear();
    }

    pub fn set_item(&mut self, slot: u8, item: ShopItem) {
        self.items.insert(slot, item);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn get_item_vnum(&self, slot: u8) -> u32 {
        self.items.get(&slot).map_or(0, |item| item.vnum)
    }

    pub fn get_item_count(&self, slot: u8) -> u16 {
        self.items.get(&slot).map_or(0, |item| item.count)
    }

    pub fn get_item_price(&self, slot: u8) -> u64 {
        self.items.get(&slot).map_or(0, |item| item.price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_mod_shop() {
        let obj = ModShop::new();
        assert!(obj.is_ready());
    }
}
