#[path = "../src/py_bridge_nonplayer_db_full.rs"]
mod py_bridge_nonplayer_db_full;

use std::ffi::CStr;
use serial_test::serial;

use m2_client::mob_proto_binary_loader::TMobTable;
use py_bridge_nonplayer_db_full::{
    get_nonplayer_manager_runtime,
    m2_py_nonplayer_get_monster_name_full,
    m2_py_nonplayer_get_monster_level_full,
    m2_py_nonplayer_get_monster_grade_full,
    m2_py_nonplayer_get_monster_type_full,
    m2_py_nonplayer_get_monster_race_flag_full,
    m2_py_nonplayer_is_monster_aggressive_full,
};

fn create_mock_mob(vnum: u32, name: &str, level: u8, rank: u8, mtype: u8, race_flag: u32, ai_flag: u32) -> TMobTable {
    let mut mob: TMobTable = unsafe { std::mem::zeroed() };
    mob.dwVnum = vnum;
    mob.bLevel = level;
    mob.bRank = rank;
    mob.bType = mtype;
    mob.dwRaceFlag = race_flag;
    mob.dwAIFlag = ai_flag;
    
    let bytes = name.as_bytes();
    let len = bytes.len().min(mob.szLocaleName.len() - 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), mob.szLocaleName.as_mut_ptr(), len);
    }
    mob
}

fn setup_db() {
    let mut mgr = get_nonplayer_manager_runtime().lock().unwrap();
    mgr.clear();
    
    // Aggressive wild dog (rank 1, type 0)
    let dog = create_mock_mob(101, "Wild Dog", 5, 1, 0, 0x1234, 1);
    mgr.insert_mob(101, dog);
    
    // Non-aggressive NPC (rank 0, type 1)
    let npc = create_mock_mob(9001, "Weaponsmith", 1, 0, 1, 0x5678, 0);
    mgr.insert_mob(9001, npc);
}

#[test]
#[serial]
fn test_get_monster_name() {
    setup_db();
    
    let cstr_ptr = m2_py_nonplayer_get_monster_name_full(101);
    let name = unsafe { CStr::from_ptr(cstr_ptr) }.to_string_lossy();
    assert_eq!(name, "Wild Dog");
    
    let cstr_ptr_empty = m2_py_nonplayer_get_monster_name_full(9999);
    let empty_name = unsafe { CStr::from_ptr(cstr_ptr_empty) }.to_string_lossy();
    assert_eq!(empty_name, "");
}

#[test]
#[serial]
fn test_get_monster_basic_attributes() {
    setup_db();
    
    assert_eq!(m2_py_nonplayer_get_monster_level_full(101), 5);
    assert_eq!(m2_py_nonplayer_get_monster_grade_full(101), 1);
    assert_eq!(m2_py_nonplayer_get_monster_type_full(101), 0);
    
    assert_eq!(m2_py_nonplayer_get_monster_level_full(9001), 1);
    assert_eq!(m2_py_nonplayer_get_monster_grade_full(9001), 0);
    assert_eq!(m2_py_nonplayer_get_monster_type_full(9001), 1);
    
    // Nonexistent
    assert_eq!(m2_py_nonplayer_get_monster_level_full(9999), 0);
    assert_eq!(m2_py_nonplayer_get_monster_grade_full(9999), 0);
    assert_eq!(m2_py_nonplayer_get_monster_type_full(9999), 0);
}

#[test]
#[serial]
fn test_get_monster_race_flag() {
    setup_db();
    
    assert_eq!(m2_py_nonplayer_get_monster_race_flag_full(101), 0x1234);
    assert_eq!(m2_py_nonplayer_get_monster_race_flag_full(9001), 0x5678);
    assert_eq!(m2_py_nonplayer_get_monster_race_flag_full(9999), 0);
}

#[test]
#[serial]
fn test_is_monster_aggressive() {
    setup_db();
    
    assert_eq!(m2_py_nonplayer_is_monster_aggressive_full(101), true);
    assert_eq!(m2_py_nonplayer_is_monster_aggressive_full(9001), false);
    assert_eq!(m2_py_nonplayer_is_monster_aggressive_full(9999), false);
}
