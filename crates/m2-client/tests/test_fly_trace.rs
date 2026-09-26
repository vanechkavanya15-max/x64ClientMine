use m2_client::fly_trace::FlyTrace;

#[test]
fn test_fly_trace_initialization() {
    let trace = FlyTrace::new(0.3, 10.0);
    assert_eq!(trace.duration, 0.3);
    assert_eq!(trace.base_width, 10.0);
    assert!(trace.points.is_empty());
    assert!(!trace.is_rect_shape);
}

#[test]
fn test_add_point() {
    let mut trace = FlyTrace::new(0.3, 10.0);
    trace.add_point((0.0, 0.0, 0.0), 1.0);
    assert_eq!(trace.points.len(), 1);
    assert_eq!(trace.points[0].position, (0.0, 0.0, 0.0));
    assert_eq!(trace.points[0].created_time, 1.0);
    assert_eq!(trace.points[0].width, 10.0);

    trace.add_point((1.0, 1.0, 1.0), 1.1);
    assert_eq!(trace.points.len(), 2);
    assert_eq!(trace.points[0].position, (1.0, 1.0, 1.0));
    assert_eq!(trace.points[0].created_time, 1.1);
}

#[test]
fn test_tail_fading() {
    let mut trace = FlyTrace::new(0.3, 10.0);
    trace.add_point((0.0, 0.0, 0.0), 1.0);
    trace.add_point((1.0, 1.0, 1.0), 1.1);
    trace.add_point((2.0, 2.0, 2.0), 1.2);
    assert_eq!(trace.points.len(), 3);

    // This point is added at 1.4, which means the point at 1.0 (age = 0.4) should be removed
    // since 0.4 > 0.3
    trace.add_point((3.0, 3.0, 3.0), 1.4);
    assert_eq!(trace.points.len(), 3);
    assert_eq!(trace.points.back().unwrap().created_time, 1.1); // oldest point is now the one added at 1.1

    // Add at 2.0, all points except the new one should be removed
    trace.add_point((4.0, 4.0, 4.0), 2.0);
    assert_eq!(trace.points.len(), 1);
    assert_eq!(trace.points[0].created_time, 2.0);
}

#[test]
fn test_generate_mesh_vertices_empty_or_one() {
    let mut trace = FlyTrace::new(0.3, 10.0);
    let vertices = trace.generate_mesh_vertices(1.0);
    assert!(vertices.is_empty());

    trace.add_point((0.0, 0.0, 0.0), 1.0);
    let vertices = trace.generate_mesh_vertices(1.0);
    assert!(vertices.is_empty());
}

#[test]
fn test_generate_mesh_vertices() {
    let mut trace = FlyTrace::new(1.0, 10.0);
    trace.add_point((0.0, 0.0, 0.0), 0.0);
    trace.add_point((10.0, 0.0, 0.0), 0.5);
    trace.add_point((20.0, 0.0, 0.0), 1.0); // Points are added in reverse order: p[0]=20, p[1]=10, p[2]=0

    let vertices = trace.generate_mesh_vertices(1.0);
    assert_eq!(vertices.len(), 6); // 3 points * 2 vertices per point

    // Point 0 (x=20) added at 1.0, age = 0.0, rate = 1.0, current_width = 10.0
    // Point 1 (x=10) added at 0.5, age = 0.5, rate = 0.5, current_width = 5.0
    // Point 2 (x=0)  added at 0.0, age = 1.0, rate = 0.0, current_width = 0.0

    // Distances: p[0] to p[1] = 10, p[1] to p[2] = 10
    // Total length = 20
    // p[0] u = 0/20 = 0
    // p[1] u = 10/20 = 0.5
    // p[2] u = 20/20 = 1.0
    assert_eq!(vertices[0].uv, (0.0, 0.0));
    assert_eq!(vertices[1].uv, (0.0, 1.0));
    assert_eq!(vertices[2].uv, (0.5, 0.0));
    assert_eq!(vertices[3].uv, (0.5, 1.0));
    assert_eq!(vertices[4].uv, (1.0, 0.0));
    assert_eq!(vertices[5].uv, (1.0, 1.0));

    // width checking
    let w0 = ((vertices[0].position.0 - vertices[1].position.0).powi(2)
        + (vertices[0].position.1 - vertices[1].position.1).powi(2)
        + (vertices[0].position.2 - vertices[1].position.2).powi(2)).sqrt();
    assert!((w0 - 10.0).abs() < 1e-4);

    let w1 = ((vertices[2].position.0 - vertices[3].position.0).powi(2)
        + (vertices[2].position.1 - vertices[3].position.1).powi(2)
        + (vertices[2].position.2 - vertices[3].position.2).powi(2)).sqrt();
    assert!((w1 - 5.0).abs() < 1e-4);

    let w2 = ((vertices[4].position.0 - vertices[5].position.0).powi(2)
        + (vertices[4].position.1 - vertices[5].position.1).powi(2)
        + (vertices[4].position.2 - vertices[5].position.2).powi(2)).sqrt();
    assert!((w2 - 0.0).abs() < 1e-4);
}

#[test]
fn test_identical_points() {
    let mut trace = FlyTrace::new(1.0, 10.0);
    trace.add_point((0.0, 0.0, 0.0), 0.0);
    trace.add_point((0.0, 0.0, 0.0), 0.5);

    let vertices = trace.generate_mesh_vertices(0.5);
    assert_eq!(vertices.len(), 4); // should not crash or produce NaNs
}
