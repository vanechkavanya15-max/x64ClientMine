#[path = "../src/py_bridge_chrmgr_instances_full.rs"]
mod py_bridge_chrmgr_instances_full;

use std::sync::Once;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        // Any global setup for the CharacterManagerCore would go here.
        // It's lazily initialized via OnceLock so calling the FFI functions is enough.
    });
}

fn cleanup(vid: u32) {
    py_bridge_chrmgr_instances_full::m2_py_chrmgr_destroy_character(vid);
}

#[test]
fn test_create_and_destroy_character() {
    setup();
    let vid = 1001;
    
    // Test creation
    let result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_create_character(vid);
    assert!(result, "Character creation should succeed");
    
    // Duplicate creation should fail (return false) according to C++ parity and Rust impl
    let duplicate_result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_create_character(vid);
    assert!(!duplicate_result, "Duplicate character creation should fail");
    
    // Test destruction
    let dest_result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_destroy_character(vid);
    assert!(dest_result, "Character destruction should succeed");
    
    // Duplicate destruction should fail
    let duplicate_dest_result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_destroy_character(vid);
    assert!(!duplicate_dest_result, "Duplicate destruction should fail");
}

#[test]
fn test_select_character() {
    setup();
    let vid = 1002;
    py_bridge_chrmgr_instances_full::m2_py_chrmgr_create_character(vid);
    
    // Test selecting existing character
    let select_result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_select_character(vid);
    assert!(select_result, "Selecting existing character should succeed");
    
    // Test selecting non-existing character
    let select_fail = py_bridge_chrmgr_instances_full::m2_py_chrmgr_select_character(9999);
    assert!(!select_fail, "Selecting non-existing character should fail");
    
    cleanup(vid);
}

#[test]
fn test_get_character_position() {
    setup();
    let vid = 1003;
    py_bridge_chrmgr_instances_full::m2_py_chrmgr_create_character(vid);
    
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 0.0;
    
    unsafe {
        // Successful position retrieval
        let result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_get_character_position(vid, &mut x, &mut y, &mut z);
        assert!(result, "Getting position should succeed for existing character");
        assert_eq!(x, 0.0, "Initial X should be 0.0");
        assert_eq!(y, 0.0, "Initial Y should be 0.0");
        assert_eq!(z, 0.0, "Initial Z should be 0.0");
        
        // Failed retrieval
        let fail_result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_get_character_position(9999, &mut x, &mut y, &mut z);
        assert!(!fail_result, "Getting position should fail for non-existing character");
        
        // Null pointer check
        let null_result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_get_character_position(vid, std::ptr::null_mut(), &mut y, &mut z);
        assert!(!null_result, "Getting position should fail with null pointers");
    }
    
    cleanup(vid);
}

#[test]
fn test_show_hide_character() {
    setup();
    let vid = 1004;
    
    // Trying to show/hide non-existing character should fail
    assert!(!py_bridge_chrmgr_instances_full::m2_py_chrmgr_show_character(vid));
    assert!(!py_bridge_chrmgr_instances_full::m2_py_chrmgr_hide_character(vid));
    
    // Create character
    py_bridge_chrmgr_instances_full::m2_py_chrmgr_create_character(vid);
    
    // Show character should succeed
    let show_result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_show_character(vid);
    assert!(show_result, "Showing existing character should succeed");
    
    // Hide character should succeed
    let hide_result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_hide_character(vid);
    assert!(hide_result, "Hiding existing character should succeed");
    
    cleanup(vid);
}

#[test]
fn test_register_point_character() {
    setup();
    let vid = 1005;
    
    // Trying to register point on non-existing character should fail
    assert!(!py_bridge_chrmgr_instances_full::m2_py_chrmgr_register_point(vid, 100.0, 200.0));
    
    // Create character
    py_bridge_chrmgr_instances_full::m2_py_chrmgr_create_character(vid);
    
    // Register point should succeed
    let reg_result = py_bridge_chrmgr_instances_full::m2_py_chrmgr_register_point(vid, 100.0, 200.0);
    assert!(reg_result, "Registering point on existing character should succeed");
    
    cleanup(vid);
}
