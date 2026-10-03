use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;
use crate::recv_guild_logic_v2::{get_guild_state, GUILD_SKILL_MAX_NUM};

// Authority Flags matching C++ SourceCodeClient
pub const GUILD_AUTH_INVITE: u8 = 1 << 0; // In C++: GUILD_AUTH_ADD_MEMBER
pub const GUILD_AUTH_NOTICE: u8 = 1 << 2;
pub const GUILD_AUTH_SKILL: u8 = 1 << 3;
pub const GUILD_AUTH_WAR: u8 = 1 << 4; // Typical for Metin2

/// Python `guild.GetGuildSkillLevel(skillIndex)`
#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_skill_level(vnum: u32) -> i32 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }
        
        let index = vnum as usize;
        if index < GUILD_SKILL_MAX_NUM {
            state.skills.skill_level[index] as i32
        } else {
            0
        }
    })
}

/// Python `guild.GetGuildSkillPoint()`
#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_skill_point() -> i32 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }
        
        state.skills.skill_point as i32
    })
}

/// Python `guild.MainPlayerHasAuthority(authorityFlag)`
#[no_mangle]
pub extern "C" fn m2_py_guild_main_player_has_authority(auth_flag: u8) -> bool {
    safe_ffi_call(false, || {
        let guild_state = get_guild_state().lock().unwrap();
        if !guild_state.is_enabled {
            return false;
        }

        let game_state = GLOBAL_GAME_STATE.read().unwrap();
        let main_vid = game_state.get_player_vid();
        
        // Find the main player's name
        let main_player_name = if let Some(actor) = game_state.get_actor(main_vid) {
            actor.name.clone()
        } else {
            return false;
        };

        // Find the member data in guild by name
        let member = guild_state.members.values().find(|m| m.name == main_player_name);
        
        if let Some(member_data) = member {
            // Find grade data
            if let Some(grade_data) = guild_state.grades.get(&member_data.grade) {
                return (grade_data.auth_flag & auth_flag) == auth_flag;
            }
        }
        
        false
    })
}

/// Python `guild.HasGuildLand()`
#[no_mangle]
pub extern "C" fn m2_py_guild_has_guild_land() -> bool {
    safe_ffi_call(false, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return false;
        }
        
        state.has_land != 0
    })
}
