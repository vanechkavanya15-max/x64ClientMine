use glam::Vec3;
use m2_client::game_skill_splash_area_geometry::{
    ConeSplash, CylinderSplash, BoxSplash, SphereSplash, SplashGeometry,
};

#[test]
fn test_cone_splash_geometry() {
    let cone = ConeSplash {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0),
        radius: 10.0,
        cone_angle_cos: 0.5, // 60 degrees half-angle (cos(60) = 0.5)
    };

    // Point inside, straight ahead
    assert!(cone.is_point_inside(Vec3::new(5.0, 0.0, 0.0)));
    // Point outside radius
    assert!(!cone.is_point_inside(Vec3::new(15.0, 0.0, 0.0)));
    // Point outside cone angle
    assert!(!cone.is_point_inside(Vec3::new(0.0, 5.0, 0.0)));

    // Sphere inside, straight ahead
    assert!(cone.is_sphere_inside(Vec3::new(5.0, 0.0, 0.0), 1.0));
    // Sphere overlapping outer radius boundary
    assert!(cone.is_sphere_inside(Vec3::new(10.5, 0.0, 0.0), 1.0));
    // Sphere far outside radius
    assert!(!cone.is_sphere_inside(Vec3::new(15.0, 0.0, 0.0), 1.0));
    // Sphere overlapping angle boundary
    assert!(cone.is_sphere_inside(Vec3::new(5.0, 5.0, 0.0), 2.5));
    // Sphere at the exact origin (testing distance <= radius fallback)
    assert!(cone.is_sphere_inside(Vec3::new(0.0, 0.0, 0.0), 1.0));
    // Sphere very close to origin (distance < radius)
    assert!(cone.is_sphere_inside(Vec3::new(0.5, 0.5, 0.0), 2.0));
}

#[test]
fn test_cylinder_splash_geometry() {
    let cylinder = CylinderSplash {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 10.0,
        height_min: -5.0,
        height_max: 5.0,
    };

    // Point inside
    assert!(cylinder.is_point_inside(Vec3::new(5.0, 5.0, 0.0)));
    // Point outside radius
    assert!(!cylinder.is_point_inside(Vec3::new(15.0, 0.0, 0.0)));
    // Point outside height
    assert!(!cylinder.is_point_inside(Vec3::new(5.0, 5.0, 10.0)));

    // Sphere inside
    assert!(cylinder.is_sphere_inside(Vec3::new(5.0, 5.0, 0.0), 1.0));
    // Sphere overlapping outer radius
    assert!(cylinder.is_sphere_inside(Vec3::new(10.5, 0.0, 0.0), 1.0));
    // Sphere overlapping height max
    assert!(cylinder.is_sphere_inside(Vec3::new(5.0, 5.0, 5.5), 1.0));
    // Sphere outside radius and height
    assert!(!cylinder.is_sphere_inside(Vec3::new(15.0, 0.0, 10.0), 1.0));

    // Test Cylinder at High Z (making sure dz is relative)
    let cylinder_high = CylinderSplash {
        center: Vec3::new(10.0, 10.0, 100.0),
        radius: 5.0,
        height_min: -2.0,
        height_max: 2.0,
    };
    
    // Inside high Z cylinder
    assert!(cylinder_high.is_point_inside(Vec3::new(12.0, 10.0, 101.0)));
    // Outside high Z cylinder (wrong Z)
    assert!(!cylinder_high.is_point_inside(Vec3::new(12.0, 10.0, 5.0)));
    // Sphere inside high Z cylinder
    assert!(cylinder_high.is_sphere_inside(Vec3::new(12.0, 10.0, 99.0), 1.0));
    // Sphere outside high Z cylinder (wrong Z)
    assert!(!cylinder_high.is_sphere_inside(Vec3::new(12.0, 10.0, 10.0), 1.0));
}

#[test]
fn test_box_splash_geometry() {
    let box_splash = BoxSplash {
        origin: Vec3::new(0.0, 0.0, 0.0),
        right: Vec3::new(1.0, 0.0, 0.0),
        forward: Vec3::new(0.0, 1.0, 0.0),
        up: Vec3::new(0.0, 0.0, 1.0),
        extents: Vec3::new(5.0, 10.0, 5.0),
    };

    // Point inside
    assert!(box_splash.is_point_inside(Vec3::new(2.0, 5.0, 2.0)));
    // Point outside X
    assert!(!box_splash.is_point_inside(Vec3::new(6.0, 5.0, 2.0)));
    // Point outside Y
    assert!(!box_splash.is_point_inside(Vec3::new(2.0, 15.0, 2.0)));

    // Sphere inside
    assert!(box_splash.is_sphere_inside(Vec3::new(2.0, 5.0, 2.0), 1.0));
    // Sphere overlapping X bound
    assert!(box_splash.is_sphere_inside(Vec3::new(5.5, 5.0, 2.0), 1.0));
    // Sphere outside Y bound
    assert!(!box_splash.is_sphere_inside(Vec3::new(2.0, 15.0, 2.0), 1.0));

    // Point outside Z bound (below base)
    assert!(!box_splash.is_point_inside(Vec3::new(2.0, 5.0, -1.0)));
    // Point inside Z bound (above base up to extents.z * 2.0)
    assert!(box_splash.is_point_inside(Vec3::new(2.0, 5.0, 9.0)));
    // Point outside Z bound (above max)
    assert!(!box_splash.is_point_inside(Vec3::new(2.0, 5.0, 11.0)));
    
    // Sphere overlapping Z bound (below base)
    assert!(box_splash.is_sphere_inside(Vec3::new(2.0, 5.0, -0.5), 1.0));
    // Sphere completely outside Z bound (below base)
    assert!(!box_splash.is_sphere_inside(Vec3::new(2.0, 5.0, -2.0), 1.0));
}

#[test]
fn test_sphere_splash_geometry() {
    let sphere = SphereSplash {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 10.0,
    };

    // Point inside
    assert!(sphere.is_point_inside(Vec3::new(5.0, 0.0, 0.0)));
    // Point outside
    assert!(!sphere.is_point_inside(Vec3::new(15.0, 0.0, 0.0)));

    // Sphere inside
    assert!(sphere.is_sphere_inside(Vec3::new(5.0, 0.0, 0.0), 1.0));
    // Sphere overlapping outer boundary
    assert!(sphere.is_sphere_inside(Vec3::new(10.5, 0.0, 0.0), 1.0));
    // Sphere far outside
    assert!(!sphere.is_sphere_inside(Vec3::new(15.0, 0.0, 0.0), 1.0));
}
