#[path = "../src/fly_manager.rs"]
pub mod fly_manager;

use fly_manager::{FlyManager, FlyProjectile};

#[test]
fn test_trajectory_and_hit() {
    let mut manager = FlyManager::new();

    let projectile = FlyProjectile {
        id: 1,
        shooter_vid: 100,
        target_vid: 200,
        cur_pos: (0.0, 0.0, 0.0),
        target_pos: (100.0, 0.0, 0.0),
        speed: 10.0,
        has_hit: false,
    };

    manager.add_projectile(projectile);

    let mut hit_count = 0;
    
    // Update step 1: Moves 10 units
    manager.update(1.0, |_, _, _| hit_count += 1);
    assert_eq!(manager.projectile_count(), 1);
    let p = manager.get_projectile(1).unwrap();
    assert_eq!(p.cur_pos, (10.0, 0.0, 0.0));
    assert_eq!(hit_count, 0);

    // Update step 2: Moves 75 units, total distance 85, distance to target is 15 (< 20.0), should hit!
    manager.update(7.5, |_, _, _| hit_count += 1);
    
    // Should be removed
    assert_eq!(manager.projectile_count(), 0);
    assert_eq!(hit_count, 1);
}

#[test]
fn test_instant_hit_zero_distance() {
    let mut manager = FlyManager::new();

    let projectile = FlyProjectile {
        id: 2,
        shooter_vid: 100,
        target_vid: 200,
        cur_pos: (10.0, 10.0, 10.0),
        target_pos: (10.0, 10.0, 10.0),
        speed: 10.0,
        has_hit: false,
    };

    manager.add_projectile(projectile);

    let mut hit_count = 0;
    manager.update(0.1, |_, _, _| hit_count += 1);

    assert_eq!(manager.projectile_count(), 0);
    assert_eq!(hit_count, 1);
}

#[test]
fn test_instant_hit_within_radius() {
    let mut manager = FlyManager::new();

    let projectile = FlyProjectile {
        id: 3,
        shooter_vid: 100,
        target_vid: 200,
        cur_pos: (0.0, 0.0, 0.0),
        target_pos: (19.9, 0.0, 0.0),
        speed: 5.0,
        has_hit: false,
    };

    manager.add_projectile(projectile);

    let mut hit_count = 0;
    manager.update(0.1, |_, _, _| hit_count += 1);

    assert_eq!(manager.projectile_count(), 0);
    assert_eq!(hit_count, 1);
}

#[test]
fn test_high_speed_overshoot() {
    let mut manager = FlyManager::new();

    let projectile = FlyProjectile {
        id: 4,
        shooter_vid: 100,
        target_vid: 200,
        cur_pos: (0.0, 0.0, 0.0),
        target_pos: (100.0, 0.0, 0.0),
        speed: 200.0, // move 200 units in 1 sec
        has_hit: false,
    };

    manager.add_projectile(projectile);

    let mut hit_count = 0;
    manager.update(1.0, |_, _, _| hit_count += 1);

    assert_eq!(manager.projectile_count(), 0);
    assert_eq!(hit_count, 1);
}

#[test]
fn test_homing_update_target() {
    let mut manager = FlyManager::new();

    let projectile = FlyProjectile {
        id: 5,
        shooter_vid: 100,
        target_vid: 200,
        cur_pos: (0.0, 0.0, 0.0),
        target_pos: (100.0, 0.0, 0.0),
        speed: 10.0,
        has_hit: false,
    };

    manager.add_projectile(projectile);

    manager.update(1.0, |_, _, _| {});
    let p = manager.get_projectile(5).unwrap();
    assert_eq!(p.cur_pos, (10.0, 0.0, 0.0));

    // Target moves!
    manager.update_target(5, (10.0, 100.0, 0.0));

    manager.update(1.0, |_, _, _| {});
    let p = manager.get_projectile(5).unwrap();
    assert_eq!(p.cur_pos, (10.0, 10.0, 0.0));
}

#[test]
fn test_multiple_projectiles() {
    let mut manager = FlyManager::new();

    manager.add_projectile(FlyProjectile {
        id: 1,
        shooter_vid: 100,
        target_vid: 200,
        cur_pos: (0.0, 0.0, 0.0),
        target_pos: (100.0, 0.0, 0.0),
        speed: 50.0,
        has_hit: false,
    });

    manager.add_projectile(FlyProjectile {
        id: 2,
        shooter_vid: 101,
        target_vid: 201,
        cur_pos: (0.0, 0.0, 0.0),
        target_pos: (30.0, 0.0, 0.0),
        speed: 50.0, // Will hit in first update
        has_hit: false,
    });

    let mut hits = std::collections::HashSet::new();
    
    manager.update(1.0, |id, _, _| {
        hits.insert(id);
    });

    // Proj 1 distance remaining: 50
    // Proj 2 distance remaining: -20, so it hits
    assert_eq!(manager.projectile_count(), 1);
    assert!(hits.contains(&2));
    assert!(!hits.contains(&1));

    manager.update(1.0, |id, _, _| {
        hits.insert(id);
    });

    assert_eq!(manager.projectile_count(), 0);
    assert!(hits.contains(&1));
}
