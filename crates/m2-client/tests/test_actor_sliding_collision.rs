use glam::Vec3;
use m2_client::actor_sliding_collision::adjust_collision_movement;

#[test]
fn test_no_movement() {
    let cur_pos = Vec3::new(10.0, 10.0, 10.0);
    let mut delta_move = Vec3::ZERO;
    let normal = Vec3::new(1.0, 0.0, 0.0);

    let new_pos = adjust_collision_movement(cur_pos, &mut delta_move, normal, 5.0);

    assert_eq!(new_pos, cur_pos);
    assert_eq!(delta_move, Vec3::ZERO);
}

#[test]
fn test_slide_against_wall() {
    let cur_pos = Vec3::new(10.0, 10.0, 10.0);
    // Moving diagonally against a wall facing -X
    let mut delta_move = Vec3::new(2.0, 2.0, 0.0);
    let normal = Vec3::new(-1.0, 0.0, 0.0); // Wall normal faces left

    let new_pos = adjust_collision_movement(cur_pos, &mut delta_move, normal, 5.0);

    // dot = (2.0)*(-1.0) = -2.0
    // slide_delta = (2,2,0) - (-1,0,0)*(-2) = (2,2,0) - (2,0,0) = (0,2,0)
    assert_eq!(new_pos, Vec3::new(10.0, 12.0, 10.0));
    assert_eq!(delta_move, Vec3::new(0.0, 2.0, 0.0));
}

#[test]
fn test_slide_with_clamp() {
    let cur_pos = Vec3::new(10.0, 10.0, 10.0);
    // Move very fast diagonally against a wall facing -X
    let mut delta_move = Vec3::new(10.0, 10.0, 0.0);
    let normal = Vec3::new(-1.0, 0.0, 0.0); 

    let new_pos = adjust_collision_movement(cur_pos, &mut delta_move, normal, 5.0);

    // Initial movement length is sqrt(200) ~ 14.14, clamped to 5.0
    // clamped = (10, 10, 0) * 5 / 14.14 = (3.535, 3.535, 0)
    // dot = (3.535, 3.535, 0) dot (-1, 0, 0) = -3.535
    // slide_delta = (3.535, 3.535, 0) - (-1, 0, 0) * -3.535 = (0, 3.535, 0)
    // length is 3.535 < 5.0, so no second clamp
    
    let expected_delta = Vec3::new(0.0, 10.0, 0.0).normalize() * (5.0 / (2.0_f32).sqrt());
    
    assert!((delta_move.x - expected_delta.x).abs() < 1e-4);
    assert!((delta_move.y - expected_delta.y).abs() < 1e-4);
    assert!((delta_move.z - expected_delta.z).abs() < 1e-4);

    let expected_pos = cur_pos + expected_delta;
    assert!((new_pos.x - expected_pos.x).abs() < 1e-4);
    assert!((new_pos.y - expected_pos.y).abs() < 1e-4);
    assert!((new_pos.z - expected_pos.z).abs() < 1e-4);
}

#[test]
fn test_perpendicular_collision() {
    let cur_pos = Vec3::new(0.0, 0.0, 0.0);
    // Hitting a wall straight on
    let mut delta_move = Vec3::new(5.0, 0.0, 0.0);
    let normal = Vec3::new(-1.0, 0.0, 0.0); 

    let new_pos = adjust_collision_movement(cur_pos, &mut delta_move, normal, 5.0);
    
    // dot = 5.0 * -1.0 = -5.0
    // slide_delta = (5,0,0) - (-1,0,0)*(-5) = (0,0,0)
    
    assert_eq!(new_pos, cur_pos);
    assert_eq!(delta_move, Vec3::ZERO);
}
