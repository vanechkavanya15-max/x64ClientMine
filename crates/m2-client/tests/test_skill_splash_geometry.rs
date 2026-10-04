use glam::Vec3;
use m2_client::game_skill_splash_area_geometry::{
    ConeSplash, CylinderSplash, BoxSplash, SphereSplash, SplashGeometry,
};

#[test]
fn test_cone_splash_geometry_refactored() {
    let cone = ConeSplash {
        origin: Vec3::ZERO,
        direction: Vec3::X, // Direction along positive X
        radius: 12.0,
        cone_angle_cos: 0.70710678, // 45 degrees half-angle (cos(45) approx 0.707)
    };

    // Point exactly at origin (dist_sq <= EPSILON case)
    assert!(cone.is_point_inside(Vec3::ZERO));
    // Point inside radius, well within angle
    assert!(cone.is_point_inside(Vec3::new(8.0, 0.0, 0.0)));
    // Point outside radius
    assert!(!cone.is_point_inside(Vec3::new(13.0, 0.0, 0.0)));
    // Point outside angle (e.g. along Y axis)
    assert!(!cone.is_point_inside(Vec3::new(0.0, 8.0, 0.0)));
    
    // Boundary test for exact radius touch
    assert!(cone.is_point_inside(Vec3::new(12.0, 0.0, 0.0)));

    // Sphere at origin
    assert!(cone.is_sphere_inside(Vec3::ZERO, 2.0));
    // Sphere fully inside
    assert!(cone.is_sphere_inside(Vec3::new(6.0, 0.0, 0.0), 1.0));
    // Sphere outside origin but touching angle boundary due to radius
    assert!(cone.is_sphere_inside(Vec3::new(5.0, 5.0, 0.0), 1.5));
    // Sphere far outside
    assert!(!cone.is_sphere_inside(Vec3::new(20.0, 20.0, 0.0), 1.0));
}

#[test]
fn test_cylinder_splash_geometry_refactored() {
    let cyl = CylinderSplash {
        center: Vec3::new(10.0, 10.0, 10.0),
        radius: 8.0,
        height_min: -5.0, // Relative to center.z (which is 10.0, so absolute 5.0 to 20.0)
        height_max: 10.0,
    };

    // Valid point inside
    assert!(cyl.is_point_inside(Vec3::new(12.0, 12.0, 12.0)));
    // Point below min height
    assert!(!cyl.is_point_inside(Vec3::new(12.0, 12.0, 4.0)));
    // Point above max height
    assert!(!cyl.is_point_inside(Vec3::new(12.0, 12.0, 21.0)));
    // Point outside radius
    assert!(!cyl.is_point_inside(Vec3::new(20.0, 10.0, 12.0)));

    // Sphere completely inside
    assert!(cyl.is_sphere_inside(Vec3::new(10.0, 10.0, 15.0), 2.0));
    // Sphere touching the top boundary
    assert!(cyl.is_sphere_inside(Vec3::new(10.0, 10.0, 21.0), 2.0));
    // Sphere completely outside the top boundary
    assert!(!cyl.is_sphere_inside(Vec3::new(10.0, 10.0, 25.0), 2.0));
    // Sphere touching the lateral radius boundary
    assert!(cyl.is_sphere_inside(Vec3::new(19.0, 10.0, 15.0), 2.0));
}

#[test]
fn test_box_splash_geometry_refactored() {
    let box_aoe = BoxSplash {
        origin: Vec3::new(0.0, 0.0, 0.0),
        right: Vec3::X,
        forward: Vec3::Y,
        up: Vec3::Z,
        extents: Vec3::new(5.0, 10.0, 4.0), // Extents are half-sizes, so X is -5 to 5, Y is -10 to 10, Z is 0 to 8 (because check uses extents.z * 2.0)
    };

    // Point strictly inside
    assert!(box_aoe.is_point_inside(Vec3::new(2.0, -5.0, 4.0)));
    // Point outside X boundary
    assert!(!box_aoe.is_point_inside(Vec3::new(6.0, 0.0, 4.0)));
    // Point negative Z (should be false since base is at origin)
    assert!(!box_aoe.is_point_inside(Vec3::new(0.0, 0.0, -1.0)));
    // Point above max Z
    assert!(!box_aoe.is_point_inside(Vec3::new(0.0, 0.0, 9.0)));

    // Sphere strictly inside
    assert!(box_aoe.is_sphere_inside(Vec3::new(0.0, 0.0, 4.0), 1.0));
    // Sphere touching X boundary from outside
    assert!(box_aoe.is_sphere_inside(Vec3::new(5.5, 0.0, 4.0), 1.0));
    // Sphere touching negative Z boundary
    assert!(box_aoe.is_sphere_inside(Vec3::new(0.0, 0.0, -0.5), 1.0));
    // Sphere strictly outside negative Z boundary
    assert!(!box_aoe.is_sphere_inside(Vec3::new(0.0, 0.0, -2.0), 1.0));
}

#[test]
fn test_sphere_splash_geometry_refactored() {
    let sphere = SphereSplash {
        center: Vec3::new(5.0, 5.0, 5.0),
        radius: 10.0,
    };

    // Point exactly at center
    assert!(sphere.is_point_inside(Vec3::new(5.0, 5.0, 5.0)));
    // Point on the boundary
    assert!(sphere.is_point_inside(Vec3::new(15.0, 5.0, 5.0)));
    // Point outside
    assert!(!sphere.is_point_inside(Vec3::new(16.0, 5.0, 5.0)));

    // Target sphere completely inside
    assert!(sphere.is_sphere_inside(Vec3::new(5.0, 5.0, 5.0), 2.0));
    // Target sphere overlapping boundary
    assert!(sphere.is_sphere_inside(Vec3::new(16.0, 5.0, 5.0), 2.0));
    // Target sphere far outside
    assert!(!sphere.is_sphere_inside(Vec3::new(20.0, 20.0, 20.0), 1.0));
}
