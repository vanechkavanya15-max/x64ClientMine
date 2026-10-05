use m2_client::game_auto_hunt_ai::{AutoHuntAction, AutoHuntAiManager, AutoHuntState, MonsterTarget, PlayerState};
use std::time::{Duration, Instant};

#[test]
fn test_ai_state_transitions() {
    let mut ai = AutoHuntAiManager::new(1500.0, 200.0);
    assert_eq!(ai.state, AutoHuntState::Idle);

    ai.start();
    assert_eq!(ai.state, AutoHuntState::Search);

    let monsters = vec![MonsterTarget {
        vid: 101,
        x: 1000.0,
        y: 1000.0,
        hp: 100,
        max_hp: 100,
        is_dead: false,
    }];
    let time = Instant::now();
    let mut player = PlayerState::default();

    // From Search -> Approach
    let action = ai.update(&player, &monsters, time);
    assert_eq!(action, AutoHuntAction::SetTarget(101));
    assert_eq!(ai.state, AutoHuntState::Approach);

    // From Approach -> Approach (MoveTo)
    let action = ai.update(&player, &monsters, time);
    assert_eq!(action, AutoHuntAction::MoveTo(1000.0, 1000.0));
    assert_eq!(ai.state, AutoHuntState::Approach);

    // From Approach -> Attack
    player.x = 900.0;
    player.y = 1000.0;
    let action = ai.update(&player, &monsters, time); // 100 dist
    assert_eq!(action, AutoHuntAction::StartAttack(101));
    assert_eq!(ai.state, AutoHuntState::Attack);

    ai.stop();
    assert_eq!(ai.state, AutoHuntState::Idle);
}

#[test]
fn test_ai_optimal_target_selection() {
    let mut ai = AutoHuntAiManager::new(1500.0, 200.0);
    ai.start();

    let monsters = vec![
        MonsterTarget {
            vid: 1,
            x: 1000.0, // dist = 1000
            y: 0.0,
            hp: 1000,
            max_hp: 1000, // hp_factor = 1.0
            is_dead: false,
        },
        MonsterTarget {
            vid: 2,
            x: 800.0, // dist = 800
            y: 0.0,
            hp: 1000,
            max_hp: 1000, // hp_factor = 1.0
            is_dead: false,
        },
        MonsterTarget {
            vid: 3,
            x: 900.0, // dist = 900
            y: 0.0,
            hp: 10,
            max_hp: 1000, // hp_factor = 0.01 (score = 900 + 2 = 902) vs (800 + 200 = 1000)
            is_dead: false,
        },
        MonsterTarget {
            vid: 4, // dead
            x: 100.0,
            y: 0.0,
            hp: 0,
            max_hp: 100,
            is_dead: true,
        },
    ];

    let time = Instant::now();
    let player = PlayerState::default();
    let action = ai.update(&player, &monsters, time);

    // Should pick 3 because HP weight makes its lowest (902 vs 1000 for vid 2)
    assert_eq!(action, AutoHuntAction::SetTarget(3));
}

#[test]
fn test_ai_skill_usage() {
    let mut ai = AutoHuntAiManager::new(1500.0, 200.0);
    ai.add_skill(50, Duration::from_secs(5));
    ai.start();

    let monsters = vec![MonsterTarget {
        vid: 99,
        x: 100.0,
        y: 0.0,
        hp: 100,
        max_hp: 100,
        is_dead: false,
    }];
    
    let time = Instant::now();
    let player = PlayerState::default();

    // Search
    ai.update(&player, &monsters, time);
    // Approach -> Attack
    ai.update(&player, &monsters, time);

    // In Attack state: first use skill
    let action1 = ai.update(&player, &monsters, time);
    assert_eq!(action1, AutoHuntAction::UseSkill(50));

    // Next frame: skill on CD, combo attack
    let action2 = ai.update(&player, &monsters, time);
    assert_eq!(action2, AutoHuntAction::ComboAttack(99));

    // Time passes: CD over
    let action3 = ai.update(&player, &monsters, time + Duration::from_secs(6));
    assert_eq!(action3, AutoHuntAction::UseSkill(50));
}

#[test]
fn test_ai_target_death() {
    let mut ai = AutoHuntAiManager::new(1500.0, 200.0);
    ai.start();

    let mut monsters = vec![MonsterTarget {
        vid: 7,
        x: 100.0,
        y: 0.0,
        hp: 100,
        max_hp: 100,
        is_dead: false,
    }];
    let time = Instant::now();
    let player = PlayerState::default();

    // Search
    ai.update(&player, &monsters, time);
    // Approach -> Attack
    ai.update(&player, &monsters, time);

    // Target dies
    monsters[0].hp = 0;
    monsters[0].is_dead = true;

    // AI should transition back to search and do nothing
    let action = ai.update(&player, &monsters, time);
    assert_eq!(action, AutoHuntAction::None);
    assert_eq!(ai.state, AutoHuntState::Search);
}

#[test]
fn test_ai_player_edge_cases() {
    let mut ai = AutoHuntAiManager::new(1500.0, 200.0);
    ai.start();

    let monsters = vec![MonsterTarget {
        vid: 7,
        x: 100.0,
        y: 0.0,
        hp: 100,
        max_hp: 100,
        is_dead: false,
    }];
    let time = Instant::now();
    let mut player = PlayerState::default();

    // Sleeping
    player.is_sleeping = true;
    let action = ai.update(&player, &monsters, time);
    assert_eq!(action, AutoHuntAction::None);
    assert_eq!(ai.state, AutoHuntState::Search); // Remains unchanged
    player.is_sleeping = false;

    // Charging skill
    player.is_charging_skill = true;
    let action = ai.update(&player, &monsters, time);
    assert_eq!(action, AutoHuntAction::None);
    assert_eq!(ai.state, AutoHuntState::Search); // Remains unchanged
    player.is_charging_skill = false;

    // Horse mounting without attack skill
    player.is_mounting_horse = true;
    player.can_attack_horse_level = false;
    let action = ai.update(&player, &monsters, time);
    assert_eq!(action, AutoHuntAction::None); // Transitions to search due to logic
    assert_eq!(ai.state, AutoHuntState::Search);
    player.is_mounting_horse = false;
    
    // Death
    player.is_dead = true;
    let action = ai.update(&player, &monsters, time);
    assert_eq!(action, AutoHuntAction::None);
    assert_eq!(ai.state, AutoHuntState::Idle); // Transitions to Idle and clears target
}
