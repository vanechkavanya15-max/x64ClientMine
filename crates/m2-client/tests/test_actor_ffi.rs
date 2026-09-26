// Dolaczamy modul przez path, poniewaz nie modyfikujemy lib.rs jak bylo zakazane
#[path = "../src/actor_ffi.rs"]
mod actor_ffi;

use std::sync::Mutex;

use actor_ffi::{
    m2_chr_create, m2_chr_delete, m2_chr_get_count, m2_chr_update, test_clear_actors,
    test_get_actor,
};

static TEST_MUTEX: Mutex<()> = Mutex::new(());

// Funkcja pomocnicza czyszczaca stan przed kazdym testem
fn setup() -> std::sync::MutexGuard<'static, ()> {
    let guard = TEST_MUTEX.lock().unwrap();
    test_clear_actors();
    guard
}

#[test]
fn test_actor_create_and_delete_lifecycle() {
    let _guard = setup();

    // Sprawdzamy poczatkowy stan
    assert_eq!(m2_chr_get_count(), 0);

    // Tworzymy nowego aktora (VID = 100)
    let created = m2_chr_create(100, 1, 10.5, 20.5, 30.5, 45.0);
    assert!(created, "Tworzenie aktora powinno sie udac.");

    // Licznik powinien wzrosnac do 1
    assert_eq!(m2_chr_get_count(), 1);

    // Sprawdzamy, czy dane wewnetrzne sa poprawne
    let actor = test_get_actor(100).expect("Aktor powinien istniec.");
    assert_eq!(actor.vid, 100);
    assert_eq!(actor.race, 1);
    assert_eq!(actor.x, 10.5);
    assert_eq!(actor.y, 20.5);
    assert_eq!(actor.z, 30.5);
    assert_eq!(actor.rot, 45.0);
    assert_eq!(actor.time_alive_ms, 0);

    // Proba utworzenia aktora z tym samym VID
    let duplicate_created = m2_chr_create(100, 2, 0.0, 0.0, 0.0, 0.0);
    assert!(!duplicate_created, "Nie powinno sie udac utworzenie duplikatu.");
    assert_eq!(m2_chr_get_count(), 1, "Licznik nie powinien wzrosnac przy duplikacie.");

    // Usuniecie aktora
    let deleted = m2_chr_delete(100);
    assert!(deleted, "Usuwanie istniejacego aktora powinno zwrocic true.");
    assert_eq!(m2_chr_get_count(), 0, "Licznik powinien spasc do 0.");

    // Proba usuniecia nieistniejacego aktora
    let deleted_again = m2_chr_delete(100);
    assert!(!deleted_again, "Usuwanie nieistniejacego aktora powinno zwrocic false.");
}

#[test]
fn test_actor_get_count_multiple() {
    let _guard = setup();

    assert_eq!(m2_chr_get_count(), 0);

    m2_chr_create(1, 1, 0.0, 0.0, 0.0, 0.0);
    m2_chr_create(2, 2, 0.0, 0.0, 0.0, 0.0);
    m2_chr_create(3, 3, 0.0, 0.0, 0.0, 0.0);

    assert_eq!(m2_chr_get_count(), 3);

    m2_chr_delete(2);
    assert_eq!(m2_chr_get_count(), 2);

    m2_chr_delete(1);
    m2_chr_delete(3);
    assert_eq!(m2_chr_get_count(), 0);
}

#[test]
fn test_actor_update_logic() {
    let _guard = setup();

    // Tworzymy dwoch aktorow
    m2_chr_create(10, 1, 0.0, 0.0, 0.0, 0.0);
    m2_chr_create(20, 2, 0.0, 0.0, 0.0, 0.0);

    // Sprawdzamy poczatkowy czas
    assert_eq!(test_get_actor(10).unwrap().time_alive_ms, 0);
    assert_eq!(test_get_actor(20).unwrap().time_alive_ms, 0);

    // Pierwszy update o 100ms
    m2_chr_update(100);

    assert_eq!(test_get_actor(10).unwrap().time_alive_ms, 100);
    assert_eq!(test_get_actor(20).unwrap().time_alive_ms, 100);

    // Kolejny update o 50ms
    m2_chr_update(50);

    assert_eq!(test_get_actor(10).unwrap().time_alive_ms, 150);
    assert_eq!(test_get_actor(20).unwrap().time_alive_ms, 150);
}

#[test]
fn test_actor_update_saturating_add_edge_case() {
    let _guard = setup();

    // Test zapobiegajacy overflow na time_alive_ms
    m2_chr_create(999, 1, 0.0, 0.0, 0.0, 0.0);
    
    // Duzy update, bliski max
    m2_chr_update(u32::MAX - 100);
    assert_eq!(test_get_actor(999).unwrap().time_alive_ms, u32::MAX - 100);

    // Kolejny duzy update, ktory normalnie zrobilby overflow
    m2_chr_update(200);
    assert_eq!(test_get_actor(999).unwrap().time_alive_ms, u32::MAX);
}
