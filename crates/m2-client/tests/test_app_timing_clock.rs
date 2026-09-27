use m2_client::app_timing_clock::{AppTimingClock, EngineTime, FrameAccumulator};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_clock_initialization() {
    let clock = AppTimingClock::new(0.016); // ~60fps fixed step
    let time = clock.engine_time();
    
    assert_eq!(time.now_seconds, 0.0);
    assert_eq!(time.delta_seconds, 0.0);
    assert_eq!(time.global_time_ms, 0);
    assert_eq!(time.frame_index, 0);
    
    assert_eq!(clock.get_current_fps(), 0.0);
}

#[test]
fn test_clock_tick_advances_time() {
    let mut clock = AppTimingClock::new(0.016);
    
    // Simulate some time passage
    sleep(Duration::from_millis(10));
    
    let (delta, global_ms) = clock.tick();
    
    assert!(delta > 0.0);
    assert!(delta <= 0.1); // Max allowed delta
    assert!(global_ms >= 10);
    
    let time = clock.engine_time();
    assert_eq!(time.delta_seconds, delta);
    assert_eq!(time.global_time_ms, global_ms);
    assert_eq!(time.frame_index, 1);
}

#[test]
fn test_spiral_of_death_protection() {
    let mut clock = AppTimingClock::new(0.016);
    
    // Sleep for 150ms, which is > 100ms (0.1s max cap)
    sleep(Duration::from_millis(150));
    
    let (delta, _global_ms) = clock.tick();
    
    // The delta should be capped at 0.1s
    assert_eq!(delta, 0.1);
}

#[test]
fn test_fixed_step_accumulator() {
    let mut clock = AppTimingClock::new(0.05); // 50ms step
    
    // First tick, let's sleep for 60ms
    sleep(Duration::from_millis(60));
    clock.tick();
    
    // The accumulator should have >= 0.06 seconds
    // Since fixed_step is 0.05, `has_fixed_step()` should return true once,
    // and then false on the second call.
    assert!(clock.has_fixed_step());
    assert!(!clock.has_fixed_step());
}

#[test]
fn test_multiple_fixed_steps() {
    let mut clock = AppTimingClock::new(0.02); // 20ms step
    
    // Simulate a long frame (e.g., 90ms) - will be capped at 100ms (0.1s)
    sleep(Duration::from_millis(150));
    let (delta, _) = clock.tick();
    assert_eq!(delta, 0.1); // exactly 100ms
    
    // With 100ms accumulator and 20ms fixed step, we should get exactly 5 fixed steps
    assert!(clock.has_fixed_step()); // 1 (80ms left)
    assert!(clock.has_fixed_step()); // 2 (60ms left)
    assert!(clock.has_fixed_step()); // 3 (40ms left)
    assert!(clock.has_fixed_step()); // 4 (20ms left)
    assert!(clock.has_fixed_step()); // 5 (0ms left)
    assert!(!clock.has_fixed_step()); // No more steps
}

#[test]
fn test_fps_calculation() {
    let mut clock = AppTimingClock::new(0.016);
    
    // Force enough ticks to surpass 1.0 second in the timer
    // We'll simulate very fast frames where each takes ~20ms
    let target_frames = 55;
    for _ in 0..target_frames {
        sleep(Duration::from_millis(20));
        clock.tick();
    }
    
    // It should have calculated FPS at least once
    let fps = clock.get_current_fps();
    assert!(fps > 0.0);
    // Rough estimation: 1000ms / 20ms = ~50 FPS
    assert!(fps > 40.0 && fps < 60.0);
}

#[test]
fn test_clock_reset() {
    let mut clock = AppTimingClock::new(0.016);
    
    sleep(Duration::from_millis(20));
    clock.tick();
    
    assert!(clock.engine_time().frame_index > 0);
    assert!(clock.engine_time().global_time_ms > 0);
    
    clock.reset();
    
    let time = clock.engine_time();
    assert_eq!(time.now_seconds, 0.0);
    assert_eq!(time.delta_seconds, 0.0);
    assert_eq!(time.global_time_ms, 0);
    assert_eq!(time.frame_index, 0);
    assert_eq!(clock.get_current_fps(), 0.0);
    assert!(!clock.has_fixed_step());
}
