#[path = "../src/character_manager_core.rs"]
mod character_manager_core;

use character_manager_core::CharacterManagerCore;

#[test]
fn test_create_instance_success() {
    let mut manager = CharacterManagerCore::new();
    
    // Test poprawnego dodania nowej postaci
    let success = manager.create_instance(1, 0, 100.0, 200.0, 300.0);
    assert!(success, "Postac powinna zostac dodana pomyslnie");
    assert_eq!(manager.alive_instances.len(), 1);
    
    let instance = manager.get_instance(1).expect("Instancja powinna istniec");
    assert_eq!(instance.vid, 1);
    assert_eq!(instance.race, 0);
    assert_eq!(instance.x, 100.0);
    assert_eq!(instance.y, 200.0);
    assert_eq!(instance.z, 300.0);
}

#[test]
fn test_create_instance_duplicate() {
    let mut manager = CharacterManagerCore::new();
    
    // Dodanie pierwszej postaci
    assert!(manager.create_instance(10, 1, 300.0, 400.0, 0.0));
    
    // Proba dodania postaci o tym samym VID (powinna zwrocic false)
    let duplicate_success = manager.create_instance(10, 2, 0.0, 0.0, 0.0);
    assert!(!duplicate_success, "Zduplikowany VID nie powinien zostac dodany");
    
    // Sprawdzenie czy stara instancja pozostala nietknieta
    let instance = manager.get_instance(10).unwrap();
    assert_eq!(instance.race, 1);
    assert_eq!(manager.alive_instances.len(), 1);
}

#[test]
fn test_delete_instance_success() {
    let mut manager = CharacterManagerCore::new();
    manager.create_instance(5, 0, 0.0, 0.0, 0.0);
    
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
    manager.create_instance(100, 0, 0.0, 0.0, 0.0);
    
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
    manager.create_instance(42, 3, 50.0, 50.0, 0.0);
    
    // Modyfikacja pozycji uzywajac get_instance_mut
    if let Some(instance) = manager.get_instance_mut(42) {
        instance.x = 150.0;
        instance.y = 150.0;
    }
    
    // Weryfikacja zmian
    let updated = manager.get_instance(42).unwrap();
    assert_eq!(updated.x, 150.0);
    assert_eq!(updated.y, 150.0);
}

#[test]
fn test_get_main_instance_not_found() {
    let mut manager = CharacterManagerCore::new();
    manager.create_instance(1, 0, 0.0, 0.0, 0.0);
    
    // Ustawiono VID na nieistniejacy
    manager.main_actor_vid = Some(999);
    
    assert!(manager.get_main_instance().is_none(), "Powinno zwrocic None jezeli glowna postac nie istnieje na mapie");
}

#[test]
fn test_thousand_instances() {
    let mut manager = CharacterManagerCore::new();
    
    // Dodanie 1000 postaci
    for i in 1..=1000 {
        assert!(manager.create_instance(i, 0, i as f32, i as f32, i as f32));
    }
    
    assert_eq!(manager.count(), 1000);
    
    // Odczyt 1000 postaci
    for i in 1..=1000 {
        let instance = manager.get_instance(i).unwrap();
        assert_eq!(instance.vid, i);
        assert_eq!(instance.x, i as f32);
    }
    
    // Usuniecie 1000 postaci
    for i in 1..=1000 {
        assert!(manager.delete_instance(i));
    }
    
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_ffi_exports() {
    use character_manager_core::{m2_chr_mgr_create_instance, m2_chr_mgr_delete_instance, m2_chr_mgr_get_count, get_character_manager};
    
    // Zapewniamy, ze stan poczatkowy jest pusty
    get_character_manager().write().unwrap().clear();

    assert_eq!(m2_chr_mgr_get_count(), 0);
    
    assert!(m2_chr_mgr_create_instance(2000, 1, 10.0, 20.0, 30.0));
    assert_eq!(m2_chr_mgr_get_count(), 1);
    
    assert!(!m2_chr_mgr_create_instance(2000, 1, 0.0, 0.0, 0.0)); // Duplicate
    assert_eq!(m2_chr_mgr_get_count(), 1);
    
    assert!(m2_chr_mgr_delete_instance(2000));
    assert_eq!(m2_chr_mgr_get_count(), 0);
    
    assert!(!m2_chr_mgr_delete_instance(2000)); // Not found
}
