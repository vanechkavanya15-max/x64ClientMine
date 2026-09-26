use m2_client::texture_atlas_cache::TextureCache;

#[test]
fn test_cache_add_and_get() {
    let mut cache = TextureCache::new(1024 * 1024); // 1 MB limit

    let result = cache.get_or_load("d/ymir work/ui/test.png", 100, || {
        Ok((128, 128, vec![255; 128 * 128 * 4]))
    });

    assert!(result.is_ok());
    let tex = result.unwrap();
    assert_eq!(tex.width, 128);
    assert_eq!(tex.height, 128);
    assert_eq!(tex.last_used_tick, 100);
    assert_eq!(tex.rgba_data.len(), 128 * 128 * 4);
    assert_eq!(cache.current_memory_usage(), 128 * 128 * 4);

    // Get it again, should return from cache
    let result_again = cache.get_or_load("d/ymir work/ui/test.png", 200, || {
        panic!("Should not be called!");
    });

    assert!(result_again.is_ok());
    let tex_again = result_again.unwrap();
    assert_eq!(tex_again.last_used_tick, 200); // Updated tick
    assert_eq!(cache.current_memory_usage(), 128 * 128 * 4); // Memory shouldn't increase
}

#[test]
fn test_cache_tick_update() {
    let mut cache = TextureCache::new(1024 * 1024);

    let _ = cache.get_or_load("a.png", 10, || Ok((10, 10, vec![0; 400])));
    let _ = cache.get_or_load("b.png", 20, || Ok((10, 10, vec![0; 400])));

    // Access "a.png" again to update its tick
    let tex = cache.get_or_load("a.png", 30, || panic!("Not needed")).unwrap();
    assert_eq!(tex.last_used_tick, 30);
}

#[test]
fn test_cache_lru_eviction() {
    // 1000 bytes limit
    let mut cache = TextureCache::new(1000);

    // Add A: 400 bytes
    assert!(cache.get_or_load("a.png", 10, || Ok((10, 10, vec![1; 400]))).is_ok());
    assert_eq!(cache.current_memory_usage(), 400);

    // Add B: 400 bytes
    assert!(cache.get_or_load("b.png", 20, || Ok((10, 10, vec![2; 400]))).is_ok());
    assert_eq!(cache.current_memory_usage(), 800);

    // Add C: 400 bytes, total 1200 > 1000. 
    // "a.png" is oldest (tick 10), so it should be evicted.
    assert!(cache.get_or_load("c.png", 30, || Ok((10, 10, vec![3; 400]))).is_ok());
    assert_eq!(cache.current_memory_usage(), 800); // B + C

    // Verify A is gone
    let mut a_loaded = false;
    let _ = cache.get_or_load("a.png", 40, || {
        a_loaded = true;
        Ok((10, 10, vec![1; 400]))
    });
    assert!(a_loaded, "A should have been evicted and reloaded");
    
    // Now B was oldest (tick 20), C (tick 30).
    // Adding A again (tick 40) pushes total to 1200. B gets evicted.
    assert_eq!(cache.current_memory_usage(), 800); // C + A
}

#[test]
fn test_cache_immediate_eviction() {
    // Edge case: single texture is larger than the limit
    let mut cache = TextureCache::new(500);

    let result = cache.get_or_load("huge.png", 10, || {
        Ok((20, 20, vec![0; 1600])) // 1600 > 500
    });

    assert!(result.is_err());
    assert_eq!(result.err(), Some("Texture was loaded but immediately evicted due to memory limits"));
    assert_eq!(cache.current_memory_usage(), 0);
}

#[test]
fn test_cache_clear() {
    let mut cache = TextureCache::new(10000);
    let _ = cache.get_or_load("a", 1, || Ok((1, 1, vec![0; 4])));
    assert_eq!(cache.current_memory_usage(), 4);
    
    cache.clear();
    assert_eq!(cache.current_memory_usage(), 0);
    
    let mut a_reloaded = false;
    let _ = cache.get_or_load("a", 2, || {
        a_reloaded = true;
        Ok((1, 1, vec![0; 4]))
    });
    assert!(a_reloaded);
}

#[test]
fn test_cache_path_normalization() {
    let mut cache = TextureCache::new(10000);
    
    let _ = cache.get_or_load("D:\\Ymir Work\\UI\\Test.PNG", 100, || {
        Ok((2, 2, vec![0; 16]))
    });

    // Should hit cache despite different casing and slashes
    let mut reloaded = false;
    let _ = cache.get_or_load("d:/ymir work/ui/test.png", 200, || {
        reloaded = true;
        Ok((2, 2, vec![0; 16]))
    });
    
    assert!(!reloaded, "Texture was not found in cache due to path normalization failure");
    
    let tex = cache.get_or_load("d:/ymir work/ui/test.png", 300, || panic!("")).unwrap();
    assert_eq!(tex.last_used_tick, 300);
}

#[test]
fn test_cache_load_error() {
    let mut cache = TextureCache::new(10000);
    let result = cache.get_or_load("err.png", 10, || {
        Err("Failed to read file")
    });
    assert!(result.is_err());
    assert_eq!(result.err(), Some("Failed to read file"));
    assert_eq!(cache.current_memory_usage(), 0);
}
