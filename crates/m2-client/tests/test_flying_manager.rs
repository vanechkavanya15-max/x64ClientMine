use m2_client::flying_manager::{FlyingData, FlyingManager};
use std::collections::HashMap;

#[test]
fn test_spawn_projectile() {
    let mut manager = FlyingManager::new();
    let config = FlyingData { pierce_count: 0 };
    
    let id1 = manager.spawn_projectile(100, 200, (0.0, 0.0, 0.0), config);
    let id2 = manager.spawn_projectile(100, 201, (10.0, 10.0, 10.0), config);

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(manager.instances.len(), 2);
}

#[test]
fn test_update_position() {
    let mut manager = FlyingManager::new();
    let config = FlyingData { pierce_count: 0 };
    
    manager.spawn_projectile(100, 200, (0.0, 0.0, 0.0), config);

    let mut actors = HashMap::new();
    actors.insert(200, (200.0, 0.0, 0.0));

    // dt = 0.05, speed = 1000.0, move_dist = 50.0
    manager.update(0.05, &actors, |_, _, _| {});

    assert_eq!(manager.instances.len(), 1);
    assert_eq!(manager.instances[0].current_pos.0, 50.0);
    assert_eq!(manager.instances[0].current_pos.1, 0.0);
    assert_eq!(manager.instances[0].current_pos.2, 0.0);

    // Another update to reach 100.0
    manager.update(0.05, &actors, |_, _, _| {});
    assert_eq!(manager.instances.len(), 1);
    assert_eq!(manager.instances[0].current_pos.0, 100.0);
    
    // Now at 100.0, target is at 200.0. Distance is 100.0 > 50.0
    // Another update to reach 150.0
    manager.update(0.05, &actors, |_, _, _| {});
    // Wait, distance from 150.0 to 200.0 is 50.0. Collision radius is 50.0, so it will hit!
    // Since pierce_count is 0, it will be destroyed.
    assert_eq!(manager.instances.len(), 0);
}

#[test]
fn test_hit_registration_and_callback() {
    let mut manager = FlyingManager::new();
    let config = FlyingData { pierce_count: 0 };
    
    manager.spawn_projectile(100, 200, (0.0, 0.0, 0.0), config);

    let mut actors = HashMap::new();
    // 200 is the target, position is within collision radius (50.0)
    actors.insert(200, (10.0, 0.0, 0.0));

    let mut hits = Vec::new();
    manager.update(0.01, &actors, |shooter, target, pos| {
        hits.push((shooter, target, pos));
    });

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].0, 100);
    assert_eq!(hits[0].1, 200);
    // Projectile moved towards target, then exploded
    assert_eq!(hits[0].2.0, 10.0);

    // pierce_count was 0, so it should be removed
    assert_eq!(manager.instances.len(), 0);
}

#[test]
fn test_pierce_count() {
    let mut manager = FlyingManager::new();
    let config = FlyingData { pierce_count: 2 };
    
    manager.spawn_projectile(100, 200, (0.0, 0.0, 0.0), config);

    let mut actors = HashMap::new();
    actors.insert(201, (10.0, 0.0, 0.0)); // Another actor in the way

    let mut hits = Vec::new();
    // Move slightly, hitting actor 201
    manager.update(0.01, &actors, |shooter, target, _| {
        hits.push((shooter, target));
    });

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0], (100, 201));
    
    // Pierce count should be 1, instance still alive
    assert_eq!(manager.instances.len(), 1);
    assert_eq!(manager.instances[0].config.pierce_count, 1);
    // Target was hit, shouldn't be hit again
    assert!(manager.instances[0].hit_targets.contains(&201));

    // Update again, shouldn't hit 201 again
    hits.clear();
    manager.update(0.01, &actors, |shooter, target, _| {
        hits.push((shooter, target));
    });
    assert_eq!(hits.len(), 0);
    assert_eq!(manager.instances[0].config.pierce_count, 1);
}

#[test]
fn test_hit_multiple_in_one_frame() {
    let mut manager = FlyingManager::new();
    let config = FlyingData { pierce_count: 1 }; // Can pierce once, hit twice total
    
    manager.spawn_projectile(100, 200, (0.0, 0.0, 0.0), config);

    let mut actors = HashMap::new();
    actors.insert(201, (10.0, 0.0, 0.0));
    actors.insert(202, (10.0, 10.0, 0.0)); // Both within 50.0 radius

    let mut hits = Vec::new();
    manager.update(0.01, &actors, |shooter, target, _| {
        hits.push((shooter, target));
    });

    // Should hit both targets
    assert_eq!(hits.len(), 2);
    // Projectile destroyed because it only had 1 pierce left, hit 201 (pierce = 0), then hit 202 (destroyed)
    assert_eq!(manager.instances.len(), 0);
}
