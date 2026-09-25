use m2_client::instance_motion::{
    InstanceMotion, MotionMode, MotionName, MotionPushType,
};

#[test]
fn test_integration_instance_motion() {
    let mut instance = InstanceMotion::new();
    assert!(instance.is_ready());

    assert_eq!(instance.get_motion_mode(), MotionMode::General);
    instance.set_motion_mode(MotionMode::OnehandSword);
    assert_eq!(instance.get_motion_mode(), MotionMode::OnehandSword);

    instance.push_once_motion(MotionName::NormalAttack, 0.1, 1.0);
    assert_eq!(instance.motion_deque.len(), 1);

    let first_mot = &instance.motion_deque[0];
    assert_eq!(first_mot.motion_type, MotionPushType::Once);
    assert_eq!(first_mot.motion_key, MotionName::NormalAttack as u32);
    assert_eq!(first_mot.blend_time, 0.1);

    instance.push_loop_motion(MotionName::Run, 0.2, 1.5);
    assert_eq!(instance.motion_deque.len(), 2);
    
    let second_mot = &instance.motion_deque[1];
    assert_eq!(second_mot.motion_type, MotionPushType::Loop);
    assert_eq!(second_mot.motion_key, MotionName::Run as u32);
    assert_eq!(second_mot.blend_time, 0.2);
    assert_eq!(second_mot.speed_ratio, 1.5);

    instance.set_loop_motion(MotionName::Wait, 0.3, 1.0);
    assert_eq!(instance.motion_deque.len(), 1); 
    
    let new_mot = &instance.motion_deque[0];
    assert_eq!(new_mot.motion_type, MotionPushType::Loop);
    assert_eq!(new_mot.motion_key, MotionName::Wait as u32);

    instance.clear_motion();
    assert_eq!(instance.motion_deque.len(), 0);
}

#[test]
fn test_reserving_and_current_motion_process() {
    let mut instance = InstanceMotion::new();
    
    // Simulate time progression
    instance.set_local_time(1.0);
    
    instance.push_once_motion(MotionName::Walk, 0.0, 1.0);
    
    assert_eq!(instance.motion_deque.len(), 1);
    
    // Process reserving motion queue - should move node to current
    instance.reserving_motion_process();
    
    assert_eq!(instance.motion_deque.len(), 0);
    assert_eq!(instance.cur_mot_node.motion_key, MotionName::Walk as u32);
    assert_eq!(instance.cur_mot_node.motion_type, MotionPushType::Once);
    
    // Check end time calculation
    assert_eq!(instance.cur_mot_node.start_time, 1.0);
    assert_eq!(instance.cur_mot_node.end_time, 2.0); // 1.0 (local) + 1.0 (mock duration)
    
    // Advance time slightly
    instance.set_local_time(1.5);
    instance.current_motion_process();
    
    // Should still be Walk
    assert_eq!(instance.cur_mot_node.motion_key, MotionName::Walk as u32);
    
    // Advance time past end time
    instance.set_local_time(2.1);
    instance.current_motion_process();
    
    // Walk (Once) should fall back to Wait (Loop) logic according to default branch in match
    // Which means it clears the queue and pushes Wait
    assert_eq!(instance.motion_deque.len(), 1);
    assert_eq!(instance.motion_deque[0].motion_key, MotionName::Wait as u32);
    assert_eq!(instance.motion_deque[0].motion_type, MotionPushType::Loop);
}
