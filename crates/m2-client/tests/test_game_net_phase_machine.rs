use m2_client::game_net_phase_machine::{GameNetPhaseMachine, Phase, PhaseWindow};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_phase_transitions() {
    let mut machine = GameNetPhaseMachine::new();
    assert_eq!(machine.current_phase, Phase::OffLine);
    
    // Offline -> Login
    machine.process_phase_packet(Phase::Login);
    assert_eq!(machine.current_phase, Phase::Login);
    let first_time = machine.changing_phase_time;
    
    sleep(Duration::from_millis(10));
    
    // Login -> Select
    machine.process_phase_packet(Phase::Select);
    assert_eq!(machine.current_phase, Phase::Select);
    assert!(machine.changing_phase_time > first_time);

    // Select -> Loading
    machine.process_phase_packet(Phase::Loading);
    assert_eq!(machine.current_phase, Phase::Loading);

    // Loading -> Game
    machine.process_phase_packet(Phase::Game);
    assert_eq!(machine.current_phase, Phase::Game);
    
    // Game -> Offline via close
    machine.close_phase();
    assert_eq!(machine.current_phase, Phase::OffLine);
}

#[test]
fn test_packet_filtering_in_login_phase() {
    let mut machine = GameNetPhaseMachine::new();
    machine.process_phase_packet(Phase::Login);
    
    // Dozwolone kontrolne (PING, PHASE, KEY_CHALLENGE, KEY_COMPLETE)
    assert!(machine.is_packet_allowed(0x0007)); // PING
    assert!(machine.is_packet_allowed(0x0008)); // PHASE
    
    // Dozwolone w logowaniu (GC::LOGIN_SUCCESS3, LOGIN_SUCCESS4, FAILURE, KEY, EMPIRE)
    assert!(machine.is_packet_allowed(0x0104)); // LOGIN_SUCCESS3
    assert!(machine.is_packet_allowed(0x0106)); // LOGIN_FAILURE
    assert!(machine.is_packet_allowed(0x0109)); // EMPIRE
    
    // ZABRONIONE w logowaniu (np. gra, ruch, walka)
    assert!(!machine.is_packet_allowed(0x0302)); // MOVE
    assert!(!machine.is_packet_allowed(0x0410)); // DAMAGE_INFO
    assert!(!machine.is_packet_allowed(0x0512)); // ITEM_USE
}

#[test]
fn test_packet_filtering_in_game_and_loading() {
    let mut machine = GameNetPhaseMachine::new();
    
    // W fazie Loading i Game akceptujemy pakiety z gry
    machine.process_phase_packet(Phase::Loading);
    assert!(machine.is_packet_allowed(0x0302)); // MOVE
    assert!(machine.is_packet_allowed(0x0217)); // DEAD
    
    machine.process_phase_packet(Phase::Game);
    assert!(machine.is_packet_allowed(0x0410)); // DAMAGE_INFO
    assert!(machine.is_packet_allowed(0x0810)); // SHOP
}

#[test]
fn test_phase_window_handlers() {
    let mut machine = GameNetPhaseMachine::new();
    let handler1: usize = 0x1111;
    let handler2: usize = 0x2222;
    
    machine.set_phase_window(PhaseWindow::Login, handler1);
    machine.set_phase_window(PhaseWindow::Game, handler2);
    
    assert_eq!(machine.phase_windows[PhaseWindow::Login as usize], handler1);
    assert_eq!(machine.phase_windows[PhaseWindow::Game as usize], handler2);
    
    // Proba wyczyszczenia zlym handlerem
    machine.clear_phase_window(PhaseWindow::Login, 0x9999);
    assert_eq!(machine.phase_windows[PhaseWindow::Login as usize], handler1); // Niezmienione
    
    // Wyczyszczenie poprawnym handlerem
    machine.clear_phase_window(PhaseWindow::Login, handler1);
    assert_eq!(machine.phase_windows[PhaseWindow::Login as usize], 0); // Zresetowane
    
    // close_phase resetuje wszystko
    machine.close_phase();
    assert_eq!(machine.phase_windows[PhaseWindow::Game as usize], 0);
}
