use m2_client::game_pair_emotion_controller::{get_degree_from_position, get_degree_from_position2, ActorInstance, PixelPosition};

#[test]
fn test_degree_from_position() {
    // Test basic angles matching Metin2 C++ logic (South is -y, dot with (0, -1))
    // (0, -1) -> dot(v, (0, -1)) = -y. acos(-y).
    // Let's test directions.

    // Straight down (0, -1)
    let deg = get_degree_from_position(0.0, -1.0);
    assert!((deg - 0.0).abs() < 1e-4, "Expected 0, got {}", deg);

    // Straight up (0, 1)
    let deg = get_degree_from_position(0.0, 1.0);
    assert!((deg - 180.0).abs() < 1e-4, "Expected 180, got {}", deg);

    // Right (1, 0)
    let deg = get_degree_from_position(1.0, 0.0);
    assert!((deg - 90.0).abs() < 1e-4, "Expected 90, got {}", deg);

    // Left (-1, 0)
    let deg = get_degree_from_position(-1.0, 0.0);
    assert!((deg - 270.0).abs() < 1e-4, "Expected 270, got {}", deg);

    // get_degree_from_position2
    let deg2 = get_degree_from_position2(100.0, 100.0, 100.0, 101.0); // (0, 1) -> 180
    assert!((deg2 - 180.0).abs() < 1e-4, "Expected 180, got {}", deg2);
}

#[test]
fn test_look_at_dest_instance() {
    let mut actor1 = ActorInstance::new(1, 0, true);
    actor1.cur_pixel_position = PixelPosition::new(0.0, 0.0, 0.0);

    let mut actor2 = ActorInstance::new(2, 1, false);
    actor2.cur_pixel_position = PixelPosition::new(10.0, 0.0, 0.0);

    actor1.look_at_dest_instance(&actor2);
    assert!((actor1.rotation - 90.0).abs() < 1e-4);
    assert!((actor1.advancing_rotation - 90.0).abs() < 1e-4);

    actor2.look_at_dest_instance(&actor1);
    assert!((actor2.rotation - 270.0).abs() < 1e-4);
    assert!((actor2.advancing_rotation - 270.0).abs() < 1e-4);
}

#[test]
fn test_act_dual_emotion_sync() {
    let mut actor1 = ActorInstance::new(1, 0, true);
    actor1.cur_pixel_position = PixelPosition::new(0.0, 0.0, 0.0);
    actor1.is_waiting = true;
    
    let mut actor2 = ActorInstance::new(2, 2, false);
    actor2.cur_pixel_position = PixelPosition::new(200.0, 0.0, 0.0);
    actor2.is_waiting = true;

    actor1.act_dual_emotion(&mut actor2, 1000, 2000);
    
    // Check moving setup
    assert_eq!(actor1.tcp_move_after_func.e_func, m2_client::game_pair_emotion_controller::FUNC_EMOTION);
    assert_eq!(actor1.tcp_move_after_func.u_arg_expanded, 2);
    
    // Check process function emotion
    actor1.process_function_emotion(actor1.tcp_move_after_func.u_arg, 2, actor1.tcp_move_after_func.pos_dst, &mut actor2);
    
    // Race 0 -> job 0, Race 2 -> job 2
    // motion1 = 1000, motion2 = 2000
    // actor1 should get motion1 + actor2_job (1000 + 2 = 1002)
    // actor2 should get motion2 + actor1_job (2000 + 0 = 2000)
    assert_eq!(actor1.current_motion, 1002);
    assert_eq!(actor2.current_motion, 2000);
    
    assert_eq!(actor1.motion_loop, false);
    assert_eq!(actor2.motion_loop, false);
}

#[test]
fn test_emotion_distance_and_movement() {
    let mut actor1 = ActorInstance::new(1, 0, true);
    actor1.cur_pixel_position = PixelPosition::new(0.0, 0.0, 0.0);
    actor1.is_waiting = true;

    let mut actor2 = ActorInstance::new(2, 1, false);
    actor2.cur_pixel_position = PixelPosition::new(200.0, 0.0, 0.0);
    actor2.is_waiting = true;

    actor1.act_dual_emotion(&mut actor2, 1000, 2000);

    let dst = actor1.tcp_move_after_func.pos_dst;
    // Actor 1 is at 0, Actor 2 is at 200. Distance = 200. Target is towards 0 from 200 by 100 units = 200 + (-200/200)*100 = 100.
    assert!((dst.x - 100.0).abs() < 1e-4);
    assert!((dst.y - 0.0).abs() < 1e-4);
}
