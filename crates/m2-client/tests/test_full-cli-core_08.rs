#[path = "../src/full_cli_core_08.rs"]
mod full_cli_core_08;

use full_cli_core_08::crash_dump_stack_tracer::{setup_panic_hook, capture_registers};
use std::fs;
use std::panic;
use std::path::Path;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref LOCK: Mutex<()> = Mutex::new(());
}

#[test]
fn test_capture_registers() {
    let regs = capture_registers();
    #[cfg(target_arch = "x86_64")]
    {
        assert!(regs.rip != 0, "RIP should be non-zero on x86_64");
    }
}

#[test]
fn test_crash_report_file_creation() {
    let _guard = LOCK.lock().unwrap();
    let _ = fs::remove_file("crash_report.txt");
    setup_panic_hook();
    let result = panic::catch_unwind(|| {
        panic!("test panic for file creation");
    });
    assert!(result.is_err());
    
    assert!(Path::new("crash_report.txt").exists());
    let content = fs::read_to_string("crash_report.txt").unwrap_or_default();
    assert!(content.contains("test panic for file creation"));
    assert!(content.contains("=== CRASH REPORT ==="));
    assert!(content.contains("--- Stack Trace ---"));
    assert!(content.contains("--- CPU Registers (x86_64) ---"));
    
    let _ = fs::remove_file("crash_report.txt");
}

#[test]
fn test_crash_report_with_string_payload() {
    let _guard = LOCK.lock().unwrap();
    let _ = fs::remove_file("crash_report.txt");
    setup_panic_hook();
    let result = panic::catch_unwind(|| {
        panic!("{}", String::from("test panic with String"));
    });
    assert!(result.is_err());
    
    let content = fs::read_to_string("crash_report.txt").unwrap_or_default();
    assert!(content.contains("test panic with String"));
    let _ = fs::remove_file("crash_report.txt");
}

#[test]
fn test_crash_report_with_unknown_payload() {
    let _guard = LOCK.lock().unwrap();
    let _ = fs::remove_file("crash_report.txt");
    setup_panic_hook();
    let result = panic::catch_unwind(|| {
        panic::panic_any(42);
    });
    assert!(result.is_err());
    
    let content = fs::read_to_string("crash_report.txt").unwrap_or_default();
    assert!(content.contains("Unknown payload"));
    let _ = fs::remove_file("crash_report.txt");
}

#[test]
fn test_crash_report_registers_included() {
    let _guard = LOCK.lock().unwrap();
    let _ = fs::remove_file("crash_report.txt");
    setup_panic_hook();
    let result = panic::catch_unwind(|| {
        panic!("test registers");
    });
    assert!(result.is_err());
    
    let content = fs::read_to_string("crash_report.txt").unwrap_or_default();
    assert!(content.contains("RAX:"));
    assert!(content.contains("RBX:"));
    assert!(content.contains("RSP:"));
    assert!(content.contains("RIP:"));
    let _ = fs::remove_file("crash_report.txt");
}
