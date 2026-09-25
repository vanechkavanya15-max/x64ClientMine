use m2_client_pyapi::mod_app::ModApp;

#[test]
fn test_integration_mod_app() {
    let mut instance = ModApp::new();
    assert!(instance.is_ready());

    // Test cursor position
    assert_eq!(instance.get_cursor_position(), (0, 0));
    instance.set_cursor_position(100, 200);
    assert_eq!(instance.get_cursor_position(), (100, 200));

    // Test exit
    assert!(!instance.is_exit_requested());
    instance.exit();
    assert!(instance.is_exit_requested());

    // Test time (should be monotonic and >= 0)
    let global_time_1 = instance.get_global_time();
    let time_1 = instance.get_time();
    
    // Simulate some delay
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    let global_time_2 = instance.get_global_time();
    let time_2 = instance.get_time();
    
    assert!(global_time_2 >= global_time_1);
    assert!(time_2 >= time_1);
}
