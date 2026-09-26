use m2_client_bridge::api::actor_distance::m2_actor_get_distance;
use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, Actor};
use std::ffi::c_void;

#[test]
fn test_distance_null() {
    let ptr: *const c_void = std::ptr::null();
    let result = m2_actor_get_distance(ptr);
    assert_eq!(result, 0.0);
}

#[test]
fn test_distance_nonexistent() {
    let ptr = 9999 as *const c_void;
    let result = m2_actor_get_distance(ptr);
    assert_eq!(result, 0.0);
}

#[test]
fn test_distance_real_calculation() {
    let mut state = GLOBAL_GAME_STATE.write().unwrap();
    state.set_player_pos(10.0, 20.0, 0.0, 0.0);
    state.add_or_update_actor(Actor {
        vid: 777,
        x: 13.0,
        y: 24.0,
        z: 0.0,
        rot: 0.0,
        race: 101,
        b_type: 2,
        hp: 100,
        max_hp: 100,
        name: "TestMob".to_string(),
    });
    drop(state);

    let ptr = 777 as *const c_void;
    let dist = m2_actor_get_distance(ptr);
    assert!((dist - 5.0).abs() < 1e-4);

    let mut state = GLOBAL_GAME_STATE.write().unwrap();
    state.remove_actor(777);
}
