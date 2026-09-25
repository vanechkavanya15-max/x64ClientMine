//! Modul `player_core` — Kompletny stan glownego gracza CPythonPlayer
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayer.cpp`
//! Opis: Odwzorowanie CPythonPlayer: tablica statystyk Point[255], 180 slotow ekwipunku, zalozone przedmioty, zloto (Yang), aktualny cel (target VID), stan zycia/smierci.

pub const POINT_MAX_NUM: usize = 255;
pub const INVENTORY_MAX_NUM: usize = 180;
pub const EQUIPMENT_MAX_NUM: usize = 32;
pub const POINT_MAGIC_NUMBER: i32 = -415583782; // 0xe73ac1da as i32
pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

pub const POINT_HP: usize = 5;
pub const POINT_GOLD: usize = 11;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ItemAttribute {
    pub attr_type: u8,
    pub value: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub sockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub attributes: [ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[derive(Debug, Clone)]
pub struct PlayerCore {
    pub initialized: bool,
    points: [i32; POINT_MAX_NUM],
    pub inventory: [ItemData; INVENTORY_MAX_NUM],
    pub equipment: [ItemData; EQUIPMENT_MAX_NUM],
    pub target_vid: u32,
}

impl Default for PlayerCore {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerCore {
    pub fn new() -> Self {
        let mut points = [0; POINT_MAX_NUM];
        for i in 0..POINT_MAX_NUM {
            points[i] = 0 ^ POINT_MAGIC_NUMBER;
        }

        Self {
            initialized: true,
            points,
            inventory: [ItemData::default(); INVENTORY_MAX_NUM],
            equipment: [ItemData::default(); EQUIPMENT_MAX_NUM],
            target_vid: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_point(&mut self, point_type: usize, value: i32) {
        if point_type < POINT_MAX_NUM {
            self.points[point_type] = value ^ POINT_MAGIC_NUMBER;
        }
    }

    pub fn get_point(&self, point_type: usize) -> i32 {
        if point_type < POINT_MAX_NUM {
            self.points[point_type] ^ POINT_MAGIC_NUMBER
        } else {
            0
        }
    }

    pub fn set_target(&mut self, target_vid: u32) {
        self.target_vid = target_vid;
    }

    pub fn get_target(&self) -> u32 {
        self.target_vid
    }

    pub fn clear_target(&mut self) {
        self.target_vid = 0;
    }

    pub fn is_dead(&self) -> bool {
        self.get_point(POINT_HP) <= 0
    }

    pub fn set_inventory_item(&mut self, slot: usize, item: ItemData) {
        if slot < INVENTORY_MAX_NUM {
            self.inventory[slot] = item;
        }
    }

    pub fn get_inventory_item(&self, slot: usize) -> Option<&ItemData> {
        self.inventory.get(slot)
    }

    pub fn set_equipment_item(&mut self, slot: usize, item: ItemData) {
        if slot < EQUIPMENT_MAX_NUM {
            self.equipment[slot] = item;
        }
    }

    pub fn get_equipment_item(&self, slot: usize) -> Option<&ItemData> {
        self.equipment.get(slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_player_core() {
        let instance = PlayerCore::new();
        assert!(instance.is_ready());
    }
}
