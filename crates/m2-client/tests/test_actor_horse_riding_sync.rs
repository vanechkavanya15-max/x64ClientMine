use m2_client::actor_horse_riding_sync::{ActorInstance, Vector3};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_mount_horse_initializes_horse_transform() {
    let mut rider = ActorInstance::new(1);
    rider.set_cur_pixel_position(Vector3::new(100.0, 200.0, 300.0));
    rider.set_rotation(45.0);
    
    let horse = Rc::new(RefCell::new(ActorInstance::new(2)));
    
    rider.mount_horse(Some(horse.clone()));
    
    let horse_ref = horse.borrow();
    let pos = horse_ref.get_cur_pixel_position();
    assert_eq!(pos.x, 100.0);
    assert_eq!(pos.y, 200.0);
    assert_eq!(pos.z, 300.0);
    assert_eq!(horse_ref.get_rotation(), 45.0);
    assert_eq!(horse_ref.advancing_rotation, 45.0);
}

#[test]
fn test_instance_base_transform_inherits_from_horse() {
    let mut rider = ActorInstance::new(1);
    let horse = Rc::new(RefCell::new(ActorInstance::new(2)));
    rider.mount_horse(Some(horse.clone()));
    
    {
        let mut h_ref = horse.borrow_mut();
        h_ref.set_cur_pixel_position(Vector3::new(500.0, 100.0, 200.0));
    }
    
    rider.instance_base_transform();
    
    assert_eq!(rider.x, 500.0);
    assert_eq!(rider.y, -100.0); // y should be inverted according to C++
    assert_eq!(rider.z, 200.0);
    assert!(rider.need_update_collision);
}

#[test]
fn test_on_end_combo_loops_when_mounted() {
    let mut rider = ActorInstance::new(1);
    rider.cur_combo_index = 3;
    
    // Not mounted
    rider.on_end_combo();
    assert_eq!(rider.cur_combo_index, 3);
    
    // Mounted
    let horse = Rc::new(RefCell::new(ActorInstance::new(2)));
    rider.mount_horse(Some(horse));
    
    rider.on_end_combo();
    assert_eq!(rider.cur_combo_index, 1);
}

#[test]
fn test_add_movement_delegates_to_horse() {
    let mut rider = ActorInstance::new(1);
    let horse = Rc::new(RefCell::new(ActorInstance::new(2)));
    rider.mount_horse(Some(horse.clone()));
    
    rider.add_movement(10.0, 20.0, 30.0);
    
    let horse_ref = horse.borrow();
    assert_eq!(horse_ref.movement.x, 10.0);
    assert_eq!(horse_ref.movement.y, 20.0);
    assert_eq!(horse_ref.movement.z, 30.0);
    
    assert_eq!(rider.movement.x, 0.0);
    assert_eq!(rider.movement.y, 0.0);
    assert_eq!(rider.movement.z, 0.0);
}

#[test]
fn test_is_movement_delegates_to_horse() {
    let mut rider = ActorInstance::new(1);
    
    // Rider moving alone
    rider.add_movement(5.0, 0.0, 0.0);
    assert!(rider.is_movement());
    
    // Reset rider
    rider.movement = Vector3::new(0.0, 0.0, 0.0);
    assert!(!rider.is_movement());
    
    // Mount horse
    let horse = Rc::new(RefCell::new(ActorInstance::new(2)));
    rider.mount_horse(Some(horse.clone()));
    
    assert!(!rider.is_movement()); // Neither moving
    
    // Horse moves
    horse.borrow_mut().add_movement(0.0, 0.0, 5.0);
    assert!(rider.is_movement()); // Rider reports movement because horse is moving
}

#[test]
fn test_set_motion_sync_with_horse() {
    use m2_client::actor_horse_riding_sync::{SetMotionData, MotionType, MotionMode};
    let mut rider = ActorInstance::new(1);
    let horse = Rc::new(RefCell::new(ActorInstance::new(2)));
    horse.borrow_mut().motion_mode = MotionMode::Horse;
    rider.mount_horse(Some(horse.clone()));
    
    // Test looping motion (e.g. idle or walk)
    let set_mot_data = SetMotionData {
        blend_time: 0.1,
        loop_count: 0,
        speed_ratio: 1.0,
    };
    
    rider.set_motion(0x00010002, set_mot_data); // Sub index = 1, Index = 2
    
    let h_ref = horse.borrow();
    // Validate horse motion type
    assert_eq!(h_ref.cur_mot_node.motion_type, MotionType::Loop);
    // duration for sub_index 1 is 1.0 + 1 * 0.1 = 1.1 -> 1.1 * 60 = 66
    assert_eq!(h_ref.cur_mot_node.frame_count, 66); 
    
    // Validate child motion key generation
    // Horse mode (1) << 24 | Sub (1) << 16 | Index (2)
    assert_eq!(h_ref.cur_mot_node.motion_key, (1 << 24) | (1 << 16) | 2);
}
