use m2_client_bridge::state::phase::{Phase, StateManager, StateError};

#[test]
fn test_state_manager_initial_phase() {
    let manager = StateManager::new();
    assert_eq!(manager.get_phase(), Phase::Offline);
}

#[test]
fn test_state_manager_phase_transitions() {
    let mut manager = StateManager::new();
    
    manager.set_phase(Phase::Handshake);
    assert_eq!(manager.get_phase(), Phase::Handshake);
    
    manager.set_phase(Phase::Login);
    assert_eq!(manager.get_phase(), Phase::Login);
    
    manager.set_phase(Phase::Select);
    assert_eq!(manager.get_phase(), Phase::Select);
    
    manager.set_phase(Phase::Game);
    assert_eq!(manager.get_phase(), Phase::Game);
    
    manager.set_phase(Phase::Offline);
    assert_eq!(manager.get_phase(), Phase::Offline);
}

#[test]
fn test_state_manager_add_and_remove_vid() {
    let mut manager = StateManager::new();
    
    assert_eq!(manager.add_vid(1), Ok(()));
    assert!(manager.has_vid(1));
    
    assert_eq!(manager.add_vid(2), Ok(()));
    assert!(manager.has_vid(2));
    
    assert_eq!(manager.remove_vid(1), Ok(()));
    assert!(!manager.has_vid(1));
    assert!(manager.has_vid(2));
}

#[test]
fn test_state_manager_invalid_vid() {
    let mut manager = StateManager::new();
    
    // Dodawanie VID 0 powinno zwrocic blad.
    assert_eq!(manager.add_vid(0), Err(StateError::InvalidVid(0)));
    assert!(!manager.has_vid(0));
    
    // Usuwanie VID 0 powinno zwrocic blad.
    assert_eq!(manager.remove_vid(0), Err(StateError::InvalidVid(0)));
    
    // Usuwanie nieistniejacego VID.
    assert_eq!(manager.remove_vid(999), Err(StateError::InvalidVid(999)));
}

#[test]
fn test_state_manager_out_of_bounds_vid() {
    let mut manager = StateManager::new();
    
    assert_eq!(manager.add_vid(100), Ok(()));
    assert_eq!(manager.add_vid(200), Ok(()));
    
    assert_eq!(manager.get_vid_at(0), Ok(100));
    assert_eq!(manager.get_vid_at(1), Ok(200));
    
    // Pobranie elementu spoza zakresu.
    assert_eq!(manager.get_vid_at(2), Err(StateError::IndexOutOfBounds(2)));
}

#[test]
fn test_state_manager_clear_vids() {
    let mut manager = StateManager::new();
    
    let _ = manager.add_vid(1);
    let _ = manager.add_vid(2);
    let _ = manager.add_vid(3);
    
    assert!(manager.has_vid(1));
    assert!(manager.has_vid(2));
    assert!(manager.has_vid(3));
    
    manager.clear_vids();
    
    assert!(!manager.has_vid(1));
    assert!(!manager.has_vid(2));
    assert!(!manager.has_vid(3));
    
    assert_eq!(manager.get_vid_at(0), Err(StateError::IndexOutOfBounds(0)));
}
