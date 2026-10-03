#[path = "../src/horse_controller_complete.rs"]
mod horse_controller_complete;

use horse_controller_complete::{SHorse, PixelPosition, RaceMotionMode};

#[test]
fn test_horse_create_destroy() {
    let mut horse = SHorse::new();
    assert!(!horse.is_mounting());
    
    horse.create(PixelPosition::new(10.0, 20.0, 30.0), 20101, 100);
    assert!(horse.is_mounting());
    
    let actor = horse.get_actor_ptr().unwrap();
    assert_eq!(actor.get_race(), 20101);
    assert_eq!(actor.cur_pixel_position, PixelPosition::new(10.0, 20.0, 30.0));
    assert_eq!(actor.battle_hit_effect, 100);
    assert_eq!(actor.alpha_value, 0.0);
    assert_eq!(actor.move_speed, 1.0);
    assert_eq!(actor.attack_speed, 1.0);
    assert_eq!(actor.motion_mode, RaceMotionMode::General);
    assert!(actor.stopped);
    assert!(actor.refreshed);

    horse.destroy();
    assert!(!horse.is_mounting());
    assert!(horse.get_actor_ptr().is_none());
}

#[test]
fn test_horse_speeds() {
    let mut horse = SHorse::new();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20101, 100);
    
    horse.set_attack_speed(150);
    horse.set_move_speed(120);

    let actor = horse.get_actor_ptr().unwrap();
    assert_eq!(actor.attack_speed, 1.5);
    assert_eq!(actor.move_speed, 1.2);
}

#[test]
fn test_get_level_basic() {
    let mut horse = SHorse::new();
    
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20101, 100);
    assert_eq!(horse.get_level(), 1);
    
    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20105, 100);
    assert_eq!(horse.get_level(), 2);

    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20111, 100);
    assert_eq!(horse.get_level(), 3);
}

#[test]
fn test_get_level_special() {
    let mut horse = SHorse::new();
    
    // Ramadan/Halloween event mounts (Should be level 2)
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20119, 100);
    assert_eq!(horse.get_level(), 2);
    
    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20220, 100);
    assert_eq!(horse.get_level(), 2);

    // Advanced mounts checking
    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20205, 100);
    assert_eq!(horse.get_level(), 2);

    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20211, 100);
    assert_eq!(horse.get_level(), 3);
}

#[test]
fn test_is_new_mount() {
    let mut horse = SHorse::new();
    
    // Regular mount
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20101, 100);
    assert!(!horse.is_new_mount());
    
    // New mount (Level 2 variant)
    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20205, 100);
    assert!(horse.is_new_mount());

    // New mount (Level 3 variant)
    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20212, 100);
    assert!(horse.is_new_mount());
}

#[test]
fn test_can_attack() {
    let mut horse = SHorse::new();
    
    // Level 1 mount -> cannot attack
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20101, 100);
    assert!(!horse.can_attack());
    
    // Level 2 mount -> can attack
    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20104, 100);
    assert!(horse.can_attack());

    // Level 3 mount -> can attack
    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20111, 100);
    assert!(horse.can_attack());
}

#[test]
fn test_can_use_skill() {
    let mut horse = SHorse::new();
    
    // Level 1 mount -> cannot use skill
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20101, 100);
    assert!(!horse.can_use_skill());
    
    // Level 2 mount -> cannot use skill
    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20104, 100);
    assert!(!horse.can_use_skill());

    // Level 3 mount -> can use skill
    horse.destroy();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20111, 100);
    assert!(horse.can_use_skill());
}

#[test]
fn test_get_actor_ref() {
    let mut horse = SHorse::new();
    horse.create(PixelPosition::new(0.0, 0.0, 0.0), 20101, 100);
    
    let actor = horse.get_actor_ref();
    assert_eq!(actor.get_race(), 20101);
}

