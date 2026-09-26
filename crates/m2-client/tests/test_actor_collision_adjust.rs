use m2_client::actor_collision_adjust::adjust_collision;

const BODY_RADIUS: f32 = 50.0;
const MIN_DISTANCE: f32 = 2.0 * BODY_RADIUS; // 100.0

#[test]
fn test_no_interference_when_far_away() {
    let actor_pos = (200.0, 200.0);
    let other_actor_pos = (0.0, 0.0);
    // Distance is sqrt(40000 + 40000) = 282.84, which is > 100.0
    assert_eq!(adjust_collision(actor_pos, other_actor_pos), None);

    let actor_pos_edge = (100.0, 0.0);
    let other_actor_pos_edge = (0.0, 0.0);
    // Distance is exactly 100.0
    assert_eq!(adjust_collision(actor_pos_edge, other_actor_pos_edge), None);
}

#[test]
fn test_collision_detection_and_push_back() {
    let actor_pos = (60.0, 0.0);
    let other_actor_pos = (0.0, 0.0);
    // Distance is 60.0, push back needed by 40.0
    let result = adjust_collision(actor_pos, other_actor_pos);
    assert!(result.is_some());
    let (new_x, new_y) = result.unwrap();
    
    // Assert new position
    assert!((new_x - 100.0).abs() < 1e-4);
    assert!((new_y - 0.0).abs() < 1e-4);
    
    // Assert distance between new position and other actor is exactly MIN_DISTANCE
    let new_distance = ((new_x - other_actor_pos.0).powi(2) + (new_y - other_actor_pos.1).powi(2)).sqrt();
    assert!((new_distance - MIN_DISTANCE).abs() < 1e-4);
}

#[test]
fn test_collision_detection_and_push_back_diagonal() {
    let actor_pos = (30.0, 40.0);
    let other_actor_pos = (0.0, 0.0);
    // Distance is sqrt(900 + 1600) = 50.0, push back needed by 50.0
    // Vector is (30/50, 40/50) = (0.6, 0.8)
    // New pos should be 30 + 0.6 * 50 = 60.0, 40 + 0.8 * 50 = 80.0
    let result = adjust_collision(actor_pos, other_actor_pos);
    assert!(result.is_some());
    let (new_x, new_y) = result.unwrap();
    
    // Assert new position
    assert!((new_x - 60.0).abs() < 1e-4);
    assert!((new_y - 80.0).abs() < 1e-4);
    
    // Assert distance between new position and other actor is exactly MIN_DISTANCE
    let new_distance = ((new_x - other_actor_pos.0).powi(2) + (new_y - other_actor_pos.1).powi(2)).sqrt();
    assert!((new_distance - MIN_DISTANCE).abs() < 1e-4);
}

#[test]
fn test_collision_identical_positions() {
    let actor_pos = (10.0, 10.0);
    let other_actor_pos = (10.0, 10.0);
    // Distance is 0.0. Needs to push exactly MIN_DISTANCE in default direction (1.0, 0.0)
    let result = adjust_collision(actor_pos, other_actor_pos);
    assert!(result.is_some());
    let (new_x, new_y) = result.unwrap();
    
    assert!((new_x - (10.0 + MIN_DISTANCE)).abs() < 1e-4);
    assert!((new_y - 10.0).abs() < 1e-4);

    let new_distance = ((new_x - other_actor_pos.0).powi(2) + (new_y - other_actor_pos.1).powi(2)).sqrt();
    assert!((new_distance - MIN_DISTANCE).abs() < 1e-4);
}

#[test]
fn test_collision_negative_coordinates() {
    let actor_pos = (-20.0, -20.0);
    let other_actor_pos = (-50.0, -50.0);
    // Distance is sqrt(30^2 + 30^2) = 42.4264
    // Should be pushed away
    let result = adjust_collision(actor_pos, other_actor_pos);
    assert!(result.is_some());
    let (new_x, new_y) = result.unwrap();

    let new_distance = ((new_x - other_actor_pos.0).powi(2) + (new_y - other_actor_pos.1).powi(2)).sqrt();
    assert!((new_distance - MIN_DISTANCE).abs() < 1e-4);
}
