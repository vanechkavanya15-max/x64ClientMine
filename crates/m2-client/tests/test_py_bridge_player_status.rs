use m2_client::py_bridge_player_status::*;
use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, POINT_GOLD};
use m2_client::character_manager_core::get_character_manager;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_get_exp() {
    let _guard = TEST_MUTEX.lock().unwrap();
    unsafe {
        m2_client_bridge::state::game_state::m2_game_state_reset();
        
        {
            let mut state = GLOBAL_GAME_STATE.write().unwrap();
            state.set_point(POINT_EXP, 12345);
        }
        
        let exp = m2_py_player_get_exp();
        assert_eq!(exp, 12345);
    }
}

#[test]
fn test_get_money() {
    let _guard = TEST_MUTEX.lock().unwrap();
    unsafe {
        m2_client_bridge::state::game_state::m2_game_state_reset();
        
        {
            let mut state = GLOBAL_GAME_STATE.write().unwrap();
            state.set_point(POINT_GOLD, 987654321);
        }
        
        let money = m2_py_player_get_money();
        assert_eq!(money, 987654321);
        
        let elk = m2_py_player_get_elk();
        assert_eq!(elk, 987654321);
    }
}

#[test]
fn test_get_play_time() {
    let _guard = TEST_MUTEX.lock().unwrap();
    unsafe {
        m2_client_bridge::state::game_state::m2_game_state_reset();
        
        {
            let mut state = GLOBAL_GAME_STATE.write().unwrap();
            state.set_point(POINT_PLAYTIME, 54321);
        }
        
        let play_time = m2_py_player_get_play_time();
        assert_eq!(play_time, 54321);
    }
}

#[test]
fn test_get_job_and_race() {
    let _guard = TEST_MUTEX.lock().unwrap();
    unsafe {
        // Clear manager
        {
            let mut mgr = get_character_manager().write().unwrap();
            mgr.clear();
            mgr.create_instance_full(100, 5, 0.0, 0.0, 0.0, "Test".to_string(), false);
            mgr.main_actor_vid = Some(100);
        }
        
        let race = m2_py_player_get_race();
        assert_eq!(race, 5); // 5 to Ninja w Metin2
        
        let job = m2_py_player_get_job();
        assert_eq!(job, 1); // 5 % 4 = 1
    }
}

#[test]
fn test_get_name() {
    let _guard = TEST_MUTEX.lock().unwrap();
    unsafe {
        m2_client_bridge::state::game_state::m2_game_state_reset();
        {
            let mut state = GLOBAL_GAME_STATE.write().unwrap();
            state.set_player_vid(100);
        }
        
        {
            let mut mgr = get_character_manager().write().unwrap();
            mgr.clear();
            mgr.create_instance_full(100, 5, 0.0, 0.0, 0.0, "PlayerNameTest".to_string(), false);
            mgr.main_actor_vid = Some(100);
        }
        
        let c_name_ptr = m2_py_player_get_name();
        assert!(!c_name_ptr.is_null());
        
        let c_str = std::ffi::CStr::from_ptr(c_name_ptr);
        assert_eq!(c_str.to_str().unwrap(), "PlayerNameTest");
    }
}

#[test]
fn test_get_character_distance() {
    let _guard = TEST_MUTEX.lock().unwrap();
    unsafe {
        m2_client_bridge::state::game_state::m2_game_state_reset();
        
        {
            let mut state = GLOBAL_GAME_STATE.write().unwrap();
            state.set_player_pos(0.0, 0.0, 0.0, 0.0);
            
            let mut actor = m2_client_bridge::state::game_state::Actor::default();
            actor.vid = 500;
            actor.x = 300.0;
            actor.y = 400.0; // Distance = sqrt(300^2 + 400^2) = 500.0
            state.add_or_update_actor(actor);
        }
        
        let dist = m2_py_player_get_character_distance(500);
        assert!((dist - 500.0).abs() < 0.1);
        
        let missing_dist = m2_py_player_get_character_distance(999);
        assert_eq!(missing_dist, 0.0);
    }
}
