#[path = "../src/actor_motion_queue.rs"]
mod actor_motion_queue;

use actor_motion_queue::*;

#[test]
fn test_looping_animations() {
    let mut queue = ActorMotionQueue::new();
    queue.set_motion(MOTION_RUN, 1000, true);
    assert_eq!(queue.cur_motion_id, MOTION_RUN);
    assert_eq!(queue.motion_elapsed_ms, 0);

    let finished = queue.update_motion(500);
    assert!(!finished);
    assert_eq!(queue.motion_elapsed_ms, 500);

    let finished = queue.update_motion(600);
    assert!(!finished);
    assert_eq!(queue.motion_elapsed_ms, 100); // 1100 % 1000
}

#[test]
fn test_one_time_animation_finish() {
    let mut queue = ActorMotionQueue::new();
    queue.set_motion(MOTION_DAMAGE, 1000, false);
    assert_eq!(queue.cur_motion_id, MOTION_DAMAGE);
    assert_eq!(queue.motion_elapsed_ms, 0);

    let finished = queue.update_motion(500);
    assert!(!finished);
    assert_eq!(queue.motion_elapsed_ms, 500);

    let finished = queue.update_motion(500);
    assert!(finished);
    assert_eq!(queue.motion_elapsed_ms, 1000);

    let finished = queue.update_motion(100);
    assert!(!finished);
    assert_eq!(queue.motion_elapsed_ms, 1100);
}

#[test]
fn test_blocking_animations() {
    let mut queue = ActorMotionQueue::new();
    queue.set_motion(MOTION_DAMAGE, 1000, false);
    
    // Slabsze animacje nie moga przerwac damage
    queue.set_motion(MOTION_RUN, 500, true);
    assert_eq!(queue.cur_motion_id, MOTION_DAMAGE);

    queue.set_motion(MOTION_WALK, 500, true);
    assert_eq!(queue.cur_motion_id, MOTION_DAMAGE);

    queue.set_motion(MOTION_WAIT, 500, true);
    assert_eq!(queue.cur_motion_id, MOTION_DAMAGE);

    // Inne (np. combos albo dead) powinny móc przerwac
    queue.set_motion(MOTION_COMBO, 2000, false);
    assert_eq!(queue.cur_motion_id, MOTION_COMBO);

    // Podobnie z combo
    queue.set_motion(MOTION_RUN, 500, true);
    assert_eq!(queue.cur_motion_id, MOTION_COMBO);
}

#[test]
fn test_dead_animation_lock() {
    let mut queue = ActorMotionQueue::new();
    queue.set_motion(MOTION_DEAD, 2000, false);
    
    queue.set_motion(MOTION_RUN, 1000, true);
    assert_eq!(queue.cur_motion_id, MOTION_DEAD);

    queue.set_motion(MOTION_WAIT, 1000, true);
    assert_eq!(queue.cur_motion_id, MOTION_DEAD);

    queue.set_motion(MOTION_DAMAGE, 1000, false);
    assert_eq!(queue.cur_motion_id, MOTION_DEAD);

    // DEAD mozna nadpisac DEAD
    queue.set_motion(MOTION_DEAD, 3000, false);
    assert_eq!(queue.cur_motion_id, MOTION_DEAD);
    assert_eq!(queue.motion_duration_ms, 3000);
}

#[test]
fn test_edge_cases() {
    let mut queue = ActorMotionQueue::new();
    
    // Duration 0 with loop
    queue.set_motion(MOTION_WAIT, 0, true);
    let finished = queue.update_motion(100);
    assert!(!finished);
    assert_eq!(queue.motion_elapsed_ms, 100);

    // Duration 0 without loop
    queue.set_motion(MOTION_WAIT, 0, false);
    let finished = queue.update_motion(100);
    assert!(finished); // Bylo 0, dodalismy 100, przeszlo przez bariere >= duration_ms
}

#[test]
fn test_damage_blocking_ends_after_duration() {
    let mut queue = ActorMotionQueue::new();
    queue.set_motion(MOTION_DAMAGE, 1000, false);
    queue.update_motion(1000);

    // Now it should allow RUN
    queue.set_motion(MOTION_RUN, 500, true);
    assert_eq!(queue.cur_motion_id, MOTION_RUN);
}
