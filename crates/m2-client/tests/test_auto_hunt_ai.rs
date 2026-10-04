#[path = "../src/game_auto_hunt_ai.rs"]
mod game_auto_hunt_ai;

use game_auto_hunt_ai::{AutoHuntAction, AutoHuntActorInfo, AutoHuntAi, AutoHuntConfig, AutoHuntState};
use glam::Vec2;

#[test]
fn test_auto_hunt_state_transitions() {
    let config = AutoHuntConfig {
        scan_radius: 1000.0,
        attack_range: 200.0,
        use_skills: false,
        skill_cooldown_ms: 3000,
    };
    let mut ai = AutoHuntAi::new(config);
    let my_pos = Vec2::new(0.0, 0.0);

    // Initial state is Idle
    assert_eq!(ai.get_state(), AutoHuntState::Idle);

    let actors = vec![AutoHuntActorInfo {
        vid: 100,
        position: Vec2::new(500.0, 0.0), // out of attack range, in scan range
        is_monster: true,
        is_dead: false,
        hp_percent: 1.0,
    }];

    // Update 1: Idle -> Search
    let action = ai.update(0, my_pos, &actors);
    assert_eq!(ai.get_state(), AutoHuntState::Search);
    assert_eq!(action, AutoHuntAction::None);

    // Update 2: Search -> Approach (found target)
    let action = ai.update(0, my_pos, &actors);
    assert_eq!(ai.get_state(), AutoHuntState::Approach);
    assert_eq!(ai.get_target_vid(), Some(100));
    assert_eq!(action, AutoHuntAction::None); // Transitions wait a tick

    // Update 3: Approach -> MoveTo
    let action = ai.update(0, my_pos, &actors);
    assert_eq!(ai.get_state(), AutoHuntState::Approach);
    assert_eq!(action, AutoHuntAction::MoveTo(Vec2::new(500.0, 0.0)));

    // Simulate moving into range
    let my_pos = Vec2::new(350.0, 0.0); // distance is now 150.0 (< 200.0)

    // Update 4: Approach -> Attack state transition
    let action = ai.update(0, my_pos, &actors);
    assert_eq!(ai.get_state(), AutoHuntState::Attack);
    assert_eq!(action, AutoHuntAction::None);

    // Update 5: Attack -> Perform Attack
    let action = ai.update(0, my_pos, &actors);
    assert_eq!(ai.get_state(), AutoHuntState::Attack);
    assert_eq!(action, AutoHuntAction::Attack(100));
}

#[test]
fn test_auto_hunt_target_selection() {
    let config = AutoHuntConfig {
        scan_radius: 1000.0,
        ..Default::default()
    };
    let mut ai = AutoHuntAi::new(config);
    let my_pos = Vec2::new(0.0, 0.0);

    let actors = vec![
        AutoHuntActorInfo {
            vid: 1, // Close but high HP
            position: Vec2::new(300.0, 0.0), // Dist = 300, HP Penalty = 500 => Score = 800
            is_monster: true,
            is_dead: false,
            hp_percent: 1.0,
        },
        AutoHuntActorInfo {
            vid: 2, // Far but low HP
            position: Vec2::new(600.0, 0.0), // Dist = 600, HP Penalty = 50 => Score = 650
            is_monster: true,
            is_dead: false,
            hp_percent: 0.1,
        },
        AutoHuntActorInfo {
            vid: 3, // Very Far
            position: Vec2::new(900.0, 0.0),
            is_monster: true,
            is_dead: false,
            hp_percent: 1.0,
        },
    ];

    ai.set_state(AutoHuntState::Search);
    ai.update(0, my_pos, &actors);

    // AI should pick target 2 because of lower score (distance + hp_penalty)
    assert_eq!(ai.get_target_vid(), Some(2));
}

#[test]
fn test_auto_hunt_target_loss_and_death() {
    let mut ai = AutoHuntAi::new(AutoHuntConfig::default());
    let my_pos = Vec2::new(0.0, 0.0);

    ai.set_state(AutoHuntState::Approach);
    
    // Simulate finding target
    let actors = vec![AutoHuntActorInfo {
        vid: 99,
        position: Vec2::new(500.0, 0.0),
        is_monster: true,
        is_dead: false,
        hp_percent: 1.0,
    }];
    ai.set_state(AutoHuntState::Search);
    ai.update(0, my_pos, &actors);
    assert_eq!(ai.get_state(), AutoHuntState::Approach);

    // Target dies
    let actors_dead = vec![AutoHuntActorInfo {
        vid: 99,
        position: Vec2::new(500.0, 0.0),
        is_monster: true,
        is_dead: true,
        hp_percent: 0.0,
    }];
    let action = ai.update(0, my_pos, &actors_dead);
    
    // Should clear target and go back to Search
    assert_eq!(ai.get_state(), AutoHuntState::Search);
    assert_eq!(ai.get_target_vid(), None);
    assert_eq!(action, AutoHuntAction::None);
}

#[test]
fn test_auto_hunt_skill_usage() {
    let config = AutoHuntConfig {
        attack_range: 200.0,
        use_skills: true,
        skill_cooldown_ms: 3000,
        ..Default::default()
    };
    let mut ai = AutoHuntAi::new(config);
    let my_pos = Vec2::new(0.0, 0.0);

    let actors = vec![AutoHuntActorInfo {
        vid: 55,
        position: Vec2::new(100.0, 0.0), // in range
        is_monster: true,
        is_dead: false,
        hp_percent: 1.0,
    }];

    // Force into attack state directly
    ai.set_state(AutoHuntState::Attack);
    // Needs a target set manually or via update
    ai.set_state(AutoHuntState::Search);
    ai.update(0, my_pos, &actors); // -> Approach
    ai.update(0, my_pos, &actors); // -> Attack
    
    // First attack should be a skill if time >= cooldown (time=3000, cd=3000 -> ok)
    let action = ai.update(3000, my_pos, &actors);
    assert_eq!(action, AutoHuntAction::UseSkill(1, 55));

    // Next attack immediately should be normal attack because skill is on cooldown
    let action = ai.update(3100, my_pos, &actors);
    assert_eq!(action, AutoHuntAction::Attack(55));

    // Next attack after cooldown should be skill again
    let action = ai.update(6000, my_pos, &actors);
    assert_eq!(action, AutoHuntAction::UseSkill(1, 55));
}
