#[path = "../src/py_bridge_player_target.rs"]
mod py_bridge_player_target;

use crate::py_bridge_player_target::*;
use crate::character_manager_core::{get_character_manager, CharacterManagerCore, InstanceBase};
use crate::network_actor_mgr::NetworkActorData;
use crate::terrain_attr_map::TerrainAttrMap;
use crate::terrain_ffi::get_terrain_ctx;

#[test]
fn test_player_target_set_clear_get() {
    unsafe {
        assert!(m2_py_player_set_target(100));
        assert_eq!(m2_py_player_get_target_vid(), 100);
        
        assert!(m2_py_player_clear_target());
        assert_eq!(m2_py_player_get_target_vid(), 0);
    }
}

#[test]
fn test_player_get_pk_mode() {
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
    // Setup character
    let mut mgr = get_character_manager().write().unwrap();
    mgr.alive_instances.clear();
    mgr.main_actor_vid = Some(1);
    
    let main_actor = InstanceBase::new(1, 0, 10.0, 10.0, 0.0);
    mgr.alive_instances.insert(1, main_actor);
    drop(mgr);
    
    // Setup terrain map mock manually through get_terrain_ctx
    let mut terrain_ctx = get_terrain_ctx().write().unwrap();
    let mut map_data = vec![0u8; 256 * 256];
    // Coordinate 10,10 has ATTR_BANPK (4)
    map_data[10 * 256 + 10] = 4;
    
    // Format header and append data to simulate a proper AttrMap bytes
    let mut full_data = vec![0u8; 6];
    full_data[0..2].copy_from_slice(&2634u16.to_le_bytes()); // ATTRMAP_MAGIC
    full_data[2..4].copy_from_slice(&256u16.to_le_bytes()); // WIDTH
    full_data[4..6].copy_from_slice(&256u16.to_le_bytes()); // HEIGHT
    full_data.extend(map_data);
    
    let attr_map = crate::terrain_attr_parser::AttrMap::from_bytes(&full_data).unwrap();
    
    // For m2_terrain_is_attr from bridge to work, we need to adapt map to the TerrainAttrMap which uses map.is_safe_zone
    // Currently, our terrain_attr_parser.rs implements is_safe_zone, but wait... terrain_ffi_bridge_v2 relies on get_terrain_ctx().attr_map.
    terrain_ctx.attr_map = Some(TerrainAttrMap::new(256, 256));
    // Manually set attribute in TerrainAttrMap
    if let Some(ref mut attr_map) = terrain_ctx.attr_map {
        attr_map.set_attr(10, 10, crate::terrain_attr_map::ATTR_BANPK);
    }
    
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
    crate::pvp_gvg_manager_v2::PvpGvgManager::declare_guild_war(100, 200);
    
    unsafe {
        assert!(m2_py_player_is_pvp_instance(2));
    }
    
    // End guild war
    crate::pvp_gvg_manager_v2::PvpGvgManager::end_guild_war(100, 200);
    
    // Enable duel
    crate::pvp_gvg_manager_v2::PvpGvgManager::add_duel(1, 2);
    
    unsafe {
        assert!(m2_py_player_is_pvp_instance(2));
    }
}
