#[path = "../src/gpu_frame_diagnostics.rs"]
mod gpu_frame_diagnostics;

use std::time::{Duration, Instant};
use gpu_frame_diagnostics::GpuFrameDiagnostics;

#[test]
fn test_initial_state() {
    let diags = GpuFrameDiagnostics::new();
    let data = diags.get_diagnostics();
    assert_eq!(data.fps, 0);
    assert_eq!(data.cpu_frame_time_ms, 0.0);
    assert_eq!(data.gpu_frame_time_ms, 0.0);
    assert_eq!(data.triangle_count, 0);
    assert_eq!(data.draw_calls, 0);
}

#[test]
fn test_record_draw_call() {
    let mut diags = GpuFrameDiagnostics::new();
    let start_time = Instant::now();
    diags.begin_frame_with_time(start_time);
    
    diags.record_draw_call(100);
    diags.record_draw_call(50);
    diags.record_draw_call(25);
    
    diags.end_frame_with_time(start_time + Duration::from_millis(16), 12.5);
    
    let data = diags.get_diagnostics();
    assert_eq!(data.draw_calls, 3);
    assert_eq!(data.triangle_count, 175);
}

#[test]
fn test_begin_frame_resets_accumulators() {
    let mut diags = GpuFrameDiagnostics::new();
    let start_time = Instant::now();
    
    // First frame
    diags.begin_frame_with_time(start_time);
    diags.record_draw_call(100);
    diags.end_frame_with_time(start_time + Duration::from_millis(16), 10.0);
    
    let data = diags.get_diagnostics();
    assert_eq!(data.draw_calls, 1);
    assert_eq!(data.triangle_count, 100);
    
    // Second frame
    diags.begin_frame_with_time(start_time + Duration::from_millis(16));
    diags.record_draw_call(200);
    diags.record_draw_call(50);
    diags.end_frame_with_time(start_time + Duration::from_millis(32), 15.0);
    
    let data2 = diags.get_diagnostics();
    assert_eq!(data2.draw_calls, 2);
    assert_eq!(data2.triangle_count, 250);
}

#[test]
fn test_end_frame_calculates_times() {
    let mut diags = GpuFrameDiagnostics::new();
    let start_time = Instant::now();
    
    diags.begin_frame_with_time(start_time);
    
    // Simulate 10ms CPU time
    let end_time = start_time + Duration::from_millis(10);
    let gpu_time_ms = 8.5;
    
    diags.end_frame_with_time(end_time, gpu_time_ms);
    
    let data = diags.get_diagnostics();
    assert_eq!(data.cpu_frame_time_ms, 10.0);
    assert_eq!(data.gpu_frame_time_ms, 8.5);
}

#[test]
fn test_fps_calculation() {
    let start_time = Instant::now();
    let mut diags = GpuFrameDiagnostics::new_with_time(start_time);
    
    // Simulate 60 frames in exactly 1 second
    let frame_duration = Duration::from_nanos(1_000_000_000 / 60);
    
    for i in 0..60 {
        let frame_start = start_time + frame_duration * i;
        diags.begin_frame_with_time(frame_start);
        
        let frame_end = frame_start + frame_duration;
        // On the 60th frame, time_since_last_fps_update will be 1 sec
        // NOTE: if division makes it slightly short of 1.0, we just do duration_since, so let's guarantee >= 1s:
        let end_to_use = if i == 59 {
            start_time + Duration::from_secs(1)
        } else {
            frame_end
        };
        diags.end_frame_with_time(end_to_use, 5.0);
    }
    
    let data = diags.get_diagnostics();
    // After exactly 1 second, it should register 60 FPS
    assert_eq!(data.fps, 60);
    
    // Simulate 30 frames in the next second
    let second_second_start = start_time + Duration::from_secs(1);
    let slow_frame_duration = Duration::from_nanos(1_000_000_000 / 30);
    
    for i in 0..30 {
        let frame_start = second_second_start + slow_frame_duration * i;
        diags.begin_frame_with_time(frame_start);
        
        let frame_end = frame_start + slow_frame_duration;
        let end_to_use = if i == 29 {
            second_second_start + Duration::from_secs(1)
        } else {
            frame_end
        };
        diags.end_frame_with_time(end_to_use, 5.0);
    }
    
    let data2 = diags.get_diagnostics();
    // Should now be updated to 30 FPS for the second second
    assert_eq!(data2.fps, 30);
}
