#[path = "../src/actor_movement_interpolator_v3.rs"]
mod actor_movement_interpolator_v3;

use actor_movement_interpolator_v3::{
    MovementInterpolator, D3DXQUATERNION, TPixelPosition, InterpolationError
};

#[test]
fn test_slerp_identity() {
    let q1 = D3DXQUATERNION::new(0.0, 0.0, 0.0, 1.0);
    let q2 = D3DXQUATERNION::new(0.0, 0.0, 0.0, 1.0);
    
    let result = MovementInterpolator::slerp(q1, q2, 0.5).unwrap();
    
    assert!((result.x - 0.0).abs() < 1e-5);
    assert!((result.y - 0.0).abs() < 1e-5);
    assert!((result.z - 0.0).abs() < 1e-5);
    assert!((result.w - 1.0).abs() < 1e-5);
}

#[test]
fn test_slerp_invalid_time() {
    let q1 = D3DXQUATERNION::new(0.0, 0.0, 0.0, 1.0);
    let q2 = D3DXQUATERNION::new(0.0, 1.0, 0.0, 0.0);
    
    let result_neg = MovementInterpolator::slerp(q1, q2, -0.1);
    let result_over = MovementInterpolator::slerp(q1, q2, 1.1);
    
    assert!(matches!(result_neg, Err(InterpolationError::InvalidTime)));
    assert!(matches!(result_over, Err(InterpolationError::InvalidTime)));
}

#[test]
fn test_catmull_rom_interpolation() {
    let p0 = TPixelPosition::new(0.0, 0.0, 0.0);
    let p1 = TPixelPosition::new(10.0, 0.0, 0.0);
    let p2 = TPixelPosition::new(20.0, 0.0, 0.0);
    let p3 = TPixelPosition::new(30.0, 0.0, 0.0);
    
    // At t=0.5, for a straight line with uniform distances, it should be exactly halfway between p1 and p2 (15.0)
    let result = MovementInterpolator::interpolate_catmull_rom(p0, p1, p2, p3, 0.5).unwrap();
    
    assert!((result.x - 15.0).abs() < 1e-5);
    assert!((result.y - 0.0).abs() < 1e-5);
    assert!((result.z - 0.0).abs() < 1e-5);
}

#[test]
fn test_hermite_interpolation() {
    let p1 = TPixelPosition::new(10.0, 0.0, 0.0);
    let t1 = TPixelPosition::new(10.0, 0.0, 0.0);
    let p2 = TPixelPosition::new(20.0, 0.0, 0.0);
    let t2 = TPixelPosition::new(10.0, 0.0, 0.0);
    
    // At t=0.5 for uniform tangents, it should be the midpoint.
    let result = MovementInterpolator::interpolate_hermite(p1, t1, p2, t2, 0.5).unwrap();
    
    assert!((result.x - 15.0).abs() < 1e-5);
    assert!((result.y - 0.0).abs() < 1e-5);
    assert!((result.z - 0.0).abs() < 1e-5);
}

#[test]
fn test_velocity_extrapolation() {
    let p_last = TPixelPosition::new(0.0, 0.0, 0.0);
    let p_cur = TPixelPosition::new(100.0, 0.0, 0.0); // moved 100 units
    let time_diff_ms = 1000; // in 1 second -> v = 100 units/s
    let extrapolate_time_ms = 500; // extrapolate 0.5 seconds -> should add 50 units
    
    let result = MovementInterpolator::extrapolate_velocity(p_last, p_cur, time_diff_ms, extrapolate_time_ms).unwrap();
    
    assert!((result.x - 150.0).abs() < 1e-5);
    assert!((result.y - 0.0).abs() < 1e-5);
    assert!((result.z - 0.0).abs() < 1e-5);
}

#[test]
fn test_state_queuing_and_processing() {
    let mut interpolator = MovementInterpolator::new();
    
    // Also use the expanded state push to fix dead code warning
    interpolator.push_tcp_state_expanded(
        500,
        TPixelPosition::new(50.0, 50.0, 0.0),
        0.0,
        1,
        0,
        999
    );
    let p_dst = TPixelPosition::new(100.0, 200.0, 0.0);
    
    // push a tcp state (simulates incoming packet)
    interpolator.push_tcp_state(
        1000,   // dw_cmd_time
        p_dst,  // dst pos
        45.0,   // rot
        1,      // eFunc
        0,      // uArg
        1050,   // server_frame_msec (gap = 50)
    );
    
    // Average network gap should be updated: (0 * 70 + 50 * 30) / 100 = 15
    assert_eq!(interpolator.m_nAverageNetworkGap, 15);
    
    // Our process_state checks if cur_chk_time < dw_dst_chk_time.
    // We added the expanded state push which has chk time = 500 + 100 = 600.
    // The regular state push has chk time = 1000 + 15 = 1015.
    
    // pop1 should retrieve the first item (chk time 600) if cur_chk_time = 1000.
    let pop1 = interpolator.process_state(1000);
    assert!(pop1.is_some());
    let cmd1 = pop1.unwrap();
    assert_eq!(cmd1.m_dwChkTime, 600);
    
    // Process next item (chk time 1015) with cur_chk_time = 1000, it shouldn't pop
    let pop2 = interpolator.process_state(1000);
    assert!(pop2.is_none());
    
    // If we process state with cur_chk_time = 1020, it should pop (1020 >= 1015)
    let pop3 = interpolator.process_state(1020);
    assert!(pop3.is_some());
    let cmd2 = pop3.unwrap();
    
    assert_eq!(cmd2.m_kPPosDst.x, 100.0);
    assert_eq!(cmd2.m_dwCmdTime, 1000);
    assert_eq!(cmd2.m_dwChkTime, 1015);
    
    // Queue should now be empty
    let pop4 = interpolator.process_state(1020);
    assert!(pop4.is_none());
}
