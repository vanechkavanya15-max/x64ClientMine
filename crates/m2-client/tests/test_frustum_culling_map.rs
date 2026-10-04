use m2_client::wgpu_frustum_culling_map_engine::*;
use glam::{Mat4, Vec3};

#[test]
fn test_frustum_build() {
    let mut frustum = FrustumCullingMapEngine::new();
    
    // Create an identity matrix representing a trivial view-projection
    let mat = Mat4::IDENTITY;
    frustum.build_view_frustum(mat);

    // With Mat4::IDENTITY, the view space is mapping straightforwardly,
    // though the near/far planes are exactly at standard boundaries.
    
    // Near plane should have a normal
    assert_ne!(frustum.planes[0].length(), 0.0);
}

#[test]
fn test_frustum_aabb_inside() {
    let mut frustum = FrustumCullingMapEngine::new();
    
    // Create a perspective view-projection matrix
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -10.0), Vec3::Y);
    let proj = Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0);
    let vp = proj * view;
    
    frustum.build_view_frustum(vp);
    
    // Test an AABB entirely inside the frustum
    let aabb = Aabb {
        min: Vec3::new(-1.0, -1.0, -5.0),
        max: Vec3::new(1.0, 1.0, -3.0),
    };
    
    let result = frustum.test_aabb(&aabb);
    assert_eq!(result, ViewState::Inside);
}

#[test]
fn test_frustum_aabb_outside() {
    let mut frustum = FrustumCullingMapEngine::new();
    
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -10.0), Vec3::Y);
    let proj = Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0);
    let vp = proj * view;
    
    frustum.build_view_frustum(vp);
    
    // Test an AABB completely behind the camera
    let aabb = Aabb {
        min: Vec3::new(-1.0, -1.0, 5.0),
        max: Vec3::new(1.0, 1.0, 10.0),
    };
    
    let result = frustum.test_aabb(&aabb);
    assert_eq!(result, ViewState::Outside);
    assert!(frustum.should_cull_aabb(aabb.min, aabb.max));
}

#[test]
fn test_frustum_aabb_partial() {
    let mut frustum = FrustumCullingMapEngine::new();
    
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -10.0), Vec3::Y);
    let proj = Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0);
    let vp = proj * view;
    
    frustum.build_view_frustum(vp);
    
    // Test an AABB that spans across the near plane (z = -0.1)
    let aabb = Aabb {
        min: Vec3::new(-1.0, -1.0, -1.0),
        max: Vec3::new(1.0, 1.0, 1.0),
    };
    
    let result = frustum.test_aabb(&aabb);
    assert_eq!(result, ViewState::Partial);
    assert!(!frustum.should_cull_aabb(aabb.min, aabb.max));
}

#[test]
fn test_frustum_sphere() {
    let mut frustum = FrustumCullingMapEngine::new();
    
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -10.0), Vec3::Y);
    let proj = Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0);
    let vp = proj * view;
    
    frustum.build_view_frustum(vp);
    
    // Inside sphere
    assert_eq!(frustum.test_sphere(Vec3::new(0.0, 0.0, -5.0), 1.0), ViewState::Inside);
    
    // Outside sphere
    assert_eq!(frustum.test_sphere(Vec3::new(0.0, 0.0, 5.0), 1.0), ViewState::Outside);
    
    // Partial sphere intersecting near plane
    assert_eq!(frustum.test_sphere(Vec3::new(0.0, 0.0, -0.1), 0.5), ViewState::Partial);
}
