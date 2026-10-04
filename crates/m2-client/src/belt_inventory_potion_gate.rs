// Implements the Belt Inventory Gate for potions in pure Rust

pub const BELT_INVENTORY_SLOT_COUNT: usize = 16;

/// Represents the grade requirement for each slot in the 4x4 belt inventory.
/// Based on the C++ `CBeltInventoryHelper::GetAvailableRuleTableByGrade()`.
const BELT_SLOT_GRADE_RULES: [u8; BELT_INVENTORY_SLOT_COUNT] = [
    1, 2, 4, 6,
    3, 3, 4, 6,
    5, 5, 5, 6,
    7, 7, 7, 7,
];

/// Belt inventory helper equivalent to C++ `CBeltInventoryHelper`.
pub struct BeltInventoryHelper;

impl BeltInventoryHelper {
    /// Returns the belt grade (0-7) based on the belt's refine level (+0 to +9).
    pub fn get_belt_grade_by_refine_level(refine_level: u8) -> u8 {
        match refine_level {
            0 => 0,
            1 | 2 => 1,
            3 | 4 => 2,
            5 => 3,
            6 => 4,
            7 => 5,
            8 => 6,
            9 => 7,
            _ => 0, // Fallback for invalid levels
        }
    }

    /// Checks if a specific cell (0-15) is available given the belt's grade.
    pub fn is_available_cell(cell: usize, belt_grade: u8) -> bool {
        if cell >= BELT_INVENTORY_SLOT_COUNT {
            return false;
        }
        BELT_SLOT_GRADE_RULES[cell] <= belt_grade
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
