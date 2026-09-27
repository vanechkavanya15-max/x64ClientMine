use m2_client::main_binary_entry::{
    AppConfig, EngineError, EngineState, Metin2Engine, write_syserr, install_crash_handler,
};
use std::fs;
use std::path::Path;

#[test]
fn test_app_config_parse_empty() {
    let args: Vec<&str> = vec!["m2_client.exe"];
    let config = AppConfig::parse_args(args);
    
    assert_eq!(config.config_file, None);
    assert_eq!(config.windowed, false);
    assert_eq!(config.connect_ip, None);
    assert_eq!(config.connect_port, None);
}

#[test]
fn test_app_config_parse_all_args() {
    let args: Vec<&str> = vec![
        "m2_client.exe",
        "--config",
        "my_config.ini",
        "--windowed",
        "--connect",
        "127.0.0.1",
        "50000",
    ];
    let config = AppConfig::parse_args(args);
    
    assert_eq!(config.config_file, Some("my_config.ini".to_string()));
    assert_eq!(config.windowed, true);
    assert_eq!(config.connect_ip, Some("127.0.0.1".to_string()));
    assert_eq!(config.connect_port, Some(50000));
}

#[test]
fn test_engine_initial_state() {
    let engine = Metin2Engine::new();
    assert_eq!(engine.state, EngineState::Uninitialized);
    assert_eq!(engine.config, AppConfig::default());
    assert!(engine.hwnd.is_none());
    assert!(engine.d3d9_device.is_none());
}

#[test]
fn test_engine_load_config() {
    let mut engine = Metin2Engine::new();
    let config = AppConfig {
        config_file: Some("test.ini".to_string()),
        windowed: true,
        connect_ip: None,
        connect_port: None,
    };
    
    engine.load_config(config.clone());
    assert_eq!(engine.state, EngineState::ConfigLoaded);
    assert_eq!(engine.config, config);
}

#[test]
fn test_engine_d3d9_needs_window() {
    let mut engine = Metin2Engine::new();
    // Attempt to init d3d9 without creating a window first
    let result = engine.init_d3d9();
    assert_eq!(result, Err(EngineError::WindowNotCreated));
    assert_eq!(engine.state, EngineState::Uninitialized);
}

#[test]
fn test_engine_lifecycle_success() {
    let mut engine = Metin2Engine::new();
    
    engine.load_config(AppConfig::default());
    assert_eq!(engine.state, EngineState::ConfigLoaded);
    
    assert!(engine.create_window().is_ok());
    assert_eq!(engine.state, EngineState::WindowCreated);
    assert!(engine.hwnd.is_some());
    
    assert!(engine.init_d3d9().is_ok());
    assert_eq!(engine.state, EngineState::D3D9Initialized);
    assert!(engine.d3d9_device.is_some());
    
    assert!(engine.start_python().is_ok());
    assert_eq!(engine.state, EngineState::PythonStarted);
    
    engine.run_game_loop();
    assert_eq!(engine.state, EngineState::Stopped);
}

#[test]
fn test_write_syserr() {
    let test_msg = "test_syserr_message_12345";
    write_syserr(test_msg);
    
    let content = fs::read_to_string("syserr.txt").unwrap_or_default();
    assert!(content.contains(test_msg));
}

#[test]
fn test_engine_run_helper() {
    let mut engine = Metin2Engine::new();
    let result = engine.run(AppConfig::default());
    assert!(result.is_ok());
    assert_eq!(engine.state, EngineState::Stopped);
}
