use m2_client_bridge::state::inventory_item::{InventoryItem, ItemAttribute};

#[test]
fn test_item_creation() {
    let item = InventoryItem::new(123, 456, 1);
    assert_eq!(item.vid, 123);
    assert_eq!(item.vnum, 456);
    assert_eq!(item.count, 1);
    assert_eq!(item.sockets, [0; 3]);
    assert_eq!(item.attributes, [ItemAttribute { b_type: 0, value: 0 }; 7]);
    assert!(item.is_valid());
}

#[test]
fn test_invalid_vid() {
    let mut item = InventoryItem::new(0, 456, 1);
    assert!(!item.is_valid());

    assert_eq!(item.set_socket(0, 100), Err("Niepoprawne VID przedmiotu"));
    assert_eq!(item.get_socket(0), None);

    assert_eq!(item.set_attribute(0, 1, 10), Err("Niepoprawne VID przedmiotu"));
    assert_eq!(item.get_attribute(0), None);
}

#[test]
fn test_valid_operations() {
    let mut item = InventoryItem::new(1, 100, 1);

    assert_eq!(item.set_socket(1, 1234), Ok(()));
    assert_eq!(item.get_socket(1), Some(1234));

    assert_eq!(item.set_attribute(3, 5, 100), Ok(()));
    assert_eq!(
        item.get_attribute(3),
        Some(ItemAttribute { b_type: 5, value: 100 })
    );
}

#[test]
fn test_set_socket_bounds() {
    let mut item = InventoryItem::new(1, 100, 1);
    assert_eq!(item.set_socket(3, 100), Err("Indeks gniazda poza zakresem"));
}

#[test]
fn test_get_socket_bounds() {
    let item = InventoryItem::new(1, 100, 1);
    assert_eq!(item.get_socket(3), None);
}

#[test]
fn test_set_attribute_bounds() {
    let mut item = InventoryItem::new(1, 100, 1);
    assert_eq!(item.set_attribute(7, 1, 10), Err("Indeks bonusu poza zakresem"));
}

#[test]
fn test_get_attribute_bounds() {
    let item = InventoryItem::new(1, 100, 1);
    assert_eq!(item.get_attribute(7), None);
}
