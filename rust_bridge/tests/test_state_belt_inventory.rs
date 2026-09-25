use m2_client_bridge::state::belt_inventory::*;
use m2_client_bridge::state::inventory_item::InventoryItem;

#[test]
fn test_belt_inventory_unlocked_slots() {
    let mut belt = BeltInventory::new();
    assert_eq!(belt.unlocked_slots, 0);

    belt.set_unlocked(4);
    assert_eq!(belt.unlocked_slots, 4);

    belt.set_unlocked(20);
    assert_eq!(belt.unlocked_slots, 16);
}

#[test]
fn test_belt_inventory_put_item_blocked() {
    let mut belt = BeltInventory::new();
    belt.set_unlocked(2);

    let item = InventoryItem::new(1, 27001, 10);

    // Slot 0 and 1 are unlocked
    assert!(belt.put_item(0, item.clone()).is_ok());
    assert!(belt.put_item(1, item.clone()).is_ok());

    // Slot 2 is blocked
    assert!(belt.put_item(2, item).is_err());
}

#[test]
fn test_belt_inventory_ffi() {
    unsafe {
        m2_belt_init();
        m2_belt_set_unlocked(2);

        let item = InventoryItem::new(1, 50051, 1);
        assert!(m2_belt_put_item_rust(0, item).is_ok());

        assert_eq!(m2_belt_get_vnum(0), 50051);
        assert_eq!(m2_belt_get_vnum(1), 0); // Empty but unlocked
        assert_eq!(m2_belt_get_vnum(2), 0); // Blocked

        m2_belt_cleanup();
    }
}
