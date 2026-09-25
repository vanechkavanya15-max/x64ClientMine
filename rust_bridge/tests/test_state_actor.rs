use m2_client_bridge::state::actor::{Actor, ActorManager};

#[test]
fn test_actor_creation() {
    // Tworzenie aktora i sprawdzanie pol
    let actor = Actor::new(100, 10.0, 20.0, 30.0, 90.0, 1, 1000, true);
    assert_eq!(actor.vid, 100);
    assert_eq!(actor.x, 10.0);
    assert_eq!(actor.y, 20.0);
    assert_eq!(actor.z, 30.0);
    assert_eq!(actor.rotation, 90.0);
    assert_eq!(actor.race, 1);
    assert_eq!(actor.hp, 1000);
    assert!(actor.is_aggressive);
}

#[test]
fn test_actor_manager_add_and_get() {
    // Inicjalizacja menedzera i dodawanie aktora
    let mut manager = ActorManager::new();
    let actor = Actor::new(1, 0.0, 0.0, 0.0, 0.0, 0, 100, false);
    
    manager.add_or_update_actor(actor.clone());
    
    // Sprawdzanie czy aktor zostal poprawnie dodany i mozna go pobrac
    assert_eq!(manager.count(), 1);
    
    let retrieved_actor = manager.get_actor(1).unwrap();
    assert_eq!(retrieved_actor.vid, 1);
    assert_eq!(retrieved_actor.hp, 100);
    assert_eq!(*retrieved_actor, actor);
}

#[test]
fn test_actor_manager_update() {
    let mut manager = ActorManager::new();
    let actor = Actor::new(2, 10.0, 10.0, 10.0, 0.0, 2, 500, true);
    manager.add_or_update_actor(actor);
    
    // Aktualizacja pozycji
    let position_updated = manager.update_position(2, 15.0, 20.0, 25.0, 180.0);
    assert!(position_updated);
    
    let updated_actor = manager.get_actor(2).unwrap();
    assert_eq!(updated_actor.x, 15.0);
    assert_eq!(updated_actor.y, 20.0);
    assert_eq!(updated_actor.z, 25.0);
    assert_eq!(updated_actor.rotation, 180.0);
    
    // Aktualizacja HP
    let hp_updated = manager.update_hp(2, 250);
    assert!(hp_updated);
    
    let updated_actor = manager.get_actor(2).unwrap();
    assert_eq!(updated_actor.hp, 250);
}

#[test]
fn test_actor_manager_remove() {
    let mut manager = ActorManager::new();
    let actor = Actor::new(3, 0.0, 0.0, 0.0, 0.0, 0, 100, false);
    manager.add_or_update_actor(actor);
    
    // Usuwanie istniejacego aktora
    let removed = manager.remove_actor(3);
    assert!(removed);
    assert_eq!(manager.count(), 0);
    assert!(manager.get_actor(3).is_none());
    
    // Proba usuniecia nieistniejacego aktora
    let removed_nonexistent = manager.remove_actor(3);
    assert!(!removed_nonexistent);
}

#[test]
fn test_actor_manager_invalid_vid() {
    let mut manager = ActorManager::new();
    let actor = Actor::new(5, 0.0, 0.0, 0.0, 0.0, 0, 100, false);
    manager.add_or_update_actor(actor);
    
    // Odpornosc na bledne/nieistniejace VID (pobieranie)
    let invalid_actor = manager.get_actor(9999);
    assert!(invalid_actor.is_none());
    
    // Odpornosc na bledne VID (aktualizacja pozycji)
    let position_updated = manager.update_position(9999, 1.0, 2.0, 3.0, 4.0);
    assert!(!position_updated);
    
    // Odpornosc na bledne VID (aktualizacja HP)
    let hp_updated = manager.update_hp(9999, 0);
    assert!(!hp_updated);
    
    // Stan menedzera nie powinien ulec zmianie
    assert_eq!(manager.count(), 1);
}

#[test]
fn test_actor_manager_clear() {
    let mut manager = ActorManager::new();
    manager.add_or_update_actor(Actor::new(1, 0.0, 0.0, 0.0, 0.0, 0, 100, false));
    manager.add_or_update_actor(Actor::new(2, 0.0, 0.0, 0.0, 0.0, 0, 100, false));
    
    assert_eq!(manager.count(), 2);
    
    // Czyszczenie wszystkich aktorow
    manager.clear();
    
    assert_eq!(manager.count(), 0);
    assert!(manager.get_actor(1).is_none());
    assert!(manager.get_actor(2).is_none());
}
