#[path = "../src/inventory_grid_transactions.rs"]
mod inventory_grid_transactions;
#[path = "../src/send_item_packets_v2.rs"]
mod send_item_packets_v2;

use inventory_grid_transactions::{InventoryGrid, ItemData, WINDOW_TYPE_INVENTORY};
use send_item_packets_v2::{HEADER_CG_ITEM_MOVE};

#[test]
fn test_inventory_grid_move_1x3_sword() {
    let mut grid = InventoryGrid::new();
    
    // Add a 1x3 sword at slot 0
    let sword = ItemData { vnum: 199, count: 1, size: 3 };
    grid.set_item(0, Some(sword));
    grid.set_item(5, Some(sword)); // Cell 5 (row 1, col 0)
    grid.set_item(10, Some(sword)); // Cell 10 (row 2, col 0)

    // Ensure it can be moved to a valid empty space (slot 2)
    assert!(grid.can_place_item_ignore(2, sword.size, 0));
    
    // Test moving it to slot 2 (which is valid and empty)
    let move_res = grid.move_item(0, 2);
    assert!(move_res.is_ok());
    let pkt = move_res.unwrap();
    assert_eq!({pkt.header}, HEADER_CG_ITEM_MOVE);
    assert_eq!({pkt.pos.window_type}, WINDOW_TYPE_INVENTORY);
    assert_eq!({pkt.pos.cell}, 0);
    assert_eq!({pkt.change_pos.window_type}, WINDOW_TYPE_INVENTORY);
    assert_eq!({pkt.change_pos.cell}, 2);
    
    // Try moving to slot 40, which would cross the boundary of page 1 (since 40 % 45 / 5 = 8. 8 + 3 - 1 = 10 > 9)
    assert!(!grid.can_place_item(40, sword.size));
    let bad_move_res = grid.move_item(2, 40);
    assert!(bad_move_res.is_err());
    
    // Place an obstacle in cell 6 (row 1, col 1) so 1x3 sword at slot 1 collides
    grid.set_item(6, Some(ItemData { vnum: 27001, count: 1, size: 1 }));
    assert!(!grid.can_place_item_ignore(1, sword.size, 2));
    let collision_move_res = grid.move_item(2, 1);
    assert!(collision_move_res.is_err());
}

#[test]
fn test_inventory_grid_swap_items() {
    let mut grid = InventoryGrid::new();
    
    let potion = ItemData { vnum: 27001, count: 200, size: 1 };
    let sword = ItemData { vnum: 199, count: 1, size: 3 };
    
    // Setup sword at 0, 5, 10
    grid.set_item(0, Some(sword));
    grid.set_item(5, Some(sword));
    grid.set_item(10, Some(sword));
    
    // Setup potion at 1
    grid.set_item(1, Some(potion));
    
    // Trying to swap potion (1x1) with sword (1x3) starting at slot 0
    // This is valid because potion fits in 0, and sword fits in 1 (1, 6, 11 are empty)
    let swap_res = grid.swap_items(1, 0);
    assert!(swap_res.is_ok());
    let pkt = swap_res.unwrap();
    assert_eq!({pkt.header}, HEADER_CG_ITEM_MOVE);
    assert_eq!({pkt.pos.cell}, 1);
    assert_eq!({pkt.change_pos.cell}, 0);
    assert_eq!({pkt.num}, potion.count);
    
    assert_eq!(grid.get_item(0).unwrap().vnum, potion.vnum);
    assert_eq!(grid.get_item(1).unwrap().vnum, sword.vnum);
}

#[test]
fn test_inventory_grid_split_stack() {
    let mut grid = InventoryGrid::new();
    
    let potion = ItemData { vnum: 27001, count: 200, size: 1 };
    grid.set_item(0, Some(potion));
    
    let split_res = grid.split_stack(0, 1, 50);
    assert!(split_res.is_ok());
    let pkt = split_res.unwrap();
    assert_eq!({pkt.header}, HEADER_CG_ITEM_MOVE);
    assert_eq!({pkt.pos.cell}, 0);
    assert_eq!({pkt.change_pos.cell}, 1);
    assert_eq!({pkt.num}, 50);
    
    assert_eq!(grid.get_item(0).unwrap().count, 150);
    assert_eq!(grid.get_item(1).unwrap().count, 50);
    
    // Cannot split to occupied cell
    grid.set_item(2, Some(potion));
    assert!(grid.split_stack(0, 2, 10).is_err());
    
    // Cannot split >= count
    assert!(grid.split_stack(0, 3, 150).is_err());
}
