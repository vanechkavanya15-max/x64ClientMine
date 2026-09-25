use m2_client_audio::sound_buffer::{SoundBuffer, SoundData};

fn create_dummy_sound_data(size: usize) -> SoundData {
    SoundData {
        data: vec![0; size],
        sample_rate: 44100,
        channels: 2,
    }
}

#[test]
fn test_integration_sound_buffer() {
    let instance = SoundBuffer::new();
    assert!(instance.is_ready());
}

#[test]
fn test_sound_buffer_memory_limit_eviction() {
    // Limit to 100 bytes
    let mut buffer = SoundBuffer::with_capacity(100);
    
    // Add first element: 60 bytes
    buffer.insert("sound1".to_string(), create_dummy_sound_data(60));
    assert_eq!(buffer.current_memory_bytes(), 60);
    assert!(buffer.get("sound1").is_some());
    
    // Add second element: 50 bytes (total 110, exceeds 100 limit, triggers eviction)
    buffer.insert("sound2".to_string(), create_dummy_sound_data(50));
    
    // "sound1" should be evicted
    assert!(buffer.get("sound1").is_none());
    assert!(buffer.get("sound2").is_some());
    assert_eq!(buffer.current_memory_bytes(), 50);
}

#[test]
fn test_sound_buffer_lru_update_on_get() {
    // Limit to 100 bytes
    let mut buffer = SoundBuffer::with_capacity(100);
    
    // Add 3 sounds of 30 bytes each
    buffer.insert("sound1".to_string(), create_dummy_sound_data(30));
    buffer.insert("sound2".to_string(), create_dummy_sound_data(30));
    buffer.insert("sound3".to_string(), create_dummy_sound_data(30));
    assert_eq!(buffer.current_memory_bytes(), 90);
    
    // Access "sound1" so it becomes the most recently used
    assert!(buffer.get("sound1").is_some());
    
    // Insert new element of 30 bytes (total 120, exceeds limit)
    buffer.insert("sound4".to_string(), create_dummy_sound_data(30));
    
    // Since "sound1" was accessed, "sound2" should be the oldest and evicted
    assert!(buffer.get("sound2").is_none());
    assert!(buffer.get("sound1").is_some());
    assert!(buffer.get("sound3").is_some());
    assert!(buffer.get("sound4").is_some());
    assert_eq!(buffer.current_memory_bytes(), 90);
}

#[test]
fn test_sound_buffer_remove_and_clear() {
    let mut buffer = SoundBuffer::with_capacity(200);
    buffer.insert("sound1".to_string(), create_dummy_sound_data(40));
    buffer.insert("sound2".to_string(), create_dummy_sound_data(40));
    
    assert_eq!(buffer.current_memory_bytes(), 80);
    
    let removed = buffer.remove("sound1");
    assert!(removed);
    assert_eq!(buffer.current_memory_bytes(), 40);
    assert!(buffer.get("sound1").is_none());
    
    buffer.clear();
    assert_eq!(buffer.current_memory_bytes(), 0);
    assert!(buffer.get("sound2").is_none());
}

#[test]
fn test_sound_buffer_huge_element() {
    let mut buffer = SoundBuffer::with_capacity(100);
    buffer.insert("sound1".to_string(), create_dummy_sound_data(40));
    
    // Insert element larger than max capacity
    buffer.insert("huge".to_string(), create_dummy_sound_data(150));
    
    // "sound1" should be evicted
    assert!(buffer.get("sound1").is_none());
    assert_eq!(buffer.current_memory_bytes(), 150);
    assert!(buffer.get("huge").is_some());
}

#[test]
fn test_sound_buffer_replace_existing() {
    let mut buffer = SoundBuffer::with_capacity(100);
    buffer.insert("sound1".to_string(), create_dummy_sound_data(40));
    
    // Replace with a new sound under the same name but different size
    buffer.insert("sound1".to_string(), create_dummy_sound_data(60));
    
    assert_eq!(buffer.current_memory_bytes(), 60);
    assert!(buffer.get("sound1").is_some());
}
