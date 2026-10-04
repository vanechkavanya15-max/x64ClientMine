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

    /// Validates if an item type and subtype is allowed in the belt inventory.
    /// Only potions (Potions, Elixirs, Dews, Blends) are allowed.
    pub fn is_item_allowed_in_belt(item_type: u8, item_sub_type: u8) -> bool {
        // ITEM_TYPE_USE = 3
        const ITEM_TYPE_USE: u8 = 3;
        // ITEM_TYPE_BLEND = 27
        const ITEM_TYPE_BLEND: u8 = 27;
        
        // USE_POTION = 0
        const USE_POTION: u8 = 0;
        // USE_POTION_NODELAY = 11
        const USE_POTION_NODELAY: u8 = 11;
        // USE_ABILITY_UP = 7
        const USE_ABILITY_UP: u8 = 7; // Dews often use this subtype
        
        if item_type == ITEM_TYPE_BLEND {
            return true;
        }
        
        if item_type == ITEM_TYPE_USE {
            if item_sub_type == USE_POTION || item_sub_type == USE_POTION_NODELAY || item_sub_type == USE_ABILITY_UP {
                return true;
            }
        }

        false
    }
    
    /// Checks if an item from the belt inventory can be moved/used in the quickslot.
    /// As per requirement, only potions in the belt inventory can be dragged to or used automatically from quickslot.
    pub fn can_move_belt_item_to_quickslot(item_type: u8, item_sub_type: u8) -> bool {
        Self::is_item_allowed_in_belt(item_type, item_sub_type)
    }
    
    /// Checks if an item from the belt inventory can be automatically used (e.g. auto potion).
    pub fn can_auto_use_belt_item(item_type: u8, item_sub_type: u8) -> bool {
        Self::is_item_allowed_in_belt(item_type, item_sub_type)
    }
}
