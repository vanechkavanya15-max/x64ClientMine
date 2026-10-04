#[path = "../src/belt_inventory_potion_gate.rs"]
mod belt_inventory_potion_gate;

use belt_inventory_potion_gate::*;

#[test]
fn test_belt_grade_by_refine_level() {
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(0), 0);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(1), 1);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(2), 1);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(3), 2);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(4), 2);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(5), 3);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(6), 4);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(7), 5);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(8), 6);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(9), 7);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(10), 0); // fallback
}

#[test]
fn test_is_available_cell_for_belt_grade() {
    // With grade 0 (+0 belt), no cells are available
    for i in 0..16 {
        assert!(!BeltInventoryHelper::is_available_cell(i, 0));
    }

    // With grade 1 (+1/+2 belt), only cell 0 is available
    assert!(BeltInventoryHelper::is_available_cell(0, 1));
    for i in 1..16 {
        assert!(!BeltInventoryHelper::is_available_cell(i, 1));
    }
    
    // With grade 7 (+9 belt), all cells are available
    for i in 0..16 {
        assert!(BeltInventoryHelper::is_available_cell(i, 7));
    }

    // specific spot checks based on BELT_SLOT_GRADE_RULES
    // 1, 2, 4, 6,
    // 3, 3, 4, 6,
    // 5, 5, 5, 6,
    // 7, 7, 7, 7,
    assert!(BeltInventoryHelper::is_available_cell(1, 2)); // cell 1 needs grade 2
    assert!(!BeltInventoryHelper::is_available_cell(1, 1));
    
    assert!(BeltInventoryHelper::is_available_cell(4, 3)); // cell 4 needs grade 3
    assert!(!BeltInventoryHelper::is_available_cell(4, 2));

    assert!(BeltInventoryHelper::is_available_cell(8, 5)); // cell 8 needs grade 5
    assert!(!BeltInventoryHelper::is_available_cell(8, 4));
}

#[test]
fn test_is_item_allowed_in_belt() {
    // 3 is USE, 27 is BLEND
    // 0 is POTION, 11 is POTION_NODELAY, 7 is ABILITY_UP

    // Allowed
    assert!(BeltInventoryHelper::is_item_allowed_in_belt(3, 0)); // Potion
    assert!(BeltInventoryHelper::is_item_allowed_in_belt(3, 11)); // Potion no-delay
    assert!(BeltInventoryHelper::is_item_allowed_in_belt(3, 7)); // Ability up (Dews)
    assert!(BeltInventoryHelper::is_item_allowed_in_belt(27, 0)); // Blend

    // Not allowed
    assert!(!BeltInventoryHelper::is_item_allowed_in_belt(1, 0)); // Weapon
    assert!(!BeltInventoryHelper::is_item_allowed_in_belt(2, 0)); // Armor
    assert!(!BeltInventoryHelper::is_item_allowed_in_belt(3, 1)); // Talisman
    assert!(!BeltInventoryHelper::is_item_allowed_in_belt(3, 2)); // Tuning
}

#[test]
fn test_can_move_belt_item_to_quickslot() {
    assert!(BeltInventoryHelper::can_move_belt_item_to_quickslot(3, 0));
    assert!(!BeltInventoryHelper::can_move_belt_item_to_quickslot(1, 0));
}

#[test]
fn test_can_auto_use_belt_item() {
    assert!(BeltInventoryHelper::can_auto_use_belt_item(3, 11));
    assert!(!BeltInventoryHelper::can_auto_use_belt_item(2, 0));
}
