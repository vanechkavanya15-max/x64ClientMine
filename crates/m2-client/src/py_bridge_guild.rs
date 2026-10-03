use crate::recv_guild_logic_v2::get_guild_state;
use m2_client_bridge::ffi_guard::safe_ffi_call;

/// Python `guild.GetGuildID()`
/// Returns the current guild ID, or 0 if not enabled.
#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_id() -> u32 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }
        state.guild_id
    })
}

/// Python `guild.GetGuildName()`
/// Copies the guild name to `out_buf` (up to `max_len` bytes including null terminator).
/// Returns true if successful and enabled.
#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_name(out_buf: *mut u8, max_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || max_len == 0 {
            return false;
        }
        
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return false;
        }
        
        let name_bytes = state.name.as_bytes();
        let copy_len = name_bytes.len().min(max_len - 1);
        
        unsafe {
            std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), out_buf, copy_len);
            *out_buf.add(copy_len) = 0; // null terminator
        }
        
        true
    })
}

/// Python `guild.GetMasterName()`
/// Copies the guild master's name to `out_buf`.
/// Returns true if successful and enabled.
#[no_mangle]
pub extern "C" fn m2_py_guild_get_master_name(out_buf: *mut u8, max_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || max_len == 0 {
            return false;
        }

        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return false;
        }
        
        let master_name = state.members.get(&state.master_pid)
            .map(|m| m.name.as_str())
            .unwrap_or("");
            
        let name_bytes = master_name.as_bytes();
        let copy_len = name_bytes.len().min(max_len - 1);
        
        unsafe {
            std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), out_buf, copy_len);
            *out_buf.add(copy_len) = 0;
        }
        
        true
    })
}

/// Python `guild.GetGuildLevel()`
/// Returns the current guild level, or 0 if not enabled.
#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_level_legacy() -> u8 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }
        state.level
    })
}

/// Python `guild.GetGuildExperience()`
/// Returns the current guild experience, or 0 if not enabled.
#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_experience_legacy() -> u32 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }
        state.exp
    })
}

/// Python `guild.GetMemberCount()`
/// Returns the current member count, or 0 if not enabled.
#[no_mangle]
pub extern "C" fn m2_py_guild_get_member_count() -> u16 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }
        state.member_count
    })
}
