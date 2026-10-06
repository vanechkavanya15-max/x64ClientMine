#![allow(non_snake_case)]

#[path = "../src/actor_fly_projectile_tracker_v2.rs"]
mod actor_fly_projectile_tracker_v2;

use actor_fly_projectile_tracker_v2::{
    FlyProjectileTracker, FlyTarget, FlyingData, FlyingStatus,
    square_distance_between_linesegment_and_point,
};

#[test]
fn test_math_square_distance_between_linesegment_and_point() {
    let p1 = (0.0, 0.0, 0.0);
    let p2 = (10.0, 0.0, 0.0);
    
    // Point on the line
    assert_eq!(square_distance_between_linesegment_and_point(p1, p2, (5.0, 0.0, 0.0)), 0.0);
    
    // Point before p1
    assert_eq!(square_distance_between_linesegment_and_point(p1, p2, (-5.0, 0.0, 0.0)), 25.0);
    
    // Point after p2
    assert_eq!(square_distance_between_linesegment_and_point(p1, p2, (15.0, 0.0, 0.0)), 25.0);
    
    // Point above the line segment
    assert_eq!(square_distance_between_linesegment_and_point(p1, p2, (5.0, 5.0, 0.0)), 25.0);
}

#[test]
fn test_ballistic_trajectory_no_homing() {
    let mut tracker = FlyProjectileTracker::new();
    
    let mut data = FlyingData::default();
    data.init_vel = 100.0;
    data.gravity = 10.0; // Gravity pulling down
    data.accel = (0.0, 0.0, 0.0);
    data.range = 500.0;
    
    let start_pos = (0.0, 0.0, 0.0);
    let target = FlyTarget::Position(200.0, 0.0, 0.0);
    
    let id = tracker.create_flying_instance(data, start_pos, target, false);
    let instance = tracker.get_instance_mut(id).unwrap();
    
    // Initial velocity should be towards target (100, 0, 0)
    assert_eq!(instance.velocity.0, 100.0);
    assert_eq!(instance.velocity.1, 0.0);
    assert_eq!(instance.velocity.2, 0.0);
    
    let dt = 1.0;
    let get_target_pos = |_vid| None;
    let get_objects_in_range = |_pos, _radius| vec![];
    
    // Update step 1
    let status = instance.update(dt, &get_target_pos, &get_objects_in_range);
    assert_eq!(status, FlyingStatus::Flying);
    
    // Velocity z should be modified by gravity
    assert_eq!(instance.velocity.2, -10.0);
    
    // Position should be updated
    assert_eq!(instance.pos.0, 100.0);
    assert_eq!(instance.pos.2, -10.0);
}

#[test]
fn test_hit_detection_target_position() {
    let mut tracker = FlyProjectileTracker::new();
    
    let mut data = FlyingData::default();
    data.init_vel = 100.0;
    data.bomb_range = 10.0;
    data.range = 500.0;
    
    let start_pos = (0.0, 0.0, 0.0);
    let target = FlyTarget::Position(150.0, 0.0, 0.0);
    
    let id = tracker.create_flying_instance(data, start_pos, target, true);
    let instance = tracker.get_instance_mut(id).unwrap();
    
    let get_target_pos = |_vid| None;
    let get_objects_in_range = |_pos, _radius| vec![];
    
    // Advance 1 second -> x=100 (dist 50)
    let status1 = instance.update(1.0, &get_target_pos, &get_objects_in_range);
    assert_eq!(status1, FlyingStatus::Flying);
    assert_eq!(instance.target_hitted, false);
    
    // Advance 1 second -> x=200 (passed target at 150)
    // The line segment will go from (100,0,0) to (200,0,0). Target (150,0,0) is on this segment.
    let status2 = instance.update(1.0, &get_target_pos, &get_objects_in_range);
    assert_eq!(status2, FlyingStatus::HitTarget);
    assert_eq!(instance.target_hitted, true);
    assert_eq!(instance.is_alive, false);
}

