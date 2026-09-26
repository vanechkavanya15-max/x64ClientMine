#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemData {
    pub vnum: u32,
    pub wearable_flag: u32,
}

// Wear positions based on CItemData::EWearPositions
pub const SLOT_ARMOR: u8 = 0; // WEAR_BODY
pub const SLOT_HEAD: u8 = 1;
pub const SLOT_FOOTS: u8 = 2;
pub const SLOT_WRIST: u8 = 3;
pub const SLOT_WEAPON: u8 = 4;
pub const SLOT_NECK: u8 = 5;
pub const SLOT_EAR: u8 = 6;
pub const SLOT_SHIELD: u8 = 10;
pub const SLOT_RING1: u8 = 21;
pub const SLOT_RING2: u8 = 22;
pub const SLOT_BELT: u8 = 23;

// Wearable flags based on CItemData::EItemWearableFlag
pub const WEARABLE_BODY: u32 = 1 << 0;
pub const WEARABLE_HEAD: u32 = 1 << 1;
pub const WEARABLE_FOOTS: u32 = 1 << 2;
pub const WEARABLE_WRIST: u32 = 1 << 3;
pub const WEARABLE_WEAPON: u32 = 1 << 4;
pub const WEARABLE_NECK: u32 = 1 << 5;
pub const WEARABLE_EAR: u32 = 1 << 6;
pub const WEARABLE_UNIQUE: u32 = 1 << 7;
pub const WEARABLE_SHIELD: u32 = 1 << 8;
pub const WEARABLE_BELT: u32 = 1 << 14;

pub struct EquipmentManager {
    slots: [Option<ItemData>; 32],
}

impl EquipmentManager {
    pub fn new() -> Self {
        const INIT: Option<ItemData> = None;
        Self {
            slots: [INIT; 32],
        }
    }

    pub fn can_equip(&self, slot: u8, item: &ItemData) -> bool {
        match slot {
            SLOT_ARMOR => (item.wearable_flag & WEARABLE_BODY) != 0,
            SLOT_HEAD => (item.wearable_flag & WEARABLE_HEAD) != 0,
            SLOT_FOOTS => (item.wearable_flag & WEARABLE_FOOTS) != 0,
            SLOT_WRIST => (item.wearable_flag & WEARABLE_WRIST) != 0,
            SLOT_WEAPON => (item.wearable_flag & WEARABLE_WEAPON) != 0,
            SLOT_NECK => (item.wearable_flag & WEARABLE_NECK) != 0,
            SLOT_EAR => (item.wearable_flag & WEARABLE_EAR) != 0,
            SLOT_SHIELD => (item.wearable_flag & WEARABLE_SHIELD) != 0,
            SLOT_BELT => (item.wearable_flag & WEARABLE_BELT) != 0,
            SLOT_RING1 | SLOT_RING2 => (item.wearable_flag & WEARABLE_UNIQUE) != 0,
            _ => false, // Invalid or unhandled slot
        }
    }

    pub fn equip_item(&mut self, slot: u8, item: ItemData) -> Option<ItemData> {
        if slot >= 32 {
            return Some(item); // Out of bounds, reject item and return it
        }

        if !self.can_equip(slot, &item) {
            return Some(item); // Cannot equip due to type mismatch, reject item and return it
        }

        let previous_item = self.slots[slot as usize].take();
        self.slots[slot as usize] = Some(item);
        previous_item
    }

    pub fn unequip_item(&mut self, slot: u8) -> Option<ItemData> {
        if slot >= 32 {
            return None;
        }
        self.slots[slot as usize].take()
    }

    pub fn get_item(&self, slot: u8) -> Option<&ItemData> {
        if slot >= 32 {
            return None;
        }
        self.slots[slot as usize].as_ref()
    }
}
