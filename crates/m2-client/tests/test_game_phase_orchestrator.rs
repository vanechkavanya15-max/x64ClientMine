


use m2_client::client_master_game_phase_orchestrator::{
    ClientPhase, GamePhaseOrchestrator, PhaseWindow,
};

#[test]
fn test_game_phase_orchestrator_initial_state() {
    let orchestrator = GamePhaseOrchestrator::new();
    assert_eq!(orchestrator.current_phase, ClientPhase::Offline);
    assert!(!orchestrator.is_online);
    assert_eq!(orchestrator.wgpu_manager.initialized, false);
    assert_eq!(orchestrator.ui_manager.active_windows, [None; PhaseWindow::Num as usize]);
    assert_eq!(orchestrator.main_actor_vid, None);
}

#[test]
fn test_game_phase_orchestrator_full_progression_normal() {
    let mut orchestrator = GamePhaseOrchestrator::new();
    orchestrator.ui_manager.direct_enter_mode = false;
    
    // Offline -> Handshake
    assert!(orchestrator.execute_phase_progression_step());
    assert_eq!(orchestrator.current_phase, ClientPhase::Handshake);
    assert!(orchestrator.is_online);

    // Handshake -> Login
    assert!(orchestrator.execute_phase_progression_step());
    assert_eq!(orchestrator.current_phase, ClientPhase::Login);

    // Login -> Select
    assert!(orchestrator.execute_phase_progression_step());
    assert_eq!(orchestrator.current_phase, ClientPhase::Select);

    // Select -> Loading
    assert!(orchestrator.execute_phase_progression_step());
    assert_eq!(orchestrator.current_phase, ClientPhase::Loading);
    assert_eq!(orchestrator.wgpu_manager.initialized, false); // WGPU initializes in Game phase

    // Loading -> Game
    assert!(orchestrator.execute_phase_progression_step());
    assert_eq!(orchestrator.current_phase, ClientPhase::Game);
    assert_eq!(orchestrator.wgpu_manager.initialized, true);
    assert_eq!(orchestrator.wgpu_manager.active_pipelines_mask, 0xFFFFFFFF);

    // Game -> end (false)
    assert!(!orchestrator.execute_phase_progression_step());
    assert_eq!(orchestrator.current_phase, ClientPhase::Game);
}

#[test]
fn test_game_phase_orchestrator_full_progression_direct_enter() {
    let mut orchestrator = GamePhaseOrchestrator::new();
    orchestrator.ui_manager.direct_enter_mode = true;
    
    // Offline -> Handshake
    assert!(orchestrator.execute_phase_progression_step());
    assert_eq!(orchestrator.current_phase, ClientPhase::Handshake);

    // Handshake -> Login
    assert!(orchestrator.execute_phase_progression_step());
    assert_eq!(orchestrator.current_phase, ClientPhase::Login);

    // Login -> Loading (skips Select due to direct_enter_mode)
    assert!(orchestrator.execute_phase_progression_step());
    assert_eq!(orchestrator.current_phase, ClientPhase::Loading);

    // Loading -> Game
    assert!(orchestrator.execute_phase_progression_step());
    assert_eq!(orchestrator.current_phase, ClientPhase::Game);
    assert_eq!(orchestrator.wgpu_manager.initialized, true);
}

#[test]
fn test_game_phase_orchestrator_network_error_normal_mode() {
    let mut orchestrator = GamePhaseOrchestrator::new();
    orchestrator.ui_manager.direct_enter_mode = false;
    
    // Zalogowani i jestesmy np w Game Phase
    orchestrator.set_game_phase();
    assert_eq!(orchestrator.current_phase, ClientPhase::Game);
    assert_eq!(orchestrator.wgpu_manager.initialized, true);

    // Wystapil blad sieci
    orchestrator.handle_network_error();

    // Normalnie: clear pipelines, przejscie do Offline, a potem od razu do Login by UI moglo wyswietlic komunikat "OnConnectFailure"
    assert_eq!(orchestrator.current_phase, ClientPhase::Login);
    assert_eq!(orchestrator.wgpu_manager.initialized, false);
    assert_eq!(orchestrator.wgpu_manager.active_pipelines_mask, 0);
}

#[test]
fn test_game_phase_orchestrator_network_error_direct_enter_mode() {
    let mut orchestrator = GamePhaseOrchestrator::new();
    orchestrator.ui_manager.direct_enter_mode = true;
    
    orchestrator.ui_manager.set_phase_window(PhaseWindow::Login, 12345);
    orchestrator.set_loading_phase();

    // Wystapil blad sieci w trakcie ladowania
    orchestrator.handle_network_error();

    // W direct enter close_phase: Offline -> czyszczenie okien -> powrót do Login
    assert_eq!(orchestrator.current_phase, ClientPhase::Login);
    // Setting login phase updates it to `Some(1)` even if cleared prior
    let mut expected_windows = [None; PhaseWindow::Num as usize];
    // In `handle_network_error` -> `close_phase` -> `set_login_phase`, and if `direct_enter_mode` is true, 
    // `set_login_phase` does NOT set the active window for login. So it remains all None.
    assert_eq!(orchestrator.ui_manager.active_windows, expected_windows);
}

#[test]
fn test_game_phase_orchestrator_select_empire_vs_character() {
    let mut orchestrator = GamePhaseOrchestrator::new();
    orchestrator.ui_manager.direct_enter_mode = false;
    
    // Test case 1: No selected empire -> goes to PhaseWindow::Empire
    orchestrator.ui_manager.selected_empire = 0;
    orchestrator.set_select_phase();
    assert_eq!(orchestrator.current_phase, ClientPhase::Select);
    assert_eq!(orchestrator.ui_manager.active_windows[PhaseWindow::Empire as usize], Some(1));
    assert_eq!(orchestrator.ui_manager.active_windows[PhaseWindow::Select as usize], None);
    
    // Reset orchestrator and test case 2: Selected empire -> goes to PhaseWindow::Select
    orchestrator.ui_manager.clear_all_windows();
    orchestrator.ui_manager.selected_empire = 2; // e.g. Chunjo
    orchestrator.set_select_phase();
    assert_eq!(orchestrator.current_phase, ClientPhase::Select);
    assert_eq!(orchestrator.ui_manager.active_windows[PhaseWindow::Select as usize], Some(1));
    assert_eq!(orchestrator.ui_manager.active_windows[PhaseWindow::Empire as usize], None);
}

#[test]
fn test_game_phase_orchestrator_ui_window_management() {
    let mut ui_manager = m2_client::client_master_game_phase_orchestrator::UiWindowManager::new();
    
    ui_manager.set_phase_window(PhaseWindow::Login, 100);
    ui_manager.set_phase_window(PhaseWindow::Game, 200);

    assert_eq!(ui_manager.active_windows[PhaseWindow::Login as usize], Some(100));
    assert_eq!(ui_manager.active_windows[PhaseWindow::Game as usize], Some(200));
    assert!(ui_manager.is_window_active(PhaseWindow::Login));

    ui_manager.clear_phase_window(PhaseWindow::Login);
    assert_eq!(ui_manager.active_windows[PhaseWindow::Login as usize], None);
    assert_eq!(ui_manager.active_windows[PhaseWindow::Game as usize], Some(200));
    assert!(!ui_manager.is_window_active(PhaseWindow::Login));

    ui_manager.clear_all_windows();
    assert_eq!(ui_manager.active_windows, [None; PhaseWindow::Num as usize]);
}
