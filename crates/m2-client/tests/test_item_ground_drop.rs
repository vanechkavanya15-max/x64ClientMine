#[path = "../src/item_ground_drop.rs"]
mod item_ground_drop;

use item_ground_drop::{GroundItem, ItemGroundDropManager, PickupResult};

#[test]
fn test_pickup_success_by_owner_within_protection() {
    let mut manager = ItemGroundDropManager::new();
    manager.drop_item(GroundItem {
        vid: 1,
        vnum: 50051,
        count: 1,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        owner_name: Some("marichka".to_string()),
        drop_time_ms: 1000,
    });

    let result = manager.try_pickup(1, "marichka", 1500); // 0.5s later
    assert_eq!(result, PickupResult::Success);
    assert!(manager.get_item(1).is_none());
}

#[test]
fn test_pickup_blocked_by_foreign_player_within_protection() {
    let mut manager = ItemGroundDropManager::new();
    manager.drop_item(GroundItem {
        vid: 2,
        vnum: 50051,
        count: 1,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        owner_name: Some("marichka".to_string()),
        drop_time_ms: 1000,
    });

    let result = manager.try_pickup(2, "thief", 20000); // 19s later
    assert_eq!(result, PickupResult::Protected("marichka".to_string()));
    assert!(manager.get_item(2).is_some());
}

#[test]
fn test_pickup_success_by_foreign_player_after_protection() {
    let mut manager = ItemGroundDropManager::new();
    manager.drop_item(GroundItem {
        vid: 3,
        vnum: 50051,
        count: 1,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        owner_name: Some("marichka".to_string()),
        drop_time_ms: 1000,
    });

    // 30000 ms = 30 seconds protection
    let result = manager.try_pickup(3, "thief", 31001); // 30.001s later
    assert_eq!(result, PickupResult::Success);
    assert!(manager.get_item(3).is_none());
}

#[test]
fn test_pickup_no_owner() {
    let mut manager = ItemGroundDropManager::new();
    manager.drop_item(GroundItem {
        vid: 4,
        vnum: 50051,
        count: 1,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        owner_name: None,
        drop_time_ms: 1000,
    });

    let result = manager.try_pickup(4, "random_guy", 1500); // 0.5s later, within "protection" time but no owner
    assert_eq!(result, PickupResult::Success);
}

#[test]
fn test_update_despawn_after_180_seconds() {
    let mut manager = ItemGroundDropManager::new();
    manager.drop_item(GroundItem {
        vid: 5,
        vnum: 50051,
        count: 1,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        owner_name: Some("marichka".to_string()),
        drop_time_ms: 1000,
    });

    manager.update(150000); // 149 seconds later -> should remain
    assert!(manager.get_item(5).is_some());

    manager.update(181001); // 180.001 seconds later -> should despawn
    assert!(manager.get_item(5).is_none());
}

#[test]
fn test_pickup_not_found() {
    let mut manager = ItemGroundDropManager::new();
    let result = manager.try_pickup(999, "marichka", 1000);
    assert_eq!(result, PickupResult::NotFound);
}

#[test]
fn test_boundary_protection_time() {
    let mut manager = ItemGroundDropManager::new();
    manager.drop_item(GroundItem {
        vid: 6,
        vnum: 50051,
        count: 1,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        owner_name: Some("marichka".to_string()),
        drop_time_ms: 1000,
    });

    // exactly 30000 elapsed
    let result = manager.try_pickup(6, "thief", 31000);
    assert_eq!(result, PickupResult::Success); // Since condition is elapsed < 30_000, elapsed == 30_000 means false
}

#[test]
fn test_boundary_despawn_time() {
    let mut manager = ItemGroundDropManager::new();
    manager.drop_item(GroundItem {
        vid: 7,
        vnum: 50051,
        count: 1,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        owner_name: Some("marichka".to_string()),
        drop_time_ms: 1000,
    });

    // exactly 180000 elapsed -> should remain
    manager.update(181000); 
    assert!(manager.get_item(7).is_some());
    
    // 180001 elapsed -> should despawn
    manager.update(181001);
    assert!(manager.get_item(7).is_none());
}
