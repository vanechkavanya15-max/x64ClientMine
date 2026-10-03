#[path = "../src/frustum_culling_complete.rs"]
mod frustum_culling_complete;

use frustum_culling_complete::{FrustumCulling, TerrainQuadtreeNode, ViewState};
use glam::{Mat4, Vec3};

#[test]
fn test_frustum_initialization() {
    let frustum = FrustumCulling::new();
    assert_eq!(frustum.is_using_sphere, false);
    assert_eq!(frustum.radius, 0.0);
    assert_eq!(frustum.center, Vec3::ZERO);
}

#[test]
fn test_build_view_frustum_normalization() {
    let mut frustum = FrustumCulling::new();
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    frustum.build_view_frustum(vp);

    for i in 0..6 {
        let length = (frustum.planes[i].x * frustum.planes[i].x
            + frustum.planes[i].y * frustum.planes[i].y
            + frustum.planes[i].z * frustum.planes[i].z)
            .sqrt();
        assert!((length - 1.0).abs() < 1e-5, "Plane {} normal length is not 1.0: {}", i, length);
    }
}

#[test]
fn test_build_view_frustum_2() {
    let mut frustum = FrustumCulling::new();
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    frustum.build_view_frustum_2(
        vp,
        0.1,
        100.0,
        std::f32::consts::FRAC_PI_4,
        1.0,
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, -1.0)
    );

    assert_eq!(frustum.is_using_sphere, true);
    assert!(frustum.radius > 0.0);
}

#[test]
fn test_sphere_inside() {
    let mut frustum = FrustumCulling::new();
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    frustum.build_view_frustum(vp);
    
    // Sphere at the center (0, 0, 0)
    let state = frustum.test_sphere(Vec3::ZERO, 0.5);
    assert_eq!(state, ViewState::Inside);
    assert_eq!(frustum.should_skip_draw_call_sphere(Vec3::ZERO, 0.5), false);
}

#[test]
fn test_sphere_outside() {
    let mut frustum = FrustumCulling::new();
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    frustum.build_view_frustum(vp);
    
    // Sphere behind camera
    let center = Vec3::new(0.0, 0.0, 10.0);
    let state = frustum.test_sphere(center, 0.5);
    assert_eq!(state, ViewState::Outside);
    assert_eq!(frustum.should_skip_draw_call_sphere(center, 0.5), true);
}

#[test]
fn test_aabb_inside() {
    let mut frustum = FrustumCulling::new();
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    frustum.build_view_frustum(vp);

    let min = Vec3::new(-0.5, -0.5, -0.5);
    let max = Vec3::new(0.5, 0.5, 0.5);

    let state = frustum.test_aabb(min, max);
    assert_eq!(state, ViewState::Inside);
    assert_eq!(frustum.should_skip_draw_call_aabb(min, max), false);
}

#[test]
fn test_aabb_outside() {
    let mut frustum = FrustumCulling::new();
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    frustum.build_view_frustum(vp);

    let min = Vec3::new(-1.0, -1.0, 10.0);
    let max = Vec3::new(1.0, 1.0, 12.0);

    let state = frustum.test_aabb(min, max);
    assert_eq!(state, ViewState::Outside);
    assert_eq!(frustum.should_skip_draw_call_aabb(min, max), true);
}

#[test]
fn test_terrain_quadtree_node() {
    let node = TerrainQuadtreeNode::default();
    
    // Check sizes and default values
    assert_eq!(node.x0, 0);
    assert_eq!(node.y0, 0);
    assert_eq!(node.x1, 0);
    assert_eq!(node.y1, 0);
    assert!(node.nw_node.is_null());
    assert!(node.ne_node.is_null());
    assert!(node.sw_node.is_null());
    assert!(node.se_node.is_null());
    assert_eq!(node.size, 0);
    assert_eq!(node.patch_num, 0);
    assert_eq!(node.center, Vec3::new(-1.0, -1.0, -1.0));
    assert_eq!(node.radius, 0.0);
    assert_eq!(node.by_lod_level, 0);
}
