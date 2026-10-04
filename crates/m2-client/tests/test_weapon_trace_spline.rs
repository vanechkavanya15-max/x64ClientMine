use m2_client::wgpu_weapon_trace_spline_mesh::WeaponTraceSplineMesh;
use glam::Vec3;

#[test]
fn test_weapon_trace_spline_turn_on_off() {
    let mut trace = WeaponTraceSplineMesh::new();
    assert!(!trace.is_playing);
    
    trace.turn_on();
    assert!(trace.is_playing);
    
    trace.turn_off();
    assert!(!trace.is_playing);
}

#[test]
fn test_weapon_trace_spline_culling() {
    let mut trace = WeaponTraceSplineMesh::new();
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
fn test_weapon_trace_spline_build_vertices() {
    let mut trace = WeaponTraceSplineMesh::new();
    trace.turn_on();
    trace.set_life_time(0.5);
    trace.set_sampling_time(0.1);
    
    // 0 points
    assert!(!trace.build_vertices());
    
    // Add 3 points to ensure we have enough data to generate spline segments
    trace.add_point(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    trace.update(0.2);
    trace.add_point(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
    trace.update(0.2);
    trace.add_point(Vec3::new(2.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 0.0));
    
    assert!(trace.build_vertices());
    
    // We expect some vertices generated from spline
    assert!(trace.vertices.len() >= 4);
    assert_eq!(trace.vertices.len() % 2, 0); // They should come in pairs (long, short)

    for i in 0..trace.vertices.len() {
        if i % 2 == 0 {
            // long vertex: V coordinate should be 0
            assert_eq!(trace.vertices[i].tex_coord.v, 0.0);
            
            // Should have dynamic alpha depending on progression
            // But verify RGB components are roughly 0.3, 0.8, 1.0 (76, 204, 255)
            let color = trace.vertices[i].diffuse;
            let r = (color >> 16) & 0xFF;
            let g = (color >> 8) & 0xFF;
            let b = color & 0xFF;
            
            assert_eq!(r, 76);
            assert_eq!(g, 204);
            assert_eq!(b, 255);
        } else {
            // short vertex: V coordinate should be 1
            assert_eq!(trace.vertices[i].tex_coord.v, 1.0);
            
            // Alpha should be 0 for short vertices
            let color = trace.vertices[i].diffuse;
            let a = (color >> 24) & 0xFF;
            assert_eq!(a, 0);
        }
    }
}

#[test]
fn test_weapon_trace_spline_clear() {
    let mut trace = WeaponTraceSplineMesh::new();
    trace.turn_on();
    trace.add_point(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    
    assert_eq!(trace.short_points.len(), 1);
    assert_eq!(trace.long_points.len(), 1);
    
    trace.clear();
    
    assert_eq!(trace.short_points.len(), 0);
    assert_eq!(trace.long_points.len(), 0);
    assert_eq!(trace.vertices.len(), 0);
}
