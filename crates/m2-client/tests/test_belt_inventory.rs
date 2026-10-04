use m2_client::game_belt_inventory_manager::{
    BeltInventoryHelper,
    BELT_INVENTORY_SLOT_START,
    BELT_INVENTORY_SLOT_END,
    BELT_INVENTORY_SLOT_COUNT,
};

#[test]
fn test_is_belt_inventory_slot() {
    // Valid slots
    for cell in BELT_INVENTORY_SLOT_START..BELT_INVENTORY_SLOT_END {
        assert!(BeltInventoryHelper::is_belt_inventory_slot(cell), "Cell {} should be a belt inventory slot", cell);
    }

    // Invalid slots
    assert!(!BeltInventoryHelper::is_belt_inventory_slot(BELT_INVENTORY_SLOT_START - 1));
    assert!(!BeltInventoryHelper::is_belt_inventory_slot(BELT_INVENTORY_SLOT_END));
    assert!(!BeltInventoryHelper::is_belt_inventory_slot(0));
}

#[test]
fn test_get_belt_grade_by_refine_level() {
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
    
    // Out of bounds test
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(10), 0);
    assert_eq!(BeltInventoryHelper::get_belt_grade_by_refine_level(-1), 0);
}

#[test]
fn test_is_available_cell() {
    // Rule table:
    // 1, 2, 4, 6
    // 3, 3, 4, 6
    // 5, 5, 5, 6
    // 7, 7, 7, 7

    // Belt grade 0 (Level +0) -> Should have NO available cells
    for cell in 0..BELT_INVENTORY_SLOT_COUNT {
        assert!(!BeltInventoryHelper::is_available_cell(cell, 0), "Cell {} should NOT be available for grade 0", cell);
    }

    // Belt grade 1 (Level +1, +2)
    assert!(BeltInventoryHelper::is_available_cell(0, 1));
    assert!(!BeltInventoryHelper::is_available_cell(1, 1));

    // Belt grade 7 (Level +9) -> Should have ALL available cells
    for cell in 0..BELT_INVENTORY_SLOT_COUNT {
        assert!(BeltInventoryHelper::is_available_cell(cell, 7), "Cell {} should be available for grade 7", cell);
    }

    // specific spot checks based on the grid
    // Grade 3 (Level +5) should unlock 1, 2, 3
    assert!(BeltInventoryHelper::is_available_cell(4, 3)); // rule 3
    assert!(BeltInventoryHelper::is_available_cell(5, 3)); // rule 3
    assert!(!BeltInventoryHelper::is_available_cell(2, 3)); // rule 4 (needs grade 4)

    // Cell out of bounds
    assert!(!BeltInventoryHelper::is_available_cell(16, 7));
}
