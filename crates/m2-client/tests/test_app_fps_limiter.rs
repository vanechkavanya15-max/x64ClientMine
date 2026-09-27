use std::time::Instant;
use m2_client::app_fps_limiter::{FpsLimit, FramePacer};

#[test]
fn test_frame_budget_calculation_30_fps() {
    let limit = FpsLimit::Fps30;
    assert_eq!(limit.budget_us(), 33333);
}

#[test]
fn test_frame_budget_calculation_60_fps() {
    let limit = FpsLimit::Fps60;
    assert_eq!(limit.budget_us(), 16666);
}

#[test]
fn test_frame_budget_calculation_144_fps() {
    let limit = FpsLimit::Fps144;
    assert_eq!(limit.budget_us(), 6944);
}

#[test]
fn test_set_target_fps() {
    let mut pacer = FramePacer::new(FpsLimit::Fps60);
    assert_eq!(pacer.get_target_fps(), FpsLimit::Fps60);

    pacer.set_target_fps(FpsLimit::Fps120);
    assert_eq!(pacer.get_target_fps(), FpsLimit::Fps120);
    // 1_000_000 / 120 = 8333
}

#[test]
fn test_get_target_fps() {
    let pacer = FramePacer::new(FpsLimit::Fps240);
    assert_eq!(pacer.get_target_fps(), FpsLimit::Fps240);
}

#[test]
fn test_pace_frame_uncapped() {
    let mut pacer = FramePacer::new(FpsLimit::Uncapped);
    
    let start = Instant::now();
    pacer.pace_frame();
    let elapsed = start.elapsed().as_micros();
    
    // Uncapped should return almost immediately
    assert!(elapsed < 500, "Uncapped took too long: {}us", elapsed);
}

#[test]
fn test_pace_frame_fps60_delay() {
    let mut pacer = FramePacer::new(FpsLimit::Fps60);
    
    // Reset the internal instant just before test starts to ensure clean state
    pacer.pace_frame();
    
    let start = Instant::now();
    pacer.pace_frame();
    let elapsed = start.elapsed().as_micros() as u64;
    
    let target = FpsLimit::Fps60.budget_us();
    // Allow a small error margin (~2ms) for OS scheduling variations
    let margin = 2500;
    
    assert!(elapsed >= target.saturating_sub(margin), "Paced frame was too fast: {}us (target {}us)", elapsed, target);
    assert!(elapsed <= target + margin, "Paced frame was too slow: {}us (target {}us)", elapsed, target);
}

#[test]
fn test_pace_frame_multiple_calls() {
    let mut pacer = FramePacer::new(FpsLimit::Fps120);
    
    pacer.pace_frame(); // Init
    
    let start = Instant::now();
    
    // Pace 3 frames at 120 fps
    pacer.pace_frame();
    pacer.pace_frame();
    pacer.pace_frame();
    
    let elapsed = start.elapsed().as_micros() as u64;
    
    let target = FpsLimit::Fps120.budget_us() * 3;
    let margin = 4000; // Larger margin for multiple frames
    
    assert!(elapsed >= target.saturating_sub(margin), "Multiple paced frames were too fast: {}us (target {}us)", elapsed, target);
    assert!(elapsed <= target + margin, "Multiple paced frames were too slow: {}us (target {}us)", elapsed, target);
}
