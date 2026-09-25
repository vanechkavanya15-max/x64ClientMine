use m2_client_bridge::state::phase::{ClientPhase, PhaseManager, m2_phase_get, m2_phase_manager_free, GLOBAL_PHASE_MANAGER};

#[test]
fn test_phase_manager_initial_phase() {
    let manager = PhaseManager::new();
    assert_eq!(manager.get_phase(), ClientPhase::Off);
    assert!(!manager.is_in_game());
}

#[test]
fn test_phase_manager_phase_transitions() {
    let mut manager = PhaseManager::new();
    
    assert_eq!(manager.set_phase(ClientPhase::HandShake), Ok(()));
    assert_eq!(manager.get_phase(), ClientPhase::HandShake);
    
    assert_eq!(manager.set_phase(ClientPhase::Login), Ok(()));
    assert_eq!(manager.get_phase(), ClientPhase::Login);
    
    assert_eq!(manager.set_phase(ClientPhase::Select), Ok(()));
    assert_eq!(manager.get_phase(), ClientPhase::Select);

    assert_eq!(manager.set_phase(ClientPhase::Loading), Ok(()));
    assert_eq!(manager.get_phase(), ClientPhase::Loading);
    
    assert_eq!(manager.set_phase(ClientPhase::Game), Ok(()));
    assert_eq!(manager.get_phase(), ClientPhase::Game);
    assert!(manager.is_in_game());
    
    assert_eq!(manager.set_phase(ClientPhase::Off), Ok(()));
    assert_eq!(manager.get_phase(), ClientPhase::Off);
    assert!(!manager.is_in_game());
}

#[test]
fn test_phase_manager_game_to_login_blocked() {
    let mut manager = PhaseManager::new();
    
    assert_eq!(manager.set_phase(ClientPhase::Game), Ok(()));
    assert_eq!(manager.get_phase(), ClientPhase::Game);
    
    let result = manager.set_phase(ClientPhase::Login);
    assert!(result.is_err());
    assert_eq!(manager.get_phase(), ClientPhase::Game);
    
    assert_eq!(manager.set_phase(ClientPhase::Off), Ok(()));
    assert_eq!(manager.set_phase(ClientPhase::Login), Ok(()));
}

#[test]
fn test_ffi_global_phase_get() {
    {
        let mut manager = GLOBAL_PHASE_MANAGER.lock().unwrap();
        let _ = manager.set_phase(ClientPhase::Off);
    }
    
    let phase = unsafe { m2_phase_get() };
    assert_eq!(phase, ClientPhase::Off as u8);
    
    {
        let mut manager = GLOBAL_PHASE_MANAGER.lock().unwrap();
        let _ = manager.set_phase(ClientPhase::Game);
    }
    
    let phase = unsafe { m2_phase_get() };
    assert_eq!(phase, ClientPhase::Game as u8);
}

#[test]
fn test_ffi_free() {
    let ptr = Box::into_raw(Box::new(PhaseManager::new()));
    
    unsafe {
        m2_phase_manager_free(ptr);
    }
    
    unsafe {
        m2_phase_manager_free(std::ptr::null_mut());
    }
}
