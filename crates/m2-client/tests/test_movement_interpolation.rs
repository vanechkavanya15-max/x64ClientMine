#[path = "../src/movement_interpolation.rs"]
mod movement_interpolation;

use movement_interpolation::MovementInterpolator;

#[test]
fn test_midpoint_interpolation() {
    let mut interpolator = MovementInterpolator::new();
    // Simulate setting initial position at t=0
    interpolator.last_network_pos = (0.0, 0.0);
    interpolator.last_network_time_ms = 0;
    
    // Server says we should be at (10.0, 10.0) at t=1000
    interpolator.target_network_pos = (10.0, 10.0);
    interpolator.target_network_time_ms = 1000;
    
    // Verify halfway point
    let pos = interpolator.get_interpolated_position(500);
    assert_eq!(pos, (5.0, 5.0));
}

#[test]
fn test_target_reached_at_end_of_interval() {
    let mut interpolator = MovementInterpolator::new();
    interpolator.last_network_pos = (0.0, 0.0);
    interpolator.last_network_time_ms = 1000;
    
    interpolator.target_network_pos = (20.0, 30.0);
    interpolator.target_network_time_ms = 2000;
    
    let pos = interpolator.get_interpolated_position(2000);
    assert_eq!(pos, (20.0, 30.0));
}

#[test]
fn test_no_extrapolation_above_1() {
    let mut interpolator = MovementInterpolator::new();
    interpolator.last_network_pos = (0.0, 0.0);
    interpolator.last_network_time_ms = 1000;
    
    interpolator.target_network_pos = (20.0, 30.0);
    interpolator.target_network_time_ms = 2000;
    
    // Time passed target time
    let pos = interpolator.get_interpolated_position(3000);
    assert_eq!(pos, (20.0, 30.0));
}

#[test]
fn test_negative_ping_or_past_target_time() {
    let mut interpolator = MovementInterpolator::new();
    interpolator.last_network_pos = (0.0, 0.0);
    interpolator.last_network_time_ms = 1000;
    
    // Target time is in the past compared to last network time
    interpolator.target_network_pos = (20.0, 30.0);
    interpolator.target_network_time_ms = 500;
    
    let pos = interpolator.get_interpolated_position(1000);
    assert_eq!(pos, (20.0, 30.0));
}

#[test]
fn test_zero_interval() {
    let mut interpolator = MovementInterpolator::new();
    interpolator.last_network_pos = (0.0, 0.0);
    interpolator.last_network_time_ms = 1000;
    
    interpolator.target_network_pos = (20.0, 30.0);
    // Interval will be 0
    interpolator.target_network_time_ms = 1000;
    
    let pos = interpolator.get_interpolated_position(1000);
    assert_eq!(pos, (20.0, 30.0));
}

#[test]
fn test_receive_network_packet_eliminates_jumps() {
    let mut interpolator = MovementInterpolator::new();
    interpolator.last_network_pos = (0.0, 0.0);
    interpolator.last_network_time_ms = 0;
    
    interpolator.target_network_pos = (100.0, 0.0);
    interpolator.target_network_time_ms = 1000;
    
    // We are at t=500, halfway there -> pos = (50.0, 0.0)
    let pos = interpolator.get_interpolated_position(500);
    assert_eq!(pos, (50.0, 0.0));
    
    // New packet arrives, target changed drastically
    interpolator.receive_network_packet((100.0, 100.0), 1500, 500);
    
    // Verify that the new starting point is our CURRENT interpolated position, not the old last_network_pos or target
    assert_eq!(interpolator.last_network_pos, (50.0, 0.0));
    assert_eq!(interpolator.last_network_time_ms, 500);
    assert_eq!(interpolator.target_network_pos, (100.0, 100.0));
    assert_eq!(interpolator.target_network_time_ms, 1500);
    
    // Test moving from the new starting point
    let pos2 = interpolator.get_interpolated_position(1000);
    assert_eq!(pos2, (75.0, 50.0));
}

#[test]
fn test_receive_network_packet_initial_state() {
    let mut interpolator = MovementInterpolator::new();
    
    // Fresh interpolator receives its first packet
    interpolator.receive_network_packet((10.0, 20.0), 100, 0);
    
    assert_eq!(interpolator.last_network_pos, (0.0, 0.0));
    assert_eq!(interpolator.last_network_time_ms, 0);
    assert_eq!(interpolator.target_network_pos, (10.0, 20.0));
    assert_eq!(interpolator.target_network_time_ms, 100);
}
