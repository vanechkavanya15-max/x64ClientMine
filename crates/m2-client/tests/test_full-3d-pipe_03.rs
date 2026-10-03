#[path = "../src/frustum_culling_3d.rs"]
mod frustum_culling_3d;

use frustum_culling_3d::{Aabb, BoundingSphere, Frustum};
use glam::{Mat4, Vec3};

#[test]
fn test_frustum_sphere_completely_inside() {
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;
    let frustum = Frustum::from_view_proj(vp);

    let sphere = BoundingSphere::new(Vec3::ZERO, 0.5);
    assert!(frustum.intersects_sphere(&sphere));
}

#[test]
fn test_frustum_sphere_completely_outside() {
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;
    let frustum = Frustum::from_view_proj(vp);

    // Behind the camera
    let sphere = BoundingSphere::new(Vec3::new(0.0, 0.0, 10.0), 0.5);
    assert!(!frustum.intersects_sphere(&sphere));
}

#[test]
fn test_frustum_sphere_partially_intersecting() {
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;
    let frustum = Frustum::from_view_proj(vp);

    // At the near plane, 0.1 units from the camera at 5.0 => z = 4.9.
    // The camera looks at ZERO from 5.0, so the near plane is at Z=4.9.
    // Center at Z=4.9 and Radius 0.5 will partially intersect.
    let sphere = BoundingSphere::new(Vec3::new(0.0, 0.0, 4.9), 0.5);
    assert!(frustum.intersects_sphere(&sphere));
}

#[test]
fn test_frustum_aabb_completely_inside() {
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;
    let frustum = Frustum::from_view_proj(vp);

    let aabb = Aabb::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0.5, 0.5, 0.5));
    assert!(frustum.intersects_aabb(&aabb));
}

#[test]
fn test_frustum_aabb_completely_outside() {
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;
    let frustum = Frustum::from_view_proj(vp);

    let aabb = Aabb::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(1.0, 1.0, 11.0));
    assert!(!frustum.intersects_aabb(&aabb));
}

#[test]
fn test_frustum_aabb_partially_intersecting() {
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;
    let frustum = Frustum::from_view_proj(vp);

    let aabb = Aabb::new(Vec3::new(-1.0, -1.0, 4.8), Vec3::new(1.0, 1.0, 5.0));
    assert!(frustum.intersects_aabb(&aabb));
}
