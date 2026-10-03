#[path = "../src/py_bridge_guild_skills.rs"]
mod py_bridge_guild_skills;

use py_bridge_guild_skills::*;
use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;
use crate::recv_guild_logic_v2::{get_guild_state, GuildGradeData, GuildMemberData};
use crate::game_state_unified::ActorInstanceData;
use serial_test::serial;

fn setup_test_state() {
    let mut guild = get_guild_state().lock().unwrap();
    guild.is_enabled = true;
    guild.has_land = 1;
    guild.skills.skill_point = 5;
    guild.skills.skill_level = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0];
    
    guild.members.clear();
    guild.members.insert(1001, GuildMemberData {
        pid: 1001,
        grade: 2,
        is_general: 0,
        job: 1,
        level: 50,
        offer: 1000,
        name: "TestPlayer".to_string(),
    });
    
    guild.grades.clear();
    guild.grades.insert(2, GuildGradeData {
        auth_flag: GUILD_AUTH_SKILL | GUILD_AUTH_NOTICE,
        name: "Officer".to_string(),
    });

    let game = GLOBAL_GAME_STATE.write().unwrap();
    // Simulate player selection
    game.set_player_vid(1001);
    
    let mut game_write = GLOBAL_GAME_STATE.write().unwrap();
    let mut actors = game_write.actors.write().unwrap();
    actors.insert(1001, ActorInstanceData {
        vid: 1001,
        name: "TestPlayer".to_string(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
        rotation: 0.0,
    });
}

fn clear_test_state() {
    let mut guild = get_guild_state().lock().unwrap();
    guild.is_enabled = false;
    guild.has_land = 0;
    guild.skills.skill_point = 0;
    guild.skills.skill_level = [0; 12];
    guild.members.clear();
    guild.grades.clear();
    
    let game_write = GLOBAL_GAME_STATE.write().unwrap();
    let mut actors = game_write.actors.write().unwrap();
    actors.clear();
}

#[test]
#[serial]
fn test_m2_py_guild_get_guild_skill_level_valid() {
    setup_test_state();
    assert_eq!(m2_py_guild_get_guild_skill_level(0), 1);
    assert_eq!(m2_py_guild_get_guild_skill_level(3), 4);
    clear_test_state();
}

#[test]
#[serial]
fn test_m2_py_guild_get_guild_skill_level_invalid_index() {
    setup_test_state();
    assert_eq!(m2_py_guild_get_guild_skill_level(15), 0);
    clear_test_state();
}

#[test]
#[serial]
fn test_m2_py_guild_get_guild_skill_level_disabled() {
    setup_test_state();
    {
        let mut guild = get_guild_state().lock().unwrap();
        guild.is_enabled = false;
    }
    assert_eq!(m2_py_guild_get_guild_skill_level(0), 0);
    clear_test_state();
}

#[test]
#[serial]
fn test_m2_py_guild_get_guild_skill_point() {
    setup_test_state();
    assert_eq!(m2_py_guild_get_guild_skill_point(), 5);
    
    {
        let mut guild = get_guild_state().lock().unwrap();
        guild.is_enabled = false;
    }
    assert_eq!(m2_py_guild_get_guild_skill_point(), 0);
    clear_test_state();
}

#[test]
#[serial]
fn test_m2_py_guild_has_guild_land() {
    setup_test_state();
    assert_eq!(m2_py_guild_has_guild_land(), true);
    
    {
        let mut guild = get_guild_state().lock().unwrap();
        guild.has_land = 0;
    }
    assert_eq!(m2_py_guild_has_guild_land(), false);
    
    {
        let mut guild = get_guild_state().lock().unwrap();
        guild.has_land = 1;
        guild.is_enabled = false;
    }
    assert_eq!(m2_py_guild_has_guild_land(), false);
    clear_test_state();
}

#[test]
#[serial]
fn test_m2_py_guild_main_player_has_authority() {
    setup_test_state();
    
    assert_eq!(m2_py_guild_main_player_has_authority(GUILD_AUTH_SKILL), true);
    assert_eq!(m2_py_guild_main_player_has_authority(GUILD_AUTH_NOTICE), true);
    assert_eq!(m2_py_guild_main_player_has_authority(GUILD_AUTH_INVITE), false);
    assert_eq!(m2_py_guild_main_player_has_authority(GUILD_AUTH_WAR), false);
    
    // Check with missing grade
    {
        let mut guild = get_guild_state().lock().unwrap();
        guild.grades.clear();
    }
    assert_eq!(m2_py_guild_main_player_has_authority(GUILD_AUTH_SKILL), false);
    
    clear_test_state();
}

#[test]
#[serial]
fn test_m2_py_guild_main_player_has_authority_no_guild() {
    setup_test_state();
    {
        let mut guild = get_guild_state().lock().unwrap();
        guild.is_enabled = false;
    }
    assert_eq!(m2_py_guild_main_player_has_authority(GUILD_AUTH_SKILL), false);
    clear_test_state();
}
