use m2_client::py_bridge_guild::*;
use m2_client::recv_guild_logic_v2::{get_guild_state, test_clear_guild_state, GuildMemberData};

#[test]
fn test_py_bridge_guild_all() {
    test_clear_guild_state();
    
    // Default state: disabled, should return 0
    assert_eq!(m2_py_guild_get_guild_id(), 0);
    
    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        state.guild_id = 12345;
    }
    
    assert_eq!(m2_py_guild_get_guild_id(), 12345);

    // --- test_py_guild_get_guild_name ---
    test_clear_guild_state();
    test_clear_guild_state();
    let mut buf = [0u8; 32];
    
    // Default state: disabled, should return false
    assert_eq!(m2_py_guild_get_guild_name(buf.as_mut_ptr(), buf.len()), false);
    
    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        state.name = "MyGuild".to_string();
    }
    
    assert_eq!(m2_py_guild_get_guild_name(buf.as_mut_ptr(), buf.len()), true);
    
    let c_str = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const i8) };
    assert_eq!(c_str.to_str().unwrap(), "MyGuild");
    
    // Boundary check: short buffer
    let mut short_buf = [0u8; 3];
    assert_eq!(m2_py_guild_get_guild_name(short_buf.as_mut_ptr(), short_buf.len()), true);
    let short_c_str = unsafe { std::ffi::CStr::from_ptr(short_buf.as_ptr() as *const i8) };
    assert_eq!(short_c_str.to_str().unwrap(), "My");
    
    // Boundary check: null ptr
    assert_eq!(m2_py_guild_get_guild_name(std::ptr::null_mut(), 32), false);
    
    // Boundary check: zero len
    assert_eq!(m2_py_guild_get_guild_name(buf.as_mut_ptr(), 0), false);

    // --- test_py_guild_get_master_name ---
    test_clear_guild_state();
    let mut buf = [0u8; 32];
    
    // Default state: disabled, should return false
    assert_eq!(m2_py_guild_get_master_name(buf.as_mut_ptr(), buf.len()), false);
    
    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        state.master_pid = 777;
        
        let mut mem = GuildMemberData::default();
        mem.pid = 777;
        mem.name = "MasterYoda".to_string();
        state.members.insert(777, mem);
    }
    
    assert_eq!(m2_py_guild_get_master_name(buf.as_mut_ptr(), buf.len()), true);
    let c_str = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const i8) };
    assert_eq!(c_str.to_str().unwrap(), "MasterYoda");
    
    // Test missing master member logic (returns empty string)
    {
        let mut state = get_guild_state().lock().unwrap();
        state.members.clear();
    }
    assert_eq!(m2_py_guild_get_master_name(buf.as_mut_ptr(), buf.len()), true);
    let c_str_empty = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const i8) };
    assert_eq!(c_str_empty.to_str().unwrap(), "");
    
    // Null pointer test
    assert_eq!(m2_py_guild_get_master_name(std::ptr::null_mut(), 32), false);

    // --- test_py_guild_get_guild_level ---
    test_clear_guild_state();
    
    assert_eq!(m2_py_guild_get_guild_level(), 0);
    
    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        state.level = 20;
    }
    
    assert_eq!(m2_py_guild_get_guild_level(), 20);

    // --- test_py_guild_get_guild_experience ---
    test_clear_guild_state();
    
    assert_eq!(m2_py_guild_get_guild_experience(), 0);
    
    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        state.exp = 999999;
    }
    
    assert_eq!(m2_py_guild_get_guild_experience(), 999999);

    // --- test_py_guild_get_member_count ---
    test_clear_guild_state();
    
    assert_eq!(m2_py_guild_get_member_count(), 0);
    
    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        state.member_count = 42;
    }
    
    assert_eq!(m2_py_guild_get_member_count(), 42);
}
