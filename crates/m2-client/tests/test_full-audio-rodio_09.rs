use m2_client::audio_voice_pool_manager::{AudioVoicePoolManager, MAX_VOICES};
use rodio::Sink;

fn create_idle_sink(append_dummy: bool) -> Sink {
    let (sink, _queue) = Sink::new_idle();
    if append_dummy {
        // Append an endless zero source so it is never "empty"
        let source = rodio::source::Zero::<f32>::new(1, 44100);
        sink.append(source);
    }
    sink
}

#[test]
fn test_add_voice_up_to_limit() {
    let mut manager = AudioVoicePoolManager::new();

    for i in 0..MAX_VOICES {
        let sink = create_idle_sink(true);
        let added = manager.add_voice(sink, 1.0);
        assert!(added, "Failed to add voice {}", i);
    }

    assert_eq!(manager.voices.len(), MAX_VOICES);
}

#[test]
fn test_reject_quieter_voice_when_full() {
    let mut manager = AudioVoicePoolManager::new();

    for _ in 0..MAX_VOICES {
        let sink = create_idle_sink(true);
        manager.add_voice(sink, 0.5);
    }

    let quiet_sink = create_idle_sink(true);
    let added = manager.add_voice(quiet_sink, 0.3);

    assert!(!added, "Quieter voice should have been rejected");
    assert_eq!(manager.voices.len(), MAX_VOICES);
    
    for voice in &manager.voices {
        assert_eq!(voice.volume, 0.5);
    }
}

#[test]
fn test_evict_quietest_voice_when_full() {
    let mut manager = AudioVoicePoolManager::new();

    for _ in 0..(MAX_VOICES - 1) {
        let sink = create_idle_sink(true);
        manager.add_voice(sink, 0.8);
    }

    let quiet_sink = create_idle_sink(true);
    manager.add_voice(quiet_sink, 0.2);

    assert_eq!(manager.voices.len(), MAX_VOICES);

    let new_sink = create_idle_sink(true);
    let added = manager.add_voice(new_sink, 0.5);

    assert!(added, "Louder voice should have been accepted");
    assert_eq!(manager.voices.len(), MAX_VOICES);

    let mut found_0_5 = false;
    let mut found_0_2 = false;
    let mut count_0_8 = 0;

    for voice in &manager.voices {
        if voice.volume == 0.5 {
            found_0_5 = true;
        } else if voice.volume == 0.2 {
            found_0_2 = true;
        } else if voice.volume == 0.8 {
            count_0_8 += 1;
        }
    }

    assert!(found_0_5, "The new voice (0.5) should be in the pool");
    assert!(!found_0_2, "The quietest voice (0.2) should have been evicted");
    assert_eq!(count_0_8, MAX_VOICES - 1, "The other voices should remain unchanged");
}

#[test]
fn test_cleanup_finished_voices() {
    let mut manager = AudioVoicePoolManager::new();
    
    // Add 10 voices that are empty immediately
    for _ in 0..10 {
        let sink = create_idle_sink(false);
        manager.add_voice(sink, 1.0);
    }
    
    // Add 5 voices that are not empty
    for _ in 0..5 {
        let sink = create_idle_sink(true);
        manager.add_voice(sink, 1.0);
    }

    manager.cleanup_finished_voices();
    
    assert_eq!(manager.voices.len(), 5, "Empty sinks should be cleaned up, others kept");
}

#[test]
fn test_exact_capacity_boundary() {
    let mut manager = AudioVoicePoolManager::new();

    for i in 0..MAX_VOICES {
        let sink = create_idle_sink(true);
        let added = manager.add_voice(sink, 1.0);
        assert!(added, "Failed to add voice {} at boundary", i);
    }
    
    assert_eq!(manager.voices.len(), MAX_VOICES);

    let extra_sink = create_idle_sink(true);
    let added_extra = manager.add_voice(extra_sink, 1.0);
    assert!(!added_extra, "Should reject voice when exactly full and not louder");
    
    let louder_sink = create_idle_sink(true);
    let added_louder = manager.add_voice(louder_sink, 1.1);
    
    assert!(added_louder, "Should accept louder voice when exactly full");
    assert_eq!(manager.voices.len(), MAX_VOICES);
}

#[test]
fn test_finds_quietest_among_multiple_levels() {
    let mut manager = AudioVoicePoolManager::new();

    for i in 0..MAX_VOICES {
        let sink = create_idle_sink(true);
        manager.add_voice(sink, (i as f32 + 1.0) / 100.0);
    }

    let mid_sink = create_idle_sink(true);
    let added = manager.add_voice(mid_sink, 0.05);

    assert!(added);
    assert_eq!(manager.voices.len(), MAX_VOICES);

    let mut min_vol = 100.0;
    for voice in &manager.voices {
        if voice.volume < min_vol {
            min_vol = voice.volume;
        }
    }
    assert_eq!(min_vol, 0.02, "The quietest voice should now be 0.02");
}
