use m2_client::syslog_structured_logger::SyslogStructuredLogger;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::thread;

fn setup_test_file(name: &str) -> String {
    let path = format!("test_{}.txt", name);
    let _ = fs::remove_file(&path); // Ensure file doesn't exist before test
    path
}

fn teardown_test_file(path: &str) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_logger_creation() {
    let path = setup_test_file("creation");
    
    let result = SyslogStructuredLogger::new(&path);
    assert!(result.is_ok());
    
    assert!(Path::new(&path).exists());
    
    teardown_test_file(&path);
}

#[test]
fn test_log_engine_stage() {
    let path = setup_test_file("engine_stage");
    
    let logger = SyslogStructuredLogger::new(&path).unwrap();
    assert!(logger.log_engine_stage("Initialization").is_ok());
    
    let contents = fs::read_to_string(&path).unwrap();
    assert!(contents.contains("[ENGINE] Stage: Initialization\n"));
    
    teardown_test_file(&path);
}

#[test]
fn test_log_gpu_info() {
    let path = setup_test_file("gpu_info");
    
    let logger = SyslogStructuredLogger::new(&path).unwrap();
    assert!(logger.log_gpu_info("NVIDIA RTX 4090").is_ok());
    
    let contents = fs::read_to_string(&path).unwrap();
    assert!(contents.contains("[GPU] Detected GPU: NVIDIA RTX 4090\n"));
    
    teardown_test_file(&path);
}

#[test]
fn test_log_loaded_module() {
    let path = setup_test_file("loaded_module");
    
    let logger = SyslogStructuredLogger::new(&path).unwrap();
    assert!(logger.log_loaded_module("NetworkSystem").is_ok());
    
    let contents = fs::read_to_string(&path).unwrap();
    assert!(contents.contains("[MODULE] Loaded module: NetworkSystem\n"));
    
    teardown_test_file(&path);
}

#[test]
fn test_concurrent_logging() {
    let path = setup_test_file("concurrent");
    
    let logger = Arc::new(SyslogStructuredLogger::new(&path).unwrap());
    
    let mut handles = vec![];
    
    for i in 0..10 {
        let logger_clone = Arc::clone(&logger);
        let handle = thread::spawn(move || {
            let module_name = format!("Module_{}", i);
            logger_clone.log_loaded_module(&module_name).unwrap();
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let contents = fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    
    assert_eq!(lines.len(), 10);
    for i in 0..10 {
        assert!(contents.contains(&format!("[MODULE] Loaded module: Module_{}", i)));
    }
    
    teardown_test_file(&path);
}
