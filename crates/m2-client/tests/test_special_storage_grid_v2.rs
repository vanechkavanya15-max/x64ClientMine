use m2_client::special_storage_grid_v2::*;

#[test]
fn test_special_storage_grid_initialization() {
    let grid = SpecialStorageGridV2::new();
    
    // Check initial state
    assert_eq!(grid.get_item(SpecialStorageType::SkillBook, 0), None);
    assert_eq!(grid.get_item(SpecialStorageType::Stone, 179), None);
    assert_eq!(grid.get_item(SpecialStorageType::Upgrade, 45), None);
}

#[test]
fn test_determine_storage_type_by_item_type() {
    // 17 = Skillbook, 10 = Metin(stone), 5 = Material(upgrade)
    assert_eq!(
        SpecialStorageGridV2::determine_storage_type_by_item_type(17),
        Some(SpecialStorageType::SkillBook)
    );
    assert_eq!(
        SpecialStorageGridV2::determine_storage_type_by_item_type(10),
        Some(SpecialStorageType::Stone)
    );
    assert_eq!(
        SpecialStorageGridV2::determine_storage_type_by_item_type(5),
        Some(SpecialStorageType::Upgrade)
    );
    // Invalid types should return None
    assert_eq!(SpecialStorageGridV2::determine_storage_type_by_item_type(0), None);
    assert_eq!(SpecialStorageGridV2::determine_storage_type_by_item_type(99), None);
}

#[test]
fn test_put_and_get_item() {
    let mut grid = SpecialStorageGridV2::new();
    
    // Put valid item in SkillBook
    assert!(grid.put_item(SpecialStorageType::SkillBook, 10, 50401, 1));
    let item = grid.get_item(SpecialStorageType::SkillBook, 10).unwrap();
    assert_eq!(item.vnum, 50401);
    assert_eq!(item.count, 1);
    
    // Out of bounds
    assert!(!grid.put_item(SpecialStorageType::SkillBook, 180, 50401, 1));
    assert_eq!(grid.get_item(SpecialStorageType::SkillBook, 180), None);
}

#[test]
fn test_remove_item() {
    let mut grid = SpecialStorageGridV2::new();
    grid.put_item(SpecialStorageType::Stone, 5, 28430, 1);
    
    // Verify it exists
    assert!(grid.get_item(SpecialStorageType::Stone, 5).is_some());
    
    // Remove it
    assert!(grid.remove_item(SpecialStorageType::Stone, 5));
    
    // Verify it's gone
    assert!(grid.get_item(SpecialStorageType::Stone, 5).is_none());
    
    // Remove empty slot
    assert!(!grid.remove_item(SpecialStorageType::Stone, 5));
    
    // Remove out of bounds
    assert!(!grid.remove_item(SpecialStorageType::Stone, 180));
}

#[test]
fn test_swap_items() {
    let mut grid = SpecialStorageGridV2::new();
    grid.put_item(SpecialStorageType::Upgrade, 0, 27992, 1);
    grid.put_item(SpecialStorageType::Upgrade, 1, 27993, 2);
    
    assert!(grid.swap_items(SpecialStorageType::Upgrade, 0, 1));
    
    let item0 = grid.get_item(SpecialStorageType::Upgrade, 0).unwrap();
    assert_eq!(item0.vnum, 27993);
    assert_eq!(item0.count, 2);
    
    let item1 = grid.get_item(SpecialStorageType::Upgrade, 1).unwrap();
    assert_eq!(item1.vnum, 27992);
    assert_eq!(item1.count, 1);
    
    // Out of bounds swap
    assert!(!grid.swap_items(SpecialStorageType::Upgrade, 0, 180));
}

#[test]
fn test_ffi_m2_special_storage_put() {
    // Put to SkillBook (0)
    assert!(m2_special_storage_put(0, 0, 50401, 1));
    
    // Put to Stone (1)
    assert!(m2_special_storage_put(1, 100, 28430, 1));
    
    // Put to Upgrade (2)
    assert!(m2_special_storage_put(2, 179, 27992, 5));
    
    // Invalid storage type (3)
    assert!(!m2_special_storage_put(3, 0, 1234, 1));
    
    // Invalid slot (180)
    assert!(!m2_special_storage_put(0, 180, 1234, 1));
}
