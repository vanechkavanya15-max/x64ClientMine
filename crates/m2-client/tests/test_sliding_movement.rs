use glam::Vec3;
use m2_client::engine_sliding_collision_movement::{
    calculate_aabb_collision_normal, calculate_cylinder_collision_normal,
    calculate_sliding_movement, calculate_terrain_tile_collision_normal,
    GC_F_ACTOR_SLIDE_MOVE_SPEED,
};

#[test]
fn test_sliding_into_wall_45_degrees() {
    let cur_pos = Vec3::new(0.0, 0.0, 0.0);
    let velocity = Vec3::new(3.0, 3.0, 0.0); // Moving diagonally
    let normal = Vec3::new(-1.0, 0.0, 0.0); // Hitting a wall facing -x

    let (new_pos, new_velocity) = calculate_sliding_movement(cur_pos, velocity, normal);
    
    // We expect the x velocity to be zeroed out by the normal, leaving only y
    assert_eq!(new_velocity.x, 0.0);
    assert_eq!(new_velocity.y, 3.0);
    assert_eq!(new_velocity.z, 0.0);
    
    assert_eq!(new_pos.x, 0.0);
    assert_eq!(new_pos.y, 3.0);
    assert_eq!(new_pos.z, 0.0);
}

#[test]
fn test_running_parallel_to_wall() {
    let cur_pos = Vec3::new(0.0, 0.0, 0.0);
    let velocity = Vec3::new(0.0, 4.0, 0.0); // Moving along y
    let normal = Vec3::new(-1.0, 0.0, 0.0); // Wall is facing -x, so we are parallel

    let (new_pos, new_velocity) = calculate_sliding_movement(cur_pos, velocity, normal);
    
    // Velocity should be untouched
    assert_eq!(new_velocity.x, 0.0);
    assert_eq!(new_velocity.y, 4.0);
    assert_eq!(new_velocity.z, 0.0);
    
    assert_eq!(new_pos.x, 0.0);
    assert_eq!(new_pos.y, 4.0);
    assert_eq!(new_pos.z, 0.0);
}

#[test]
fn test_speed_clamping() {
    let cur_pos = Vec3::new(0.0, 0.0, 0.0);
    let velocity = Vec3::new(10.0, 0.0, 0.0); // Moving very fast along x
    let normal = Vec3::new(0.0, -1.0, 0.0); // Hitting wall facing -y

    let (new_pos, new_velocity) = calculate_sliding_movement(cur_pos, velocity, normal);
    
    // Velocity should be clamped to 5.0
    assert_eq!(new_velocity.x, GC_F_ACTOR_SLIDE_MOVE_SPEED);
    assert_eq!(new_velocity.y, 0.0);
    assert_eq!(new_velocity.z, 0.0);
    
    assert_eq!(new_pos.x, GC_F_ACTOR_SLIDE_MOVE_SPEED);
    assert_eq!(new_pos.y, 0.0);
    assert_eq!(new_pos.z, 0.0);
}

#[test]
fn test_tree_cylinder_collision() {
    let cur_pos = Vec3::new(0.0, 0.0, 0.0);
    let velocity = Vec3::new(4.0, 3.0, 0.0);
    
    // Normal of a cylinder (say, tree is at 2, 2, we are at 0, 0, normal points roughly -1, -1 normalized)
    let normal = Vec3::new(-1.0, -1.0, 0.0).normalize();

    let (_new_pos, new_velocity) = calculate_sliding_movement(cur_pos, velocity, normal);
    
    // Since normal is (-0.707, -0.707, 0)
    // dot = (4 * -0.707) + (3 * -0.707) = -7.07
    // slide = V - normal * dot
    // x = 4 - (-0.707 * -7.07) = 4 - 5 = -1
    // y = 3 - (-0.707 * -7.07) = 3 - 5 = -2
    // Since tree normal opposes our velocity, we are reflected/sliding tangentially.
    // The main point is checking our logic holds for arbitrary normal vectors.
    let expected_dot = velocity.dot(normal);
    let expected_slide = velocity - normal * expected_dot;

    // Use epsilon for float comparison
    assert!((new_velocity.x - expected_slide.x).abs() < 1e-5);
    assert!((new_velocity.y - expected_slide.y).abs() < 1e-5);
    assert!((new_velocity.z - expected_slide.z).abs() < 1e-5);
}

