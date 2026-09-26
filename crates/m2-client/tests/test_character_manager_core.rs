#[path = "../src/character_manager_core.rs"]
mod character_manager_core;

use character_manager_core::CharacterManagerCore;

#[test]
fn test_create_instance_success() {
    let mut manager = CharacterManagerCore::new();
    
    // Test poprawnego dodania nowej postaci
    let success = manager.create_instance(1, 0, "Gracz1".to_string(), (100, 200));
    assert!(success, "Postac powinna zostac dodana pomyslnie");
    assert_eq!(manager.alive_instances.len(), 1);
    
    let instance = manager.get_instance(1).expect("Instancja powinna istniec");
    assert_eq!(instance.vid, 1);
    assert_eq!(instance.race, 0);
    assert_eq!(instance.name, "Gracz1");
    assert_eq!(instance.pos, (100, 200));
}

#[test]
fn test_create_instance_duplicate() {
    let mut manager = CharacterManagerCore::new();
    
    // Dodanie pierwszej postaci
    assert!(manager.create_instance(10, 1, "Gracz2".to_string(), (300, 400)));
    
    // Proba dodania postaci o tym samym VID (powinna zwrocic false)
    let duplicate_success = manager.create_instance(10, 2, "Klon".to_string(), (0, 0));
    assert!(!duplicate_success, "Zduplikowany VID nie powinien zostac dodany");
    
    // Sprawdzenie czy stara instancja pozostala nietknieta
    let instance = manager.get_instance(10).unwrap();
    assert_eq!(instance.name, "Gracz2");
    assert_eq!(manager.alive_instances.len(), 1);
}

#[test]
fn test_delete_instance_success() {
    let mut manager = CharacterManagerCore::new();
    manager.create_instance(5, 0, "Test".to_string(), (0, 0));
    
    // Usuwanie istniejacej postaci
    let deleted = manager.delete_instance(5);
    assert!(deleted, "Instancja powinna zostac pomyslnie usunieta");
    assert!(manager.get_instance(5).is_none(), "Instancja nie powinna juz istniec");
    assert_eq!(manager.alive_instances.len(), 0);
}

#[test]
fn test_delete_instance_not_found() {
    let mut manager = CharacterManagerCore::new();
    
    // Proba usuniecia nieistniejacej postaci
    let deleted = manager.delete_instance(99);
    assert!(!deleted, "Usuwanie nieistniejacej instancji powinno zwrocic false");
}

#[test]
fn test_delete_main_instance() {
    let mut manager = CharacterManagerCore::new();
    manager.create_instance(100, 0, "MainPlayer".to_string(), (0, 0));
    
    // Ustawienie glownej postaci recznie w tescie
    manager.main_actor_vid = Some(100);
    
    // Sprawdzenie czy pobranie glownego dziala
    assert!(manager.get_main_instance().is_some());
    
    // Usuniecie glownego gracza
    manager.delete_instance(100);
    
    // Wskaznik main_actor_vid powinien zostac zresetowany
    assert!(manager.main_actor_vid.is_none(), "Wskaznik glownej postaci powinien zostac wyzerowany");
    assert!(manager.get_main_instance().is_none(), "Glownej postaci nie powinno juz byc w menedzerze");
}

#[test]
fn test_get_instance_mut() {
    let mut manager = CharacterManagerCore::new();
    manager.create_instance(42, 3, "NPC".to_string(), (50, 50));
    
    // Modyfikacja pozycji uzywajac get_instance_mut
    if let Some(instance) = manager.get_instance_mut(42) {
        instance.pos = (150, 150);
        instance.name = "NPC_Zmodyfikowany".to_string();
    }
    
    // Weryfikacja zmian
    let updated = manager.get_instance(42).unwrap();
    assert_eq!(updated.pos, (150, 150));
    assert_eq!(updated.name, "NPC_Zmodyfikowany");
}

#[test]
fn test_get_main_instance_not_found() {
    let mut manager = CharacterManagerCore::new();
    manager.create_instance(1, 0, "InnyGracz".to_string(), (0, 0));
    
    // Ustawiono VID na nieistniejacy
    manager.main_actor_vid = Some(999);
    
    assert!(manager.get_main_instance().is_none(), "Powinno zwrocic None jezeli glowna postac nie istnieje na mapie");
}
