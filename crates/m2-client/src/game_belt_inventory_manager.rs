pub const BELT_INVENTORY_SLOT_START: u16 = 152;
pub const BELT_INVENTORY_SLOT_COUNT: u16 = 16;
pub const BELT_INVENTORY_SLOT_END: u16 = 168;

pub struct BeltInventoryHelper;

impl BeltInventoryHelper {
    pub fn get_belt_grade_by_refine_level(refine_level: i32) -> u8 {
        let belt_grade_by_level_table: [u8; 10] = [
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
        
        if refine_level >= 0 && (refine_level as usize) < belt_grade_by_level_table.len() {
            belt_grade_by_level_table[refine_level as usize]
        } else {
            0
        }
    }

    pub fn get_available_rule_table_by_grade() -> [u8; 16] {
        [
            1, 2, 4, 6,
            3, 3, 4, 6,
            5, 5, 5, 6,
            7, 7, 7, 7,
        ]
    }

    pub fn is_available_cell(cell: u16, belt_grade: u8) -> bool {
        if cell < BELT_INVENTORY_SLOT_COUNT {
            let rule_table = Self::get_available_rule_table_by_grade();
            rule_table[cell as usize] <= belt_grade
        } else {
            false
        }
    }

    pub fn is_belt_inventory_slot(cell: u16) -> bool {
        cell >= BELT_INVENTORY_SLOT_START && cell < BELT_INVENTORY_SLOT_END
    }
}
