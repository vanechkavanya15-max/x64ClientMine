#[path = "../src/actor_motion_queue.rs"]
mod actor_motion_queue;

use actor_motion_queue::*;

#[test]
fn test_looping_animations() {
    let mut queue = ActorMotionQueue::new();
    let set_success = queue.set_motion(MOTION_RUN, 1000, true);
    assert!(set_success);
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
    let run_success = queue.set_motion(MOTION_RUN, 500, true);
    assert!(!run_success);
    assert_eq!(queue.cur_motion_id, MOTION_DAMAGE);

    queue.set_motion(MOTION_WALK, 500, true);
    assert_eq!(queue.cur_motion_id, MOTION_DAMAGE);

    queue.set_motion(MOTION_WAIT, 500, true);
    assert_eq!(queue.cur_motion_id, MOTION_DAMAGE);

    // Inne (np. combos albo dead) powinny móc przerwac
    let combo_success = queue.set_motion(MOTION_COMBO_ATTACK, 2000, false);
    assert!(combo_success);
    assert_eq!(queue.cur_motion_id, MOTION_COMBO_ATTACK);

    // Podobnie z combo
    let run_combo_success = queue.set_motion(MOTION_RUN, 500, true);
    assert!(!run_combo_success);
    assert_eq!(queue.cur_motion_id, MOTION_COMBO_ATTACK);
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
    
    // RESPAWN moze przerwac DEAD
    let respawn_success = queue.set_motion(MOTION_RESPAWN, 1000, false);
    assert!(respawn_success);
    assert_eq!(queue.cur_motion_id, MOTION_RESPAWN);
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

#[test]
fn test_damage_interrupts_normal_attack() {
    let mut queue = ActorMotionQueue::new();
    queue.set_motion(MOTION_NORMAL_ATTACK, 1000, false);
    
    let wait_success = queue.set_motion(MOTION_WAIT, 1000, true);
    assert!(!wait_success);
    assert_eq!(queue.cur_motion_id, MOTION_NORMAL_ATTACK);
    
    let dmg_success = queue.set_motion(MOTION_DAMAGE, 1000, false);
    assert!(dmg_success);
    assert_eq!(queue.cur_motion_id, MOTION_DAMAGE);
}

#[test]
fn test_ffi_functions() {
    let vid = 1001;
    
    // Upewnijmy sie ze aktor 1001 na poczatku nie istnieje/zwraca MOTION_WAIT
    assert_eq!(m2_motion_get_state(vid), MOTION_WAIT);
    assert!(!m2_motion_is_dead(vid));
    
    // Ustawiamy MOTION_RUN
    let success = m2_motion_set_state(vid, MOTION_RUN);
    assert!(success);
    assert_eq!(m2_motion_get_state(vid), MOTION_RUN);
    assert!(!m2_motion_is_dead(vid));
    
    // Ustawiamy MOTION_DEAD
    let dead_success = m2_motion_set_state(vid, MOTION_DEAD);
    assert!(dead_success);
    assert_eq!(m2_motion_get_state(vid), MOTION_DEAD);
    assert!(m2_motion_is_dead(vid));
    
    // Probujemy przerwac DEAD przez RUN
    let run_success = m2_motion_set_state(vid, MOTION_RUN);
    assert!(!run_success);
    assert_eq!(m2_motion_get_state(vid), MOTION_DEAD);
    assert!(m2_motion_is_dead(vid));
    
    // RESPAWN
    let respawn_success = m2_motion_set_state(vid, MOTION_RESPAWN);
    assert!(respawn_success);
    assert_eq!(m2_motion_get_state(vid), MOTION_RESPAWN);
    assert!(!m2_motion_is_dead(vid));
}
