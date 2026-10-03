use m2_client::rodio_audio_device_init::RodioAudioDevice;

// Test 1: Basic struct instantiation
#[test]
fn test_rodio_audio_device_new() {
    let device = RodioAudioDevice::new();
    assert_eq!(device.is_initialized, false);
    assert!(device.stream.is_none());
    assert!(device.handle.is_none());
}

// Test 2: Initialization failure fallback handles state cleanly
#[test]
fn test_init_cleans_state_on_failure() {
    let mut device = RodioAudioDevice::new();
    // Pre-seed with true to ensure it resets on failure (mostly relevant for non-Windows mock)
    device.is_initialized = true;
    
    #[cfg(not(windows))]
    {
        let res = device.init();
        assert!(res.is_err());
        assert_eq!(device.is_initialized, false);
    }
}

// Test 3: Initialization success (on Windows)
#[test]
fn test_init_rodio_audio_device_success() {
    let mut device = RodioAudioDevice::new();
    let result = device.init();
    
    #[cfg(windows)]
    {
        if result.is_ok() {
            assert!(device.is_initialized);
            assert!(device.stream.is_some());
            assert!(device.handle.is_some());
        } else {
            // CI without audio device
            assert!(!device.is_initialized);
            assert!(device.stream.is_none());
            assert!(device.handle.is_none());
        }
    }
}

// Test 4: Re-initialization
#[test]
fn test_reinitialization() {
    let mut device = RodioAudioDevice::new();
    let _res1 = device.init();
    let res2 = device.init(); // Second call replaces the stream
    
    #[cfg(windows)]
    if res2.is_ok() {
        assert!(device.is_initialized);
        assert!(device.stream.is_some());
        assert!(device.handle.is_some());
    }
}

// Test 5: Drop behavior (Rust naturally drops Option<OutputStream>)
#[test]
fn test_device_drop() {
    // Simply ensure that dropping the device struct doesn't panic and we can create a new one
    {
        let mut device = RodioAudioDevice::new();
        let _ = device.init();
    } // Dropped here

    let mut device2 = RodioAudioDevice::new();
    let _ = device2.init(); // Should still be able to init
}
