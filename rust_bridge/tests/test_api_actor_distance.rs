use m2_client_bridge::api::actor_distance::m2_actor_get_distance;
use std::ffi::c_void;

#[test]
fn test_distance_null() {
    let ptr: *const c_void = std::ptr::null();
    let result = m2_actor_get_distance(ptr);
    assert_eq!(result, 0.0);
}

#[test]
fn test_distance_valid() {
    let ptr = 1 as *const c_void;
    let result = m2_actor_get_distance(ptr);
    assert_eq!(result, 0.0);
}
