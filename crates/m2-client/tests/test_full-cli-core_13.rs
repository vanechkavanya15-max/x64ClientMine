use m2_client::engine_headless_simulator::HeadlessSimulator;
use m2_client::client_engine_main::{EnginePhase, EngineMessage};

#[test]
fn test_headless_initialization() {
    let mut simulator = HeadlessSimulator::new();
    assert!(!simulator.window().is_windowed);
    assert!(!simulator.window().vsync);
    assert!(!simulator.is_running());
    assert_eq!(simulator.current_phase(), EnginePhase::Init);
    
    assert!(simulator.initialize().is_ok());
    assert!(simulator.is_running());
    assert_eq!(simulator.current_phase(), EnginePhase::Init);
}

#[test]
fn test_headless_shutdown() {
    let mut simulator = HeadlessSimulator::new();
    assert!(simulator.initialize().is_ok());
    assert!(simulator.is_running());
    
    simulator.shutdown();
    assert!(!simulator.is_running());
    assert_eq!(simulator.current_phase(), EnginePhase::Exit);
}

#[test]
fn test_headless_phase_change() {
    let mut simulator = HeadlessSimulator::new();
    assert!(simulator.initialize().is_ok());
    
    simulator.switch_phase(EnginePhase::Game);
    assert_eq!(simulator.current_phase(), EnginePhase::Game);
}

#[test]
fn test_headless_frame_execution() {
    let mut simulator = HeadlessSimulator::new();
    assert!(simulator.initialize().is_ok());
    
    assert_eq!(simulator.frame_count(), 0);
    simulator.run_headless_frame();
    assert_eq!(simulator.frame_count(), 1);
    simulator.run_headless_frame();
    assert_eq!(simulator.frame_count(), 2);
}

#[test]
fn test_headless_message_queue() {
    let mut simulator = HeadlessSimulator::new();
    assert!(simulator.initialize().is_ok());
    
    simulator.push_message(EngineMessage::Quit);
    simulator.run_headless_frame();
    
    assert!(!simulator.is_running());
    assert_eq!(simulator.current_phase(), EnginePhase::Exit);
}
