use m2_client_bridge::state::actor_manager::{ActorManager, Actor};

#[test]
fn test_insert_and_get() {
    let mut manager = ActorManager::new();
    
    manager.insert(Actor { vid: 1001, x: 50.0, y: 60.0 });
    
    let actor = manager.get(1001);
    assert!(actor.is_some());
    let actor = actor.unwrap();
    assert_eq!(actor.vid, 1001);
    assert_eq!(actor.x, 50.0);
    assert_eq!(actor.y, 60.0);
}

#[test]
fn test_remove() {
    let mut manager = ActorManager::new();
    
    manager.insert(Actor { vid: 1001, x: 50.0, y: 60.0 });
    assert!(manager.get(1001).is_some());
    
    manager.remove(1001);
    assert!(manager.get(1001).is_none());
}

#[test]
fn test_find_nearest_enemy_50_actors() {
    let mut manager = ActorManager::new();
    
    // Dodajemy 50 aktorow
    for i in 1..=50 {
        manager.insert(Actor {
            vid: i,
            x: (i * 10) as f32,
            y: (i * 10) as f32,
        });
    }
    
    // Sprawdzamy najblizszego aktora od (55, 55) - powinien to byc aktor o vid=5 lub vid=6,
    // (50, 50) dla vid=5, dystans to 5^2+5^2 = 50.
    // (60, 60) dla vid=6, dystans to 5^2+5^2 = 50.
    // W zaleznosci od iteracji moze byc 5 lub 6.
    // Aby test byl deterministyczny poszukamy od punktu blizej 50: (51, 51)
    let nearest = manager.find_nearest_enemy(51, 51);
    assert!(nearest.is_some());
    assert_eq!(nearest.unwrap(), 5);
    
    // Test dla oddalonego punktu
    let nearest_far = manager.find_nearest_enemy(1000, 1000);
    assert!(nearest_far.is_some());
    assert_eq!(nearest_far.unwrap(), 50); // aktor 50 jest na pozycji (500, 500), najblizej 1000
}

#[test]
fn test_get_nonexistent() {
    let manager = ActorManager::new();
    assert!(manager.get(999).is_none());
}
