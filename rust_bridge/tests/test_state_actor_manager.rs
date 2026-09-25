use m2_client_bridge::state::actor_manager::ActorManager;

#[test]
fn test_add_and_get_actor() {
    let mut manager = ActorManager::new();
    
    // Dodajemy aktora
    manager.add_or_update_actor(1001, 50.0, 60.0);
    
    let actor = manager.get_actor(1001);
    assert!(actor.is_some());
    let actor = actor.unwrap();
    assert_eq!(actor.vid, 1001);
    assert_eq!(actor.x, 50.0);
    assert_eq!(actor.y, 60.0);
}

#[test]
fn test_update_actor() {
    let mut manager = ActorManager::new();
    
    // Dodajemy aktora
    manager.add_or_update_actor(1001, 50.0, 60.0);
    
    // Aktualizujemy aktora
    manager.add_or_update_actor(1001, 70.0, 80.0);
    
    let actor = manager.get_actor(1001).unwrap();
    assert_eq!(actor.x, 70.0);
    assert_eq!(actor.y, 80.0);
}

#[test]
fn test_remove_actor() {
    let mut manager = ActorManager::new();
    
    manager.add_or_update_actor(1001, 50.0, 60.0);
    
    // Usuwamy istniejacego aktora
    let removed = manager.remove_actor(1001);
    assert!(removed);
    
    let actor = manager.get_actor(1001);
    assert!(actor.is_none());
    
    // Usuwamy nieistniejacego aktora
    let removed_again = manager.remove_actor(1001);
    assert!(!removed_again);
}

#[test]
fn test_get_nonexistent_actor() {
    let manager = ActorManager::new();
    
    let actor = manager.get_actor(9999);
    assert!(actor.is_none());
}

#[test]
fn test_find_actors_in_radius() {
    let mut manager = ActorManager::new();
    
    manager.add_or_update_actor(1, 0.0, 0.0);
    manager.add_or_update_actor(2, 10.0, 0.0);
    manager.add_or_update_actor(3, 0.0, 10.0);
    manager.add_or_update_actor(4, 10.0, 10.0);
    manager.add_or_update_actor(5, 100.0, 100.0);
    
    // Sprawdzamy promien 15 od (0, 0). Powinno znalezc 1, 2, 3, 4,
    // poniewaz pierwiastek(10^2 + 10^2) to ok. 14.14, wiec <= 15
    let mut in_radius = manager.find_actors_in_radius(0.0, 0.0, 15.0);
    in_radius.sort_by(|a, b| a.vid.cmp(&b.vid));
    
    assert_eq!(in_radius.len(), 4);
    assert_eq!(in_radius[0].vid, 1);
    assert_eq!(in_radius[1].vid, 2);
    assert_eq!(in_radius[2].vid, 3);
    assert_eq!(in_radius[3].vid, 4);
    
    // Sprawdzamy promien 5 od (0, 0). Powinno znalezc tylko 1
    let in_radius_small = manager.find_actors_in_radius(0.0, 0.0, 5.0);
    assert_eq!(in_radius_small.len(), 1);
    assert_eq!(in_radius_small[0].vid, 1);
    
    // Sprawdzamy promien 200 od (0, 0). Powinno znalezc wszystko (5 elementow)
    let in_radius_large = manager.find_actors_in_radius(0.0, 0.0, 200.0);
    assert_eq!(in_radius_large.len(), 5);
}
