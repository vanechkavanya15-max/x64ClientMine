use m2_client_bridge::state::points_registry::*;

#[test]
fn test_points_registry_constants() {
    assert_eq!(POINTS_REGISTRY, 0);
    assert_eq!(POINT_HP, 5);
    assert_eq!(POINT_SP, 6);
    assert_eq!(POINT_MAX_HP, 7);
    assert_eq!(POINT_ATT_SPEED, 19);
    assert_eq!(POINT_MOV_SPEED, 20);
    assert_eq!(POINT_DEF_GRADE, 21);
}

#[test]
fn test_points_registry_struct() {
    let mut reg = PointsRegistry::new();
    
    // Sprawdz wartosc domyslna
    assert_eq!(reg.get(POINT_HP), 0);
    
    // Ustaw i pobierz wartosc
    reg.set(POINT_HP, 1500);
    assert_eq!(reg.get(POINT_HP), 1500);
    
    // Ustaw i pobierz inna wartosc
    reg.set(POINT_MAX_HP, 5000);
    assert_eq!(reg.get(POINT_MAX_HP), 5000);
    
    // Sprawdzanie granic
    reg.set(129, 999);
    assert_eq!(reg.get(129), 999);
    
    // Poza zakresem - nie powinno panikowac, a pobieranie powinno zwracac 0
    reg.set(130, 100);
    assert_eq!(reg.get(130), 0);
    
    reg.set(255, 123);
    assert_eq!(reg.get(255), 0);
}

#[test]
fn test_ffi_functions_global() {
    // Inicjalizacja rejestru globalnego
    m2_points_registry_init();
    
    // Ustawienie i pobranie z globalnego rejestru
    m2_player_set_stat(POINT_HP, 2500);
    assert_eq!(m2_player_get_stat(POINT_HP), 2500);
    
    m2_player_set_stat(POINT_MAX_HP, 6000);
    assert_eq!(m2_player_get_stat(POINT_MAX_HP), 6000);
    
    // Zabezpieczenie przed przekroczeniem limitu
    m2_player_set_stat(130, 9999);
    assert_eq!(m2_player_get_stat(130), 0);
}

#[test]
fn test_ffi_functions_pointer() {
    let ptr = m2_points_registry_create();
    assert!(!ptr.is_null());
    
    // Ustaw i pobierz
    m2_points_registry_set(ptr, POINT_SP, 300);
    assert_eq!(m2_points_registry_get(ptr, POINT_SP), 300);
    
    // Przekroczenie limitu indeksu
    m2_points_registry_set(ptr, 150, 1234);
    assert_eq!(m2_points_registry_get(ptr, 150), 0);
    
    // Sprawdzenie na null - nie powinno panikowac
    m2_points_registry_set(std::ptr::null_mut(), POINT_HP, 100);
    assert_eq!(m2_points_registry_get(std::ptr::null(), POINT_HP), 0);
    
    // Zwolnienie pamieci
    m2_points_registry_free(ptr);
    
    // Przekazanie null do free - nie powinno panikowac
    m2_points_registry_free(std::ptr::null_mut());
}
