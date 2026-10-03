#[path = "../src/full-grn-anim_13.rs"]
mod full_grn_anim_13;

use full_grn_anim_13::granny_model_lru_cache::{GrannyModelLruCache, GrannyModelData};
use full_grn_anim_13::granny_model_lru_cache::granny_gr2_binary_parser::{GrannyMesh, GrannySkeleton};
use std::sync::Arc;

fn create_dummy_data() -> Arc<GrannyModelData> {
    Arc::new(GrannyModelData {
        meshes: vec![],
        skeletons: vec![],
    })
}

#[test]
fn test_lru_cache_add_and_get() {
    let mut cache = GrannyModelLruCache::new(2);
    let data = create_dummy_data();
    
    cache.put("model1".to_string(), Arc::clone(&data));
    assert_eq!(cache.len(), 1);
    
    let retrieved = cache.get("model1");
    assert!(retrieved.is_some());
    assert_eq!(Arc::as_ptr(&retrieved.unwrap()), Arc::as_ptr(&data));
}

#[test]
fn test_lru_cache_eviction() {
    let mut cache = GrannyModelLruCache::new(2);
    
    cache.put("m1".to_string(), create_dummy_data());
    cache.put("m2".to_string(), create_dummy_data());
    cache.put("m3".to_string(), create_dummy_data());
    
    assert_eq!(cache.len(), 2);
    // m1 should be evicted because m2 and m3 were added recently
    assert!(cache.get("m1").is_none());
    assert!(cache.get("m2").is_some());
    assert!(cache.get("m3").is_some());
}

#[test]
fn test_lru_cache_update_mru() {
    let mut cache = GrannyModelLruCache::new(2);
    
    cache.put("m1".to_string(), create_dummy_data());
    cache.put("m2".to_string(), create_dummy_data());
    
    // Access m1, making it MRU
    assert!(cache.get("m1").is_some());
    
    // Add m3, causing m2 to be evicted (as m1 is now MRU)
    cache.put("m3".to_string(), create_dummy_data());
    
    assert_eq!(cache.len(), 2);
    assert!(cache.get("m2").is_none());
    assert!(cache.get("m1").is_some());
    assert!(cache.get("m3").is_some());
}

#[test]
fn test_lru_cache_zero_capacity() {
    let mut cache = GrannyModelLruCache::new(0);
    cache.put("m1".to_string(), create_dummy_data());
    
    assert_eq!(cache.len(), 0);
    assert!(cache.get("m1").is_none());
}

#[test]
fn test_lru_cache_overwrite_key() {
    let mut cache = GrannyModelLruCache::new(2);
    let data1 = create_dummy_data();
    let data2 = create_dummy_data();
    
    cache.put("m1".to_string(), Arc::clone(&data1));
    cache.put("m1".to_string(), Arc::clone(&data2));
    
    assert_eq!(cache.len(), 1);
    
    let retrieved = cache.get("m1").unwrap();
    // Pointers should match data2
    assert_eq!(Arc::as_ptr(&retrieved), Arc::as_ptr(&data2));
    assert_ne!(Arc::as_ptr(&retrieved), Arc::as_ptr(&data1));
}

#[test]
fn test_lru_cache_clear() {
    let mut cache = GrannyModelLruCache::new(2);
    cache.put("m1".to_string(), create_dummy_data());
    cache.clear();
    
    assert_eq!(cache.len(), 0);
    assert!(cache.get("m1").is_none());
}
