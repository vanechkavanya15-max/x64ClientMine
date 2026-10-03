#[path = "../src/performance_telemetry_bridge.rs"]
mod performance_telemetry_bridge;

use performance_telemetry_bridge::*;
use std::ptr;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_TEST_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
fn test_initial_state() {
    let telemetry = PerformanceTelemetry::new();
    let data = telemetry.get_data();
    assert_eq!(data.ram_usage_bytes, 0);
    assert_eq!(data.allocation_count, 0);
    assert_eq!(data.last_frame_time_us, 0);
    assert_eq!(data.avg_frame_time_us, 0);
    assert_eq!(data.min_frame_time_us, 0);
    assert_eq!(data.max_frame_time_us, 0);
}

#[test]
fn test_ram_usage() {
    let _lock = GLOBAL_TEST_MUTEX.lock().unwrap();
    
    let telemetry = PerformanceTelemetry::new();
    telemetry.update_ram_usage(1024);
    assert_eq!(telemetry.get_data().ram_usage_bytes, 1024);

    telemetry.update_ram_usage(2048);
    assert_eq!(telemetry.get_data().ram_usage_bytes, 2048);
    
    m2_telemetry_reset();
    m2_telemetry_update_ram_usage(4096);
    
    let mut data = TelemetryData::default();
    m2_telemetry_get_data(&mut data);
    assert_eq!(data.ram_usage_bytes, 4096);
}

#[test]
fn test_allocations() {
    let _lock = GLOBAL_TEST_MUTEX.lock().unwrap();
    
    let telemetry = PerformanceTelemetry::new();
    telemetry.add_allocations(10);
    assert_eq!(telemetry.get_data().allocation_count, 10);
    telemetry.add_allocations(5);
    assert_eq!(telemetry.get_data().allocation_count, 15);
    
    m2_telemetry_reset();
    m2_telemetry_add_allocations(100);
    
    let mut data = TelemetryData::default();
    m2_telemetry_get_data(&mut data);
    assert_eq!(data.allocation_count, 100);
}

#[test]
fn test_frame_history_wrap() {
    let telemetry = PerformanceTelemetry::new();
    for i in 1..=70 {
        telemetry.record_frame_time(i * 1000);
    }
    let data = telemetry.get_data();
    
    // We added 70 items, MAX is 60, so we have 11000 to 70000.
    // min should be 11000, max should be 70000.
    assert_eq!(data.min_frame_time_us, 11000);
    assert_eq!(data.max_frame_time_us, 70000);
    assert_eq!(data.last_frame_time_us, 70000);
    
    // Sum from 11 to 70 is (70*71/2) - (10*11/2) = 2485 - 55 = 2430
    // Avg should be 2430 * 1000 / 60 = 40500
    assert_eq!(data.avg_frame_time_us, 40500);
}

#[test]
fn test_ffi_null_pointer() {
    // Should not panic or crash
    m2_telemetry_get_data(ptr::null_mut());
}

#[test]
fn test_reset_functionality() {
    let _lock = GLOBAL_TEST_MUTEX.lock().unwrap();
    
    m2_telemetry_update_ram_usage(12345);
    m2_telemetry_add_allocations(678);
    m2_telemetry_record_frame_time(16000);
    
    let mut data = TelemetryData::default();
    m2_telemetry_get_data(&mut data);
    assert_eq!(data.ram_usage_bytes, 12345);
    assert_eq!(data.allocation_count, 678);
    assert_eq!(data.last_frame_time_us, 16000);
    
    m2_telemetry_reset();
    m2_telemetry_get_data(&mut data);
    assert_eq!(data.ram_usage_bytes, 0);
    assert_eq!(data.allocation_count, 0);
    assert_eq!(data.last_frame_time_us, 0);
    assert_eq!(data.min_frame_time_us, 0);
    assert_eq!(data.max_frame_time_us, 0);
    assert_eq!(data.avg_frame_time_us, 0);
}
