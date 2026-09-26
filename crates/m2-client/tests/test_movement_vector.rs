#[path = "../src/movement_vector.rs"]
pub mod movement_vector;

use movement_vector::MovementVector;

#[test]
fn test_standard_displacement_over_time() {
    let mut mv = MovementVector::new(0.0, 0.0);
    mv.set_speed(100.0);
    mv.set_destination(100.0, 0.0);
    
    // Half a second at 100 units/sec should move 50 units.
    mv.update(0.5);
    
    assert_eq!(mv.cur_x, 50.0);
    assert_eq!(mv.cur_y, 0.0);
    assert!(mv.is_moving);
}

#[test]
fn test_stops_exactly_at_target() {
    let mut mv = MovementVector::new(0.0, 0.0);
    mv.set_speed(100.0);
    mv.set_destination(100.0, 0.0);
    
    // 2 seconds at 100 units/sec should overshoot, but we should stop at exactly 100.0
    mv.update(2.0);
    
    assert_eq!(mv.cur_x, 100.0);
    assert_eq!(mv.cur_y, 0.0);
    assert!(!mv.is_moving); // Movement should be completed
}

#[test]
fn test_displacement_scaling_with_doubled_speed() {
    let mut mv1 = MovementVector::new(0.0, 0.0);
    mv1.set_speed(100.0);
    mv1.set_destination(200.0, 0.0);
    
    let mut mv2 = MovementVector::new(0.0, 0.0);
    mv2.set_speed(200.0);
    mv2.set_destination(200.0, 0.0);
    
    mv1.update(0.5); // Moves 50
    mv2.update(0.5); // Moves 100
    
    assert_eq!(mv1.cur_x, 50.0);
    assert_eq!(mv2.cur_x, 100.0);
    assert_eq!(mv2.cur_x, mv1.cur_x * 2.0);
}

#[test]
fn test_diagonal_movement() {
    let mut mv = MovementVector::new(0.0, 0.0);
    mv.set_speed(100.0);
    // Destination at (300, 400), distance is 500
    mv.set_destination(300.0, 400.0);
    
    mv.update(1.0); // Move 100 units
    // Should move 1/5th of the way to the destination
    // dx/dist = 300/500 = 0.6, so x should be 0.6 * 100 = 60
    // dy/dist = 400/500 = 0.8, so y should be 0.8 * 100 = 80
    assert!((mv.cur_x - 60.0).abs() < f32::EPSILON * 100.0);
    assert!((mv.cur_y - 80.0).abs() < f32::EPSILON * 100.0);
    assert!(mv.is_moving);
}

#[test]
fn test_zero_distance() {
    let mut mv = MovementVector::new(10.0, 10.0);
    mv.set_speed(100.0);
    mv.set_destination(10.0, 10.0); // Already at destination
    
    mv.update(1.0);
    
    assert_eq!(mv.cur_x, 10.0);
    assert_eq!(mv.cur_y, 10.0);
    assert!(!mv.is_moving);
}

#[test]
fn test_negative_coordinates() {
    let mut mv = MovementVector::new(-50.0, -50.0);
    mv.set_speed(50.0);
    mv.set_destination(-100.0, -50.0);
    
    mv.update(0.5); // Move 25 units left
    
    assert_eq!(mv.cur_x, -75.0);
    assert_eq!(mv.cur_y, -50.0);
    assert!(mv.is_moving);
    
    mv.update(0.5); // Move another 25 units left, reaching -100
    assert_eq!(mv.cur_x, -100.0);
    assert_eq!(mv.cur_y, -50.0);
    assert!(!mv.is_moving);
}

#[test]
fn test_float_edge_behavior() {
    let mut mv = MovementVector::new(0.0, 0.0);
    mv.set_speed(100.0);
    mv.set_destination(0.0001, 0.0);
    
    mv.update(0.0000001); // Very tiny step
    
    // Should step towards but not complete if step is smaller than dist
    // step = 100.0 * 0.0000001 = 0.00001
    // dist = 0.0001
    assert!(mv.cur_x > 0.0);
    assert!(mv.cur_x < 0.0001);
    assert!(mv.is_moving);
    
    // Move enough to finish
    mv.update(1.0);
    assert_eq!(mv.cur_x, 0.0001);
    assert!(!mv.is_moving);
}
