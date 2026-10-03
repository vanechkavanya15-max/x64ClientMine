use std::sync::{Mutex, OnceLock};

use m2_client::actor_polymorph_v2;
use m2_client::character_manager_core::get_character_manager;

static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|e| e.into_inner())
}

fn ensure_instance(vid: u32) {
    let mut mgr = get_character_manager().write().unwrap();
    if mgr.get_instance(vid).is_none() {
        mgr.create_instance(vid, 0, 0.0, 0.0, 0.0);
    }
}

#[test]
fn test_polymorph_apply_successful() {
    let _lock = get_test_lock();
    let vid = 1001;
    let mob_vnum = 101; // Wild Dog
    ensure_instance(vid);
    
    // Apply polymorph
    let result = actor_polymorph_v2::m2_poly_apply(vid, mob_vnum, 1.5, 1.5, 1.5);
    assert!(result, "Polymorph should apply successfully");
    
    // Verify state
    assert!(actor_polymorph_v2::m2_poly_is_active(vid));
    assert_eq!(actor_polymorph_v2::m2_poly_get_vnum(vid), mob_vnum);
    assert!(actor_polymorph_v2::m2_poly_is_equipment_hidden(vid));
    
    let mut out_x = 0.0;
    let mut out_y = 0.0;
    let mut out_z = 0.0;
    
    let scale_result = actor_polymorph_v2::m2_poly_get_scale(vid, &mut out_x, &mut out_y, &mut out_z);
    assert!(scale_result);
    assert_eq!(out_x, 1.5);
    assert_eq!(out_y, 1.5);
    assert_eq!(out_z, 1.5);
}

#[test]
fn test_polymorph_disable() {
    let _lock = get_test_lock();
    let vid = 1002;
    ensure_instance(vid);
    
    // Setup polymorph
    actor_polymorph_v2::m2_poly_apply(vid, 101, 1.0, 1.0, 1.0);
    assert!(actor_polymorph_v2::m2_poly_is_active(vid));
    
    // Disable polymorph
    let result = actor_polymorph_v2::m2_poly_apply(vid, 0, 1.0, 1.0, 1.0);
    assert!(result);
    
    // Verify state reverted
    assert!(!actor_polymorph_v2::m2_poly_is_active(vid));
    assert_eq!(actor_polymorph_v2::m2_poly_get_vnum(vid), 0);
    assert!(!actor_polymorph_v2::m2_poly_is_equipment_hidden(vid));
    
    let mut out_x = 0.0;
    let mut out_y = 0.0;
    let mut out_z = 0.0;
    assert!(!actor_polymorph_v2::m2_poly_get_scale(vid, &mut out_x, &mut out_y, &mut out_z));
}

#[test]
fn test_polymorph_reapply() {
    let _lock = get_test_lock();
    let vid = 1003;
    ensure_instance(vid);
    
    // Apply first time
    actor_polymorph_v2::m2_poly_apply(vid, 101, 1.0, 1.0, 1.0);
    assert_eq!(actor_polymorph_v2::m2_poly_get_vnum(vid), 101);
    
    // Apply second time with different mob
    let result = actor_polymorph_v2::m2_poly_apply(vid, 102, 2.0, 2.0, 2.0);
    assert!(result);
    
    assert_eq!(actor_polymorph_v2::m2_poly_get_vnum(vid), 102);
    
    let mut out_x = 0.0;
    let mut out_y = 0.0;
    let mut out_z = 0.0;
    assert!(actor_polymorph_v2::m2_poly_get_scale(vid, &mut out_x, &mut out_y, &mut out_z));
    assert_eq!(out_x, 2.0);
}

#[test]
fn test_polymorph_non_existent_vid() {
    let _lock = get_test_lock();
    let vid = 9999;
    
    assert!(!actor_polymorph_v2::m2_poly_is_active(vid));
    assert_eq!(actor_polymorph_v2::m2_poly_get_vnum(vid), 0);
    assert!(!actor_polymorph_v2::m2_poly_is_equipment_hidden(vid));
    
    let mut out_x = 0.0;
    let mut out_y = 0.0;
    let mut out_z = 0.0;
    assert!(!actor_polymorph_v2::m2_poly_get_scale(vid, &mut out_x, &mut out_y, &mut out_z));
}

#[test]
fn test_polymorph_null_pointers() {
    let _lock = get_test_lock();
    let vid = 1004;
    ensure_instance(vid);
    
    actor_polymorph_v2::m2_poly_apply(vid, 101, 1.0, 1.0, 1.0);
    
    // Pass null pointers
    assert!(!actor_polymorph_v2::m2_poly_get_scale(vid, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut()));
}

#[test]
fn test_polymorph_equipment_hidden() {
    let _lock = get_test_lock();
    let vid = 1005;
    ensure_instance(vid);
    
    // Normal state
    assert!(!actor_polymorph_v2::m2_poly_is_equipment_hidden(vid));
    
    // Polymorph state
    actor_polymorph_v2::m2_poly_apply(vid, 101, 1.0, 1.0, 1.0);
    assert!(actor_polymorph_v2::m2_poly_is_equipment_hidden(vid));
    
    // Revert state
    actor_polymorph_v2::m2_poly_apply(vid, 0, 1.0, 1.0, 1.0);
    assert!(!actor_polymorph_v2::m2_poly_is_equipment_hidden(vid));
}

#[test]
fn test_polymorph_partial_null_pointers() {
    let _lock = get_test_lock();
    let vid = 1006;
    ensure_instance(vid);
    
    actor_polymorph_v2::m2_poly_apply(vid, 101, 1.0, 1.0, 1.0);
    
    let mut out_x = 0.0;
    // Test combinations of null pointers
    assert!(!actor_polymorph_v2::m2_poly_get_scale(vid, &mut out_x, std::ptr::null_mut(), std::ptr::null_mut()));
}
