use glam::{Mat4, Vec3};
use m2_client::camera_ray_unproject::{screen_to_world_ray};

// Helper function to create a basic view-projection matrix for testing
fn create_test_view_proj() -> [[f32; 4]; 4] {
    // A simple orthographic or perspective projection
    let view = Mat4::look_at_rh(
        Vec3::new(0.0, -100.0, 100.0), // Camera pos
        Vec3::new(0.0, 0.0, 0.0),      // Look at
        Vec3::new(0.0, 0.0, 1.0),      // Up vector
    );
    let proj = Mat4::perspective_rh_gl(std::f32::consts::FRAC_PI_4, 800.0 / 600.0, 0.1, 1000.0);
    let view_proj = proj * view;
    view_proj.to_cols_array_2d()
}

fn create_identity_view_proj() -> [[f32; 4]; 4] {
    Mat4::IDENTITY.to_cols_array_2d()
}

#[test]
fn test_center_screen_ray() {
    let view_proj = create_identity_view_proj();
    let ray = screen_to_world_ray(400.0, 300.0, 800.0, 600.0, &view_proj);
    
    // NDC will be (0, 0)
    // Inverse VP is identity.
    // Near: (0, 0, 0, 1) -> (0, 0, 0)
    // Far: (0, 0, 1, 1) -> (0, 0, 1)
    
    // We expect origin near 0,0,0
    assert!(ray.origin[0].abs() < 1e-5);
    assert!(ray.origin[1].abs() < 1e-5);
    assert!(ray.origin[2].abs() < 1e-5);

    // Direction should be (0,0,1) normalized
    assert!(ray.direction[0].abs() < 1e-5);
    assert!(ray.direction[1].abs() < 1e-5);
    assert!((ray.direction[2] - 1.0).abs() < 1e-5);
}

#[test]
fn test_screen_corners() {
    let view_proj = create_identity_view_proj();
    let w = 800.0;
    let h = 600.0;

    // Top-Left (0, 0) -> NDC (-1, 1)
    let ray_tl = screen_to_world_ray(0.0, 0.0, w, h, &view_proj);
    assert!((ray_tl.origin[0] - (-1.0)).abs() < 1e-5);
    assert!((ray_tl.origin[1] - 1.0).abs() < 1e-5);
    assert!(ray_tl.origin[2].abs() < 1e-5);
    assert!((ray_tl.direction[2] - 1.0).abs() < 1e-5); // Direction still Z since Far is (-1, 1, 1)

    // Bottom-Right (800, 600) -> NDC (1, -1)
    let ray_br = screen_to_world_ray(w, h, w, h, &view_proj);
    assert!((ray_br.origin[0] - 1.0).abs() < 1e-5);
    assert!((ray_br.origin[1] - (-1.0)).abs() < 1e-5);
    assert!(ray_br.origin[2].abs() < 1e-5);
    assert!((ray_br.direction[2] - 1.0).abs() < 1e-5);
}

#[test]
fn test_normalized_direction_length() {
    let view_proj = create_test_view_proj();
    let ray = screen_to_world_ray(250.0, 120.0, 800.0, 600.0, &view_proj);

    let dx = ray.direction[0];
    let dy = ray.direction[1];
    let dz = ray.direction[2];
    
    let length = (dx * dx + dy * dy + dz * dz).sqrt();
    assert!((length - 1.0).abs() < 1e-5, "Direction vector must be normalized, length was {}", length);
}

#[test]
fn test_z_zero_floor_intersection() {
    let view_proj = create_test_view_proj();
    // Center of screen
    let ray = screen_to_world_ray(400.0, 300.0, 800.0, 600.0, &view_proj);

    // Ray equation: P(t) = Origin + t * Direction
    // Z = 0 -> Origin.Z + t * Direction.Z = 0 -> t = -Origin.Z / Direction.Z
    let t = -ray.origin[2] / ray.direction[2];
    
    let intersect_x = ray.origin[0] + t * ray.direction[0];
    let intersect_y = ray.origin[1] + t * ray.direction[1];
    let intersect_z = ray.origin[2] + t * ray.direction[2];

    // Camera is looking at 0,0,0, so center screen should hit close to 0,0,0
    assert!(intersect_x.abs() < 0.1);
    assert!(intersect_y.abs() < 0.1);
    assert!(intersect_z.abs() < 1e-5);
}

#[test]
fn test_edge_cases_zero_dimensions() {
    let view_proj = create_identity_view_proj();
    
    // Width = 0
    let ray_w0 = screen_to_world_ray(100.0, 100.0, 0.0, 600.0, &view_proj);
    assert_eq!(ray_w0.origin, [0.0, 0.0, 0.0]);
    assert_eq!(ray_w0.direction, [0.0, 0.0, 1.0]);

    // Height = 0
    let ray_h0 = screen_to_world_ray(100.0, 100.0, 800.0, 0.0, &view_proj);
    assert_eq!(ray_h0.origin, [0.0, 0.0, 0.0]);
    assert_eq!(ray_h0.direction, [0.0, 0.0, 1.0]);
    
    // Negative dimensions
    let ray_neg = screen_to_world_ray(100.0, 100.0, -800.0, -600.0, &view_proj);
    assert_eq!(ray_neg.origin, [0.0, 0.0, 0.0]);
    assert_eq!(ray_neg.direction, [0.0, 0.0, 1.0]);
}

#[test]
fn test_edge_cases_out_of_bounds_screen() {
    let view_proj = create_identity_view_proj();
    
    // Screen coords out of bounds (e.g. -100, -100) should still compute a ray mathematically
    let ray = screen_to_world_ray(-400.0, -300.0, 800.0, 600.0, &view_proj);
    
    // NDC: x = 2*(-400)/800 - 1 = -1 - 1 = -2
    // NDC: y = 1 - 2*(-300)/600 = 1 - (-1) = 2
    assert!((ray.origin[0] - (-2.0)).abs() < 1e-5);
    assert!((ray.origin[1] - 2.0).abs() < 1e-5);
    assert!(ray.origin[2].abs() < 1e-5);
    
    let dx = ray.direction[0];
    let dy = ray.direction[1];
    let dz = ray.direction[2];
    let length = (dx * dx + dy * dy + dz * dz).sqrt();
    assert!((length - 1.0).abs() < 1e-5);
}
