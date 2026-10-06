#[path = "../src/actor_horse_riding_system_v2.rs"]
mod actor_horse_riding_system_v2;

use actor_horse_riding_system_v2::{HorseRidingSystem, MotionMode};

#[test]
fn test_mount_dismount() {
    let mut horse_sys = HorseRidingSystem::new();

    assert!(!horse_sys.is_mounted());
    assert_eq!(horse_sys.get_mount_vnum(), 0);
    assert_eq!(horse_sys.get_level(), 0);

    horse_sys.mount(20101); // level 1 horse
    assert!(horse_sys.is_mounted());
    assert_eq!(horse_sys.get_mount_vnum(), 20101);
    assert_eq!(horse_sys.get_level(), 1);

    horse_sys.dismount();
    assert!(!horse_sys.is_mounted());
    assert_eq!(horse_sys.get_mount_vnum(), 0);
    assert_eq!(horse_sys.get_level(), 0);
}

#[test]
fn test_horse_level_mapping() {
    let mut horse_sys = HorseRidingSystem::new();

    // Level 1: 20101, 20102, 20103
    horse_sys.mount(20102);
    assert_eq!(horse_sys.get_level(), 1);

    // Level 2: 20104, 20105, 20106
    horse_sys.mount(20104);
    assert_eq!(horse_sys.get_level(), 2);

    // Level 3: 20107..20118
    horse_sys.mount(20110); // Bojowiec/Militarny C++ match
    assert_eq!(horse_sys.get_level(), 3);

    // Special Level 2
    horse_sys.mount(20119);
    assert_eq!(horse_sys.get_level(), 2);
    
    // Special Level 3 mappings
    horse_sys.mount(20210);
    assert_eq!(horse_sys.get_level(), 3);
}

#[test]
fn test_can_attack() {
    let mut horse_sys = HorseRidingSystem::new();

    // Unmounted -> can attack
    assert!(horse_sys.can_attack());

    // Mounted level 1 -> cannot attack
    horse_sys.mount(20101);
    assert!(!horse_sys.can_attack());

    // Mounted level 2 -> can attack
    horse_sys.mount(20104);
    assert!(horse_sys.can_attack());

    // Mounted level 3 -> can attack
    horse_sys.mount(20110);
    assert!(horse_sys.can_attack());
}

#[test]
fn test_can_use_skill() {
    let mut horse_sys = HorseRidingSystem::new();

    // Unmounted -> can use skill
    assert!(horse_sys.can_use_skill());

    // Mounted level 1 -> cannot use skill
    horse_sys.mount(20101);
    assert!(!horse_sys.can_use_skill());

    // Mounted level 2 -> cannot use skill
    horse_sys.mount(20104);
    assert!(!horse_sys.can_use_skill());

    // Mounted level 3 -> can use skill
    horse_sys.mount(20110);
    assert!(horse_sys.can_use_skill());

    // Special Mount that allows skill
    horse_sys.mount(20210);
    assert!(horse_sys.can_use_skill());
}

#[test]
fn test_motion_mode_mapping() {
    assert_eq!(HorseRidingSystem::get_horse_motion_mode(MotionMode::General), MotionMode::Horse);
    assert_eq!(HorseRidingSystem::get_horse_motion_mode(MotionMode::OneHandSword), MotionMode::HorseOneHandSword);
    assert_eq!(HorseRidingSystem::get_horse_motion_mode(MotionMode::TwoHandSword), MotionMode::HorseTwoHandSword);
    assert_eq!(HorseRidingSystem::get_horse_motion_mode(MotionMode::DualHandSword), MotionMode::HorseDualHandSword);
    assert_eq!(HorseRidingSystem::get_horse_motion_mode(MotionMode::Bow), MotionMode::HorseBow);
    assert_eq!(HorseRidingSystem::get_horse_motion_mode(MotionMode::Fan), MotionMode::HorseFan);
    assert_eq!(HorseRidingSystem::get_horse_motion_mode(MotionMode::Bell), MotionMode::HorseBell);
    
    // Default fallback
    assert_eq!(HorseRidingSystem::get_horse_motion_mode(MotionMode::Fishing), MotionMode::Horse);
}
