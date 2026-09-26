use m2_client::sound_buffer_cache::{SoundBufferCache, WavAudio};

#[test]
fn test_cache_addition_and_retrieval() {
    let mut cache = SoundBufferCache::new(1024);

    let mut load_count = 0;
    let loader = || {
        load_count += 1;
        Ok(WavAudio::new(
            "test1.wav".to_string(),
            vec![1, 2, 3, 4, 5],
        ))
    };

    let audio = cache.get_or_load("test1.wav", 1000, loader).unwrap();
    assert_eq!(audio.name, "test1.wav");
    assert_eq!(audio.size_in_bytes(), 5);
    assert_eq!(load_count, 1);

    // Drugie pobranie nie powinno ladowac
    let loader2 = || -> Result<WavAudio, &'static str> {
        panic!("Nie powinno ladowac znowu!");
    };

    let audio2 = cache.get_or_load("test1.wav", 2000, loader2).unwrap();
    assert_eq!(audio2.name, "test1.wav");
    assert_eq!(audio2.size_in_bytes(), 5);
}

#[test]
fn test_memory_usage_calculation() {
    let mut cache = SoundBufferCache::new(1024);

    let loader1 = || Ok(WavAudio::new("test1.wav".to_string(), vec![0; 10]));
    let loader2 = || Ok(WavAudio::new("test2.wav".to_string(), vec![0; 20]));

    assert_eq!(cache.current_memory_usage(), 0);

    let _ = cache.get_or_load("test1.wav", 1000, loader1);
    assert_eq!(cache.current_memory_usage(), 10);

    let _ = cache.get_or_load("test2.wav", 2000, loader2);
    assert_eq!(cache.current_memory_usage(), 30);
}

#[test]
fn test_lru_eviction() {
    // Limit to 50 bytes
    let mut cache = SoundBufferCache::new(50);

    let loader1 = || Ok(WavAudio::new("test1.wav".to_string(), vec![0; 20]));
    let loader2 = || Ok(WavAudio::new("test2.wav".to_string(), vec![0; 20]));
    let loader3 = || Ok(WavAudio::new("test3.wav".to_string(), vec![0; 20]));

    // Adding test1 (20 bytes, time 1000)
    let _ = cache.get_or_load("test1.wav", 1000, loader1);
    // Adding test2 (20 bytes, time 2000)
    let _ = cache.get_or_load("test2.wav", 2000, loader2);
    
    assert_eq!(cache.current_memory_usage(), 40);

    // Odswiez test1
    let loader1_fail = || -> Result<WavAudio, &'static str> { panic!("Nie laduj"); };
    let _ = cache.get_or_load("test1.wav", 3000, loader1_fail);

    // Adding test3 (20 bytes). Wymaga usuniecia czegos, bo 40 + 20 > 50.
    // LRU to test2 (czas 2000 vs 3000 dla test1).
    let _ = cache.get_or_load("test3.wav", 4000, loader3);

    // Po wyrzuceniu test2, obecne sa test1(20) i test3(20). Suma 40.
    assert_eq!(cache.current_memory_usage(), 40);

    // Sprobujmy zaladowac test2, bedzie ladowany od nowa.
    let mut load_count = 0;
    let loader2_new = || {
        load_count += 1;
        Ok(WavAudio::new("test2.wav".to_string(), vec![0; 20]))
    };
    let _ = cache.get_or_load("test2.wav", 5000, loader2_new);
    assert_eq!(load_count, 1);
}

#[test]
fn test_clear() {
    let mut cache = SoundBufferCache::new(1024);

    let loader = || Ok(WavAudio::new("test1.wav".to_string(), vec![0; 100]));
    let _ = cache.get_or_load("test1.wav", 1000, loader);

    assert_eq!(cache.current_memory_usage(), 100);

    cache.clear();

    assert_eq!(cache.current_memory_usage(), 0);

    let mut load_count = 0;
    let loader_new = || {
        load_count += 1;
        Ok(WavAudio::new("test1.wav".to_string(), vec![0; 100]))
    };
    let _ = cache.get_or_load("test1.wav", 2000, loader_new);
    assert_eq!(load_count, 1);
}

#[test]
fn test_evict_too_large() {
    // Limit to 50 bytes
    let mut cache = SoundBufferCache::new(50);
    
    let loader1 = || Ok(WavAudio::new("test1.wav".to_string(), vec![0; 20]));
    let _ = cache.get_or_load("test1.wav", 1000, loader1);

    let loader_large = || Ok(WavAudio::new("test2.wav".to_string(), vec![0; 100]));
    let result = cache.get_or_load("test2.wav", 2000, loader_large);

    // Even though it's too large, evict_until_fits will clear the whole cache,
    // and then insert it anyway, bypassing the strict limit just to serve the request.
    assert!(result.is_ok());
    assert_eq!(cache.current_memory_usage(), 100);
}
