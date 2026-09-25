use m2_client_bridge::state::ground_items::GroundItemManager;

#[test]
fn test_ground_item_manager_add_and_get() {
    let mut manager = GroundItemManager::new();
    manager.add_item(100, 5001, 10.0, 20.0, 30.0, 1);

    let item = manager.get_item(100);
    assert!(item.is_some());
    let item = item.unwrap();
    assert_eq!(item.vid, 100);
    assert_eq!(item.vnum, 5001);
    assert_eq!(item.x, 10.0);
    assert_eq!(item.y, 20.0);
    assert_eq!(item.z, 30.0);
    assert_eq!(item.owner, 1);
}

#[test]
fn test_ground_item_manager_get_invalid() {
    let manager = GroundItemManager::new();
    assert!(manager.get_item(999).is_none());
}

#[test]
fn test_ground_item_manager_remove() {
    let mut manager = GroundItemManager::new();
    manager.add_item(100, 5001, 10.0, 20.0, 30.0, 1);
    
    assert!(manager.remove_item(100));
    assert!(manager.get_item(100).is_none());

    // Usuwanie nieistniejacego
    assert!(!manager.remove_item(999));
}

#[test]
fn test_ground_item_manager_update_owner() {
    let mut manager = GroundItemManager::new();
    manager.add_item(100, 5001, 10.0, 20.0, 30.0, 1);

    assert!(manager.update_item_owner(100, 2));
    let item = manager.get_item(100).unwrap();
    assert_eq!(item.owner, 2);

    // Aktualizacja nieistniejacego
    assert!(!manager.update_item_owner(999, 2));
}

#[test]
fn test_ground_item_manager_clear() {
    let mut manager = GroundItemManager::new();
    manager.add_item(100, 5001, 10.0, 20.0, 30.0, 1);
    manager.add_item(101, 5002, 11.0, 21.0, 31.0, 2);

    manager.clear_items();
    assert!(manager.get_item(100).is_none());
    assert!(manager.get_item(101).is_none());
    assert_eq!(manager.get_all_items().len(), 0);
}

#[test]
fn test_ground_item_manager_add_overwrite() {
    let mut manager = GroundItemManager::new();
    manager.add_item(100, 5001, 10.0, 20.0, 30.0, 1);
    // Nadpisz ten sam VID
    manager.add_item(100, 6001, 15.0, 25.0, 35.0, 3);

    let item = manager.get_item(100).unwrap();
    assert_eq!(item.vnum, 6001);
    assert_eq!(item.x, 15.0);
    assert_eq!(item.y, 25.0);
    assert_eq!(item.z, 35.0);
    assert_eq!(item.owner, 3);
}

#[test]
fn test_ground_item_manager_get_all() {
    let mut manager = GroundItemManager::new();
    manager.add_item(100, 5001, 10.0, 20.0, 30.0, 1);
    manager.add_item(101, 5002, 11.0, 21.0, 31.0, 2);

    let items = manager.get_all_items();
    assert_eq!(items.len(), 2);
    
    let has_100 = items.iter().any(|i| i.vid == 100);
    let has_101 = items.iter().any(|i| i.vid == 101);
    assert!(has_100);
    assert!(has_101);
}
