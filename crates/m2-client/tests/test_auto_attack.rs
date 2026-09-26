use m2_client::auto_attack::{distance_approx, Actor, AutoAttackManager, AutoAttackResult};

fn create_test_actor(vid: u32, x: i32, y: i32, hp: i32) -> Actor {
    Actor {
        vid,
        name: format!("Actor{}", vid),
        race: 0,
        x,
        y,
        z: 0,
        rot: 0.0,
        hp,
        max_hp: 1000,
        is_pc: true,
        is_enemy: false,
    }
}

#[test]
fn test_distance_approx_math() {
    // Tests for specific bitshift manhattan distance approx calculation logic
    let dist1 = distance_approx(0, 0, 100, 100);
    let max = 100;
    let min = 100;
    let expected = ((max << 8) + (max << 3) - (max << 4) - (max << 1) + (min << 7) - (min << 5) + (min << 3) - (min << 1)) >> 8;
    assert_eq!(dist1, expected);
    
    // Diagonal test (100, 100 is length ~141 in euclidian, approx uses slightly different scale)
    // Here we just test if the approximation function returns expected values from algorithm
    
    let dist2 = distance_approx(50, 20, 10, -10);
    assert!(dist2 > 0);
}

#[test]
fn test_auto_attack_disabled_does_nothing() {
    let mut manager = AutoAttackManager::new(300, 1.0);
    let main = create_test_actor(1, 0, 0, 100);
    let target = create_test_actor(2, 100, 100, 100);
    
    let result = manager.update(0.1, &main, Some(&target));
    assert_eq!(result, AutoAttackResult {
        need_approach: false,
        trigger_strike: false,
        clear_target: false,
    });
}

#[test]
fn test_auto_attack_triggers_strike() {
    let mut manager = AutoAttackManager::new(300, 1.0);
    manager.set_target(2);
    
    let main = create_test_actor(1, 0, 0, 100);
    let target = create_test_actor(2, 50, 50, 100); // within range (300)
    
    // Time < cooldown -> no strike yet
    let result = manager.update(0.5, &main, Some(&target));
    assert_eq!(result, AutoAttackResult {
        need_approach: false,
        trigger_strike: false,
        clear_target: false,
    });
    
    // Time crosses cooldown -> strikes and resets timer
    let result = manager.update(0.6, &main, Some(&target));
    assert_eq!(result, AutoAttackResult {
        need_approach: false,
        trigger_strike: true, // 0.5 + 0.6 = 1.1 >= 1.0
        clear_target: false,
    });
    
    assert_eq!(manager.time_since_last_attack, 0.0);
}

#[test]
fn test_auto_attack_needs_approach() {
    let mut manager = AutoAttackManager::new(100, 1.0);
    manager.set_target(2);
    
    let main = create_test_actor(1, 0, 0, 100);
    let target = create_test_actor(2, 200, 0, 100); // 200 > 100 (range)
    
    let result = manager.update(0.5, &main, Some(&target));
    assert_eq!(result, AutoAttackResult {
        need_approach: true,
        trigger_strike: false,
        clear_target: false,
    });
    
    // time shouldn't reset if we are approaching (we add time, but trigger strike never fires if we return early)
    assert_eq!(manager.time_since_last_attack, 0.5);
}

#[test]
fn test_auto_attack_clears_on_target_death() {
    let mut manager = AutoAttackManager::new(300, 1.0);
    manager.set_target(2);
    
    let main = create_test_actor(1, 0, 0, 100);
    let target_dead = create_test_actor(2, 50, 50, 0); // hp = 0
    
    let result = manager.update(0.5, &main, Some(&target_dead));
    assert_eq!(result, AutoAttackResult {
        need_approach: false,
        trigger_strike: false,
        clear_target: true,
    });
    
    assert!(!manager.is_auto_attack_enabled);
    assert_eq!(manager.target_vid, None);
}

#[test]
fn test_auto_attack_clears_on_target_none() {
    let mut manager = AutoAttackManager::new(300, 1.0);
    manager.set_target(2);
    
    let main = create_test_actor(1, 0, 0, 100);
    
    let result = manager.update(0.5, &main, None);
    assert_eq!(result, AutoAttackResult {
        need_approach: false,
        trigger_strike: false,
        clear_target: true,
    });
    
    assert!(!manager.is_auto_attack_enabled);
    assert_eq!(manager.target_vid, None);
}

#[test]
fn test_auto_attack_clears_on_different_target_vid() {
    let mut manager = AutoAttackManager::new(300, 1.0);
    manager.set_target(2); // Set target to 2
    
    let main = create_test_actor(1, 0, 0, 100);
    let wrong_target = create_test_actor(3, 50, 50, 100); // We pass target 3
    
    let result = manager.update(0.5, &main, Some(&wrong_target));
    assert_eq!(result, AutoAttackResult {
        need_approach: false,
        trigger_strike: false,
        clear_target: true,
    });
    
    assert!(!manager.is_auto_attack_enabled);
}

#[test]
fn test_auto_attack_clears_manually() {
    let mut manager = AutoAttackManager::new(300, 1.0);
    manager.set_target(2);
    manager.time_since_last_attack = 0.5;
    
    manager.clear_target();
    assert!(!manager.is_auto_attack_enabled);
    assert_eq!(manager.target_vid, None);
    assert_eq!(manager.time_since_last_attack, 0.0);
}
