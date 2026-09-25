//! Modul mod_item dla skrzynki m2-client-pyapi
//! Modul Pythona item: baza prototypow przedmiotow
//! Metody item.SelectItem, item.GetItemName, item.GetItemType, item.GetItemPrice

use std::collections::HashMap;

/// Struktura reprezentujaca dane prototypu przedmiotu
#[derive(Debug, Clone)]
pub struct ItemData {
    pub vnum: u32,
    pub name: String,
    pub item_type: u8,
    pub price: u64,
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModItem {
    pub initialized: bool,
    prototypes: HashMap<u32, ItemData>,
    selected_vnum: Option<u32>,
}

impl ModItem {
    pub fn new() -> Self {
        Self {
            initialized: true,
            prototypes: HashMap::new(),
            selected_vnum: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Dodaje prototyp przedmiotu do bazy
    pub fn add_item_prototype(&mut self, vnum: u32, name: String, item_type: u8, price: u64) {
        self.prototypes.insert(vnum, ItemData {
            vnum,
            name,
            item_type,
            price,
        });
    }

    /// Metoda item.SelectItem: wybiera przedmiot do kolejnych operacji
    pub fn select_item(&mut self, vnum: u32) -> bool {
        if self.prototypes.contains_key(&vnum) {
            self.selected_vnum = Some(vnum);
            true
        } else {
            self.selected_vnum = None;
            false
        }
    }

    /// Metoda item.GetItemName: zwraca nazwe wybranego przedmiotu
    pub fn get_item_name(&self) -> Option<String> {
        self.selected_vnum
            .and_then(|vnum| self.prototypes.get(&vnum))
            .map(|item| item.name.clone())
    }

    /// Metoda item.GetItemType: zwraca typ wybranego przedmiotu
    pub fn get_item_type(&self) -> Option<u8> {
        self.selected_vnum
            .and_then(|vnum| self.prototypes.get(&vnum))
            .map(|item| item.item_type)
    }

    /// Metoda item.GetItemPrice: zwraca cene wybranego przedmiotu
    pub fn get_item_price(&self) -> Option<u64> {
        self.selected_vnum
            .and_then(|vnum| self.prototypes.get(&vnum))
            .map(|item| item.price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_mod_item() {
        let obj = ModItem::new();
        assert!(obj.is_ready());
    }

    #[test]
    fn test_mod_item_methods() {
        let mut obj = ModItem::new();
        obj.add_item_prototype(10, "Miecz".to_string(), 1, 100);
        
        assert_eq!(obj.select_item(10), true);
        assert_eq!(obj.get_item_name(), Some("Miecz".to_string()));
        assert_eq!(obj.get_item_type(), Some(1));
        assert_eq!(obj.get_item_price(), Some(100));
        
        assert_eq!(obj.select_item(99), false);
        assert_eq!(obj.get_item_name(), None);
    }
}