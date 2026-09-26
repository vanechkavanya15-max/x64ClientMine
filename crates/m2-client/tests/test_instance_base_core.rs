#[path = "../src/instance_base_core.rs"]
mod instance_base_core;
use instance_base_core::InstanceBase;

#[test]
fn test_instance_creation() {
    let instance = InstanceBase::new(
        1001,
        0,
        "Warrior".to_string(),
        1,
        500,
        100,
        0,
        (100.0, 200.0, 0.0),
        90.0,
        300.0,
        10,
        20,
        1,
    );
    assert_eq!(instance.vid, 1001);
    assert_eq!(instance.race, 0);
    assert_eq!(instance.name, "Warrior");
    assert_eq!(instance.empire, 1);
    assert_eq!(instance.guild_id, 500);
    assert_eq!(instance.alignment, 100);
    assert_eq!(instance.pk_mode, 0);
    assert_eq!(instance.pos, (100.0, 200.0, 0.0));
    assert_eq!(instance.rot_deg, 90.0);
    assert_eq!(instance.cur_speed, 300.0);
    assert_eq!(instance.armor_vnum, 10);
    assert_eq!(instance.weapon_vnum, 20);
    assert_eq!(instance.hair_vnum, 1);
    assert!(instance.is_alive);
    assert!(!instance.is_stunned);
    assert!(!instance.is_poisoned);
    assert!(!instance.is_dead());
}

#[test]
fn test_life_state() {
    let mut instance = InstanceBase::new(
        1001, 0, "Warrior".to_string(), 1, 500, 100, 0, (0.0, 0.0, 0.0), 0.0, 300.0, 0, 0, 0,
    );
    
    // Test set_stunned when alive
    instance.set_stunned(true);
    assert!(instance.is_stunned);

    // Test set_dead
    instance.set_dead();
    assert!(!instance.is_alive);
    assert!(instance.is_dead());
    assert_eq!(instance.cur_speed, 0.0);
    assert!(!instance.is_stunned);
    assert!(!instance.is_poisoned);

    // Test set_stunned when dead
    instance.set_stunned(true);
    assert!(!instance.is_stunned); // Should not become stunned if dead
}

#[test]
fn test_kingdom_guild() {
    let inst1 = InstanceBase::new(1, 0, "A".to_string(), 1, 100, 0, 0, (0.0, 0.0, 0.0), 0.0, 0.0, 0, 0, 0);
    let inst2 = InstanceBase::new(2, 0, "B".to_string(), 1, 100, 0, 0, (0.0, 0.0, 0.0), 0.0, 0.0, 0, 0, 0);
    let inst3 = InstanceBase::new(3, 0, "C".to_string(), 2, 200, 0, 0, (0.0, 0.0, 0.0), 0.0, 0.0, 0, 0, 0);
    let inst_neutral = InstanceBase::new(4, 0, "D".to_string(), 0, 0, 0, 0, (0.0, 0.0, 0.0), 0.0, 0.0, 0, 0, 0);

    // Empire tests
    assert!(inst1.is_same_empire(&inst2));
    assert!(!inst1.is_same_empire(&inst3));
    assert!(!inst1.is_same_empire(&inst_neutral));
    assert!(!inst_neutral.is_same_empire(&inst1));
    assert!(!inst_neutral.is_same_empire(&inst_neutral));

    // Guild tests
    assert!(inst1.is_same_guild(&inst2));
    assert!(!inst1.is_same_guild(&inst3));
    assert!(!inst1.is_same_guild(&inst_neutral));
    assert!(!inst_neutral.is_same_guild(&inst1));
    assert!(!inst_neutral.is_same_guild(&inst_neutral));
}
