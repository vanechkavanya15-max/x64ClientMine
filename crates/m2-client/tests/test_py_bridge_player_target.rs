use m2_client::py_bridge_player_target::*;
use m2_client::character_manager_core::{get_character_manager, InstanceBase};
use m2_client::network_actor_mgr::NetworkActorData;
use m2_client::terrain_attr_map::TerrainAttrMap;
use m2_client::terrain_ffi::get_terrain_ctx;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_player_target_set_clear_get() {
    let _guard = TEST_MUTEX.lock().unwrap();
    unsafe {
        assert!(m2_py_player_set_target(100));
        assert_eq!(m2_py_player_get_target_vid(), 100);
        
        assert!(m2_py_player_clear_target());
        assert_eq!(m2_py_player_get_target_vid(), 0);
    }
}

#[test]
fn test_player_get_pk_mode() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let mut mgr = get_character_manager().write().unwrap();
    mgr.alive_instances.clear();
    mgr.main_actor_vid = Some(1);
    
    let mut main_actor = InstanceBase::new(1, 0, 0.0, 0.0, 0.0);
    let mut net_data = NetworkActorData::new();
    net_data.pk_mode = 3;
    main_actor.network_data = Some(net_data);
    
    mgr.alive_instances.insert(1, main_actor);
    drop(mgr);
    
    unsafe {
        assert_eq!(m2_py_player_get_pk_mode(), 3);
    }
}

#[test]
fn test_player_is_same_empire() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let mut mgr = get_character_manager().write().unwrap();
    mgr.alive_instances.clear();
    mgr.main_actor_vid = Some(1);
    
    let mut main_actor = InstanceBase::new(1, 0, 0.0, 0.0, 0.0);
    let mut main_net = NetworkActorData::new();
    main_net.empire_id = 2; // Chunjo
    main_actor.network_data = Some(main_net);
    
    let mut target_actor_same = InstanceBase::new(2, 0, 0.0, 0.0, 0.0);
    let mut target_net_same = NetworkActorData::new();
    target_net_same.empire_id = 2; // Chunjo
    target_actor_same.network_data = Some(target_net_same);
    
    let mut target_actor_diff = InstanceBase::new(3, 0, 0.0, 0.0, 0.0);
    let mut target_net_diff = NetworkActorData::new();
    target_net_diff.empire_id = 3; // Jinno
    target_actor_diff.network_data = Some(target_net_diff);
    
    let mut target_actor_gm = InstanceBase::new(4, 0, 0.0, 0.0, 0.0);
    let mut target_net_gm = NetworkActorData::new();
    target_net_gm.empire_id = 0; // Game Master (empire 0 implies always true or GM)
    target_actor_gm.network_data = Some(target_net_gm);
    
    mgr.alive_instances.insert(1, main_actor);
    mgr.alive_instances.insert(2, target_actor_same);
    mgr.alive_instances.insert(3, target_actor_diff);
    mgr.alive_instances.insert(4, target_actor_gm);
    
    drop(mgr);
    
    unsafe {
        assert!(m2_py_player_is_same_empire(2)); // Same empire
        assert!(!m2_py_player_is_same_empire(3)); // Different empire
        assert!(m2_py_player_is_same_empire(4)); // Empire 0 (Game master/neutral -> always same)
        assert!(!m2_py_player_is_same_empire(999)); // Unknown target
    }
}

#[test]
fn test_player_is_in_safe_area() {
    let _guard = TEST_MUTEX.lock().unwrap();
    // Setup character
    let mut mgr = get_character_manager().write().unwrap();
    mgr.alive_instances.clear();
    mgr.main_actor_vid = Some(1);
    
    let main_actor = InstanceBase::new(1, 0, 10.0, 10.0, 0.0);
    mgr.alive_instances.insert(1, main_actor);
    drop(mgr);
    
    // Setup terrain map mock through get_terrain_ctx
    let mut terrain_ctx = get_terrain_ctx().write().unwrap();
    let mut map_data = vec![0u8; 256 * 256];
    // Coordinate 10,10 has ATTR_BANPK (4)
    map_data[10 * 256 + 10] = m2_client::terrain_attr_map::ATTR_BANPK;
    
    terrain_ctx.attr_map = Some(TerrainAttrMap::new(256, 256, map_data).unwrap());
    drop(terrain_ctx);
    
    unsafe {
        assert!(m2_py_player_is_in_safe_area());
    }
    
    // Move out of safe area
    let mut mgr = get_character_manager().write().unwrap();
    if let Some(actor) = mgr.alive_instances.get_mut(&1) {
        actor.x = 20.0;
        actor.y = 20.0;
    }
    drop(mgr);
    
    unsafe {
        assert!(!m2_py_player_is_in_safe_area());
    }
}

#[test]
fn test_player_is_pvp_instance() {
    let _guard = TEST_MUTEX.lock().unwrap();
    {
        let mut pvp_mgr = m2_client::pvp_gvg_manager_v2::get_pvp_gvg_manager().write().unwrap();
        pvp_mgr.clear_pvp_key_system();
    }
    let mut mgr = get_character_manager().write().unwrap();
    mgr.alive_instances.clear();
    mgr.main_actor_vid = Some(1);
    
    let mut main_actor = InstanceBase::new(1, 0, 0.0, 0.0, 0.0);
    let mut main_net = NetworkActorData::new();
    main_net.guild_id = 100;
    main_actor.network_data = Some(main_net);
    
    let mut target_actor = InstanceBase::new(2, 0, 0.0, 0.0, 0.0);
    let mut target_net = NetworkActorData::new();
    target_net.guild_id = 200;
    target_actor.network_data = Some(target_net);
    
    mgr.alive_instances.insert(1, main_actor);
    mgr.alive_instances.insert(2, target_actor);
    drop(mgr);
    
    // By default, no PVP
    unsafe {
        assert!(!m2_py_player_is_pvp_instance(2));
    }
    
    // Enable guild war
    {
        let mut pvp_mgr = m2_client::pvp_gvg_manager_v2::get_pvp_gvg_manager().write().unwrap();
        pvp_mgr.insert_gvg_key(100, 200);
    }
    
    unsafe {
        assert!(m2_py_player_is_pvp_instance(2));
    }
    
    // End guild war
    {
        let mut pvp_mgr = m2_client::pvp_gvg_manager_v2::get_pvp_gvg_manager().write().unwrap();
        pvp_mgr.remove_gvg_key(100, 200);
    }
    
    // Enable duel
    {
        let mut pvp_mgr = m2_client::pvp_gvg_manager_v2::get_pvp_gvg_manager().write().unwrap();
        pvp_mgr.insert_duel_key(1, 2);
    }
    
    unsafe {
        assert!(m2_py_player_is_pvp_instance(2));
    }
}
