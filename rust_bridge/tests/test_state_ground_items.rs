use m2_client_bridge::state::ground_items::{GroundItemsCache, GroundItem, m2_ground_find_nearest};

#[test]
fn test_ground_items_cache_add_and_get() {
    let mut cache = GroundItemsCache::new();
    let item = GroundItem {
        vid: 100,
        vnum: 5001,
        x: 10,
        y: 20,
        z: 30,
    };
    cache.add_item(item);

    let retrieved = cache.get_item(100);
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.vid, 100);
    assert_eq!(retrieved.vnum, 5001);
    assert_eq!(retrieved.x, 10);
    assert_eq!(retrieved.y, 20);
    assert_eq!(retrieved.z, 30);
}

#[test]
fn test_ground_items_cache_get_invalid() {
    let cache = GroundItemsCache::new();
    assert!(cache.get_item(999).is_none());
}

#[test]
fn test_ground_items_cache_remove() {
    let mut cache = GroundItemsCache::new();
    let item = GroundItem {
        vid: 100,
        vnum: 5001,
        x: 10,
        y: 20,
        z: 30,
    };
    cache.add_item(item);
    
    assert!(cache.remove_item(100));
    assert!(cache.get_item(100).is_none());

    // Usuwanie nieistniejacego
    assert!(!cache.remove_item(999));
}

#[test]
fn test_ground_items_cache_clear() {
    let mut cache = GroundItemsCache::new();
    cache.add_item(GroundItem { vid: 100, vnum: 5001, x: 10, y: 20, z: 30 });
    cache.add_item(GroundItem { vid: 101, vnum: 5002, x: 11, y: 21, z: 31 });

    cache.clear_items();
    assert!(cache.get_item(100).is_none());
    assert!(cache.get_item(101).is_none());
    assert_eq!(cache.get_all_items().len(), 0);
}

#[test]
fn test_ground_items_cache_add_overwrite() {
    let mut cache = GroundItemsCache::new();
    cache.add_item(GroundItem { vid: 100, vnum: 5001, x: 10, y: 20, z: 30 });
    // Nadpisz ten sam VID
    cache.add_item(GroundItem { vid: 100, vnum: 6001, x: 15, y: 25, z: 35 });

    let retrieved = cache.get_item(100).unwrap();
    assert_eq!(retrieved.vnum, 6001);
    assert_eq!(retrieved.x, 15);
    assert_eq!(retrieved.y, 25);
    assert_eq!(retrieved.z, 35);
}

#[test]
fn test_ground_items_cache_get_all() {
    let mut cache = GroundItemsCache::new();
    cache.add_item(GroundItem { vid: 100, vnum: 5001, x: 10, y: 20, z: 30 });
    cache.add_item(GroundItem { vid: 101, vnum: 5002, x: 11, y: 21, z: 31 });

    let items = cache.get_all_items();
    assert_eq!(items.len(), 2);
    
    let has_100 = items.iter().any(|i| i.vid == 100);
    let has_101 = items.iter().any(|i| i.vid == 101);
    assert!(has_100);
    assert!(has_101);
}

#[test]
fn test_ground_items_cache_find_nearest() {
    let mut cache = GroundItemsCache::new();
    cache.add_item(GroundItem { vid: 100, vnum: 5001, x: 10, y: 10, z: 0 }); // dist 0
    cache.add_item(GroundItem { vid: 101, vnum: 5002, x: 20, y: 20, z: 0 }); // dist ~14.14
    cache.add_item(GroundItem { vid: 102, vnum: 5003, x: 100, y: 100, z: 0 }); // dist 127.27

    assert_eq!(cache.find_nearest(10, 10), Some(100));
    assert_eq!(cache.find_nearest(18, 18), Some(101));
    assert_eq!(cache.find_nearest(1000, 1000), Some(102));
    
    let empty_cache = GroundItemsCache::new();
    assert_eq!(empty_cache.find_nearest(10, 10), None);
}

#[test]
fn test_m2_ground_find_nearest() {
    let cache_lock = GroundItemsCache::instance();
    
    {
        let mut cache = cache_lock.write().unwrap();
        cache.clear_items();
        cache.add_item(GroundItem { vid: 200, vnum: 7001, x: 50, y: 50, z: 0 });
        cache.add_item(GroundItem { vid: 201, vnum: 7002, x: 150, y: 150, z: 0 });
    }

    assert_eq!(m2_ground_find_nearest(51, 51), 200);
    assert_eq!(m2_ground_find_nearest(149, 149), 201);
    
    {
        let mut cache = cache_lock.write().unwrap();
        cache.clear_items();
    }
    
    assert_eq!(m2_ground_find_nearest(50, 50), 0);
}
