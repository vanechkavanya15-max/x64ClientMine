use m2_client::item_manager::{ItemManager, PixelPosition, VNUM_MONEY};

#[test]
fn test_integration_item_manager() {
    let instance = ItemManager::new();
    assert!(instance.is_ready());
}

#[test]
fn test_create_and_delete_item() {
    let mut manager = ItemManager::new();
    
    // Testowanie dodawania przedmiotu
    manager.create_item(1001, 500, 10.0, 20.0, 30.0, true);
    assert_eq!(manager.get_virtual_number_of_ground_item(1001), 500);
    
    let pos = manager.get_ground_item_position(1001).unwrap();
    assert_eq!(pos.x, 10.0);
    assert_eq!(pos.y, 20.0);
    assert_eq!(pos.z, 30.0);
    
    // Usuwanie przedmiotu
    manager.delete_item(1001);
    assert_eq!(manager.get_virtual_number_of_ground_item(1001), 0);
    assert!(manager.get_ground_item_position(1001).is_none());
}

#[test]
fn test_ownership() {
    let mut manager = ItemManager::new();
    manager.create_item(2002, 300, 0.0, 0.0, 0.0, true);
    
    // Zmiana i weryfikacja ownershipu
    manager.set_ownership(2002, "Player1");
    assert_eq!(manager.get_ownership(2002).unwrap(), "Player1");
    
    // Ownership nieistniejacego elementu
    assert!(manager.get_ownership(9999).is_none());
}

#[test]
fn test_get_close_item() {
    let mut manager = ItemManager::new();
    manager.create_item(100, 10, 100.0, 100.0, 0.0, true);
    manager.create_item(200, 10, 200.0, 200.0, 0.0, true);
    
    // Blisko przedmiotu 100
    let pos1 = PixelPosition { x: 105.0, y: -105.0, z: 0.0 };
    let close_item = manager.get_close_item(&pos1, 300);
    assert_eq!(close_item, Some(100));
    
    // Blisko przedmiotu 200
    let pos2 = PixelPosition { x: 195.0, y: -195.0, z: 0.0 };
    let close_item = manager.get_close_item(&pos2, 300);
    assert_eq!(close_item, Some(200));
    
    // Za daleko
    let pos3 = PixelPosition { x: 1000.0, y: -1000.0, z: 0.0 };
    let close_item = manager.get_close_item(&pos3, 100);
    assert_eq!(close_item, None);
}

#[test]
fn test_get_close_money() {
    let mut manager = ItemManager::new();
    manager.create_item(100, 10, 100.0, 100.0, 0.0, true); // Zwykly item
    manager.create_item(200, VNUM_MONEY, 200.0, 200.0, 0.0, true); // Pieniadze
    
    // Szukanie pieniedzy obok zwyklego przedmiotu nie zwroci go
    let pos1 = PixelPosition { x: 100.0, y: -100.0, z: 0.0 };
    let close_money = manager.get_close_money(&pos1, 500);
    assert_eq!(close_money, Some(200)); // znajdzie odleglejsze pieniadze ale z dystansem 500
    
    // Za daleko na pieniadze
    let close_money = manager.get_close_money(&pos1, 50);
    assert_eq!(close_money, None);
    
    // Znajdowanie bliskich pieniedzy
    let pos2 = PixelPosition { x: 200.0, y: -200.0, z: 0.0 };
    let close_money = manager.get_close_money(&pos2, 300);
    assert_eq!(close_money, Some(200));
}

#[test]
fn test_delete_all_items() {
    let mut manager = ItemManager::new();
    manager.create_item(1, 10, 0.0, 0.0, 0.0, true);
    manager.create_item(2, 20, 0.0, 0.0, 0.0, true);
    manager.create_item(3, 30, 0.0, 0.0, 0.0, true);
    
    manager.delete_all_items();
    
    assert_eq!(manager.get_virtual_number_of_ground_item(1), 0);
    assert_eq!(manager.get_virtual_number_of_ground_item(2), 0);
    assert_eq!(manager.get_virtual_number_of_ground_item(3), 0);
}

#[test]
fn test_update_and_stubs() {
    let mut manager = ItemManager::new();
    
    manager.create_item(55, 33, 0.0, 0.0, 0.0, true);
    let item = manager.ground_items.get_mut(&55).unwrap();
    item.end_time = 1000;
    
    assert_eq!(item.anim_ended, false);
    
    // Simulate current_time advancing beyond end_time
    manager.update(1500);
    
    let item = manager.ground_items.get(&55).unwrap();
    assert_eq!(item.anim_ended, true);
    
    // Run stubs to ensure no panics
    manager.render();
    manager.play_use_sound(55);
    manager.play_drop_sound(55);
    manager.play_use_potion_sound();
    manager.set_use_sound_file_name(0, "sound1");
    manager.set_drop_sound_file_name(0, "sound2");
    
    let info = manager.get_info();
    assert_eq!(info, "Item: Inst 1, Pool ?");
    
    assert_eq!(manager.get_no_grade_name_data_count(), 0);
}
