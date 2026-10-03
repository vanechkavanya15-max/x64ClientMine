use m2_client::character_manager_core::get_character_manager;
use m2_client::py_bridge_chr_motion;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_push_loop_motion_success() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    let vid = 100;
    mgr.create_instance(vid, 0, 0.0, 0.0, 0.0);
    mgr.main_actor_vid = Some(vid);
    drop(mgr);

    let res = py_bridge_chr_motion::m2_py_chr_push_loop_motion(42, 0.5);
    assert!(res);

    let mgr = get_character_manager().read().unwrap();
    let inst = mgr.get_instance(vid).unwrap();
    
    assert_eq!(inst.active_loop_motion, Some(42));
    assert_eq!(inst.motion_queue.cur_motion_id, 42);
    assert_eq!(inst.motion_queue.is_looping, true);
}

#[test]
fn test_push_once_motion_success() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    let vid = 101;
    mgr.create_instance(vid, 0, 0.0, 0.0, 0.0);
    mgr.main_actor_vid = Some(vid);
    drop(mgr);

    let res = py_bridge_chr_motion::m2_py_chr_push_once_motion(43, 0.5);
    assert!(res);

    let mgr = get_character_manager().read().unwrap();
    let inst = mgr.get_instance(vid).unwrap();
    
    assert_eq!(inst.active_once_motion, Some(43));
    assert_eq!(inst.motion_queue.cur_motion_id, 43);
    assert_eq!(inst.motion_queue.is_looping, false);
}

#[test]
fn test_set_loop_motion_success() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    let vid = 102;
    mgr.create_instance(vid, 0, 0.0, 0.0, 0.0);
    mgr.main_actor_vid = Some(vid);
    drop(mgr);

    let res = py_bridge_chr_motion::m2_py_chr_set_loop_motion(44, 0.5);
    assert!(res);

    let mgr = get_character_manager().read().unwrap();
    let inst = mgr.get_instance(vid).unwrap();
    
    assert_eq!(inst.active_loop_motion, Some(44));
    assert_eq!(inst.motion_queue.cur_motion_id, 44);
    assert_eq!(inst.motion_queue.is_looping, true);
}

#[test]
fn test_set_motion_mode_success() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    let vid = 103;
    mgr.create_instance(vid, 0, 0.0, 0.0, 0.0);
    mgr.main_actor_vid = Some(vid);
    drop(mgr);

    let res = py_bridge_chr_motion::m2_py_chr_set_motion_mode(9);
    assert!(res);
}

#[test]
fn test_functions_fail_without_selected_instance() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    drop(mgr);

    assert_eq!(py_bridge_chr_motion::m2_py_chr_push_loop_motion(42, 0.5), false);
    assert_eq!(py_bridge_chr_motion::m2_py_chr_push_once_motion(43, 0.5), false);
    assert_eq!(py_bridge_chr_motion::m2_py_chr_set_loop_motion(44, 0.5), false);
    assert_eq!(py_bridge_chr_motion::m2_py_chr_set_motion_mode(9), false);
}
