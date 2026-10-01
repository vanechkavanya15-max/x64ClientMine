use m2_client::character_manager_core::get_character_manager;
use m2_client::py_bridge_chr_native::*;
use serial_test::serial;

#[test]
#[serial]
fn test_chrmgr_lifecycle() {
    let vid = 1001;
    
    // Clear state
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.clear();
    }
    
    // IsInstance should be false initially
    assert_eq!(m2_py_chrmgr_is_instance(vid), false);
    
    // CreateInstance
    assert_eq!(m2_py_chrmgr_create_instance(vid), true);
    assert_eq!(m2_py_chrmgr_is_instance(vid), true);
    
    // CreateInstance again should fail
    assert_eq!(m2_py_chrmgr_create_instance(vid), false);
    
    // SelectInstance
    assert_eq!(m2_py_chrmgr_select_instance(vid), true);
    {
        let mgr = get_character_manager().read().unwrap();
        assert_eq!(mgr.main_actor_vid, Some(vid));
    }
    
    // DeleteInstance
    assert_eq!(m2_py_chrmgr_delete_instance(vid), true);
    assert_eq!(m2_py_chrmgr_is_instance(vid), false);
    
    // Verify selection cleared
    {
        let mgr = get_character_manager().read().unwrap();
        assert_eq!(mgr.main_actor_vid, None);
    }
}

#[test]
#[serial]
fn test_chr_appearance() {
    let vid = 1002;
    
    // Clear and create
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.clear();
    }
    m2_py_chrmgr_create_instance(vid);
    m2_py_chrmgr_select_instance(vid);
    
    // SetRace
    assert_eq!(m2_py_chr_set_race(1), true);
    
    // Equipment
    assert_eq!(m2_py_chr_set_armor(11299), true);
    assert_eq!(m2_py_chr_set_weapon(279), true);
    assert_eq!(m2_py_chr_set_hair(2001), true);
    
    // Verify properties
    {
        let mgr = get_character_manager().read().unwrap();
        let inst = mgr.get_instance(vid).unwrap();
        assert_eq!(inst.race, 1);
        assert_eq!(inst.armor_vnum, 11299);
        assert_eq!(inst.weapon_vnum, 279);
        assert_eq!(inst.hair_vnum, 2001);
    }
}

#[test]
#[serial]
fn test_chr_transform() {
    let vid = 1003;
    
    // Clear and create
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.clear();
    }
    m2_py_chrmgr_create_instance(vid);
    m2_py_chrmgr_select_instance(vid);
    
    // Transform
    assert_eq!(m2_py_chr_set_rotation(45.0), true);
    assert_eq!(m2_py_chr_set_pixel_position(100.0, 200.0, 300.0), true);
    
    // Verify
    {
        let mgr = get_character_manager().read().unwrap();
        let inst = mgr.get_instance(vid).unwrap();
        assert_eq!(inst.rot_deg, 45.0);
        assert_eq!(inst.x, 100.0);
        assert_eq!(inst.y, 200.0);
        assert_eq!(inst.z, 300.0);
    }
}

#[test]
#[serial]
fn test_chr_motion() {
    let vid = 1004;
    
    // Clear and create
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.clear();
    }
    m2_py_chrmgr_create_instance(vid);
    m2_py_chrmgr_select_instance(vid);
    
    // Set motions
    assert_eq!(m2_py_chr_set_motion(3), true);
    assert_eq!(m2_py_chr_push_once_motion(4), true);
    
    // Verify
    {
        let mgr = get_character_manager().read().unwrap();
        let inst = mgr.get_instance(vid).unwrap();
        assert_eq!(inst.active_loop_motion, Some(3));
        assert_eq!(inst.active_once_motion, Some(4));
    }
}

#[test]
#[serial]
fn test_invalid_vid() {
    let vid = 9999;
    
    // Clear state
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.clear();
    }
    
    // Operations on non-existent instance should fail gracefully
    assert_eq!(m2_py_chrmgr_select_instance(vid), false);
    assert_eq!(m2_py_chrmgr_delete_instance(vid), false);
    assert_eq!(m2_py_chr_set_race(1), false);
    assert_eq!(m2_py_chr_set_armor(11299), false);
    assert_eq!(m2_py_chr_set_rotation(45.0), false);
    assert_eq!(m2_py_chr_set_pixel_position(0.0, 0.0, 0.0), false);
    assert_eq!(m2_py_chr_set_motion(0), false);
}