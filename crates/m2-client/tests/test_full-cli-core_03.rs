#[path = "../src/frame_delta_time_clock.rs"]
mod frame_delta_time_clock;

use frame_delta_time_clock::FrameDeltaTimeClock;
use std::time::{Duration, Instant};

#[test]
fn test_first_update_returns_zero() {
    let mut clock = FrameDeltaTimeClock::new();
    let delta = clock.update();
    assert_eq!(delta, 0.0);
}

#[test]
fn test_normal_update_returns_actual_delta() {
    let mut clock = FrameDeltaTimeClock::new();
    let start_time = Instant::now();
    
    // First update to set the internal 'last_update'
    clock.update_with_time(start_time);
    
    // Move forward by 0.05 seconds
    let next_time = start_time + Duration::from_millis(50);
    let delta = clock.update_with_time(next_time);
    
    // Using an epsilon for floating point comparison, though from_millis(50) should be exactly 0.05
    assert!((delta - 0.05).abs() < 1e-6);
}

#[test]
fn test_delta_is_clamped_to_max() {
    let mut clock = FrameDeltaTimeClock::new();
    let start_time = Instant::now();
    
    // First update
    clock.update_with_time(start_time);
    
    // Move forward by 5.0 seconds (simulating a huge lag spike)
    let next_time = start_time + Duration::from_secs(5);
    let delta = clock.update_with_time(next_time);
    
    // Delta should be clamped to 0.1s
    assert_eq!(delta, 0.1);
}

#[test]
fn test_negative_or_zero_time_step_returns_zero() {
    let mut clock = FrameDeltaTimeClock::new();
    let start_time = Instant::now();
    
    // First update
    clock.update_with_time(start_time);
    
    // Simulating monotonic clock anomaly where time goes slightly backward
    let next_time = start_time.checked_sub(Duration::from_millis(10)).unwrap_or(start_time);
    let delta = clock.update_with_time(next_time);
    
    // saturating_duration_since should result in 0.0
    assert_eq!(delta, 0.0);
    
    // Exact same time
    let delta2 = clock.update_with_time(next_time);
    assert_eq!(delta2, 0.0);
}

#[test]
fn test_reset_behavior() {
    let mut clock = FrameDeltaTimeClock::new();
    let start_time = Instant::now();
    
    clock.update_with_time(start_time);
    
    let next_time = start_time + Duration::from_millis(50);
    let delta = clock.update_with_time(next_time);
    assert!((delta - 0.05).abs() < 1e-6);
    
    // Reset the clock
    clock.reset();
    
    // The very next update should be treated as the first update (0.0 delta)
    let after_reset_time = next_time + Duration::from_millis(50);
    let delta_after_reset = clock.update_with_time(after_reset_time);
    
    assert_eq!(delta_after_reset, 0.0);
}
