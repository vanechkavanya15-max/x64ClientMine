use m2_client::area_chunk_loader_v2::*;
use std::ptr;

#[test]
fn test_cache_initialization() {
    let cache = AreaChunkCache::new(5);
    assert_eq!(cache.get_capacity(), 5);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

#[test]
fn test_cache_default_capacity() {
    let cache = AreaChunkCache::new(0);
    assert_eq!(cache.get_capacity(), 16);
}

#[test]
fn test_load_sector_success() {
    let mut cache = AreaChunkCache::new(10);
    let mut raw_data = vec![0u8; RAW_HEIGHT_BYTES];
    
    // Set some recognizable pattern (little-endian u16)
    // 500 = 0x01F4 -> [0xF4, 0x01]
    raw_data[0] = 0xF4;
    raw_data[1] = 0x01;
    // 1000 = 0x03E8 -> [0xE8, 0x03]
    raw_data[2] = 0xE8;
    raw_data[3] = 0x03;

    assert!(cache.load_sector(1, 1, &raw_data));
    assert_eq!(cache.len(), 1);

    let sector = cache.get_sector(1, 1).expect("Sector should be loaded");
    assert_eq!(sector.version, 1);
    assert_eq!(sector.heights.len(), HEIGHT_MAP_SIZE);
    
    // Check values were properly converted to f32
    assert_eq!(sector.heights[0], 500.0);
    assert_eq!(sector.heights[1], 1000.0);
    // Rest should be 0.0
    assert_eq!(sector.heights[2], 0.0);
    assert_eq!(sector.heights[HEIGHT_MAP_SIZE - 1], 0.0);
}

#[test]
fn test_load_sector_insufficient_data() {
    let mut cache = AreaChunkCache::new(10);
    let raw_data = vec![0u8; RAW_HEIGHT_BYTES - 1]; // One byte short
    
    assert!(!cache.load_sector(1, 1, &raw_data));
    assert_eq!(cache.len(), 0);
}

#[test]
fn test_lru_eviction() {
    let mut cache = AreaChunkCache::new(3); // Capacity 3
    let raw_data = vec![0u8; RAW_HEIGHT_BYTES];

    // Load 3 sectors
    cache.load_sector(0, 0, &raw_data); // V=1
    cache.load_sector(1, 0, &raw_data); // V=2
    cache.load_sector(2, 0, &raw_data); // V=3

    assert_eq!(cache.len(), 3);
    assert!(cache.get_sector(0, 0).is_some());
    assert!(cache.get_sector(1, 0).is_some());
    assert!(cache.get_sector(2, 0).is_some());

    // Access (0,0) to make it the most recently used
    let _ = cache.get_sector(0, 0);

    // Load a 4th sector, which should evict the oldest accessed, which is now (1,0)
    cache.load_sector(3, 0, &raw_data);

    assert_eq!(cache.len(), 3);
    assert!(cache.get_sector(3, 0).is_some()); // Newest
    assert!(cache.get_sector(0, 0).is_some()); // Recently used
    assert!(cache.get_sector(2, 0).is_some()); // Not evicted yet
    
    // (1,0) should have been evicted
    assert!(cache.get_sector(1, 0).is_none());
}

#[test]
fn test_update_existing_sector() {
    let mut cache = AreaChunkCache::new(3);
    let mut raw_data1 = vec![0u8; RAW_HEIGHT_BYTES];
    raw_data1[0] = 0x01; // height = 1
    
    let mut raw_data2 = vec![0u8; RAW_HEIGHT_BYTES];
    raw_data2[0] = 0x02; // height = 2

    assert!(cache.load_sector(5, 5, &raw_data1));
    let s1 = cache.get_sector(5, 5).unwrap();
    assert_eq!(s1.heights[0], 1.0);
    assert_eq!(s1.version, 1);

    // Load same sector again with different data
    assert!(cache.load_sector(5, 5, &raw_data2));
    let s2 = cache.get_sector(5, 5).unwrap();
    assert_eq!(s2.heights[0], 2.0);
    assert_eq!(s2.version, 2);
    
    assert_eq!(cache.len(), 1);
}

#[test]
fn test_ffi_load_sector() {
    // Clear global cache before test
    if let Ok(mut cache) = AREA_CACHE.write() {
        cache.clear();
    }

    let mut raw_data = vec![0u8; RAW_HEIGHT_BYTES];
    raw_data[0] = 0x99;
    raw_data[1] = 0x00; // 0x0099 = 153

    // Test successful FFI call
    let result = m2_area_load_sector(10, 10, raw_data.as_ptr());
    assert!(result);

    // Verify side effect in global cache
    if let Ok(mut cache) = AREA_CACHE.write() {
        assert_eq!(cache.len(), 1);
        let s = cache.get_sector(10, 10).unwrap();
        assert_eq!(s.heights[0], 153.0);
    } else {
        panic!("Failed to acquire write lock");
    }

    // Test null pointer
    let result_null = m2_area_load_sector(11, 11, ptr::null());
    assert!(!result_null);
    
    if let Ok(cache) = AREA_CACHE.read() {
        assert_eq!(cache.len(), 1); // Remains 1
    }
}

#[test]
fn test_cache_clear() {
    let mut cache = AreaChunkCache::new(5);
    let raw_data = vec![0u8; RAW_HEIGHT_BYTES];
    
    cache.load_sector(1, 1, &raw_data);
    cache.load_sector(2, 2, &raw_data);
    
    assert_eq!(cache.len(), 2);
    
    cache.clear();
    
    assert_eq!(cache.len(), 0);
    assert!(cache.get_sector(1, 1).is_none());
}
