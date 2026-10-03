#[path = "../src/audio_sample_lru_cache.rs"]
mod audio_sample_lru_cache;

use audio_sample_lru_cache::{AudioSampleLruCache, DecodedAudio};

#[test]
fn test_audio_cache_put_and_get() {
    let mut cache = AudioSampleLruCache::new(1024); // 1 KB max memory
    let audio = DecodedAudio::new(1, 44100, vec![0; 100]); // 100 * 2 bytes = 200 bytes

    assert!(cache.put("sound1.wav", audio).is_ok());
    assert_eq!(cache.current_memory_usage(), 200);

    let cached_audio = cache.get_ref("sound1.wav").expect("Expected audio to be in cache");
    assert_eq!(cached_audio.samples.len(), 100);
}

#[test]
fn test_audio_cache_eviction() {
    let mut cache = AudioSampleLruCache::new(400); // 400 bytes
    
    let audio1 = DecodedAudio::new(1, 44100, vec![0; 100]); // 200 bytes
    let audio2 = DecodedAudio::new(1, 44100, vec![1; 100]); // 200 bytes
    let audio3 = DecodedAudio::new(1, 44100, vec![2; 100]); // 200 bytes

    assert!(cache.put("sound1.wav", audio1).is_ok());
    assert!(cache.put("sound2.wav", audio2).is_ok());
    
    assert_eq!(cache.current_memory_usage(), 400);
    assert!(cache.contains("sound1.wav"));
    assert!(cache.contains("sound2.wav"));

    // To add sound3, sound1 should be evicted (as it's the oldest)
    assert!(cache.put("sound3.wav", audio3).is_ok());
    
    assert_eq!(cache.current_memory_usage(), 400);
    assert!(!cache.contains("sound1.wav")); // Evicted
    assert!(cache.contains("sound2.wav"));
    assert!(cache.contains("sound3.wav"));
}

#[test]
fn test_audio_cache_lru_update_on_get() {
    let mut cache = AudioSampleLruCache::new(400); // 400 bytes
    
    let audio1 = DecodedAudio::new(1, 44100, vec![0; 100]); // 200 bytes
    let audio2 = DecodedAudio::new(1, 44100, vec![1; 100]); // 200 bytes
    let audio3 = DecodedAudio::new(1, 44100, vec![2; 100]); // 200 bytes

    assert!(cache.put("sound1.wav", audio1).is_ok());
    assert!(cache.put("sound2.wav", audio2).is_ok());
    
    // Access sound1 so sound2 becomes the LRU
    assert!(cache.get_ref("sound1.wav").is_some());

    // Adding sound3 should evict sound2 now
    assert!(cache.put("sound3.wav", audio3).is_ok());

    assert_eq!(cache.current_memory_usage(), 400);
    assert!(cache.contains("sound1.wav")); // Not evicted
    assert!(!cache.contains("sound2.wav")); // Evicted
    assert!(cache.contains("sound3.wav"));
}

#[test]
fn test_audio_cache_reject_too_large() {
    let mut cache = AudioSampleLruCache::new(100); // 100 bytes max
    let audio = DecodedAudio::new(1, 44100, vec![0; 100]); // 200 bytes

    let result = cache.put("large_sound.wav", audio);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Probka jest wieksza niz calkowita maksymalna pojemnosc cache");
    
    assert_eq!(cache.current_memory_usage(), 0);
    assert!(!cache.contains("large_sound.wav"));
}

#[test]
fn test_audio_cache_clear() {
    let mut cache = AudioSampleLruCache::new(1024);
    let audio1 = DecodedAudio::new(1, 44100, vec![0; 100]); // 200 bytes
    
    assert!(cache.put("sound1.wav", audio1).is_ok());
    assert_eq!(cache.current_memory_usage(), 200);
    
    cache.clear();
    
    assert_eq!(cache.current_memory_usage(), 0);
    assert!(!cache.contains("sound1.wav"));
    assert!(cache.get_ref("sound1.wav").is_none());
}

#[test]
fn test_audio_cache_overwrite() {
    let mut cache = AudioSampleLruCache::new(1024);
    
    let audio1 = DecodedAudio::new(1, 44100, vec![0; 100]); // 200 bytes
    let audio2 = DecodedAudio::new(1, 44100, vec![1; 150]); // 300 bytes

    assert!(cache.put("sound.wav", audio1).is_ok());
    assert_eq!(cache.current_memory_usage(), 200);

    // Overwriting the same key should update memory usage correctly
    assert!(cache.put("sound.wav", audio2).is_ok());
    assert_eq!(cache.current_memory_usage(), 300);

    let cached_audio = cache.get_ref("sound.wav").unwrap();
    assert_eq!(cached_audio.samples.len(), 150);
}