#[test]
fn test_homing_missile_adjustment() {
    let mut tracker = FlyProjectileTracker::new();
    
    let mut data = FlyingData::default();
    data.init_vel = 100.0;
    data.is_homing = true;
    data.homing_max_angle = 90.0;
    data.homing_start_time = 0.0;
    data.bomb_range = 10.0;
    data.range = 1000.0;
    
    let start_pos = (0.0, 0.0, 0.0);
    // Initially shoot towards X axis
    let target = FlyTarget::Object(1234);
    
    let id = tracker.create_flying_instance(data, start_pos, target, true);
    let instance = tracker.get_instance_mut(id).unwrap();
    
    // Force initial velocity to be (100, 0, 0)
    instance.velocity = (100.0, 0.0, 0.0);
    
    let target_vid_pos = (0.0, 100.0, 0.0); // Target is along Y axis now
    
    let get_target_pos = |vid| {
        if vid == 1234 {
            Some(target_vid_pos)
        } else {
            None
        }
    };
    let get_objects_in_range = |_pos, _radius| vec![];
    
    // Update should adjust direction towards (0, 100, 0)
    instance.set_time(1.0); // Make sure homing start time condition passes
    
    // The C++ logic requires homing_start_time + start_time < current_time.
    // However, if we just do set_time(1.0), start_time is set to 1.0, current_time to 1.0. 
    // 0.0 + 1.0 < 1.0 is false. Let's set it twice to advance current_time.
    instance.set_time(2.0); 

    instance.update(0.1, &get_target_pos, &get_objects_in_range);
    
    // Velocity should no longer be purely on X axis, Y should have increased
    assert!(instance.velocity.1 > 0.0);
}

#[test]
fn test_pierce_count() {
    let mut tracker = FlyProjectileTracker::new();
    
    let mut data = FlyingData::default();
    data.init_vel = 100.0;
    data.bomb_range = 20.0;
    data.range = 500.0;
    data.pierce_count = 1; // Can pierce one target
    data.hit_on_another_monster = true;
    
    let start_pos = (0.0, 0.0, 0.0);
    let target = FlyTarget::Position(300.0, 0.0, 0.0);
    
    let id = tracker.create_flying_instance(data, start_pos, target, true);
    let instance = tracker.get_instance_mut(id).unwrap();
    
    let get_target_pos = |_vid| None;
    
    // First step: move to (100,0,0) and hit object 111
    let status1 = instance.update(1.0, &get_target_pos, &|_, _| vec![111]);
    assert_eq!(status1, FlyingStatus::HitAnotherMonster);
    assert_eq!(instance.pierce_count, 0); // Pierce count decreased
    assert_eq!(instance.is_alive, true); // Still alive because it pierced
    
    // Second step: move to (200,0,0) and hit object 222
    let status2 = instance.update(1.0, &get_target_pos, &|_, _| vec![222]);
    assert_eq!(status2, FlyingStatus::HitAnotherMonster);
    assert_eq!(instance.pierce_count, 0); 
    assert_eq!(instance.is_alive, false); // Died because pierce count was 0
}

#[test]
fn test_max_range_reached() {
    let mut tracker = FlyProjectileTracker::new();
    
    let mut data = FlyingData::default();
    data.init_vel = 100.0;
    data.range = 50.0; // Small range
    
    let start_pos = (0.0, 0.0, 0.0);
    let target = FlyTarget::Position(200.0, 0.0, 0.0);
    
    let id = tracker.create_flying_instance(data, start_pos, target, true);
    let instance = tracker.get_instance_mut(id).unwrap();
    
    let get_target_pos = |_vid| None;
    let get_objects_in_range = |_pos, _radius| vec![];
    
    // Move 1 second at 100 units/s -> attempts to move 100 units, but range is 50.
    let status = instance.update(1.0, &get_target_pos, &get_objects_in_range);
    
    assert_eq!(status, FlyingStatus::OutOfRange);
    assert_eq!(instance.is_alive, false);
}
