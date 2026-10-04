use std::ffi::c_char;
use serial_test::serial;

use m2_client::py_bridge_nonplayer_data_complete::*;
use m2_client::mob_proto_binary_loader::TMobTable;

fn set_string(dst: &mut [u8], src: &str) {
    let bytes = src.as_bytes();
    let len = bytes.len().min(dst.len() - 1);
    dst[..len].copy_from_slice(&bytes[..len]);
    dst[len] = 0;
}

#[test]
#[serial]
fn test_py_nonplayer_data_empty() {
    // Ensure state is clear
    get_nonplayer_manager_runtime().lock().unwrap().clear();

    // Verify empty responses
    let mut out_buf = [0i8; 64];
    
    assert_eq!(
        m2_py_nonplayer_get_monster_name_complete(101, out_buf.as_mut_ptr() as *mut c_char, out_buf.len()),
        false
    );
    
    assert_eq!(m2_py_nonplayer_get_monster_level_complete(101), 0);
    assert_eq!(m2_py_nonplayer_get_monster_grade_complete(101), 0);
    assert_eq!(m2_py_nonplayer_get_monster_type_complete(101), 0);
    assert_eq!(m2_py_nonplayer_get_monster_color_complete(101), 0);
}

#[test]
#[serial]
fn test_py_nonplayer_data_read() {
    let mut mgr = get_nonplayer_manager_runtime().lock().unwrap();
    mgr.clear();
    
    // Create Wild Dog
    let mut mob_wild_dog: TMobTable = unsafe { std::mem::zeroed() };
    mob_wild_dog.dwVnum = 101;
    set_string(&mut mob_wild_dog.szLocaleName, "Wild Dog");
    mob_wild_dog.bLevel = 1;
    mob_wild_dog.bRank = 0; // RANK_PAWN
    mob_wild_dog.bType = 0; // TYPE_MONSTER
    mob_wild_dog.dwMonsterColor = 0xFFFFFFFF; // Whiteish
    
    mgr.insert_mob(101, mob_wild_dog);

    // Create NPC Guard
    let mut mob_guard: TMobTable = unsafe { std::mem::zeroed() };
    mob_guard.dwVnum = 20011;
    set_string(&mut mob_guard.szLocaleName, "City Guard");
    mob_guard.bLevel = 15;
    mob_guard.bRank = 4; // RANK_KING (Boss/Important)
    mob_guard.bType = 1; // TYPE_NPC
    mob_guard.dwMonsterColor = 0xFF00FF00; // Greenish
    
    mgr.insert_mob(20011, mob_guard);
    
    // Drop lock to let FFI functions access it
    drop(mgr);
    
    // Test Wild Dog
    let mut out_buf = [0u8; 64];
    assert_eq!(
        m2_py_nonplayer_get_monster_name_complete(101, out_buf.as_mut_ptr() as *mut c_char, out_buf.len()),
        true
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(out_buf.as_ptr() as *const c_char) };
    assert_eq!(cstr.to_str().unwrap(), "Wild Dog");
    
    assert_eq!(m2_py_nonplayer_get_monster_level_complete(101), 1);
    assert_eq!(m2_py_nonplayer_get_monster_grade_complete(101), 0);
    assert_eq!(m2_py_nonplayer_get_monster_type_complete(101), 0);
    assert_eq!(m2_py_nonplayer_get_monster_color_complete(101), 0xFFFFFFFF);
    
    // Test Guard
    let mut out_buf2 = [0u8; 64];
    assert_eq!(
        m2_py_nonplayer_get_monster_name_complete(20011, out_buf2.as_mut_ptr() as *mut c_char, out_buf2.len()),
        true
    );
    let cstr2 = unsafe { std::ffi::CStr::from_ptr(out_buf2.as_ptr() as *const c_char) };
    assert_eq!(cstr2.to_str().unwrap(), "City Guard");
    
    assert_eq!(m2_py_nonplayer_get_monster_level_complete(20011), 15);
    assert_eq!(m2_py_nonplayer_get_monster_grade_complete(20011), 4);
    assert_eq!(m2_py_nonplayer_get_monster_type_complete(20011), 1);
    assert_eq!(m2_py_nonplayer_get_monster_color_complete(20011), 0xFF00FF00);
}

#[test]
#[serial]
fn test_py_nonplayer_data_clear() {
    let mut mgr = get_nonplayer_manager_runtime().lock().unwrap();
    mgr.clear();
    
    let mut mob: TMobTable = unsafe { std::mem::zeroed() };
    mob.dwVnum = 102;
    set_string(&mut mob.szLocaleName, "Wolf");
    mob.bLevel = 2;
    mgr.insert_mob(102, mob);
    
    assert!(mgr.get_mob(102).is_some());
    mgr.clear();
    assert!(mgr.get_mob(102).is_none());
}

#[test]
#[serial]
fn test_py_nonplayer_get_non_existent() {
    let mut mgr = get_nonplayer_manager_runtime().lock().unwrap();
    mgr.clear();
    
    let mut mob: TMobTable = unsafe { std::mem::zeroed() };
    mob.dwVnum = 101;
    set_string(&mut mob.szLocaleName, "Wild Dog");
    mgr.insert_mob(101, mob);
    drop(mgr);

    let mut out_buf = [0i8; 64];
    
    // Check non existent
    assert_eq!(
        m2_py_nonplayer_get_monster_name_complete(999, out_buf.as_mut_ptr() as *mut c_char, out_buf.len()),
        false
    );
    assert_eq!(m2_py_nonplayer_get_monster_level_complete(999), 0);
}
