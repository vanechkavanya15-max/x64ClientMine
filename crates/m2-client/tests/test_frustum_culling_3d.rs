#[path = "../src/frustum_culling_3d.rs"]
mod frustum_culling_3d;

use frustum_culling_3d::{Aabb, BoundingSphere, Frustum, Plane};
use glam::{Mat4, Vec3};

#[test]
fn test_plane_normalization() {
    let mut p = Plane::new(0.0, 3.0, 4.0, 10.0); // length is 5.0
    p.normalize();
    assert!((p.normal.length() - 1.0).abs() < 1e-5);
    assert_eq!(p.normal, Vec3::new(0.0, 0.6, 0.8));
    assert_eq!(p.distance, 2.0);
}

#[test]
fn test_frustum_from_view_proj() {
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    let frustum = Frustum::from_view_proj(vp);
    assert_eq!(frustum.planes.len(), 6);
    
    for plane in &frustum.planes {
        assert!((plane.normal.length() - 1.0).abs() < 1e-5);
    }
}
