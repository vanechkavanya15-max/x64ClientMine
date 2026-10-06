#[path = "../src/py_bridge_chr_motion_full.rs"]
mod py_bridge_chr_motion_full;

use py_bridge_chr_motion_full::*;

#[test]
fn test_chr_motion_full_sequence() {
    // 1. test_set_loop_motion
    {
        let mut mgr = get_chr_manager().lock().unwrap();
        let _inst = mgr.register_instance(1);
        mgr.select_instance(1);
        drop(mgr);

        assert!(m2_py_chr_set_loop_motion(42).is_ok());

        let mgr = get_chr_manager().lock().unwrap();
        let inst_ref = mgr.get_selected_instance_ptr().unwrap();
        let inst_data = inst_ref.lock().unwrap();
        assert_eq!(inst_data.loop_motion, 42);
        assert_eq!(inst_data.loop_motion_blend, 0.0);
    }

    // 2. test_push_once_motion
    {
        let mut mgr = get_chr_manager().lock().unwrap();
        let _inst = mgr.register_instance(2);
        mgr.select_instance(2);
        drop(mgr);

        assert!(push_once_motion(10, Some(0.5)).is_ok());

        let mgr = get_chr_manager().lock().unwrap();
        let inst_ref = mgr.get_selected_instance_ptr().unwrap();
        let inst_data = inst_ref.lock().unwrap();
        assert_eq!(inst_data.once_motion, 10);
        assert_eq!(inst_data.once_motion_blend, 0.5);

        drop(inst_data);
        drop(inst_ref);
        drop(mgr);
        
        assert!(push_once_motion(11, None).is_ok());

        let mgr = get_chr_manager().lock().unwrap();
        let inst_ref = mgr.get_selected_instance_ptr().unwrap();
        let inst_data = inst_ref.lock().unwrap();
        assert_eq!(inst_data.once_motion, 11);
        assert_eq!(inst_data.once_motion_blend, 0.1);
    }

    // 3. test_set_direction_fmod
    {
        let mut mgr = get_chr_manager().lock().unwrap();
        let _inst = mgr.register_instance(3);
        mgr.select_instance(3);
        drop(mgr);

        // 0 -> (0 + 180) % 360 = 180
        assert!(set_direction(0.0).is_ok());
        let mgr = get_chr_manager().lock().unwrap();
        let inst_ref = mgr.get_selected_instance_ptr().unwrap();
        let inst_data = inst_ref.lock().unwrap();
        assert_eq!(inst_data.rotation, 180.0);

        drop(inst_data);
        drop(inst_ref);
        drop(mgr);
        
        // 200 -> (200 + 180) % 360 = 380 % 360 = 20
        assert!(set_direction(200.0).is_ok());
        
        let mgr = get_chr_manager().lock().unwrap();
        let inst_ref = mgr.get_selected_instance_ptr().unwrap();
        let inst_data = inst_ref.lock().unwrap();
        assert_eq!(inst_data.rotation, 20.0);
    }

    // 4. test_get_pixel_position
    {
        let mut mgr = get_chr_manager().lock().unwrap();
        let inst = mgr.register_instance(4);
        
        {
            let mut inst_data = inst.lock().unwrap();
            inst_data.pixel_position.x = 100.0;
            inst_data.pixel_position.y = 200.0;
            inst_data.pixel_position.z = 300.0;
        }
        drop(mgr);

        let pos = get_pixel_position(4).unwrap();
        assert_eq!(pos, (100.0, 200.0, 300.0));

        let err = get_pixel_position(999);
        assert!(err.is_err());
    }

    // 5. test_set_armor_weapon_hair
    {
        let mut mgr = get_chr_manager().lock().unwrap();
        let _inst = mgr.register_instance(5);
        mgr.select_instance(5);
        drop(mgr);

        assert!(set_armor(12010).is_ok());
        assert!(set_weapon(279).is_ok());
        assert!(set_hair(10).is_ok());

        let mgr = get_chr_manager().lock().unwrap();
        let inst_ref = mgr.get_selected_instance_ptr().unwrap();
        let inst_data = inst_ref.lock().unwrap();
        assert_eq!(inst_data.armor, 12010);
        assert_eq!(inst_data.weapon, 279);
        assert_eq!(inst_data.hair, 10);
        assert!(inst_data.has_bounding_sphere);
    }

    // 6. test_blend_loop_motion
    {
        let mut mgr = get_chr_manager().lock().unwrap();
        let _inst = mgr.register_instance(6);
        mgr.select_instance(6);
        drop(mgr);

        assert!(set_blend_loop_motion(43, 0.25).is_ok());

        let mgr = get_chr_manager().lock().unwrap();
        let inst_ref = mgr.get_selected_instance_ptr().unwrap();
        let inst_data = inst_ref.lock().unwrap();
        assert_eq!(inst_data.loop_motion, 43);
        assert_eq!(inst_data.loop_motion_blend, 0.25);
    }
}
