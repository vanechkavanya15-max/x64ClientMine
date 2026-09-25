use m2_client::weapon_trace::{WeaponTrace, Vector3};

#[test]
fn test_integration_weapon_trace_init() {
    let instance = WeaponTrace::new();
    assert!(instance.is_ready());
    assert_eq!(instance.life_time, 0.18);
    assert_eq!(instance.sampling_time, 0.003);
    assert_eq!(instance.is_playing, false);
}

#[test]
fn test_integration_weapon_trace_update_and_build() {
    let mut trace = WeaponTrace::new();
    trace.turn_on();
    trace.set_life_time(0.5);

    // Add points
    trace.update_with_points(0.0, Some(Vector3 { x: 0.0, y: 0.0, z: 0.0 }), Some(Vector3 { x: 0.0, y: 1.0, z: 0.0 }));
    trace.update_with_points(0.1, Some(Vector3 { x: 1.0, y: 0.0, z: 0.0 }), Some(Vector3 { x: 1.0, y: 1.0, z: 0.0 }));
    trace.update_with_points(0.1, Some(Vector3 { x: 2.0, y: 0.0, z: 0.0 }), Some(Vector3 { x: 2.0, y: 1.0, z: 0.0 }));
    
    assert_eq!(trace.short_time_point_list.len(), 3);
    assert_eq!(trace.long_time_point_list.len(), 3);

    // Check time values. The first inserted point should now be the oldest (index 2 in deque)
    assert!(trace.short_time_point_list[2].time >= 0.2);

    let success = trace.build_vertex();
    assert!(success, "Build vertex failed");
    
    // Check if vertices were generated
    assert!(trace.pdt_vertex_vector.len() > 0, "No vertices generated");
    
    // Prune points
    trace.update_with_points(1.0, None, None);
    assert_eq!(trace.short_time_point_list.len(), 0);
    assert_eq!(trace.long_time_point_list.len(), 0);
}

#[test]
fn test_integration_weapon_trace_flags() {
    let mut trace = WeaponTrace::new();
    trace.turn_on();
    assert!(trace.is_playing);
    trace.turn_off();
    assert!(!trace.is_playing);
    
    trace.use_texture_mode();
    assert!(trace.use_texture);
    trace.use_alpha();
    assert!(!trace.use_texture);
}
