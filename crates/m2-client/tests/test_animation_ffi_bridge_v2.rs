#[path = "../src/animation_ffi_bridge_v2.rs"]
mod animation_ffi_bridge_v2;

use animation_ffi_bridge_v2::*;

#[test]
fn test_anim_play() {
    let vid = 1001;
    assert!(m2_anim_play(vid, 5, false));
    
    // Check initial state
    let mut event_type = 0;
    assert!(m2_anim_get_event(vid, &mut event_type));
    assert_eq!(event_type, 0); // No event yet
}

#[test]
fn test_anim_update_non_looping() {
    let vid = 1002;
    assert!(m2_anim_play(vid, 10, false));
    
    // Update beyond duration (1.0 default)
    assert!(m2_anim_update(vid, 1.5));
    
    let mut event_type = 0;
    assert!(m2_anim_get_event(vid, &mut event_type));
    assert_eq!(event_type, 1); // Event type 1 means ended for non-looping
}

#[test]
fn test_anim_update_looping() {
    let vid = 1003;
    assert!(m2_anim_play(vid, 20, true));
    
    // Update beyond duration (1.0 default)
    assert!(m2_anim_update(vid, 1.5));
    
    let mut event_type = 0;
    assert!(m2_anim_get_event(vid, &mut event_type));
    assert_eq!(event_type, 0); // Event type 0 because it looped
}

#[test]
fn test_anim_attach_weapon() {
    let vid = 1004;
    assert!(m2_anim_attach_weapon(vid, 2799)); // Sword VNUM
    
    assert!(m2_anim_play(vid, 1, false));
    assert!(m2_anim_update(vid, 0.1));
    
    // Weapon is attached, update succeeds
    assert!(m2_anim_attach_weapon(vid, 0)); // Unequip
}

#[test]
fn test_anim_get_event_null_pointer() {
    let vid = 1005;
    assert!(m2_anim_play(vid, 2, false));
    assert!(!m2_anim_get_event(vid, std::ptr::null_mut()));
}

#[test]
fn test_anim_update_non_existent_vid() {
    let vid = 9999;
    assert!(!m2_anim_update(vid, 1.0));
}

#[test]
fn test_anim_get_event_non_existent_vid() {
    let vid = 9998;
    let mut event_type = 0;
    assert!(!m2_anim_get_event(vid, &mut event_type));
}
