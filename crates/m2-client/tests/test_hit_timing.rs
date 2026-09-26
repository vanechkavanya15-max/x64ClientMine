#[path = "../src/hit_timing.rs"]
mod hit_timing;

use hit_timing::HitTiming;

#[test]
fn test_standard_attack_speed() {
    let timing = HitTiming::new(1000, 500, 100);
    assert_eq!(timing.scaled_duration_ms(), 1000);
    assert_eq!(timing.scaled_hit_time_ms(), 500);
}

#[test]
fn test_increased_attack_speed() {
    let timing = HitTiming::new(1000, 500, 150);
    // (1000 * 100) / 150 = 666
    assert_eq!(timing.scaled_duration_ms(), 666);
    // (500 * 100) / 150 = 333
    assert_eq!(timing.scaled_hit_time_ms(), 333);
}

#[test]
fn test_decreased_attack_speed() {
    let timing = HitTiming::new(1000, 500, 50);
    // (1000 * 100) / 50 = 2000
    assert_eq!(timing.scaled_duration_ms(), 2000);
    // (500 * 100) / 50 = 1000
    assert_eq!(timing.scaled_hit_time_ms(), 1000);
}

#[test]
fn test_zero_attack_speed_fallback() {
    let timing = HitTiming::new(1000, 500, 0);
    // fallback to 100
    assert_eq!(timing.scaled_duration_ms(), 1000);
    assert_eq!(timing.scaled_hit_time_ms(), 500);
}

#[test]
fn test_is_hit_moment_exact() {
    assert!(HitTiming::is_hit_moment(500, 500, 10));
}

#[test]
fn test_is_hit_moment_within_tolerance() {
    assert!(HitTiming::is_hit_moment(495, 500, 10));
    assert!(HitTiming::is_hit_moment(505, 500, 10));
}

#[test]
fn test_is_hit_moment_outside_tolerance() {
    assert!(!HitTiming::is_hit_moment(480, 500, 10));
    assert!(!HitTiming::is_hit_moment(520, 500, 10));
}

#[test]
fn test_is_hit_moment_zero_tolerance() {
    assert!(HitTiming::is_hit_moment(500, 500, 0));
    assert!(!HitTiming::is_hit_moment(501, 500, 0));
    assert!(!HitTiming::is_hit_moment(499, 500, 0));
}

#[test]
fn test_is_hit_moment_tolerance_overflow_underflow() {
    // saturating_sub protection check
    assert!(HitTiming::is_hit_moment(0, 10, 20));
    assert!(HitTiming::is_hit_moment(30, 10, 20));
}
