#[path = "../src/skill_cooldown_tracker.rs"]
mod skill_cooldown_tracker;

use skill_cooldown_tracker::SkillCooldownTracker;

#[test]
fn test_skill_ready_initially() {
    let tracker = SkillCooldownTracker::new();
    // Powinno byc gotowe od razu
    assert!(tracker.is_skill_ready(1, 1000));
}

#[test]
fn test_cooldown_reduction_with_cast_speed() {
    let mut tracker = SkillCooldownTracker::new();
    let start_time = 1000;
    let cast_speed = 100; // Szybkosc rzucania 100
    
    tracker.start_skill_cooldown(1, start_time, cast_speed);
    
    // Wedlug wzoru: actual = (10000 * 100) / (100 + 100) = 1000000 / 200 = 5000 ms
    // Start to 1000, end_time to 6000
    
    assert!(!tracker.is_skill_ready(1, 1000), "Tuz po uzyciu nie powinno byc gotowe");
    assert!(!tracker.is_skill_ready(1, 5999), "Przed uplywem czasu nie gotowe");
    assert!(tracker.is_skill_ready(1, 6000), "Dokladnie po uplywie gotowe");
    assert!(tracker.is_skill_ready(1, 7000), "Po czasie gotowe");
}

#[test]
fn test_zero_cast_speed_handling() {
    let mut tracker = SkillCooldownTracker::new();
    let start_time = 0;
    let cast_speed = 0;
    
    tracker.start_skill_cooldown(2, start_time, cast_speed);
    
    // actual = (10000 * 100) / 100 = 10000 ms
    assert!(!tracker.is_skill_ready(2, 9999));
    assert!(tracker.is_skill_ready(2, 10000));
}

#[test]
fn test_skill_blocked_during_cooldown() {
    let mut tracker = SkillCooldownTracker::new();
    let start_time = 500;
    let cast_speed = 0;
    
    tracker.start_skill_cooldown(3, start_time, cast_speed);
    
    // Czeka przez caly bazowy czas
    for time in 500..10500 {
        assert!(!tracker.is_skill_ready(3, time), "Zablokowane w czasie {}", time);
    }
    
    assert!(tracker.is_skill_ready(3, 10500));
}

#[test]
fn test_cooldown_wrapping_logic() {
    let mut tracker = SkillCooldownTracker::new();
    // Ustawiamy cur_time blisko u32::MAX
    let start_time = u32::MAX - 2000;
    let cast_speed = 0; // cooldown 10000 ms
    
    tracker.start_skill_cooldown(4, start_time, cast_speed);
    
    // Oczekiwany koniec to (u32::MAX - 2000).wrapping_add(10000)
    // czyli u32::MAX - 2000 + 10000 = u32::MAX + 8000
    // w u32 to po prostu 7999 (bo u32::MAX to -1 w u2)
    let end_time = start_time.wrapping_add(10000);
    
    // Przed koncem, czas = u32::MAX
    assert!(!tracker.is_skill_ready(4, u32::MAX), "Zablokowane przed przepelnieniem");
    // Po przepelnieniu, czas = 0
    assert!(!tracker.is_skill_ready(4, 0), "Zablokowane po przepelnieniu na poczatku");
    assert!(!tracker.is_skill_ready(4, end_time - 1), "Zablokowane tuz przed");
    
    assert!(tracker.is_skill_ready(4, end_time), "Gotowe rowno w czasie konca");
    assert!(tracker.is_skill_ready(4, end_time + 100), "Gotowe po");
}

#[test]
fn test_multiple_skills() {
    let mut tracker = SkillCooldownTracker::new();
    
    tracker.start_skill_cooldown(1, 1000, 0); // end = 11000
    tracker.start_skill_cooldown(2, 2000, 100); // end = 2000 + 5000 = 7000
    
    assert!(!tracker.is_skill_ready(1, 5000));
    assert!(!tracker.is_skill_ready(2, 5000));
    
    assert!(!tracker.is_skill_ready(1, 7500));
    assert!(tracker.is_skill_ready(2, 7500));
    
    assert!(tracker.is_skill_ready(1, 11000));
    assert!(tracker.is_skill_ready(2, 11000));
}
