//! Modul mod_exchange dla skrzynki m2-client-pyapi
//! Modul Pythona exchange: okno handlu gracz-gracz
//! Metody exchange.GetElkFromSelf, exchange.GetItemVnumFromSelf, exchange.IsAcceptFromSelf

use std::collections::HashMap;

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModExchange {
    pub initialized: bool,
    
    // Dane gracza
    pub elk_from_self: u64,
    pub accept_from_self: bool,
    pub items_from_self: HashMap<u8, u32>,
    
    // Dane partnera
    pub elk_from_target: u64,
    pub accept_from_target: bool,
    pub items_from_target: HashMap<u8, u32>,
}

impl ModExchange {
    pub fn new() -> Self {
        Self { 
            initialized: true,
            elk_from_self: 0,
            accept_from_self: false,
            items_from_self: HashMap::new(),
            elk_from_target: 0,
            accept_from_target: false,
            items_from_target: HashMap::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn get_elk_from_self(&self) -> u64 {
        self.elk_from_self
    }

    pub fn set_elk_from_self(&mut self, elk: u64) {
        self.elk_from_self = elk;
    }

    pub fn get_item_vnum_from_self(&self, pos: u8) -> u32 {
        *self.items_from_self.get(&pos).unwrap_or(&0)
    }

    pub fn set_item_vnum_from_self(&mut self, pos: u8, vnum: u32) {
        self.items_from_self.insert(pos, vnum);
    }

    pub fn is_accept_from_self(&self) -> bool {
        self.accept_from_self
    }

    pub fn set_accept_from_self(&mut self, accept: bool) {
        self.accept_from_self = accept;
    }

    pub fn get_elk_from_target(&self) -> u64 {
        self.elk_from_target
    }

    pub fn set_elk_from_target(&mut self, elk: u64) {
        self.elk_from_target = elk;
    }

    pub fn get_item_vnum_from_target(&self, pos: u8) -> u32 {
        *self.items_from_target.get(&pos).unwrap_or(&0)
    }

    pub fn set_item_vnum_from_target(&mut self, pos: u8, vnum: u32) {
        self.items_from_target.insert(pos, vnum);
    }

    pub fn is_accept_from_target(&self) -> bool {
        self.accept_from_target
    }

    pub fn set_accept_from_target(&mut self, accept: bool) {
        self.accept_from_target = accept;
    }

    pub fn clear(&mut self) {
        self.elk_from_self = 0;
        self.accept_from_self = false;
        self.items_from_self.clear();
        
        self.elk_from_target = 0;
        self.accept_from_target = false;
        self.items_from_target.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_mod_exchange() {
        let obj = ModExchange::new();
        assert!(obj.is_ready());
    }

    #[test]
    fn test_elk_from_self() {
        let mut obj = ModExchange::new();
        assert_eq!(obj.get_elk_from_self(), 0);
        obj.set_elk_from_self(12345);
        assert_eq!(obj.get_elk_from_self(), 12345);
    }

    #[test]
    fn test_item_vnum_from_self() {
        let mut obj = ModExchange::new();
        assert_eq!(obj.get_item_vnum_from_self(0), 0);
        obj.set_item_vnum_from_self(0, 50200);
        assert_eq!(obj.get_item_vnum_from_self(0), 50200);
        assert_eq!(obj.get_item_vnum_from_self(1), 0);
    }

    #[test]
    fn test_accept_from_self() {
        let mut obj = ModExchange::new();
        assert_eq!(obj.is_accept_from_self(), false);
        obj.set_accept_from_self(true);
        assert_eq!(obj.is_accept_from_self(), true);
    }

    #[test]
    fn test_target_methods() {
        let mut obj = ModExchange::new();
        obj.set_elk_from_target(999);
        assert_eq!(obj.get_elk_from_target(), 999);

        obj.set_item_vnum_from_target(2, 10);
        assert_eq!(obj.get_item_vnum_from_target(2), 10);

        obj.set_accept_from_target(true);
        assert_eq!(obj.is_accept_from_target(), true);
    }

    #[test]
    fn test_clear() {
        let mut obj = ModExchange::new();
        obj.set_elk_from_self(100);
        obj.set_item_vnum_from_self(1, 123);
        obj.set_accept_from_self(true);
        
        obj.clear();
        assert_eq!(obj.get_elk_from_self(), 0);
        assert_eq!(obj.get_item_vnum_from_self(1), 0);
        assert_eq!(obj.is_accept_from_self(), false);
    }
}
