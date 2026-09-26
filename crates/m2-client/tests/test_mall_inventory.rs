use m2_client::mall_inventory::MallInventory;
use m2_client_bridge::state::inventory::Inventory;
use m2_client_bridge::state::ItemData;

#[test]
fn test_checkout_mall_item_success() {
    let mut mall = MallInventory::new();
    let mut inv = Inventory::new();

    // Populate mall with an item
    let item = ItemData::new(50300, 1);
    mall.set_item(5, item.clone()).unwrap();

    // Ensure inventory is empty
    assert_eq!(inv.get(0).unwrap(), 0);

    // Checkout the item
    let checked_out_item = mall.checkout_mall_item(5, &mut inv).expect("Failed to checkout item");

    assert_eq!(checked_out_item.vnum, 50300);
    assert_eq!(checked_out_item.count, 1);

    // Verify mall slot is empty
    let res = mall.checkout_mall_item(5, &mut inv);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "No item at the specified mall slot");

    // Verify item is now in the inventory (in the first empty slot, which is 0)
    assert_eq!(inv.get(0).unwrap(), 50300);
}

#[test]
fn test_prevent_putting_items_to_mall() {
    let mut mall = MallInventory::new();
    let mut inv = Inventory::new();

    // Attempt to put item into mall from inventory
    let result = mall.insert_from_inventory(0, &mut inv, 0);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Przedmioty w Mall moga byc wylacznie wyciagane do ekwipunku");
}

#[test]
fn test_refuse_checkout_on_full_inventory() {
    let mut mall = MallInventory::new();
    let mut inv = Inventory::new();

    // Populate mall with an item
    let item = ItemData::new(27001, 200);
    mall.set_item(0, item.clone()).unwrap();

    // Fill the inventory completely
    for i in 0..180 {
        inv.set(i, 19).unwrap(); // Some random item vnum
    }

    // Attempt to checkout
    let result = mall.checkout_mall_item(0, &mut inv);

    // Should return error due to full inventory
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Brak miejsca w inwentarzu");

    // Verify item is still in the mall (by checking out to a new empty inventory)
    let mut empty_inv = Inventory::new();
    let item_again = mall.checkout_mall_item(0, &mut empty_inv).expect("Failed to checkout from non-empty mall slot");
    assert_eq!(item_again.vnum, 27001);
}

#[test]
fn test_checkout_invalid_mall_slot() {
    let mut mall = MallInventory::new();
    let mut inv = Inventory::new();

    let result = mall.checkout_mall_item(45, &mut inv);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid mall slot index");
}

#[test]
fn test_set_item_invalid_mall_slot() {
    let mut mall = MallInventory::new();
    let item = ItemData::new(50300, 1);
    
    let result = mall.set_item(45, item);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid slot index");
}
