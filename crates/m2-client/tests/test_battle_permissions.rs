use m2_client::battle_permissions::{BattleInstance, TargetType, PvpMode};

#[test]
fn test_block_attack_when_shop_is_open() {
    let mut attacker = BattleInstance::new(TargetType::Player, 1);
    let target = BattleInstance::new(TargetType::Monster, 0);

    // Initial state: can attack
    assert!(attacker.can_attack(&target));

    // Open shop
    attacker.flags.is_shop_open = true;

    // Now attack should be blocked
    assert!(!attacker.can_act());
    assert!(!attacker.can_attack(&target));
}

#[test]
fn test_block_attack_in_safe_zone() {
    let mut attacker = BattleInstance::new(TargetType::Player, 1);
    let target = BattleInstance::new(TargetType::Monster, 0);

    // Initial state: can attack
    assert!(attacker.can_attack(&target));

    // Enter safe zone
    attacker.is_in_safe_zone = true;

    // Now attack against monster should be blocked
    assert!(!attacker.can_attack(&target));

    // But attack against training dummy should be allowed
    let dummy = BattleInstance::new(TargetType::TrainingDummy, 0);
    assert!(attacker.can_attack(&dummy));
    
    // If target is in safe zone
    attacker.is_in_safe_zone = false;
    let mut safe_target = BattleInstance::new(TargetType::Monster, 0);
    safe_target.is_in_safe_zone = true;
    assert!(!attacker.can_attack(&safe_target));
    
    let mut safe_dummy = BattleInstance::new(TargetType::TrainingDummy, 0);
    safe_dummy.is_in_safe_zone = true;
    assert!(attacker.can_attack(&safe_dummy));
}

#[test]
fn test_allow_attack_after_resetting_negative_states() {
    let mut attacker = BattleInstance::new(TargetType::Player, 1);
    let target = BattleInstance::new(TargetType::Monster, 0);

    // Set a negative state (stun)
    attacker.flags.is_stun = true;
    assert!(!attacker.can_act());
    assert!(!attacker.can_attack(&target));

    // Reset negative state
    attacker.flags.is_stun = false;

    // Now attack should be allowed
    assert!(attacker.can_act());
    assert!(attacker.can_attack(&target));
}

#[test]
fn test_pvp_peace_mode_same_kingdom() {
    let attacker = BattleInstance::new(TargetType::Player, 1); // Kingdom 1
    let target = BattleInstance::new(TargetType::Player, 1);   // Kingdom 1

    // Default PvP mode is Peace. Cannot attack same kingdom.
    assert_eq!(attacker.pvp_mode, PvpMode::Peace);
    assert!(!attacker.can_attack(&target));
}

#[test]
fn test_pvp_peace_mode_different_kingdom() {
    let attacker = BattleInstance::new(TargetType::Player, 1); // Kingdom 1
    let target = BattleInstance::new(TargetType::Player, 2);   // Kingdom 2

    // Default PvP mode is Peace. Can attack different kingdom.
    assert_eq!(attacker.pvp_mode, PvpMode::Peace);
    assert!(attacker.can_attack(&target));
}

#[test]
fn test_cannot_attack_npc() {
    let attacker = BattleInstance::new(TargetType::Player, 1);
    let npc = BattleInstance::new(TargetType::Npc, 0);

    assert!(!attacker.can_attack(&npc));
}

#[test]
fn test_cannot_attack_dead_target() {
    let attacker = BattleInstance::new(TargetType::Player, 1);
    let mut target = BattleInstance::new(TargetType::Monster, 0);

    target.flags.is_dead = true;

    assert!(!attacker.can_attack(&target));
}

#[test]
fn test_all_negative_states_block_attack() {
    let target = BattleInstance::new(TargetType::Monster, 0);

    let states = vec![
        |flags: &mut m2_client::battle_permissions::InstanceFlags| flags.is_dead = true,
        |flags: &mut m2_client::battle_permissions::InstanceFlags| flags.is_stun = true,
        |flags: &mut m2_client::battle_permissions::InstanceFlags| flags.is_sleep = true,
        |flags: &mut m2_client::battle_permissions::InstanceFlags| flags.is_paralyzed = true,
        |flags: &mut m2_client::battle_permissions::InstanceFlags| flags.is_fishing = true,
        |flags: &mut m2_client::battle_permissions::InstanceFlags| flags.is_mining = true,
        |flags: &mut m2_client::battle_permissions::InstanceFlags| flags.is_shop_open = true,
    ];

    for state_setter in states {
        let mut attacker = BattleInstance::new(TargetType::Player, 1);
        state_setter(&mut attacker.flags);
        assert!(!attacker.can_act());
        assert!(!attacker.can_attack(&target));
    }
}
