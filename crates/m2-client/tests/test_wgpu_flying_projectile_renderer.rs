use m2_client::wgpu_flying_projectile_renderer::{
    cross_product, look_at_quaternion, normalize, FlyTraceRenderer, FlyingProjectileRenderer,
};

#[test]
fn test_cross_product() {
    let a = [1.0, 0.0, 0.0];
    let b = [0.0, 1.0, 0.0];
    let c = cross_product(a, b);
    assert_eq!(c, [0.0, 0.0, 1.0]);
}

#[test]
fn test_normalize() {
    let a = [3.0, 0.0, 4.0];
    let n = normalize(a);
    assert!((n[0] - 0.6).abs() < 1e-5);
    assert_eq!(n[1], 0.0);
    assert!((n[2] - 0.8).abs() < 1e-5);
}

#[test]
fn test_look_at_quaternion() {
    let vel = [0.0, 10.0, 0.0]; // Velocity pointing in positive Y direction
    let q = look_at_quaternion(vel);
    
    // We just verify the quaternion is normalized and valid structure
    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    assert!((len - 1.0).abs() < 1e-5);
}

#[test]
fn test_fly_trace_renderer_tail_dropping() {
    let mut renderer = FlyTraceRenderer::new(false, 0xFFFFFFFF, 10.0, 2.0); // 2.0 tail length
    
    // Add point at 0.0s
    renderer.update_new_position([0.0, 0.0, 0.0], 0.0);
    assert_eq!(renderer.positions.len(), 1);

    // Add point at 1.0s
    renderer.update_new_position([10.0, 0.0, 0.0], 1.0);
    assert_eq!(renderer.positions.len(), 2);

    // Add point at 2.5s (point at 0.0s should be dropped since 2.5 - 0.0 > 2.0)
    renderer.update_new_position([20.0, 0.0, 0.0], 2.5);
    assert_eq!(renderer.positions.len(), 2); 
    assert_eq!(renderer.positions.back().unwrap().time, 1.0);
}

#[test]
fn test_fly_trace_generate_vertices() {
    let mut renderer = FlyTraceRenderer::new(false, 0xFFFFFFFF, 10.0, 5.0);
    renderer.update_new_position([0.0, 0.0, 0.0], 0.0);
    renderer.update_new_position([10.0, 0.0, 0.0], 1.0);
    renderer.update_new_position([20.0, 0.0, 0.0], 2.0);

    let vertices = renderer.generate_vertices(2.0, [10.0, 10.0, 10.0], [0.0, 0.0, -1.0]);

    // 2 segments (3 points), each segment generates 6 vertices
    assert_eq!(vertices.len(), 12);
}

#[test]
fn test_flying_projectile_renderer_init() {
    let r = FlyingProjectileRenderer::new(100);
    assert_eq!(r.data_id, 100);
    assert_eq!(r.position, [0.0, 0.0, 0.0]);
    assert_eq!(r.rotation, [0.0, 0.0, 0.0, 1.0]);
}