#[path = "../src/item_stacking.rs"]
mod item_stacking;

use item_stacking::{Inventory, Item, ITEM_MAX_COUNT};

#[test]
fn test_merge_small_stacks() {
    let mut inventory = Inventory::new(2);
    inventory.slots[0] = Some(Item { vnum: 100, count: 50, has_unique_attributes: false });
    inventory.slots[1] = Some(Item { vnum: 100, count: 60, has_unique_attributes: false });

    assert_eq!(inventory.merge_slots(0, 1), Ok(()));
    
    assert_eq!(inventory.slots[0], None);
    assert_eq!(inventory.slots[1].unwrap().count, 110);
    assert_eq!(inventory.slots[1].unwrap().vnum, 100);
}

#[test]
fn test_merge_stacks_exceed_max_count() {
    let mut inventory = Inventory::new(2);
    inventory.slots[0] = Some(Item { vnum: 200, count: 150, has_unique_attributes: false });
    inventory.slots[1] = Some(Item { vnum: 200, count: 80, has_unique_attributes: false });

    assert_eq!(inventory.merge_slots(0, 1), Ok(()));
    
    // Total is 230. Target (slot 1) should get 200, source (slot 0) should get 30.
    assert_eq!(inventory.slots[1].unwrap().count, ITEM_MAX_COUNT); // 200
    assert_eq!(inventory.slots[0].unwrap().count, 30);
    assert_eq!(inventory.slots[1].unwrap().vnum, 200);
    assert_eq!(inventory.slots[0].unwrap().vnum, 200);
}

#[test]
fn test_merge_same_slot() {
    let mut inventory = Inventory::new(1);
    inventory.slots[0] = Some(Item { vnum: 100, count: 50, has_unique_attributes: false });

    assert_eq!(inventory.merge_slots(0, 0), Err("Cannot merge a slot into itself"));
}

#[test]
fn test_merge_out_of_bounds() {
    let mut inventory = Inventory::new(1);
    assert_eq!(inventory.merge_slots(0, 1), Err("Slot index out of bounds"));
    assert_eq!(inventory.merge_slots(1, 0), Err("Slot index out of bounds"));
}

#[test]
fn test_merge_empty_slots() {
    let mut inventory = Inventory::new(2);
    inventory.slots[1] = Some(Item { vnum: 100, count: 50, has_unique_attributes: false });

    assert_eq!(inventory.merge_slots(0, 1), Err("Source slot is empty"));

    inventory.slots[0] = Some(Item { vnum: 100, count: 50, has_unique_attributes: false });
    inventory.slots[1] = None;

    assert_eq!(inventory.merge_slots(0, 1), Err("Destination slot is empty"));
}

#[test]
fn test_merge_different_vnums() {
    let mut inventory = Inventory::new(2);
    inventory.slots[0] = Some(Item { vnum: 100, count: 50, has_unique_attributes: false });
    inventory.slots[1] = Some(Item { vnum: 101, count: 50, has_unique_attributes: false });

    assert_eq!(inventory.merge_slots(0, 1), Err("Items have different VNUMs"));
}

#[test]
fn test_merge_unique_attributes() {
    let mut inventory = Inventory::new(2);
    inventory.slots[0] = Some(Item { vnum: 100, count: 50, has_unique_attributes: true });
    inventory.slots[1] = Some(Item { vnum: 100, count: 50, has_unique_attributes: false });

    assert_eq!(inventory.merge_slots(0, 1), Err("Cannot merge items with unique attributes"));

    inventory.slots[0] = Some(Item { vnum: 100, count: 50, has_unique_attributes: false });
    inventory.slots[1] = Some(Item { vnum: 100, count: 50, has_unique_attributes: true });

    assert_eq!(inventory.merge_slots(0, 1), Err("Cannot merge items with unique attributes"));
}

#[test]
fn test_split_stack_partially() {
    let mut inventory = Inventory::new(2);
    inventory.slots[0] = Some(Item { vnum: 150, count: 100, has_unique_attributes: false });

    assert_eq!(inventory.split_slot(0, 1, 40), Ok(()));

    // Source should have 60 left
    assert_eq!(inventory.slots[0].unwrap().count, 60);
    // Destination should have 40
    assert_eq!(inventory.slots[1].unwrap().count, 40);
    
    assert_eq!(inventory.slots[0].unwrap().vnum, 150);
    assert_eq!(inventory.slots[1].unwrap().vnum, 150);
}

#[test]
fn test_split_stack_fully() {
    let mut inventory = Inventory::new(2);
    inventory.slots[0] = Some(Item { vnum: 150, count: 100, has_unique_attributes: false });

    assert_eq!(inventory.split_slot(0, 1, 100), Ok(()));

    // Source should be empty
    assert_eq!(inventory.slots[0], None);
    // Destination should have 100
    assert_eq!(inventory.slots[1].unwrap().count, 100);
}

#[test]
fn test_split_same_slot() {
    let mut inventory = Inventory::new(1);
    assert_eq!(inventory.split_slot(0, 0, 50), Err("Cannot split into the same slot"));
}

#[test]
fn test_split_out_of_bounds() {
    let mut inventory = Inventory::new(1);
    assert_eq!(inventory.split_slot(0, 1, 50), Err("Slot index out of bounds"));
    assert_eq!(inventory.split_slot(1, 0, 50), Err("Slot index out of bounds"));
}

#[test]
fn test_split_destination_not_empty() {
    let mut inventory = Inventory::new(2);
    inventory.slots[0] = Some(Item { vnum: 150, count: 100, has_unique_attributes: false });
    inventory.slots[1] = Some(Item { vnum: 150, count: 50, has_unique_attributes: false });

    assert_eq!(inventory.split_slot(0, 1, 40), Err("Destination slot is not empty"));
}

#[test]
fn test_split_zero_items() {
    let mut inventory = Inventory::new(2);
    inventory.slots[0] = Some(Item { vnum: 150, count: 100, has_unique_attributes: false });

    assert_eq!(inventory.split_slot(0, 1, 0), Err("Cannot split 0 items"));
}

#[test]
fn test_split_empty_source() {
    let mut inventory = Inventory::new(2);

    assert_eq!(inventory.split_slot(0, 1, 50), Err("Source slot is empty"));
}

#[test]
fn test_split_too_many_items() {
    let mut inventory = Inventory::new(2);
    inventory.slots[0] = Some(Item { vnum: 150, count: 100, has_unique_attributes: false });

    assert_eq!(inventory.split_slot(0, 1, 150), Err("Not enough items to split"));
}
