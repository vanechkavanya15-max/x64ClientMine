use m2_client::audio_engine_shutdown::AudioShutdownManager;
use m2_client::full_audio_rodio_13::{m2_audio_engine_start, m2_audio_engine_shutdown};
use std::thread;
use std::time::Duration;

#[test]
fn test_shutdown_when_not_running() {
    let mut manager = AudioShutdownManager::new();
    assert_eq!(manager.is_active(), false);
    
    // Normal shutdown of non-running engine should return Ok
    assert!(manager.shutdown().is_ok());
    assert_eq!(manager.is_active(), false);
}

#[test]
fn test_start_and_shutdown_active() {
    let mut manager = AudioShutdownManager::new();
    assert!(manager.start().is_ok());
    assert_eq!(manager.is_active(), true);

    thread::sleep(Duration::from_millis(100)); // let thread spin

    assert!(manager.shutdown().is_ok());
    assert_eq!(manager.is_active(), false);
    
    // Verify thread has cleaned buffers
    assert_eq!(manager.are_buffers_cleared(), true);
}

#[test]
fn test_multiple_shutdowns_idempotency() {
    let mut manager = AudioShutdownManager::new();
    assert!(manager.start().is_ok());
    
    assert!(manager.shutdown().is_ok());
    assert!(manager.shutdown().is_ok()); // second shutdown should be idempotent
    assert!(manager.shutdown().is_ok()); // third shutdown
    
    assert_eq!(manager.is_active(), false);
}

#[test]
fn test_cannot_start_twice() {
    let mut manager = AudioShutdownManager::new();
    assert!(manager.start().is_ok());
    
    // Should return err on second start
    assert!(manager.start().is_err());
    
    assert!(manager.shutdown().is_ok());
}

#[test]
fn test_ffi_endpoints() {
    // We are testing FFI in this test
    // Note: OnceLock means this test relies on global state being fresh or allows multiple calls
    assert_eq!(m2_audio_engine_start(), true);
    // Double start should fail
    assert_eq!(m2_audio_engine_start(), false);
    
    // Shutdown
    assert_eq!(m2_audio_engine_shutdown(), true);
    // Double shutdown returns Ok(()) so true
    assert_eq!(m2_audio_engine_shutdown(), true);
}
