//! Modul mod_safebox dla skrzynki m2-client-pyapi
//! Modul Pythona safebox: magazyn dozorcy
//! Metody safebox.GetCurrentSafeboxSize, safebox.GetItemID, safebox.GetMoney

use std::collections::HashMap;

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModSafebox {
    pub initialized: bool,
    pub size: u32,
    pub money: u64,
    pub items: HashMap<u32, u32>,
}

impl ModSafebox {
    pub fn new() -> Self {
        Self { 
            initialized: true,
            size: 0,
            money: 0,
            items: HashMap::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Ustawia rozmiar magazynu
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    /// Pobiera aktualny rozmiar magazynu
    pub fn get_current_safebox_size(&self) -> u32 {
        self.size
    }

    /// Ustawia ilosc pieniedzy
    pub fn set_money(&mut self, money: u64) {
        self.money = money;
    }

    /// Pobiera ilosc pieniedzy
    pub fn get_money(&self) -> u64 {
        self.money
    }

    /// Ustawia przedmiot w magazynie
    pub fn set_item(&mut self, slot: u32, item_id: u32) {
        self.items.insert(slot, item_id);
    }

    /// Pobiera ID przedmiotu z magazynu
    pub fn get_item_id(&self, slot: u32) -> u32 {
        *self.items.get(&slot).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_mod_safebox() {
        let obj = ModSafebox::new();
        assert!(obj.is_ready());
    }
}
