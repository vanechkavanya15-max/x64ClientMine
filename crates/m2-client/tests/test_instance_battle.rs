#[path = "../src/instance_battle.rs"]
mod instance_battle;

use instance_battle::{ActorCondition, BattleState, InstanceBattle};

#[test]
fn test_can_act_condition() {
    let mut battle = InstanceBattle::new();
    
    battle.condition = ActorCondition::Normal;
    assert!(battle.can_act(), "Should be able to act when Normal");

    battle.condition = ActorCondition::Stunned;
    assert!(!battle.can_act(), "Should NOT be able to act when Stunned");

    battle.condition = ActorCondition::Dead;
    assert!(!battle.can_act(), "Should NOT be able to act when Dead");

    battle.condition = ActorCondition::Frozen;
    assert!(!battle.can_act(), "Should NOT be able to act when Frozen");

    battle.condition = ActorCondition::Sleeping;
    assert!(!battle.can_act(), "Should NOT be able to act when Sleeping");
}

#[test]
fn test_can_attack() {
    let mut battle = InstanceBattle::new();
    battle.attack_cooldown_ms = 500;
    
    // Normal can attack
    assert!(battle.can_attack(0));
    
    // Skill blocks attack
    battle.state = BattleState::UseSkill;
    assert!(!battle.can_attack(0));
    
    // Half cooldown blocks attack when already attacking
    battle.state = BattleState::NormalAttack;
    battle.last_attack_time_ms = 1000;
    assert!(!battle.can_attack(1100), "Elapsed 100 < 250, should block");
    
    // Passed half cooldown
    assert!(battle.can_attack(1300), "Elapsed 300 >= 250, should allow (half cooldown)");
}

#[test]
fn test_process_normal_attack() {
    let mut battle = InstanceBattle::new();
    battle.attack_cooldown_ms = 500;
    
    // Attempt first attack
    let current_time = 1000;
    let rot = 45.0;
    assert!(battle.process_normal_attack(current_time, rot));
    assert_eq!(battle.state, BattleState::NormalAttack);
    assert_eq!(battle.last_attack_time_ms, current_time);
    assert_eq!(battle.current_rotation_deg, rot);
    
    // Attempt second attack too early
    assert!(!battle.process_normal_attack(1100, rot), "Should reject attack before cooldown");
    
    // Attempt second attack after cooldown
    assert!(battle.process_normal_attack(1600, rot), "Should accept attack after cooldown");
    assert_eq!(battle.last_attack_time_ms, 1600);
}

#[test]
fn test_process_normal_attack_stunned() {
    let mut battle = InstanceBattle::new();
    battle.condition = ActorCondition::Stunned;
    assert!(!battle.process_normal_attack(1000, 0.0), "Should reject attack when stunned");
}

#[test]
fn test_input_combo_attack_command() {
    let mut battle = InstanceBattle::new();
    battle.combo_window_start_ms = 300;
    battle.combo_window_end_ms = 700;
    battle.attack_cooldown_ms = 500;
    
    let time1 = 1000;
    
    // First attack starts the combo via normal attack logic
    assert!(battle.input_combo_attack_command(time1, 90.0));
    assert_eq!(battle.state, BattleState::NormalAttack);
    assert_eq!(battle.combo_index, 1);
    
    // Trying to combo too early (before window)
    let time2 = 1100;
    assert!(!battle.input_combo_attack_command(time2, 90.0), "Should reject combo before window");
    
    // Trying to combo in the valid window
    let time3 = 1400; // 400ms elapsed
    assert!(battle.input_combo_attack_command(time3, 90.0), "Should accept combo in window");
    assert_eq!(battle.state, BattleState::ComboAttack);
    assert_eq!(battle.combo_index, 2);
    assert_eq!(battle.last_attack_time_ms, time3);
    
    // Pre-input test: if pre_input is set, reject new
    battle.is_pre_input = true;
    assert!(!battle.input_combo_attack_command(time3 + 400, 90.0), "Should reject if already pre-input");
    battle.is_pre_input = false;
    
    // Trying to combo too late
    let time4 = time3 + 800; // 800ms elapsed
    assert!(!battle.input_combo_attack_command(time4, 90.0), "Should reject combo after window");
}
