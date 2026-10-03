#[path = "../src/game_loop_fps_limiter.rs"]
mod game_loop_fps_limiter;
use game_loop_fps_limiter::*;
use std::time::{Duration, Instant};

#[test]
fn test_unlimited_fps() {
    let mut limiter = FpsLimiter::new(FpsLimit::Unlimited);
    let start = Instant::now();
    
    // Simulate 10 frames
    for _ in 0..10 {
        assert!(limiter.should_render());
        limiter.wait();
    }
    
    // With unlimited, it should not delay significantly
    assert!(start.elapsed() < Duration::from_millis(50));
}

#[test]
fn test_limited_fps_60() {
    let mut limiter = FpsLimiter::new(FpsLimit::Limited(60));
    // Set sleep threshold lower for testing
    limiter.set_sleep_threshold(Duration::from_millis(1));
    
    let target_duration = Duration::from_nanos(1_000_000_000 / 60);
    
    let start = Instant::now();
    limiter.wait();
    let elapsed = start.elapsed();
    
    // It should wait at least somewhat close to 16.6ms
    assert!(elapsed > Duration::from_millis(10));
}

#[test]
fn test_should_render_timing() {
    let mut limiter = FpsLimiter::new(FpsLimit::Limited(30));
    
    // First call should be false since we just started
    assert!(!limiter.should_render());
    
    // Wait slightly more than target duration
    std::thread::sleep(Duration::from_millis(40)); // 30fps is ~33ms
    
    // Now it should be true
    assert!(limiter.should_render());
    
    // Immediately after, it should be false again
    assert!(!limiter.should_render());
}

#[test]
fn test_change_limit() {
    let mut limiter = FpsLimiter::new(FpsLimit::Limited(10));
    assert_eq!(limiter.limit(), FpsLimit::Limited(10));
    
    limiter.set_limit(FpsLimit::Unlimited);
    assert_eq!(limiter.limit(), FpsLimit::Unlimited);
}

#[test]
fn test_default() {
    let limiter = FpsLimiter::default();
    assert_eq!(limiter.limit(), FpsLimit::Unlimited);
}

#[test]
fn test_catchup_logic() {
    let mut limiter = FpsLimiter::new(FpsLimit::Limited(60)); // ~16.6ms per frame
    
    // Sleep for 2 frames
    std::thread::sleep(Duration::from_millis(40));
    
    // It should say we need to render
    assert!(limiter.should_render());
    
    // And it shouldn't reset time all the way, allowing another render immediately 
    // to catch up (as it was delayed by >1 frame)
    std::thread::sleep(Duration::from_millis(1)); // tiny sleep
    assert!(limiter.should_render());
}
