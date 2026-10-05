use m2_client::py_bridge_minimap_signals_markers::{
    get_minimap_signals_state, m2_py_minimap_add_signal_point, m2_py_minimap_add_way_point,
    m2_py_minimap_clear_all_signal_point, m2_py_minimap_remove_way_point,
    reset_minimap_signals_state,
};
use std::ffi::CString;

#[test]
fn test_minimap_add_and_remove_way_point() {
    reset_minimap_signals_state();

    let name = CString::new("TestNPC").unwrap();
    let result = m2_py_minimap_add_way_point(10, 100.0, 200.0, name.as_ptr());
    assert!(result);

    let state = get_minimap_signals_state().read().unwrap();
    assert_eq!(state.waypoints.len(), 1);
    assert_eq!(state.waypoints[0].dw_id, 10);
    assert_eq!(state.waypoints[0].x, 100.0);
    assert_eq!(state.waypoints[0].y, 200.0);
    assert_eq!(state.waypoints[0].text, "TestNPC");
    drop(state); // Drop the lock before adding again

    // Test adding a duplicate ID
    let result2 = m2_py_minimap_add_way_point(10, 150.0, 250.0, name.as_ptr());
    assert!(!result2);

    let state = get_minimap_signals_state().read().unwrap();
    assert_eq!(state.waypoints.len(), 1);
    drop(state); // Drop the lock before removing

    // Test removing the waypoint
    let result3 = m2_py_minimap_remove_way_point(10);
    assert!(result3);

    let state = get_minimap_signals_state().read().unwrap();
    assert_eq!(state.waypoints.len(), 0);
    drop(state); // Drop the lock before removing again

    // Test removing non-existent waypoint
    let result4 = m2_py_minimap_remove_way_point(99);
    assert!(!result4);
}

#[test]
fn test_minimap_add_signal_point() {
    reset_minimap_signals_state();

    let id1 = m2_py_minimap_add_signal_point(500.0, 600.0);
    assert_eq!(id1, 255); // The first signal ID should be 255 based on C++ code

    let state = get_minimap_signals_state().read().unwrap();
    assert_eq!(state.signals.len(), 1);
    assert_eq!(state.signals[0].id, 255);
    assert_eq!(state.signals[0].x, 500.0);
    assert_eq!(state.signals[0].y, 600.0);

    // It should also add a waypoint
    assert_eq!(state.waypoints.len(), 1);
    assert_eq!(state.waypoints[0].dw_id, 255);
    assert_eq!(state.waypoints[0].x, 500.0);
    assert_eq!(state.waypoints[0].y, 600.0);
    assert_eq!(state.waypoints[0].text, "");
    drop(state); // Drop the lock

    let id2 = m2_py_minimap_add_signal_point(700.0, 800.0);
    assert_eq!(id2, 256);

    let state = get_minimap_signals_state().read().unwrap();
    assert_eq!(state.signals.len(), 2);
    assert_eq!(state.waypoints.len(), 2);
}

#[test]
fn test_minimap_clear_all_signal_points() {
    reset_minimap_signals_state();

    // Add normal waypoints
    let name = CString::new("Normal").unwrap();
    m2_py_minimap_add_way_point(10, 100.0, 100.0, name.as_ptr());
    m2_py_minimap_add_way_point(20, 200.0, 200.0, name.as_ptr());

    // Add signal points
    m2_py_minimap_add_signal_point(500.0, 600.0);
    m2_py_minimap_add_signal_point(700.0, 800.0);

    let state = get_minimap_signals_state().read().unwrap();
    assert_eq!(state.waypoints.len(), 4);
    assert_eq!(state.signals.len(), 2);
    drop(state);

    // Clear all signal points
    m2_py_minimap_clear_all_signal_point();

    let state = get_minimap_signals_state().read().unwrap();
    // Normal waypoints should remain
    assert_eq!(state.waypoints.len(), 2);
    assert_eq!(state.waypoints[0].dw_id, 10);
    assert_eq!(state.waypoints[1].dw_id, 20);

    // Signal points should be cleared
    assert_eq!(state.signals.len(), 0);
}
