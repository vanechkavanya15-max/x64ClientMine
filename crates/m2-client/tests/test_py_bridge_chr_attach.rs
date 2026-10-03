use std::ffi::CString;

#[path = "../src/py_bridge_chr_attach.rs"]
mod py_bridge_chr_attach;

use py_bridge_chr_attach::*;
use crate::character_manager_core::get_character_manager;
use crate::event_effect_attach_v2::get_effect_manager;
use crate::py_bridge_item_nonplayer_runtime::get_nonplayer_manager_runtime;
use crate::mob_proto_binary_loader::TMobTable;
use crate::nonplayer_data::{TYPE_NPC, TYPE_MONSTER};
use crate::network_actor_mgr::{NetworkActorData, AffectFlagContainer};

fn setup_test_environment() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
}

#[test]
fn test_attach_effect_by_id() {
    setup_test_environment();
    
    // Create an instance and select it
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.create_instance_full(100, 0, 0.0, 0.0, 0.0, "Test".to_string(), false);
        mgr.main_actor_vid = Some(100);
    }
    
    // Clear effects to ensure clean state
    {
        let mut effect_mgr = get_effect_manager();
        effect_mgr.clear_events(100);
    }
    
    let bone_name = CString::new("Bip01 Spine").unwrap();
    let result = m2_py_chr_attach_effect_by_id(0, bone_name.as_ptr(), 42, 0);
    
    assert!(result, "m2_py_chr_attach_effect_by_id failed");
    
    let mut effect_mgr = get_effect_manager();
    assert!(effect_mgr.actors.contains_key(&100), "Actor 100 should have effects");
    let effects = &effect_mgr.actors.get(&100).unwrap().attached_effects;
    assert_eq!(effects.len(), 1);
    
    let effect = &effects[0];
    assert_eq!(effect.effect_index, 42);
    assert_eq!(effect.is_attaching, true);
    assert_eq!(effect.is_following, true);
    assert_eq!(effect.bone_name, "Bip01 Spine");
}

#[test]
fn test_attach_effect_by_name() {
    setup_test_environment();
    
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.create_instance_full(200, 0, 0.0, 0.0, 0.0, "Test".to_string(), false);
        mgr.main_actor_vid = Some(200);
    }
    
    let bone_name = CString::new("Bip01 Head").unwrap();
    let effect_name = CString::new("d:\\ymir work\\effect\\etc\\buff.mse").unwrap();
    
    let result = m2_py_chr_attach_effect_by_name(0, bone_name.as_ptr(), effect_name.as_ptr(), 0);
    
    assert!(result, "m2_py_chr_attach_effect_by_name failed");
    
    let mut effect_mgr = get_effect_manager();
    assert!(effect_mgr.actors.contains_key(&200));
    let effects = &effect_mgr.actors.get(&200).unwrap().attached_effects;
    assert_eq!(effects.len(), 1);
    
    let effect = &effects[0];
    assert_eq!(effect.is_attaching, true);
    assert_eq!(effect.bone_name, "Bip01 Head");
}

#[test]
fn test_mount_dismount_horse() {
    setup_test_environment();
    
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.create_instance_full(300, 0, 0.0, 0.0, 0.0, "Test".to_string(), false);
        mgr.main_actor_vid = Some(300);
    }
    
    assert!(m2_py_chr_mount_horse());
    
    {
        let mgr = get_character_manager().read().unwrap();
        let inst = mgr.get_instance(300).unwrap();
        assert_eq!(inst.appearance.get_motion_mode() != crate::actor_appearance::MODE_HORSE && inst.appearance.get_motion_mode() != crate::actor_appearance::MODE_GENERAL, false);
    }
    
    assert!(m2_py_chr_dismount_horse());
    
}

#[test]
fn test_is_enemy() {
    setup_test_environment();
    
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.create_instance_full(400, 101, 0.0, 0.0, 0.0, "Dog".to_string(), false);
    }
    
    {
        let mut npc_mgr = get_nonplayer_manager_runtime().lock().unwrap();
        let mut mob: TMobTable = unsafe { std::mem::zeroed() };
        mob.dwVnum = 101;
        mob.bType = TYPE_MONSTER;
        npc_mgr.insert_mob(101, mob);
    }
    
    let is_enemy = m2_py_chr_is_enemy(400);
    assert!(is_enemy, "Mob with TYPE_MONSTER should be an enemy");
}

#[test]
fn test_is_npc() {
    setup_test_environment();
    
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.create_instance_full(500, 2001, 0.0, 0.0, 0.0, "Shop".to_string(), false);
    }
    
    {
        let mut npc_mgr = get_nonplayer_manager_runtime().lock().unwrap();
        let mut mob: TMobTable = unsafe { std::mem::zeroed() };
        mob.dwVnum = 2001;
        mob.bType = TYPE_NPC;
        npc_mgr.insert_mob(2001, mob);
    }
    
    let is_npc = m2_py_chr_is_npc(500);
    assert!(is_npc, "Mob with TYPE_NPC should be an NPC");
}

#[test]
fn test_is_game_master() {
    setup_test_environment();
    
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.create_instance_full(600, 0, 0.0, 0.0, 0.0, "Admin".to_string(), false);
        
        let inst = mgr.get_instance_mut(600).unwrap();
        
        let mut net_data = NetworkActorData {
            name: "Admin".to_string(),
            affect_flags: AffectFlagContainer::new(),
            b_type: 0,
            vid: 600,
            state_flags: 0,
            empire_id: 0,
            race: 0,
            mov_spd: 0,
            atk_spd: 0,
            rot: 0.0,
            cur_x: 0,
            cur_y: 0,
            cur_z: 0,
            src_x: 0,
            src_y: 0,
        };
        // Set AFFECT_YMIR bit (bit 0)
        net_data.affect_flags.flags[0] |= 1 << 0;
        
        inst.network_data = Some(net_data);
    }
    
    assert!(m2_py_chr_is_game_master(600), "Player with AFFECT_YMIR should be Game Master");
    
    // Test fallback [GM] name
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.create_instance_full(601, 0, 0.0, 0.0, 0.0, "[GM]Test".to_string(), false);
    }
    
    assert!(m2_py_chr_is_game_master(601), "Player with [GM] prefix should be Game Master");
}
