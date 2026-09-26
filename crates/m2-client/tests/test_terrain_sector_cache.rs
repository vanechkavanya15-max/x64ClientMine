use m2_client::terrain_sector_cache::{AttrMap, RawHeightMap, SectorData, TerrainSectorCache, WaterMap};

fn create_dummy_sector(id: u8) -> SectorData {
    SectorData {
        raw_height_map: RawHeightMap {
            data: vec![id as u16; 4],
        },
        attr_map: AttrMap {
            data: vec![id; 4],
        },
        water_map: WaterMap {
            data: vec![id; 4],
        },
    }
}

#[test]
fn test_insert_and_get() {
    let mut cache = TerrainSectorCache::new(2);
    let s1 = create_dummy_sector(1);
    
    cache.insert_sector(1, 1, s1.clone(), 100);
    
    let retrieved = cache.get_sector(1, 1, 150);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().attr_map.data[0], 1);
    
    let not_found = cache.get_sector(2, 2, 160);
    assert!(not_found.is_none());
}

#[test]
fn test_lru_eviction() {
    let mut cache = TerrainSectorCache::new(2);
    let s1 = create_dummy_sector(1);
    let s2 = create_dummy_sector(2);
    let s3 = create_dummy_sector(3);
    
    cache.insert_sector(1, 1, s1, 100);
    cache.insert_sector(2, 2, s2, 200);
    
    // Now the capacity is full (2). Inserting s3 should evict s1 (last used at 100).
    cache.insert_sector(3, 3, s3, 300);
    
    assert!(cache.get_sector(1, 1, 400).is_none());
    assert!(cache.get_sector(2, 2, 400).is_some());
    assert!(cache.get_sector(3, 3, 400).is_some());
}

#[test]
fn test_access_time_update() {
    let mut cache = TerrainSectorCache::new(2);
    let s1 = create_dummy_sector(1);
    let s2 = create_dummy_sector(2);
    let s3 = create_dummy_sector(3);
    
    cache.insert_sector(1, 1, s1, 100);
    cache.insert_sector(2, 2, s2, 200);
    
    // Get s1 at time 250, updating its last used time.
    cache.get_sector(1, 1, 250);
    
    // Now s2 is the oldest (last used at 200).
    cache.insert_sector(3, 3, s3, 300);
    
    // s2 should be evicted, not s1.
    assert!(cache.get_sector(2, 2, 400).is_none());
    assert!(cache.get_sector(1, 1, 400).is_some());
    assert!(cache.get_sector(3, 3, 400).is_some());
}

#[test]
fn test_clear_cache() {
    let mut cache = TerrainSectorCache::new(5);
    cache.insert_sector(1, 1, create_dummy_sector(1), 100);
    cache.insert_sector(2, 2, create_dummy_sector(2), 200);
    
    cache.clear();
    
    assert!(cache.get_sector(1, 1, 300).is_none());
    assert!(cache.get_sector(2, 2, 300).is_none());
    
    // Test that we can insert again after clear
    cache.insert_sector(3, 3, create_dummy_sector(3), 400);
    assert!(cache.get_sector(3, 3, 500).is_some());
}

#[test]
fn test_capacity_zero() {
    let mut cache = TerrainSectorCache::new(0);
    // Should default to 16
    for i in 1..=16 {
        cache.insert_sector(i, i, create_dummy_sector(i as u8), i * 10);
    }
    assert!(cache.get_sector(1, 1, 200).is_some());
    cache.insert_sector(17, 17, create_dummy_sector(17), 300);
    // Because we just accessed (1, 1), the oldest should be (2, 2)
    assert!(cache.get_sector(2, 2, 400).is_none());
    assert!(cache.get_sector(17, 17, 400).is_some());
}

#[test]
fn test_default() {
    let mut cache = TerrainSectorCache::default();
    // Default is 16 capacity
    for i in 1..=17 {
        cache.insert_sector(i, i, create_dummy_sector(i as u8), i * 10);
    }
    // 1 should be evicted
    assert!(cache.get_sector(1, 1, 200).is_none());
    assert!(cache.get_sector(17, 17, 200).is_some());
}
