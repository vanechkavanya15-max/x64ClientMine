use m2_client::loading_phase_manager::LoadingPhaseManager;
use std::fs;

#[test]
fn test_loading_phase_manager_initialization() {
    let manager = LoadingPhaseManager::new();
    assert_eq!(manager.current_step, 0);
    assert_eq!(manager.target_progress, 0.0);
    
    let state = manager.state.lock().unwrap();
    assert_eq!(state.progress_percent, 0.0);
    assert_eq!(state.is_complete, false);
    assert_eq!(state.current_task_name, "Inicjalizacja...");
}

#[test]
fn test_loading_phase_start() {
    let mut manager = LoadingPhaseManager::new();
    manager.set_fast_mode(true); // Fast mode zeby ominac błędy braku plików
    manager.start().unwrap();
    assert_eq!(manager.current_step, 1);
    assert_eq!(manager.target_progress, 20.0);
    
    let state = manager.state.lock().unwrap();
    assert_eq!(state.current_task_name, "Ladowanie konfiguracji mapy z atlasinfo.txt...");
}

#[test]
fn test_loading_phase_step_1_dummy_file() {
    let path = "atlasinfo.txt";
    fs::write(path, "metin2_map_a1 1000 2000 2 2\nmetin2_map_b1 3000 4000 3 3").unwrap();

    let mut manager = LoadingPhaseManager::new();
    manager.start().unwrap();

    fs::remove_file(path).unwrap_or(()); 
}

#[test]
fn test_loading_phase_transitions_fast_mode() {
    let mut manager = LoadingPhaseManager::new();
    manager.set_fast_mode(true); 
    manager.start().unwrap();

    for _ in 0..10 {
        manager.update().unwrap();
    }
    
    assert_eq!(manager.current_step, 5);
    
    let state = manager.state.lock().unwrap();
    assert_eq!(state.is_complete, true);
    assert!(state.progress_percent >= 100.0);
    assert_eq!(state.current_task_name, "Zakonczono. Wchodzenie do gry...");
}

#[test]
fn test_loading_phase_progress_bounds() {
    let mut manager = LoadingPhaseManager::new();
    manager.set_fast_mode(true);
    manager.start().unwrap();
    
    manager.update().unwrap();
    
    let state = manager.state.lock().unwrap();
    assert!(state.progress_percent >= 0.0);
    assert!(state.progress_percent <= 20.0);
}

#[test]
fn test_loading_phase_step_2_protos() {
    let mut manager = LoadingPhaseManager::new();
    manager.set_fast_mode(true);
    manager.start().unwrap();
    
    manager.update().unwrap(); 
    manager.update().unwrap(); 
}

#[test]
fn test_loading_phase_progress_render() {
    let mut manager = LoadingPhaseManager::new();
    manager.set_fast_mode(true);
    manager.start().unwrap();
    manager.update().unwrap();
    
    manager.render_progress_bar().unwrap();
}
