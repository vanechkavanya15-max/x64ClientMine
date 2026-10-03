#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            WindowType::INVENTORY => 180,
            WindowType::EQUIPMENT => 32,
            WindowType::SAFEBOX => 135,
            WindowType::MALL => 45,
            WindowType::DRAGON_SOUL_INVENTORY => 960,
            WindowType::BELT_INVENTORY => 16,
        }
    }
}

pub struct GameStatePayloadValidator;

impl GameStatePayloadValidator {
    pub fn validate_yang(amount: i64) -> bool {
        amount >= 0 && amount <= 2_000_000_000
    }

    pub fn validate_inventory_slot(window_type: u8, slot: u16) -> bool {
        let win = match window_type {
            1 => WindowType::INVENTORY,
            2 => WindowType::EQUIPMENT,
            3 => WindowType::SAFEBOX,
            4 => WindowType::MALL,
            5 => WindowType::DRAGON_SOUL_INVENTORY,
            7 => WindowType::BELT_INVENTORY,
            _ => return false,
        };
        
        slot < win.max_slots()
    }
}
