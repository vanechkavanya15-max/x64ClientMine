use m2_client::terrain_camera_collision::*;

fn mock_terrain_height(_x: f32, _y: f32) -> f32 {
    100.0 // Teren jest na wysokosci Z = 100
}

#[test]
fn test_no_collision_high_above_ground() {
    let target_pos = (0.0, 0.0, 100.0);
    let eye_pos = (100.0, 100.0, 200.0);
    
    // Z terenu = 100.0, MIN_CAMERA_GROUND_CLEARANCE = 50.0. Min_z = 150.0.
    // Kamara jest na Z = 200.0. Kolizji brak.
    let new_eye = resolve_terrain_camera_collision(target_pos, eye_pos, MIN_CAMERA_GROUND_CLEARANCE, mock_terrain_height);
    
    assert_eq!(new_eye, eye_pos);
}

#[test]
fn test_elevate_camera_when_below_margin() {
    let target_pos = (0.0, 0.0, 100.0);
    let eye_pos = (100.0, 100.0, 120.0);
    
    // Teren Z = 100.0. Min_z = 150.0. Kamera jest na Z=120.0 (za nisko).
    let new_eye = resolve_terrain_camera_collision(target_pos, eye_pos, MIN_CAMERA_GROUND_CLEARANCE, mock_terrain_height);
    
    // Z powinnien byc na 150.0
    assert_eq!(new_eye.2, 150.0);
    
    // Kat po podniesieniu kamery (z Z=150.0 i celem na Z=100.0 -> dz = 50.0)
    // d2d = sqrt(100^2 + 100^2) = 141.42
    // tan(pitch) = 50 / 141.42 = 0.35, co jest < 5.67, wiec zoom in (skracanie dystansu) sie nie wlaczy.
    assert_eq!(new_eye.0, 100.0);
    assert_eq!(new_eye.1, 100.0);
}

#[test]
fn test_steep_slope_zoom_in() {
    let target_pos = (0.0, 0.0, 100.0);
    // Skracanie dystansu (zoom in)
    // d2d = 5.0, dz = 50.0. dz/d2d = 10.0 > 5.671
    let eye_pos = (5.0, 0.0, 120.0);
    
    let new_eye = resolve_terrain_camera_collision(target_pos, eye_pos, MIN_CAMERA_GROUND_CLEARANCE, mock_terrain_height);
    
    assert_eq!(new_eye.2, 150.0);
    // Nowa pozycja X powinna byc blizej celu, poniewaz kat jest zbyt stromy
    // dist2d z 5 powinno zmalec. scale < 1.0.
    assert!(new_eye.0 < 5.0 && new_eye.0 > 0.0);
    assert_eq!(new_eye.1, 0.0);
}

#[test]
fn test_looking_vector_preserved() {
    let target_pos = (0.0, 0.0, 100.0);
    let eye_pos = (10.0, 10.0, 100.0);
    let new_eye = resolve_terrain_camera_collision(target_pos, eye_pos, MIN_CAMERA_GROUND_CLEARANCE, mock_terrain_height);
    assert_eq!(new_eye.2, 150.0);
}

#[test]
fn test_edge_case_zero_2d_distance() {
    let target_pos = (0.0, 0.0, 100.0);
    let eye_pos = (0.0, 0.0, 100.0);
    let new_eye = resolve_terrain_camera_collision(target_pos, eye_pos, MIN_CAMERA_GROUND_CLEARANCE, mock_terrain_height);
    assert_eq!(new_eye.2, 150.0);
    assert_eq!(new_eye.0, 0.0);
    assert_eq!(new_eye.1, 0.0);
}
