use glam::{Mat4, Vec3};
use m2_client::view_frustum::{Aabb, Frustum, ViewState};

#[test]
fn test_frustum_normalization() {
    let mut frustum = Frustum::new();

    // Create a perspective matrix and look_at matrix
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    frustum.build_view_frustum(vp);

    for i in 0..6 {
        let length = (frustum.planes[i].x * frustum.planes[i].x
            + frustum.planes[i].y * frustum.planes[i].y
            + frustum.planes[i].z * frustum.planes[i].z)
            .sqrt();
        // Since it's normalized, the normal length should be 1.0 (with floating point error margin)
        assert!(
            (length - 1.0).abs() < 1e-5,
            "Plane {} normal length is not 1.0: {}",
            i,
            length
        );
    }
}

#[test]
fn test_aabb_inside() {
    let mut frustum = Frustum::new();

    // Camera at (0, 0, 5) looking at (0, 0, 0)
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    frustum.build_view_frustum(vp);

    // Box at the center, completely visible
    let aabb = Aabb {
        min: Vec3::new(-0.5, -0.5, -0.5),
        max: Vec3::new(0.5, 0.5, 0.5),
    };

    let state = frustum.view_volume_test_aabb(&aabb);
    assert_eq!(state, ViewState::Inside);
}

#[test]
fn test_aabb_outside_behind_camera() {
    let mut frustum = Frustum::new();

    // Camera at (0, 0, 5) looking at (0, 0, 0)
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    frustum.build_view_frustum(vp);

    // Box behind the camera (z > 5)
    let aabb = Aabb {
        min: Vec3::new(-1.0, -1.0, 10.0),
        max: Vec3::new(1.0, 1.0, 12.0),
    };

    let state = frustum.view_volume_test_aabb(&aabb);
    assert_eq!(state, ViewState::Outside);
}

#[test]
fn test_aabb_partial() {
    let mut frustum = Frustum::new();

    // Camera at (0, 0, 5) looking at (0, 0, 0)
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let vp = proj * view;

    frustum.build_view_frustum(vp);

    // Box intersecting the right edge of the view volume
    // Let's pick a box that crosses the right boundary
    // At z=0, the camera is 5 units away. Tan(22.5 deg) ~ 0.414, so at z=0, w = 5 * 0.414 ~ 2.07
    let aabb = Aabb {
        min: Vec3::new(1.5, -0.5, -0.5), // Starts inside
        max: Vec3::new(3.5, 0.5, 0.5),  // Ends outside
    };

    let state = frustum.view_volume_test_aabb(&aabb);
    assert_eq!(state, ViewState::Partial);
}

#[test]
fn test_aabb_edge_case_zero_matrix() {
    let mut frustum = Frustum::new();

    // Zero matrix should safely normalize without NaN (by checking length > 0.0)
    let vp = Mat4::ZERO;

    frustum.build_view_frustum(vp);

    for i in 0..6 {
        assert_eq!(frustum.planes[i].x, 0.0);
        assert_eq!(frustum.planes[i].y, 0.0);
        assert_eq!(frustum.planes[i].z, 0.0);
        assert_eq!(frustum.planes[i].w, 0.0);
    }
    
    // A box test against zero frustum
    let aabb = Aabb {
        min: Vec3::new(-1.0, -1.0, -1.0),
        max: Vec3::new(1.0, 1.0, 1.0),
    };

    // Because the distance will be 0.0, which is not < 0.0, it will not be outside.
    // So it will be considered Partial or Inside (in our case dn is 0.0 so Inside).
    let state = frustum.view_volume_test_aabb(&aabb);
    assert_eq!(state, ViewState::Inside);
}
