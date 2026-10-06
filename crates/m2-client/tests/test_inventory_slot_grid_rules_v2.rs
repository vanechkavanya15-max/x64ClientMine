#[path = "../src/inventory_slot_grid_rules_v2.rs"]
mod inventory_slot_grid_rules_v2;

use inventory_slot_grid_rules_v2::{
    InventoryGridRules, ItemData, INVENTORY_MAX_NUM, ITEM_ANTIFLAG_STACK, ITEM_FLAG_STACKABLE,
};

#[test]
fn test_find_blank_slot_empty_grid() {
    let grid = InventoryGridRules::new();
    assert_eq!(grid.find_blank_slot(1), Some(0));
    assert_eq!(grid.find_blank_slot(2), Some(0));
    assert_eq!(grid.find_blank_slot(3), Some(0));
}

#[test]
fn test_check_collision_and_find_blank() {
    let mut grid = InventoryGridRules::new();

    // Fill first slot
    grid.set_item(
        0,
        Some(ItemData {
            vnum: 10,
            count: 1,
            size: 1,
            flags: 0,
            anti_flags: 0,
        }),
    );
    assert!(grid.check_collision(0, 1)); // Collision with self
    assert_eq!(grid.find_blank_slot(1), Some(1));

    // Fill second slot (so slots 0 and 1 are full)
    grid.set_item(
        1,
        Some(ItemData {
            vnum: 20,
            count: 1,
            size: 2,
            flags: 0,
            anti_flags: 0,
        }),
    );
    // A size 2 item at slot 1 occupies slot 1 and slot 6
    assert!(grid.check_collision(1, 2));
    assert!(grid.check_collision(1, 1)); // Slot 1 is full

    // So next 1x1 should be at slot 2
    assert_eq!(grid.find_blank_slot(1), Some(2));
}

#[test]
fn test_stacking_rules() {
    let mut grid = InventoryGridRules::new();

    grid.set_item(
        10,
        Some(ItemData {
            vnum: 27001,
            count: 50,
            size: 1,
            flags: ITEM_FLAG_STACKABLE,
            anti_flags: 0,
        }),
    );

    let can_stack_item = ItemData {
        vnum: 27001,
        count: 50,
        size: 1,
        flags: ITEM_FLAG_STACKABLE,
        anti_flags: 0,
    };

    assert!(grid.can_stack(10, &can_stack_item));

    let different_vnum = ItemData {
        vnum: 27002,
        count: 50,
        size: 1,
        flags: ITEM_FLAG_STACKABLE,
        anti_flags: 0,
    };

    assert!(!grid.can_stack(10, &different_vnum));

    // Test full stack limit
    let large_stack = ItemData {
        vnum: 27001,
        count: 200, // 200 + 50 = 250 > 200
        size: 1,
        flags: ITEM_FLAG_STACKABLE,
        anti_flags: 0,
    };
    assert!(!grid.can_stack(10, &large_stack));

    // Test anti-flag
    grid.set_item(
        11,
        Some(ItemData {
            vnum: 27003,
            count: 10,
            size: 1,
            flags: ITEM_FLAG_STACKABLE,
            anti_flags: ITEM_ANTIFLAG_STACK,
        }),
    );

    let anti_stack_item = ItemData {
        vnum: 27003,
        count: 10,
        size: 1,
        flags: ITEM_FLAG_STACKABLE,
        anti_flags: ITEM_ANTIFLAG_STACK,
    };

    assert!(!grid.can_stack(11, &anti_stack_item));
}

#[test]
fn test_out_of_bounds() {
    let grid = InventoryGridRules::new();
    assert!(grid.check_collision(INVENTORY_MAX_NUM, 1));
    assert!(grid.check_collision(INVENTORY_MAX_NUM + 1, 2));
    assert!(!grid.can_stack(
        INVENTORY_MAX_NUM,
        &ItemData {
            vnum: 1,
            count: 1,
            size: 1,
            flags: 0,
            anti_flags: 0
        }
    ));
}

#[test]
fn test_page_bottom_boundary() {
    let grid = InventoryGridRules::new();

    // Page 1 ends at row 8 (0-indexed).
    // A 3-size item placed at row 7 (e.g. slot 35) would occupy row 7, 8, 9 (page boundary crossed)
    // Actually, row 7 is index 7. size 3 means 7, 8, 9. Row 9 is the first row of page 2 (in a continuous grid)
    // Metin2 inventory pages are isolated grids visually, a 1x3 item cannot cross from page 1 to page 2.
    // Let's test row 7 (slot 35) -> size 3 crosses to row 9 in the continuous index which is row 0 of page 2.
    assert!(grid.check_collision(35, 3));
    assert!(grid.check_collision(40, 2)); // row 8 size 2 crosses to row 9
    assert!(!grid.check_collision(30, 3)); // row 6 size 3 -> rows 6, 7, 8 (all on page 1). Valid.
}
