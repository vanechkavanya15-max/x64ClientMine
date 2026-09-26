pub const ITEM_TYPE_POTION: u8 = 0;
pub const ITEM_TYPE_USE: u8 = 3;
pub const BELT_ACCEPT: u8 = 29;

const BELT_GRADE_BY_LEVEL_TABLE: [u8; 10] = [
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

const AVAILABLE_RULE_BY_GRADE: [u8; 16] = [
    1, 2, 4, 6,
    3, 3, 4, 6,
    5, 5, 5, 6,
    7, 7, 7, 7,
];

#[no_mangle]
pub extern "C" fn m2_belt_can_store(belt_level: u8, slot: u8, item_type: u8) -> bool {
    if slot >= 16 {
        return false;
    }

    if belt_level >= 10 {
        return false;
    }

    if item_type != ITEM_TYPE_POTION && item_type != ITEM_TYPE_USE && item_type != BELT_ACCEPT {
        return false;
    }

    let belt_grade = BELT_GRADE_BY_LEVEL_TABLE[belt_level as usize];
    let rule = AVAILABLE_RULE_BY_GRADE[slot as usize];

    rule <= belt_grade
}
