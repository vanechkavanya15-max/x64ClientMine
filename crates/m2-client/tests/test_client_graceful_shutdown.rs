#[path = "../src/client_graceful_shutdown.rs"]
mod client_graceful_shutdown;

use client_graceful_shutdown::ShutdownManager;

#[test]
fn test_default_state() {
    let mgr = ShutdownManager::new();
    assert!(!mgr.is_shutting_down());
    assert_eq!(mgr.exit_code(), 0);
    assert_eq!(mgr.cleanup_stages_completed(), 0);
}

#[test]
fn test_request_shutdown() {
    let mut mgr = ShutdownManager::new();
    mgr.request_shutdown(42);
    
    assert!(mgr.is_shutting_down());
    assert_eq!(mgr.exit_code(), 42);
}

#[test]
fn test_execute_pipeline_does_nothing_if_not_shutting_down() {
    let mut mgr = ShutdownManager::new();
    
    let mut stage1 = || -> bool {
        panic!("This stage should not be executed");
    };
    
    let result = mgr.execute_cleanup_pipeline(&mut [&mut stage1 as &mut dyn FnMut() -> bool]);
    
    assert!(!result);
    assert_eq!(mgr.cleanup_stages_completed(), 0);
}

#[test]
fn test_successful_cleanup_pipeline() {
    let mut mgr = ShutdownManager::new();
    mgr.request_shutdown(0);
    
    let mut disconnected_tcp = false;
    let mut stopped_sound_workers = false;
    let mut closed_python = false;
    let mut released_d3d9 = false;
    
    let mut s1 = || { disconnected_tcp = true; true };
    let mut s2 = || { stopped_sound_workers = true; true };
    let mut s3 = || { closed_python = true; true };
    let mut s4 = || { released_d3d9 = true; true };

    let mut stages: [&mut dyn FnMut() -> bool; 4] = [
        &mut s1,
        &mut s2,
        &mut s3,
        &mut s4,
    ];
    
    let result = mgr.execute_cleanup_pipeline(&mut stages);
    
    assert!(result);
    assert!(disconnected_tcp);
    assert!(stopped_sound_workers);
    assert!(closed_python);
    assert!(released_d3d9);
    assert_eq!(mgr.cleanup_stages_completed(), 4);
}

#[test]
fn test_early_out_on_failed_stage() {
    let mut mgr = ShutdownManager::new();
    mgr.request_shutdown(1);
    
    let mut disconnected_tcp = false;
    let mut stopped_sound_workers = false;
    let mut closed_python = false;
    
    let mut s1 = || { disconnected_tcp = true; true };
    let mut s2 = || { stopped_sound_workers = true; false }; // Simulate failure here
    let mut s3 = || { closed_python = true; true };

    let mut stages: [&mut dyn FnMut() -> bool; 3] = [
        &mut s1,
        &mut s2,
        &mut s3,
    ];
    
    let result = mgr.execute_cleanup_pipeline(&mut stages);
    
    assert!(!result);
    assert!(disconnected_tcp);
    assert!(stopped_sound_workers);
    assert!(!closed_python);
    assert_eq!(mgr.cleanup_stages_completed(), 1); 
}

#[test]
fn test_multiple_pipeline_executions() {
    let mut mgr = ShutdownManager::new();
    mgr.request_shutdown(0);
    
    let mut s1 = || true;
    let mut s2 = || true;
    let mut s3 = || false;

    let mut stages1: [&mut dyn FnMut() -> bool; 3] = [&mut s1, &mut s2, &mut s3];
    let result1 = mgr.execute_cleanup_pipeline(&mut stages1);
    assert!(!result1);
    assert_eq!(mgr.cleanup_stages_completed(), 2);
    
    let mut s4 = || true;
    let mut s5 = || true;
    
    let mut stages2: [&mut dyn FnMut() -> bool; 2] = [&mut s4, &mut s5];
    let result2 = mgr.execute_cleanup_pipeline(&mut stages2);
    assert!(result2);
    assert_eq!(mgr.cleanup_stages_completed(), 2);
}
