#[path = "../src/minimap_navigation.rs"]
mod minimap_navigation;
use minimap_navigation::MinimapNavigation;

#[test]
fn test_world_to_minimap_center() {
    let nav = MinimapNavigation::new(1000.0, 1000.0);
    let (mini_x, mini_y) = nav.world_to_minimap(500.0, 500.0);
    assert_eq!(mini_x, 128.0);
    assert_eq!(mini_y, 128.0);
}

#[test]
fn test_world_to_minimap_corners() {
    let nav = MinimapNavigation::new(1000.0, 1000.0);
    
    // Top-left corner
    let (mini_x, mini_y) = nav.world_to_minimap(0.0, 0.0);
    assert_eq!(mini_x, 0.0);
    assert_eq!(mini_y, 0.0);
    
    // Top-right corner
    let (mini_x, mini_y) = nav.world_to_minimap(1000.0, 0.0);
    assert_eq!(mini_x, 256.0);
    assert_eq!(mini_y, 0.0);
    
    // Bottom-left corner
    let (mini_x, mini_y) = nav.world_to_minimap(0.0, 1000.0);
    assert_eq!(mini_x, 0.0);
    assert_eq!(mini_y, 256.0);
    
    // Bottom-right corner
    let (mini_x, mini_y) = nav.world_to_minimap(1000.0, 1000.0);
    assert_eq!(mini_x, 256.0);
    assert_eq!(mini_y, 256.0);
}

#[test]
fn test_world_to_minimap_edge_cases() {
    // Zero dimensions
    let nav = MinimapNavigation::new(0.0, 0.0);
    let (mini_x, mini_y) = nav.world_to_minimap(500.0, 500.0);
    assert_eq!(mini_x, 0.0);
    assert_eq!(mini_y, 0.0);

    // Negative coordinates (if allowed by logic, it scales down proportionally)
    let nav = MinimapNavigation::new(1000.0, 1000.0);
    let (mini_x, mini_y) = nav.world_to_minimap(-500.0, -500.0);
    assert_eq!(mini_x, -128.0);
    assert_eq!(mini_y, -128.0);
    
    // Out of bounds positive
    let nav = MinimapNavigation::new(1000.0, 1000.0);
    let (mini_x, mini_y) = nav.world_to_minimap(2000.0, 2000.0);
    assert_eq!(mini_x, 512.0);
    assert_eq!(mini_y, 512.0);
}

#[test]
fn test_calculate_arrow_rotation() {
    // Normal cases
    let rotation = MinimapNavigation::calculate_arrow_rotation(0.0);
    assert_eq!(rotation, 180.0); // 540 - 0 = 540 -> 180
    
    let rotation = MinimapNavigation::calculate_arrow_rotation(90.0);
    assert_eq!(rotation, 90.0); // 540 - 90 = 450 -> 90
    
    let rotation = MinimapNavigation::calculate_arrow_rotation(180.0);
    assert_eq!(rotation, 0.0); // 540 - 180 = 360 -> 0

    let rotation = MinimapNavigation::calculate_arrow_rotation(270.0);
    assert_eq!(rotation, 270.0); // 540 - 270 = 270 -> 270
    
    let rotation = MinimapNavigation::calculate_arrow_rotation(360.0);
    assert_eq!(rotation, 180.0); // 540 - 360 = 180 -> 180
    
    // Greater than 360
    let rotation = MinimapNavigation::calculate_arrow_rotation(720.0);
    assert_eq!(rotation, 180.0); // 540 - 720 = -180 -> 180
    
    let rotation = MinimapNavigation::calculate_arrow_rotation(450.0);
    assert_eq!(rotation, 90.0); // 540 - 450 = 90 -> 90

    // Negative angles
    let rotation = MinimapNavigation::calculate_arrow_rotation(-90.0);
    assert_eq!(rotation, 270.0); // 540 - (-90) = 630 -> 270
    
    let rotation = MinimapNavigation::calculate_arrow_rotation(-360.0);
    assert_eq!(rotation, 180.0); // 540 - (-360) = 900 -> 180
    
    let rotation = MinimapNavigation::calculate_arrow_rotation(-450.0);
    assert_eq!(rotation, 270.0); // 540 - (-450) = 990 -> 270
}
