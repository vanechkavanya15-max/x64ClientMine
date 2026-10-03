#[allow(dead_code)]
pub const ITEM_TYPE_COSTUME: u8 = 28;
pub const ITEM_TYPE_BELT: u8 = 34;

pub const COSTUME_BODY: u8 = 0;
pub const COSTUME_HAIR: u8 = 1;
pub const COSTUME_NUM_TYPES: u8 = 2;

// equipment_slots::WEAR_MAX_NUM = 32
pub const WEAR_COSTUME_BODY: u8 = 19;
pub const WEAR_COSTUME_HAIR: u8 = 20;

// Wearable flag mappings
pub const WEARABLE_COSTUME_BODY: u32 = 1 << 12;
pub const WEARABLE_COSTUME_HAIR: u32 = 1 << 13;
pub const WEARABLE_BELT: u32 = 1 << 14;

pub const APPLY_COSTUME_ATTR_BONUS: u8 = 84;

pub const C_INVENTORY_PAGE_SIZE: u32 = 5 * 9;
pub const C_INVENTORY_PAGE_COUNT: u32 = 2;
pub const C_ITEM_SLOT_COUNT: u32 = C_INVENTORY_PAGE_SIZE * C_INVENTORY_PAGE_COUNT;

pub const C_EQUIPMENT_START: u32 = C_ITEM_SLOT_COUNT;
pub const WEAR_MAX_NUM: u32 = 32;

pub const WEAR_RING1: u32 = 21;
pub const WEAR_RING2: u32 = 22;
pub const WEAR_BELT: u32 = 23;

pub const C_NEW_EQUIPMENT_START: u32 = C_EQUIPMENT_START + WEAR_RING1;
pub const C_NEW_EQUIPMENT_COUNT: u32 = WEAR_MAX_NUM - WEAR_RING1;

pub const C_EQUIPMENT_RING1: u32 = C_EQUIPMENT_START + WEAR_RING1;
pub const C_EQUIPMENT_RING2: u32 = C_EQUIPMENT_START + WEAR_RING2;
pub const C_EQUIPMENT_BELT: u32 = C_EQUIPMENT_START + WEAR_BELT;

pub const C_COSTUME_SLOT_START: u32 = C_EQUIPMENT_START + (WEAR_COSTUME_BODY as u32);
pub const C_COSTUME_SLOT_BODY: u32 = C_COSTUME_SLOT_START + (COSTUME_BODY as u32);
pub const C_COSTUME_SLOT_HAIR: u32 = C_COSTUME_SLOT_START + (COSTUME_HAIR as u32);
pub const C_COSTUME_SLOT_COUNT: u32 = COSTUME_NUM_TYPES as u32;
pub const C_COSTUME_SLOT_END: u32 = C_COSTUME_SLOT_START + C_COSTUME_SLOT_COUNT;

pub const C_DRAGON_SOUL_EQUIP_START: u32 = C_ITEM_SLOT_COUNT + WEAR_MAX_NUM;
pub const C_DRAGON_SOUL_EQUIP_SLOT_MAX: u32 = 6;
pub const DS_DECK_MAX_NUM: u32 = 2;
pub const C_DRAGON_SOUL_EQUIP_END: u32 = C_DRAGON_SOUL_EQUIP_START + (C_DRAGON_SOUL_EQUIP_SLOT_MAX * DS_DECK_MAX_NUM);
pub const C_DRAGON_SOUL_EQUIP_RESERVED_COUNT: u32 = C_DRAGON_SOUL_EQUIP_SLOT_MAX * 3;

pub const C_BELT_INVENTORY_SLOT_START: u32 = C_DRAGON_SOUL_EQUIP_END + C_DRAGON_SOUL_EQUIP_RESERVED_COUNT;
pub const C_BELT_INVENTORY_WIDTH: u32 = 4;
pub const C_BELT_INVENTORY_HEIGHT: u32 = 4;
pub const C_BELT_INVENTORY_SLOT_COUNT: u32 = C_BELT_INVENTORY_WIDTH * C_BELT_INVENTORY_HEIGHT;
pub const C_BELT_INVENTORY_SLOT_END: u32 = C_BELT_INVENTORY_SLOT_START + C_BELT_INVENTORY_SLOT_COUNT;

pub const C_INVENTORY_COUNT: u32 = C_BELT_INVENTORY_SLOT_END;

pub fn is_belt_inventory_cell(cell: u32) -> bool {
    C_BELT_INVENTORY_SLOT_START <= cell && cell < C_BELT_INVENTORY_SLOT_END
}

pub fn is_costume_slot(cell: u32) -> bool {
    C_COSTUME_SLOT_START <= cell && cell < C_COSTUME_SLOT_END
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemDataCostume {
    pub vnum: u32,
    pub item_type: u8,
    pub sub_type: u8,
    pub wearable_flag: u32,
    pub apply_type: [u8; 7],
    pub apply_value: [i32; 7],
}

impl ItemDataCostume {
    pub fn is_costume(&self) -> bool {
        self.item_type == ITEM_TYPE_COSTUME
    }
    
    pub fn is_belt(&self) -> bool {
        self.item_type == ITEM_TYPE_BELT
    }
}

pub const BELT_GRADE_BY_LEVEL_TABLE: [u8; 10] = [
    0, // +0
    1, // +1
    1, // +2
    2, // +3
    2, // +4
    3, // +5
    4, // +6
    5, // +7
    6, // +8
    7, // +9
];

pub const AVAILABLE_RULE_BY_GRADE: [u8; 16] = [
    1, 2, 4, 6,
    3, 3, 4, 6,
    5, 5, 5, 6,
    7, 7, 7, 7,
];

pub fn get_belt_grade_by_refine_level(refine_level: i32) -> u8 {
    let mut level = refine_level;
    if level < 0 {
        level = 0;
    }
    if level > 9 {
        level = 9;
    }
    BELT_GRADE_BY_LEVEL_TABLE[level as usize]
}

pub fn is_available_belt_inventory_cell(cell: u32, belt_grade: u8) -> bool {
    if cell >= 16 {
        return false;
    }
    let rule = AVAILABLE_RULE_BY_GRADE[cell as usize];
    rule <= belt_grade
}

#[no_mangle]
pub extern "C" fn m2_costume_is_costume_slot(cell: u32) -> bool {
    is_costume_slot(cell)
}

#[no_mangle]
pub extern "C" fn m2_belt_is_belt_inventory_cell(cell: u32) -> bool {
    is_belt_inventory_cell(cell)
}

#[no_mangle]
pub extern "C" fn m2_belt_is_available_cell(cell: u32, belt_grade: u8) -> bool {
    is_available_belt_inventory_cell(cell, belt_grade)
}

#[no_mangle]
pub extern "C" fn m2_belt_get_grade_by_level(refine_level: i32) -> u8 {
    get_belt_grade_by_refine_level(refine_level)
}