#[test]
fn test_calculate_cylinder_collision_normal() {
    let player = Vec3::new(10.0, 10.0, 0.0);
    let tree_center = Vec3::new(12.0, 10.0, 0.0); // player is to the left (-x direction)

    let normal = calculate_cylinder_collision_normal(player, tree_center);
    assert_eq!(normal, Vec3::new(-1.0, 0.0, 0.0));

    let player2 = Vec3::new(10.0, 12.0, 0.0);
    let tree_center2 = Vec3::new(10.0, 10.0, 0.0); // player is above (+y direction)
    let normal2 = calculate_cylinder_collision_normal(player2, tree_center2);
    assert_eq!(normal2, Vec3::new(0.0, 1.0, 0.0));
}

#[test]
fn test_calculate_aabb_collision_normal() {
    let aabb_min = Vec3::new(0.0, 0.0, 0.0);
    let aabb_max = Vec3::new(10.0, 10.0, 0.0);

    // Hit the right wall (+x)
    let p_right = Vec3::new(11.0, 5.0, 0.0);
    let normal_right = calculate_aabb_collision_normal(p_right, aabb_min, aabb_max);
    assert_eq!(normal_right, Vec3::new(1.0, 0.0, 0.0));

    // Hit the left wall (-x)
    let p_left = Vec3::new(-1.0, 5.0, 0.0);
    let normal_left = calculate_aabb_collision_normal(p_left, aabb_min, aabb_max);
    assert_eq!(normal_left, Vec3::new(-1.0, 0.0, 0.0));

    // Hit the top wall (+y)
    let p_top = Vec3::new(5.0, 11.0, 0.0);
    let normal_top = calculate_aabb_collision_normal(p_top, aabb_min, aabb_max);
    assert_eq!(normal_top, Vec3::new(0.0, 1.0, 0.0));

    // Hit the bottom wall (-y)
    let p_bottom = Vec3::new(5.0, -1.0, 0.0);
    let normal_bottom = calculate_aabb_collision_normal(p_bottom, aabb_min, aabb_max);
    assert_eq!(normal_bottom, Vec3::new(0.0, -1.0, 0.0));
}

#[test]
fn test_calculate_terrain_tile_collision_normal() {
    let tile_center = Vec3::new(100.0, 100.0, 0.0);
    let tile_size = 50.0;

    // Tile is from 75 to 125 in both X and Y.
    // Hitting from right
    let p_right = Vec3::new(126.0, 100.0, 0.0);
    let normal_right = calculate_terrain_tile_collision_normal(p_right, tile_center, tile_size);
    assert_eq!(normal_right, Vec3::new(1.0, 0.0, 0.0));

    // Hitting from bottom
    let p_bottom = Vec3::new(100.0, 74.0, 0.0);
    let normal_bottom = calculate_terrain_tile_collision_normal(p_bottom, tile_center, tile_size);
    assert_eq!(normal_bottom, Vec3::new(0.0, -1.0, 0.0));
}

#[test]
fn test_run_45_deg_into_wall_sliding() {
    let cur_pos = Vec3::new(0.0, 0.0, 0.0);
    let velocity = Vec3::new(4.0, 4.0, 0.0); // Running up-right

    // Wall (building) from x = 1 to 5, y = -10 to 10
    // We are at 0, 0, so we hit the left wall (-x)
    let aabb_min = Vec3::new(1.0, -10.0, 0.0);
    let aabb_max = Vec3::new(5.0, 10.0, 0.0);

    let normal = calculate_aabb_collision_normal(cur_pos, aabb_min, aabb_max);
    assert_eq!(normal, Vec3::new(-1.0, 0.0, 0.0));

    let (new_pos, new_velocity) = calculate_sliding_movement(cur_pos, velocity, normal);

    // Since normal is (-1, 0, 0), the x velocity should be blocked, leaving only y velocity.
    // However, since original speed is sqrt(16+16) = 5.6568, it gets clamped to 5.0 initially:
    // velocity before projection: 5.0 / 5.6568 * 4.0 = 3.535534...
    assert!((new_velocity.x - 0.0).abs() < 1e-5);
    assert!((new_velocity.y - 3.535534).abs() < 1e-5);
    assert!((new_velocity.z - 0.0).abs() < 1e-5);

    assert!((new_pos.x - 0.0).abs() < 1e-5);
    assert!((new_pos.y - 3.535534).abs() < 1e-5);
    assert!((new_pos.z - 0.0).abs() < 1e-5);
}
