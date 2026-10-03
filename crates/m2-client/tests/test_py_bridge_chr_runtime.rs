use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;
use m2_client::py_bridge_chr_runtime::*;
use std::ffi::CString;
use std::os::raw::c_char;

static TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn test_create_and_select_instance() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    unsafe {
        // Create instance
        let created = m2_py_chr_create_instance_legacy(42);
        assert!(created);

        // Verify it exists
        {
            let state = GLOBAL_GAME_STATE.read().unwrap();
            assert!(state.get_actor(42).is_some());
        }

        // Select instance
        let selected = m2_py_chr_select_instance(42);
        assert!(selected);

        // Verify it's selected
        assert!(m2_py_chrmgr_is_selected_vid(42));
    }
}

#[test]
fn test_delete_instance() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    unsafe {
        m2_py_chr_create_instance_legacy(99);
        assert!(GLOBAL_GAME_STATE.read().unwrap().get_actor(99).is_some());

        let deleted = m2_py_chr_delete_instance_legacy(99);
        assert!(deleted);
        
        assert!(GLOBAL_GAME_STATE.read().unwrap().get_actor(99).is_none());
    }
}

#[test]
fn test_set_properties() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    unsafe {
        m2_py_chr_create_instance_legacy(123);
        m2_py_chr_select_instance(123);

        m2_py_chr_set_instance_type(3); // NPC
        m2_py_chr_rt_set_pixel_position(100, 200, 300);
        m2_py_chr_rt_set_rotation(90.0);
        m2_py_chr_set_motion_mode_legacy(2);
        m2_py_chr_rt_push_once_motion(5);

        let state = GLOBAL_GAME_STATE.read().unwrap();
        let actor = state.get_actor(123).unwrap();

        assert_eq!(actor.b_type, 3);
        assert_eq!(actor.x, 100.0);
        assert_eq!(actor.y, 200.0);
        assert_eq!(actor.z, 300.0);
        assert_eq!(actor.rot, 90.0);
        assert_eq!(actor.motion_mode, 2);
        assert_eq!(actor.current_motion, 5);
    }
}

#[test]
fn test_get_name_and_is_dead() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    unsafe {
        m2_py_chr_create_instance_legacy(777);
        m2_py_chr_select_instance(777);

        // Default name is empty
        let name_ptr = m2_py_chr_get_name();
        let name = std::ffi::CStr::from_ptr(name_ptr).to_str().unwrap();
        assert_eq!(name, "");

        assert!(!m2_py_chr_is_dead());

        // Update name and dead status manually for test
        {
            let mut state = GLOBAL_GAME_STATE.write().unwrap();
            let actor = state.actors.get_mut(&777).unwrap();
            actor.name = "TestBoss".to_string();
            actor.is_dead = true;
        }

        let name_ptr = m2_py_chr_get_name();
        let name = std::ffi::CStr::from_ptr(name_ptr).to_str().unwrap();
        assert_eq!(name, "TestBoss");
        assert!(m2_py_chr_is_dead());
    }
}

#[test]
fn test_chrmgr_get_vid_by_name() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    unsafe {
        m2_py_chr_create_instance_legacy(111);
        m2_py_chr_create_instance_legacy(222);

        {
            let mut state = GLOBAL_GAME_STATE.write().unwrap();
            state.actors.get_mut(&111).unwrap().name = "PlayerOne".to_string();
            state.actors.get_mut(&222).unwrap().name = "PlayerTwo".to_string();
        }

        let name1 = CString::new("PlayerOne").unwrap();
        let name2 = CString::new("PlayerTwo").unwrap();
        let name_missing = CString::new("Missing").unwrap();

        assert_eq!(m2_py_chrmgr_get_vid_by_name(name1.as_ptr()), 111);
        assert_eq!(m2_py_chrmgr_get_vid_by_name(name2.as_ptr()), 222);
        assert_eq!(m2_py_chrmgr_get_vid_by_name(name_missing.as_ptr()), 0);
    }
}
