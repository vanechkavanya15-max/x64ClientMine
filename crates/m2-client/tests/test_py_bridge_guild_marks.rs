use m2_client::py_bridge_guild_marks::*;
use m2_client::recv_guild_logic_v2::{get_guild_state, GuildMemberData, test_clear_guild_state};

#[test]
fn test_mark_id_mappings() {
    let guild_id = 1001;
    let mark_id = m2_py_guild_guild_id_to_mark_id(guild_id);
    
    // Ensure deterministic hashing is working
    assert_eq!(m2_py_guild_guild_id_to_mark_id(guild_id), mark_id);
    assert_ne!(m2_py_guild_guild_id_to_mark_id(9999), mark_id);
    
    // bounds check
    let max_images = 5;
    let total_marks = 1280;
    assert!(mark_id < max_images * total_marks);
    
    assert_eq!(m2_py_guild_get_mark_index_by_mark_id(mark_id), mark_id % 1280);
}

#[test]
fn test_get_mark_image_filename() {
    let mut buf = [0u8; 256];
    let res = m2_py_guild_get_mark_image_filename_by_mark_id(1500, buf.as_mut_ptr() as *mut std::os::raw::c_char, buf.len());
    assert!(res);
    
    let path = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const std::os::raw::c_char).to_str().unwrap() };
    assert_eq!(path, "mark/10_1.tga");
}

#[test]
fn test_get_guild_master_name_and_is_enabled() {
    test_clear_guild_state();
    
    // initially disabled
    assert!(!m2_py_guild_is_guild_enable());
    
    let mut buf = [0u8; 64];
    let res = m2_py_guild_get_guild_master_name(buf.as_mut_ptr() as *mut std::os::raw::c_char, buf.len());
    assert!(res);
    let name = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const std::os::raw::c_char).to_str().unwrap() };
    assert_eq!(name, "Noname");

    // set up state
    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        state.master_pid = 42;
        state.members.insert(42, GuildMemberData {
            pid: 42,
            grade: 1,
            is_general: 0,
            job: 0,
            level: 50,
            offer: 100,
            name: "MasterYoda".to_string(),
        });
    }
    
    assert!(m2_py_guild_is_guild_enable());
    
    let res2 = m2_py_guild_get_guild_master_name(buf.as_mut_ptr() as *mut std::os::raw::c_char, buf.len());
    assert!(res2);
    let name2 = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const std::os::raw::c_char).to_str().unwrap() };
    assert_eq!(name2, "MasterYoda");
}
