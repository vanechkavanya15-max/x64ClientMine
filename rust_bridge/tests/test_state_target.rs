use m2_client_bridge::state::target::TargetManager;

#[test]
fn test_set_and_clear_target() {
    let manager = TargetManager::new();
    
    assert!(!manager.has_target());
    assert_eq!(manager.get_target_vid(), None);
    
    // Testowanie poprawnego VID
    manager.set_target(12345, 100.0, 200.0, 300.0, true);
    assert!(manager.has_target());
    assert_eq!(manager.get_target_vid(), Some(12345));
    
    // Czyszczenie celu
    manager.clear_target();
    assert!(!manager.has_target());
    assert_eq!(manager.get_target_vid(), None);
}

#[test]
fn test_invalid_vid_handling() {
    let manager = TargetManager::new();
    
    // Testowanie nieprawidlowego VID (0)
    manager.set_target(0, 10.0, 20.0, 30.0, true);
    assert!(!manager.has_target());
    
    // Proba ustawienia VID=0 podczas gdy inny cel jest juz wybrany,
    // nie powinna go nadpisac wedlug biezacej implementacji (jest po prostu ignorowana)
    manager.set_target(111, 10.0, 20.0, 30.0, true);
    assert_eq!(manager.get_target_vid(), Some(111));
    manager.set_target(0, 50.0, 60.0, 70.0, true);
    assert_eq!(manager.get_target_vid(), Some(111));
}

#[test]
fn test_target_alive_status() {
    let manager = TargetManager::new();
    
    // Sprawdzanie gdy brak celu
    assert!(!manager.is_target_alive());
    
    // Cel, ktory zyje
    manager.set_target(555, 0.0, 0.0, 0.0, true);
    assert!(manager.is_target_alive());
    
    // Cel martwy
    manager.set_target(555, 0.0, 0.0, 0.0, false);
    assert!(!manager.is_target_alive());
}

#[test]
fn test_target_distance() {
    let manager = TargetManager::new();
    
    // Sprawdzanie gdy brak celu
    assert_eq!(manager.get_target_distance(0.0, 0.0, 0.0), None);
    
    manager.set_target(777, 10.0, 20.0, 30.0, true);
    
    // Odleglosc do gracza znajdujacego sie w tym samym miejscu
    let dist_zero = manager.get_target_distance(10.0, 20.0, 30.0);
    assert_eq!(dist_zero, Some(0.0));
    
    // Obliczanie prostej odleglosci w przestrzeni 3D
    let dist = manager.get_target_distance(10.0, 20.0, 40.0);
    assert_eq!(dist, Some(10.0));
    
    let dist_complex = manager.get_target_distance(13.0, 24.0, 30.0);
    // Pierwiastek kwadratowy z (3^2 + 4^2 + 0^2) = sqrt(9 + 16) = sqrt(25) = 5
    assert_eq!(dist_complex, Some(5.0));
}

use m2_client_bridge::state::target::{TargetState, TARGET_STATE, m2_target_get_vid, m2_target_state_alloc, m2_target_state_free};

#[test]
fn test_target_state_set_and_clear() {
    let mut state = TargetState::new();
    
    assert!(!state.has_target());
    assert_eq!(state.target_vid, None);
    
    state.set_target(12345);
    assert!(state.has_target());
    assert_eq!(state.target_vid, Some(12345));
    
    state.clear_target();
    assert!(!state.has_target());
    assert_eq!(state.target_vid, None);
}

#[test]
fn test_target_state_invalid_vid_handling() {
    let mut state = TargetState::new();
    
    state.set_target(0);
    assert!(!state.has_target());
    assert_eq!(state.target_vid, None);
}

#[test]
fn test_target_state_update_hp() {
    let mut state = TargetState::new();
    
    state.update_hp(50);
    assert_eq!(state.hp_percent, 50);
    
    state.update_hp(100);
    assert_eq!(state.hp_percent, 100);
    
    state.update_hp(150); // Powinno byc przyciete do 100
    assert_eq!(state.hp_percent, 100);
}

#[test]
fn test_target_state_ffi() {
    // Reset stanu globalnego przed testem
    if let Ok(mut state) = TARGET_STATE.write() {
        state.clear_target();
    }
    
    unsafe {
        assert_eq!(m2_target_get_vid(), 0);
    }
    
    if let Ok(mut state) = TARGET_STATE.write() {
        state.set_target(98765);
    }
    
    unsafe {
        assert_eq!(m2_target_get_vid(), 98765);
    }
    
    // Reset stanu po tescie
    if let Ok(mut state) = TARGET_STATE.write() {
        state.clear_target();
    }
}

#[test]
fn test_target_state_alloc_free() {
    unsafe {
        let ptr = m2_target_state_alloc();
        assert!(!ptr.is_null());
        
        // Zmiana i testowanie w obiekcie
        (*ptr).set_target(111);
        assert_eq!((*ptr).target_vid, Some(111));
        
        m2_target_state_free(ptr);
        
        // Freeing null pointer should not panic
        m2_target_state_free(std::ptr::null_mut());
    }
}
