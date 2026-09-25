use m2_client::player_core::{
    PlayerCore, ItemData, POINT_HP, POINT_GOLD, POINT_MAX_NUM
};

#[test]
fn test_integration_player_core() {
    let instance = PlayerCore::new();
    assert!(instance.is_ready());
}

#[test]
fn test_points_obfuscation() {
    let mut player = PlayerCore::new();
    
    player.set_point(POINT_HP, 1000);
    assert_eq!(player.get_point(POINT_HP), 1000);

    player.set_point(POINT_GOLD, 50000);
    assert_eq!(player.get_point(POINT_GOLD), 50000);

    // Test out of bounds point
    player.set_point(POINT_MAX_NUM + 1, 999);
    assert_eq!(player.get_point(POINT_MAX_NUM + 1), 0);
}

#[test]
fn test_inventory_and_equipment() {
    let mut player = PlayerCore::new();

    let mut item = ItemData::default();
    item.vnum = 11299; // Some armor
    item.count = 1;
    
    player.set_inventory_item(5, item);
    
    let retrieved_item = player.get_inventory_item(5).unwrap();
    assert_eq!(retrieved_item.vnum, 11299);
    assert_eq!(retrieved_item.count, 1);

    // Test default item on another slot
    let empty_item = player.get_inventory_item(6).unwrap();
    assert_eq!(empty_item.vnum, 0);

    // Test out of bounds inventory
    player.set_inventory_item(9999, item);
    assert!(player.get_inventory_item(9999).is_none());

    let mut equip = ItemData::default();
    equip.vnum = 299; // Some sword
    player.set_equipment_item(4, equip);
    
    let retrieved_equip = player.get_equipment_item(4).unwrap();
    assert_eq!(retrieved_equip.vnum, 299);
    
    // Test out of bounds equipment
    player.set_equipment_item(9999, equip);
    assert!(player.get_equipment_item(9999).is_none());
}

#[test]
fn test_target_vid() {
    let mut player = PlayerCore::new();
    
    assert_eq!(player.get_target(), 0);
    
    player.set_target(12345);
    assert_eq!(player.get_target(), 12345);
    
    player.clear_target();
    assert_eq!(player.get_target(), 0);
}

#[test]
fn test_is_dead() {
    let mut player = PlayerCore::new();
    
    // Initially hp is 0
    assert!(player.is_dead());
    
    player.set_point(POINT_HP, 100);
    assert!(!player.is_dead());
    
    player.set_point(POINT_HP, 0);
    assert!(player.is_dead());
    
    player.set_point(POINT_HP, -50);
    assert!(player.is_dead());
}
