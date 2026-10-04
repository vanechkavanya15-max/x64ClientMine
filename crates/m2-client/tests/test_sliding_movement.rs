use glam::Vec3;
use m2_client::engine_sliding_collision_movement::{calculate_sliding_movement, GC_F_ACTOR_SLIDE_MOVE_SPEED};

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
