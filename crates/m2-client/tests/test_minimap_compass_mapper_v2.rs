use m2_client::minimap_compass_mapper_v2::*;

#[test]
fn test_target_inside_radius() {
    // 200 units (1 cellscale) to the right
    let pos = m2_minimap_world_to_screen(0.0, 0.0, 200.0, 0.0, 50.0);
    assert!((pos.x - 1.0).abs() < f32::EPSILON);
    assert!((pos.y - 0.0).abs() < f32::EPSILON);
}

#[test]
fn test_target_outside_radius() {
    // 20000 units (100 cellscale) to the right
    let pos = m2_minimap_world_to_screen(0.0, 0.0, 20000.0, 0.0, 50.0);
    // Radius is 50.0, so it should be clamped to 50.0
    assert!((pos.x - 50.0).abs() < f32::EPSILON);
    assert!((pos.y - 0.0).abs() < f32::EPSILON);
}

#[test]
fn test_target_outside_radius_diagonal() {
    // Large distance diagonally
    let pos = m2_minimap_world_to_screen(0.0, 0.0, 20000.0, 20000.0, 50.0);
    // Distance should be exactly 50.0 (radius)
    let dist = (pos.x * pos.x + pos.y * pos.y).sqrt();
    assert!((dist - 50.0).abs() < 1e-4);
}

#[test]
fn test_target_exactly_on_radius() {
    // 10000 units (50 cellscale) to the right. Radius is exactly 50.0
    let pos = m2_minimap_world_to_screen(0.0, 0.0, 10000.0, 0.0, 50.0);
    assert!((pos.x - 50.0).abs() < f32::EPSILON);
    assert!((pos.y - 0.0).abs() < f32::EPSILON);
}

#[test]
fn test_compass_rotation() {
    // Raw camera rotation mapping
    let rot = MinimapCompassMapperV2::calculate_compass_rotation(90.0);
    assert!((rot - 90.0).abs() < f32::EPSILON);
}

#[test]
fn test_pin_types() {
    // Ensure all pin types are compiled and available (C++ enums mapped)
    let p1 = PinType::Player;
    let p2 = PinType::Monster;
    let p3 = PinType::Metin;
    let p4 = PinType::Portal;
    let p5 = PinType::PartyMember;
    
    assert_ne!(p1, p2);
    assert_ne!(p2, p3);
    assert_ne!(p3, p4);
    assert_ne!(p4, p5);
}

#[test]
fn test_zero_distance() {
    // Same coordinate
    let pos = m2_minimap_world_to_screen(1234.5, 5678.9, 1234.5, 5678.9, 50.0);
    assert!((pos.x - 0.0).abs() < f32::EPSILON);
    assert!((pos.y - 0.0).abs() < f32::EPSILON);
}
