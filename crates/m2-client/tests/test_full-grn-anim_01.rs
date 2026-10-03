#[path = "../src/granny_animation_clock.rs"]
mod granny_animation_clock;

use granny_animation_clock::GrannyAnimationClock;

#[test]
fn test_basic_clock_advancement() {
    let mut clock = GrannyAnimationClock::new(2.0, 30.0, false);
    assert_eq!(clock.get_current_time(), 0.0);
    assert_eq!(clock.get_current_frame_index(), 0);
    assert!(!clock.is_finished());

    clock.advance(0.5);
    assert_eq!(clock.get_current_time(), 0.5);
    // 0.5 * 30 = 15
    assert_eq!(clock.get_current_frame_index(), 15);
    assert!(!clock.is_finished());
    
    clock.advance(0.6);
    assert_eq!(clock.get_current_time(), 1.1);
    // 1.1 * 30 = 33
    assert_eq!(clock.get_current_frame_index(), 33);
}

#[test]
fn test_looping_animation_wrapping() {
    let mut clock = GrannyAnimationClock::new(1.0, 60.0, true);
    
    // Advance beyond duration
    clock.advance(1.2);
    // Should wrap to 0.2
    assert!((clock.get_current_time() - 0.2).abs() < 1e-5);
    assert!(!clock.is_finished());
    
    // Test negative advancement via speed factor
    clock.set_speed(-1.0);
    clock.advance(0.5); // current time was 0.2, minus 0.5 is -0.3, wrapped to 0.7
    assert!((clock.get_current_time() - 0.7).abs() < 1e-5);
}

#[test]
fn test_non_looping_animation_capping() {
    let mut clock = GrannyAnimationClock::new(3.0, 24.0, false);
    
    clock.advance(2.0);
    assert_eq!(clock.get_current_time(), 2.0);
    assert!(!clock.is_finished());

    clock.advance(2.0);
    // Caps at 3.0
    assert_eq!(clock.get_current_time(), 3.0);
    assert!(clock.is_finished());
    
    // Negative speed
    clock.set_speed(-1.0);
    clock.advance(4.0); // subtracts 4.0, should cap at 0.0
    assert_eq!(clock.get_current_time(), 0.0);
    assert!(clock.is_finished());
}

#[test]
fn test_set_speed_scaling() {
    let mut clock = GrannyAnimationClock::new(5.0, 30.0, false);
    
    clock.set_speed(2.0);
    clock.advance(1.0);
    // Advanced by 1.0 * 2.0 = 2.0
    assert_eq!(clock.get_current_time(), 2.0);
    
    clock.set_speed(0.5);
    clock.advance(2.0);
    // Advanced by 2.0 * 0.5 = 1.0, total 3.0
    assert_eq!(clock.get_current_time(), 3.0);
}

#[test]
fn test_reset_and_zero_duration() {
    let mut clock = GrannyAnimationClock::new(2.0, 30.0, false);
    clock.advance(1.0);
    assert_eq!(clock.get_current_time(), 1.0);
    
    clock.reset();
    assert_eq!(clock.get_current_time(), 0.0);
    assert!(!clock.is_finished());
    
    let mut zero_clock = GrannyAnimationClock::new(0.0, 30.0, false);
    assert!(zero_clock.is_finished());
    assert_eq!(zero_clock.get_current_time(), 0.0);
    
    zero_clock.advance(1.0);
    assert_eq!(zero_clock.get_current_time(), 0.0);
    assert!(zero_clock.is_finished());
    
    let mut neg_clock = GrannyAnimationClock::new(-1.0, 30.0, false);
    assert!(neg_clock.is_finished());
}
