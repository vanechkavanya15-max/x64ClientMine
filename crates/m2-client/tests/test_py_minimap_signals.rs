use std::ffi::CString;

#[path = "../src/py_bridge_minimap_signals_markers.rs"]
mod minimap;

use minimap::{
    m2_py_minimap_add_way_point,
    m2_py_minimap_remove_way_point,
    m2_py_minimap_add_signal_point,
    m2_py_minimap_clear_all_signal_point,
    reset_minimap_signals_state,
    get_minimap_signals_state,
    AtlasMarkInfo,
};

#[test]
fn test_add_and_remove_waypoint() {
    reset_minimap_signals_state();
    
    let name = CString::new("Test NPC").unwrap();
    
    // Test adding
    assert!(m2_py_minimap_add_way_point(10, 100.0, 200.0, name.as_ptr()));
    
    {
        let state = get_minimap_signals_state().read().unwrap();
        assert_eq!(state.waypoints.len(), 1);
        assert_eq!(state.waypoints[0], AtlasMarkInfo {
            by_type: 6, // TYPE_WAYPOINT
            dw_id: 10,
            x: 100.0,
            y: 200.0,
            text: "Test NPC".to_string(),
        });
    }
    
    // Test adding duplicate
    assert!(!m2_py_minimap_add_way_point(10, 100.0, 200.0, name.as_ptr()));
    
    // Test removal
    assert!(m2_py_minimap_remove_way_point(10));
    
    {
        let state = get_minimap_signals_state().read().unwrap();
        assert_eq!(state.waypoints.len(), 0);
    }
    
    // Test removing non-existent
    assert!(!m2_py_minimap_remove_way_point(10));
}

#[test]
fn test_signal_id_increment() {
    reset_minimap_signals_state();
    
    let id1 = m2_py_minimap_add_signal_point(10.0, 20.0);
    assert_eq!(id1, 255);
    
    let id2 = m2_py_minimap_add_signal_point(30.0, 40.0);
    assert_eq!(id2, 256);
    
    let state = get_minimap_signals_state().read().unwrap();
    assert_eq!(state.next_signal_id, 257);
}

#[test]
fn test_add_and_clear_signal_point() {
    reset_minimap_signals_state();
    
    // Add a normal waypoint to ensure it's not cleared
    let name = CString::new("Normal WP").unwrap();
    m2_py_minimap_add_way_point(10, 0.0, 0.0, name.as_ptr());
    
    // Add signals
    m2_py_minimap_add_signal_point(100.0, 100.0);
    m2_py_minimap_add_signal_point(200.0, 200.0);
    
    {
        let state = get_minimap_signals_state().read().unwrap();
        // 1 normal + 2 signals
        assert_eq!(state.waypoints.len(), 3);
        assert_eq!(state.signals.len(), 2);
    }
    
    // Clear signals
    assert!(m2_py_minimap_clear_all_signal_point());
    
    {
        let state = get_minimap_signals_state().read().unwrap();
        // 1 normal WP should remain
        assert_eq!(state.waypoints.len(), 1);
        assert_eq!(state.waypoints[0].dw_id, 10);
        // Signals vector should be empty
        assert_eq!(state.signals.len(), 0);
    }
}
