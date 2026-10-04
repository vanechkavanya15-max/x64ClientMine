use m2_client::ui_private_shop_overhead_sign::{PrivateShopSignManager, ShopSignAction};
use glam::{Mat4, Vec3};

#[test]
fn test_sign_lifecycle_add_remove() {
    let mut manager = PrivateShopSignManager::new();

    // Verify empty state
    assert!(manager.get_sign(1).is_none());

    // Add a sign
    manager.add_sign(1, "Test Shop".to_string(), Vec3::new(10.0, 20.0, 30.0), 10.0);
    
    let sign = manager.get_sign(1).unwrap();
    assert_eq!(sign.vid, 1);
    assert_eq!(sign.title, "Test Shop");
    assert_eq!(sign.position, Vec3::new(10.0, 20.0, 30.0));
    assert_eq!(sign.height_offset, 10.0);

    // Update position
    assert!(manager.update_sign_position(1, Vec3::new(50.0, 50.0, 50.0)).is_ok());
    assert_eq!(manager.get_sign(1).unwrap().position, Vec3::new(50.0, 50.0, 50.0));

    // Remove the sign
    manager.remove_sign(1);
    assert!(manager.get_sign(1).is_none());

    // Clear all
    manager.add_sign(2, "Shop 2".to_string(), Vec3::ZERO, 0.0);
    manager.clear();
    assert!(manager.get_sign(2).is_none());
}

#[test]
fn test_screen_projection() {
    let mut manager = PrivateShopSignManager::new();
    
    // Add a sign at the origin
    manager.add_sign(1, "S".to_string(), Vec3::ZERO, 0.0);

    // Create an identity projection matrix
    let view_proj = Mat4::IDENTITY;

    // Viewport dimensions
    let viewport_w = 800.0;
    let viewport_h = 600.0;
    
    // Font settings
    let char_w = 10.0;
    let char_h = 20.0;
    let pad_x = 5.0;
    let pad_y = 5.0;

    manager.update_render_states(
        &view_proj,
        viewport_w,
        viewport_h,
        char_w,
        char_h,
        pad_x,
        pad_y,
    );

    let states = manager.get_render_states();
    assert_eq!(states.len(), 1);
    
    let state = &states[0];
    assert_eq!(state.vid, 1);
    
    // Origin projected to NDC via Identity matrix: (0, 0, 0, 1) -> NDC (0, 0, 0)
    // Screen X = (0 + 1) * 0.5 * 800 = 400
    // Screen Y = (1 - 0) * 0.5 * 600 = 300
    assert_eq!(state.screen_x, 400.0);
    assert_eq!(state.screen_y, 300.0);
    
    // Width = 1 char * 10 + padding (5 * 2) = 20
    assert_eq!(state.width, 20.0);
    
    // Height = char height 20 + padding (5 * 2) = 30
    assert_eq!(state.height, 30.0);
}

#[test]
fn test_single_hit_testing() {
    let mut manager = PrivateShopSignManager::new();
    manager.add_sign(1, "S".to_string(), Vec3::ZERO, 0.0);

    let view_proj = Mat4::IDENTITY;
    
    // Setup matching test_screen_projection: Center is at X=400, Y=300
    // Width is 20, Height is 30.
    // Box: Left=390, Right=410, Top=270, Bottom=300
    manager.update_render_states(&view_proj, 800.0, 600.0, 10.0, 20.0, 5.0, 5.0);

    // Hit test inside
    assert_eq!(manager.hit_test(400.0, 285.0), Some(1));
    assert_eq!(manager.handle_mouse_click(400.0, 285.0), ShopSignAction::OpenShop(1));

    // Hit test exact corners
    assert_eq!(manager.hit_test(390.0, 270.0), Some(1));
    assert_eq!(manager.hit_test(410.0, 300.0), Some(1));

    // Hit test outside
    assert_eq!(manager.hit_test(389.0, 285.0), None);
    assert_eq!(manager.hit_test(411.0, 285.0), None);
    assert_eq!(manager.hit_test(400.0, 269.0), None);
    assert_eq!(manager.hit_test(400.0, 301.0), None);
    assert_eq!(manager.handle_mouse_click(10.0, 10.0), ShopSignAction::None);
}

#[test]
fn test_overlapping_signs_z_depth_sorting() {
    let mut manager = PrivateShopSignManager::new();

    // Sign 1: Far away (Z = 100)
    manager.add_sign(1, "Far".to_string(), Vec3::new(0.0, 0.0, 100.0), 0.0);
    // Sign 2: Close (Z = 50)
    manager.add_sign(2, "Near".to_string(), Vec3::new(0.0, 0.0, 50.0), 0.0);

    // Simple orthographic-like projection matrix
    // We want smaller Z to result in smaller NDC Z.
    // Matrix * (0, 0, Z, 1) -> (0, 0, Z, 1)
    let view_proj = Mat4::IDENTITY;

    manager.update_render_states(&view_proj, 800.0, 600.0, 10.0, 20.0, 5.0, 5.0);

    // The logic inside `update_render_states` sorts by `screen_z` ascending.
    // For Identity matrix, `screen_z` = `world_z`.
    // Z=50 (Sign 2) should appear before Z=100 (Sign 1).
    let states = manager.get_render_states();
    assert_eq!(states.len(), 2);
    assert_eq!(states[0].vid, 2);
    assert_eq!(states[1].vid, 1);

    // Both signs will project to center X=400, Y=300 and have overlapping bounding boxes.
    // Width of "Near" (4 chars) = 4*10 + 10 = 50. Center=400. Box: 375-425, 270-300.
    // Width of "Far"  (3 chars) = 3*10 + 10 = 40. Center=400. Box: 380-420, 270-300.
    
    // Clicking at the center should hit the closest sign (Sign 2).
    assert_eq!(manager.hit_test(400.0, 285.0), Some(2));
}
