use m2_client::client_engine_main::{ClientEngine, EngineMessage, EnginePhase, FrameEvent, WindowConfig};

#[test]
fn test_initialization() {
    let config = WindowConfig::default();
    let mut engine = ClientEngine::new(config);
    
    assert!(!engine.is_running);
    assert_eq!(engine.current_phase, EnginePhase::Init);
    
    let res = engine.initialize();
    assert!(res.is_ok());
    assert!(engine.is_running);
    assert_eq!(engine.current_phase, EnginePhase::Init);
    
    let res2 = engine.initialize();
    assert!(res2.is_err());
}

#[test]
fn test_run_before_init() {
    let config = WindowConfig::default();
    let mut engine = ClientEngine::new(config);
    
    // Should not do anything
    engine.run_frame();
    assert_eq!(engine.frame_count, 0);
    assert!(engine.frame_events.is_empty());
}

#[test]
fn test_phase_transitions() {
    let mut engine = ClientEngine::new(WindowConfig::default());
    engine.initialize().unwrap();
    
    assert_eq!(engine.current_phase, EnginePhase::Init);
    
    engine.switch_phase(EnginePhase::Login);
    assert_eq!(engine.current_phase, EnginePhase::Login);
    
    engine.switch_phase(EnginePhase::Select);
    assert_eq!(engine.current_phase, EnginePhase::Select);
    
    engine.switch_phase(EnginePhase::Loading);
    assert_eq!(engine.current_phase, EnginePhase::Loading);
    
    engine.switch_phase(EnginePhase::Game);
    assert_eq!(engine.current_phase, EnginePhase::Game);
    
    engine.switch_phase(EnginePhase::Exit);
    assert_eq!(engine.current_phase, EnginePhase::Exit);
}

#[test]
fn test_shutdown() {
    let mut engine = ClientEngine::new(WindowConfig::default());
    engine.initialize().unwrap();
    
    assert!(engine.is_running);
    
    engine.shutdown();
    
    assert!(!engine.is_running);
    assert_eq!(engine.current_phase, EnginePhase::Exit);
}

#[test]
fn test_message_polling() {
    let mut engine = ClientEngine::new(WindowConfig::default());
    engine.initialize().unwrap();
    
    engine.push_message(EngineMessage::Resize(1024, 768));
    engine.push_message(EngineMessage::PhaseChange(EnginePhase::Game));
    
    engine.run_frame();
    
    assert_eq!(engine.window.width, 1024);
    assert_eq!(engine.window.height, 768);
    assert_eq!(engine.current_phase, EnginePhase::Game);
}

#[test]
fn test_run_frame_sequence() {
    let mut engine = ClientEngine::new(WindowConfig::default());
    engine.initialize().unwrap();
    
    engine.run_frame();
    
    let expected_events = vec![
        FrameEvent::PollMessages,
        FrameEvent::UpdateNetwork,
        FrameEvent::UpdatePhysicsAndActors,
        FrameEvent::Render3DWorldAndTerrain,
        FrameEvent::Render2DGUI,
        FrameEvent::PresentFrame,
    ];
    
    assert_eq!(engine.frame_events, expected_events);
    assert_eq!(engine.frame_count, 1);
    
    // Run another frame
    engine.run_frame();
    assert_eq!(engine.frame_events, expected_events); // Cleared at start of frame
    assert_eq!(engine.frame_count, 2);
}

#[test]
fn test_quit_message_aborts_frame() {
    let mut engine = ClientEngine::new(WindowConfig::default());
    engine.initialize().unwrap();
    
    engine.push_message(EngineMessage::Quit);
    
    engine.run_frame();
    
    // Should only have Polled Messages and then stopped
    let expected_events = vec![FrameEvent::PollMessages];
    
    assert_eq!(engine.frame_events, expected_events);
    assert_eq!(engine.frame_count, 0); // Didn't reach present_frame
    assert!(!engine.is_running);
    assert_eq!(engine.current_phase, EnginePhase::Exit);
}
