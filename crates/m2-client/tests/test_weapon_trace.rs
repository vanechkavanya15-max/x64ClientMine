use m2_client::weapon_trace::{WeaponTrace, TimePoint, TraceVertex};
use glam::Vec3;

#[test]
fn test_weapon_trace_turn_on_off() {
    let mut trace = WeaponTrace::new();
    assert!(!trace.is_playing);
    
    trace.turn_on();
    assert!(trace.is_playing);
    
    trace.turn_off();
    assert!(!trace.is_playing);
}

#[test]
fn test_weapon_trace_clear() {
    let mut trace = WeaponTrace::new();
    trace.turn_on();
    trace.add_point(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    
    assert_eq!(trace.short_points.len(), 1);
    assert_eq!(trace.long_points.len(), 1);
    
    trace.clear();
    
    assert_eq!(trace.short_points.len(), 0);
    assert_eq!(trace.long_points.len(), 0);
}

#[test]
fn test_weapon_trace_life_time_culling() {
    let mut trace = WeaponTrace::new();
    trace.turn_on();
    trace.set_life_time(0.5);
    
    // Add point at t=0
    trace.add_point(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    
    // Update by 0.3s
    trace.update(0.3);
    
    // Add point at t=0.3
    trace.add_point(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
    
    assert_eq!(trace.short_points.len(), 2);
    
    // Update by 0.3s more (total time 0.6s)
    // The first point is now 0.6s old, which is > 0.5s life_time
    // The second point is 0.3s old, which is < 0.5s life_time
    trace.update(0.3);
    
    assert_eq!(trace.short_points.len(), 1);
    assert_eq!(trace.long_points.len(), 1);
    
    // Ensure the remaining point is the second one
    assert_eq!(trace.short_points.front().unwrap().pos, Vec3::new(1.0, 0.0, 0.0));
}

#[test]
fn test_weapon_trace_build_vertices_not_enough_points() {
    let mut trace = WeaponTrace::new();
    trace.turn_on();
    
    // 0 points
    assert!(!trace.build_vertices());
    assert_eq!(trace.vertices.len(), 0);
    
    // 1 point
    trace.add_point(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    assert!(!trace.build_vertices());
    assert_eq!(trace.vertices.len(), 0);
}

#[test]
fn test_weapon_trace_build_vertices() {
    let mut trace = WeaponTrace::new();
    trace.turn_on();
    trace.set_life_time(0.5);
    trace.set_sampling_time(0.1);
    
    // Add 3 points
    trace.add_point(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    trace.update(0.2);
    trace.add_point(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
    trace.update(0.2);
    trace.add_point(Vec3::new(2.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 0.0));
    
    assert!(trace.build_vertices());
    
    // We expect some vertices generated from spline
    assert!(trace.vertices.len() >= 4);
    
    // Verify interleaving (long, short, long, short...)
    for i in 0..trace.vertices.len() {
        if i % 2 == 0 {
            // long vertex
            assert_eq!(trace.vertices[i].v, 0.0);
        } else {
            // short vertex
            assert_eq!(trace.vertices[i].v, 1.0);
            assert_eq!(trace.vertices[i].alpha, 0.0); // short point alpha is always 0.0
        }
    }
}
