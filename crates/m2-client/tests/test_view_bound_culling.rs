#[path = "../src/view_bound_culling.rs"]
mod view_bound_culling;

use view_bound_culling::{
    ActorInstance, Position, PythonCharacterManager, CHAR_STAGE_VIEW_BOUND, FADE_OUT_DURATION_MS,
};

#[test]
fn test_actor_within_range() {
    let mut manager = PythonCharacterManager::new();
    let main_pos = Position { x: 0.0, y: 0.0 };

    // Actor at (1000.0, 0.0) -> Distance 1000.0
    manager.add_actor(ActorInstance::new(1, 1000.0, 0.0));

    manager.update(&main_pos, 0);

    let actor = manager.alive_instances.get(&1).unwrap();
    assert!(!actor.is_fading_out);
    assert_eq!(actor.fade_out_start_time, None);
}

#[test]
fn test_actor_on_boundary() {
    let mut manager = PythonCharacterManager::new();
    let main_pos = Position { x: 0.0, y: 0.0 };

    // Actor exactly on boundary: 20010.0
    manager.add_actor(ActorInstance::new(1, CHAR_STAGE_VIEW_BOUND + 10.0, 0.0));

    manager.update(&main_pos, 0);

    let actor = manager.alive_instances.get(&1).unwrap();
    assert!(!actor.is_fading_out);
    assert_eq!(actor.fade_out_start_time, None);
}

#[test]
fn test_actor_outside_boundary_starts_fading() {
    let mut manager = PythonCharacterManager::new();
    let main_pos = Position { x: 0.0, y: 0.0 };

    // Actor slightly outside boundary: 20010.1
    manager.add_actor(ActorInstance::new(1, CHAR_STAGE_VIEW_BOUND + 10.1, 0.0));

    manager.update(&main_pos, 100);

    let actor = manager.alive_instances.get(&1).unwrap();
    assert!(actor.is_fading_out);
    assert_eq!(actor.fade_out_start_time, Some(100));
}

#[test]
fn test_actor_removed_after_fade_out() {
    let mut manager = PythonCharacterManager::new();
    let main_pos = Position { x: 0.0, y: 0.0 };

    // Actor slightly outside boundary
    manager.add_actor(ActorInstance::new(1, CHAR_STAGE_VIEW_BOUND + 10.1, 0.0));

    manager.update(&main_pos, 100);
    assert!(manager.alive_instances.contains_key(&1));
    let actor = manager.alive_instances.get(&1).unwrap();
    assert!(actor.is_fading_out);

    // Update before fade out duration ends
    manager.update(&main_pos, 100 + FADE_OUT_DURATION_MS - 1);
    assert!(manager.alive_instances.contains_key(&1));

    // Update exactly at fade out duration
    manager.update(&main_pos, 100 + FADE_OUT_DURATION_MS);
    assert!(!manager.alive_instances.contains_key(&1));
}

#[test]
fn test_actor_recovers_within_range() {
    let mut manager = PythonCharacterManager::new();
    let main_pos = Position { x: 0.0, y: 0.0 };

    // Actor slightly outside boundary
    let actor = ActorInstance::new(1, CHAR_STAGE_VIEW_BOUND + 10.1, 0.0);
    manager.add_actor(actor);

    manager.update(&main_pos, 100);
    assert!(manager.alive_instances.get(&1).unwrap().is_fading_out);

    // Move actor back inside range
    manager.alive_instances.get_mut(&1).unwrap().position.x = 1000.0;

    manager.update(&main_pos, 200);
    let recovered_actor = manager.alive_instances.get(&1).unwrap();
    assert!(!recovered_actor.is_fading_out);
    assert_eq!(recovered_actor.fade_out_start_time, None);
}

#[test]
fn test_multiple_actors() {
    let mut manager = PythonCharacterManager::new();
    let main_pos = Position { x: 0.0, y: 0.0 };

    manager.add_actor(ActorInstance::new(1, 1000.0, 0.0));
    manager.add_actor(ActorInstance::new(2, CHAR_STAGE_VIEW_BOUND + 11.0, 0.0));

    manager.update(&main_pos, 100);

    let actor1 = manager.alive_instances.get(&1).unwrap();
    assert!(!actor1.is_fading_out);

    let actor2 = manager.alive_instances.get(&2).unwrap();
    assert!(actor2.is_fading_out);

    manager.update(&main_pos, 100 + FADE_OUT_DURATION_MS);

    assert!(manager.alive_instances.contains_key(&1));
    assert!(!manager.alive_instances.contains_key(&2));
}
