use m2_client::character_manager_core::get_character_manager;
use m2_client::py_bridge_chr::*;
use std::ffi::CStr;
use serial_test::serial;

#[test]
#[serial]
fn test_m2_chr_get_instance_success() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    mgr.create_instance_full(1001, 1, 10.0, 20.0, 30.0, "TestHero".to_string(), false);
    drop(mgr); // Release lock

    let vid = m2_chr_get_instance(1001);
    assert_eq!(vid, 1001);
}

#[test]
#[serial]
fn test_m2_chr_get_instance_fail() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    drop(mgr);

    let vid = m2_chr_get_instance(9999);
    assert_eq!(vid, 0);
}

#[test]
#[serial]
fn test_m2_chr_select_instance() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    mgr.create_instance_full(2002, 2, 0.0, 0.0, 0.0, "Mage".to_string(), false);
    drop(mgr);

    let result = m2_chr_select_instance(2002);
    assert!(result);

    let mgr = get_character_manager().read().unwrap();
    assert_eq!(mgr.main_actor_vid, Some(2002));
}

#[test]
#[serial]
fn test_m2_chr_get_pixel_position() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    mgr.create_instance_full(3003, 3, 100.5, 200.5, 300.5, "PosTest".to_string(), false);
    drop(mgr);

    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    let success = m2_chr_get_pixel_position(3003, &mut x, &mut y, &mut z);
    assert!(success);
    assert_eq!(x, 100.5);
    assert_eq!(y, 200.5);
    assert_eq!(z, 300.5);
    
    // Test missing instance
    let success_missing = m2_chr_get_pixel_position(9999, &mut x, &mut y, &mut z);
    assert!(!success_missing);
    
    // Test null pointers
    let success_null = m2_chr_get_pixel_position(3003, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
    assert!(!success_null);
}

#[test]
#[serial]
fn test_m2_chr_get_name() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    mgr.create_instance_full(4004, 0, 0.0, 0.0, 0.0, "SuperWarrior".to_string(), false);
    drop(mgr);

    let mut buffer = vec![0u8; 64];
    let len = m2_chr_get_name(4004, buffer.as_mut_ptr(), buffer.len());
    
    assert!(len > 0);
    
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
    assert_eq!(c_str.to_str().unwrap(), "SuperWarrior");
    
    // Test missing instance
    let fail_len = m2_chr_get_name(9999, buffer.as_mut_ptr(), buffer.len());
    assert_eq!(fail_len, -1);
}

#[test]
#[serial]
fn test_m2_chr_get_race() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    mgr.create_instance_full(5005, 5, 0.0, 0.0, 0.0, "Ninja".to_string(), false);
    drop(mgr);

    let race = m2_chr_get_race(5005);
    assert_eq!(race, 5);
    
    // Test missing instance
    assert_eq!(m2_chr_get_race(9999), 0);
}

#[test]
#[serial]
fn test_m2_chr_is_dead() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    mgr.create_instance_full(6006, 0, 0.0, 0.0, 0.0, "DeadGuy".to_string(), true);
    mgr.create_instance_full(7007, 0, 0.0, 0.0, 0.0, "AliveGuy".to_string(), false);
    drop(mgr);

    assert!(m2_chr_is_dead(6006));
    assert!(!m2_chr_is_dead(7007));
    assert!(!m2_chr_is_dead(9999));
}

#[test]
#[serial]
fn test_m2_chr_blend_loop_motion() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    mgr.create_instance_full(8008, 0, 0.0, 0.0, 0.0, "Motion".to_string(), false);
    drop(mgr);

    assert!(m2_chr_blend_loop_motion(8008, 123));
    
    // Verify it actually set the loop motion
    {
        let mgr = get_character_manager().read().unwrap();
        assert_eq!(mgr.get_instance(8008).unwrap().active_loop_motion, Some(123));
    }
    
    assert!(!m2_chr_blend_loop_motion(9999, 123));
}

#[test]
#[serial]
fn test_m2_py_chr_pick() {
    m2_py_chr_set_picked_vid(4004);
    assert_eq!(m2_py_chr_pick(), 4004);

    m2_py_chr_set_picked_vid(0);
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    mgr.create_instance_full(5005, 0, 0.0, 0.0, 0.0, "PickedActor".to_string(), false);
    mgr.main_actor_vid = Some(5005);
    drop(mgr);

    assert_eq!(m2_py_chr_pick(), 5005);
}

#[test]
#[serial]
fn test_m2_py_chrmgr_create_and_load_race() {
    assert!(m2_py_chrmgr_create_race(101));
    let path = std::ffi::CString::new("d:/ymir work/monster/wolf/wolf.msm").unwrap();
    assert!(m2_py_chrmgr_load_race_data(101, path.as_ptr()));

    let reg = get_race_registry().read().unwrap();
    let data = reg.get(&101).expect("Race 101 should exist");
    assert_eq!(data.race_id, 101);
    assert_eq!(data.path, "d:/ymir work/monster/wolf/wolf.msm");
    assert!(data.loaded);
}
