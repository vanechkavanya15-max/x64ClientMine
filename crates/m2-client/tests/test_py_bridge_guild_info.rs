#[path = "../src/py_bridge_guild_info.rs"]
mod py_bridge_guild_info;

use m2_client::recv_guild_logic_v2::{get_guild_state, test_clear_guild_state, GuildMemberData, GuildGradeData};
use std::ffi::CStr;

fn reset_and_setup_guild_state() {
    test_clear_guild_state();
    let mut state = get_guild_state().lock().unwrap();
    state.is_enabled = true;
    state.level = 10;
    state.exp = 100000;
    state.member_count = 5;
    state.max_member_count = 50;

    state.members.insert(1, GuildMemberData {
        pid: 1,
        grade: 1,
        is_general: 1,
        job: 2,
        level: 50,
        offer: 1000,
        name: "Leader".to_string(),
    });
    
    state.members.insert(2, GuildMemberData {
        pid: 2,
        grade: 2,
        is_general: 0,
        job: 1,
        level: 40,
        offer: 500,
        name: "Member1".to_string(),
    });

    state.grades.insert(1, GuildGradeData {
        auth_flag: 15,
        name: "Master".to_string(),
    });
    
    state.grades.insert(2, GuildGradeData {
        auth_flag: 0,
        name: "Rookie".to_string(),
    });
}

#[test]
fn test_m2_py_guild_get_guild_level() {
    reset_and_setup_guild_state();
    let level = py_bridge_guild_info::m2_py_guild_get_guild_level();
    assert_eq!(level, 10);
    
    test_clear_guild_state();
    let level = py_bridge_guild_info::m2_py_guild_get_guild_level();
    assert_eq!(level, 0); // Not enabled
}

#[test]
fn test_m2_py_guild_get_guild_experience() {
    reset_and_setup_guild_state();
    let mut exp = 0;
    let mut last_exp = 0;
    
    let result = py_bridge_guild_info::m2_py_guild_get_guild_experience(&mut exp, &mut last_exp);
    assert!(result);
    assert_eq!(exp, 100000);
    // GUILD_EXP_LIST[10] = 334000
    // last_exp = 334000 - 100000 = 234000
    assert_eq!(last_exp, 234000);
}

#[test]
fn test_m2_py_guild_get_guild_experience_max_level() {
    reset_and_setup_guild_state();
    {
        let mut state = get_guild_state().lock().unwrap();
        state.level = 20;
    }
    let mut exp = 1;
    let mut last_exp = 1;
    
    let result = py_bridge_guild_info::m2_py_guild_get_guild_experience(&mut exp, &mut last_exp);
    assert!(result);
    assert_eq!(exp, 0);
    assert_eq!(last_exp, 0);
}

#[test]
fn test_m2_py_guild_get_guild_member_count() {
    reset_and_setup_guild_state();
    let mut count = 0;
    let mut max_count = 0;
    
    let result = py_bridge_guild_info::m2_py_guild_get_guild_member_count(&mut count, &mut max_count);
    assert!(result);
    assert_eq!(count, 5);
    assert_eq!(max_count, 50);
}

#[test]
fn test_m2_py_guild_get_member_data_success() {
    reset_and_setup_guild_state();
    
    let mut pid = 0;
    let mut name = [0u8; 32];
    let mut grade = 0;
    let mut job = 0;
    let mut level = 0;
    let mut offer = 0;
    let mut general_flag = 0;
    
    // Sorts by grade. Leader (grade 1) should be at index 0
    let result = py_bridge_guild_info::m2_py_guild_get_member_data(
        0,
        &mut pid,
        name.as_mut_ptr(),
        name.len(),
        &mut grade,
        &mut job,
        &mut level,
        &mut offer,
        &mut general_flag,
    );
    
    assert!(result);
    assert_eq!(pid, 1);
    assert_eq!(grade, 1);
    assert_eq!(job, 2);
    assert_eq!(level, 50);
    assert_eq!(offer, 1000);
    assert_eq!(general_flag, 1);
    
    let c_name = unsafe { CStr::from_ptr(name.as_ptr() as *const i8) }.to_str().unwrap();
    assert_eq!(c_name, "Leader");
}

#[test]
fn test_m2_py_guild_get_member_data_out_of_bounds() {
    reset_and_setup_guild_state();
    
    let mut pid = 0;
    let mut name = [0u8; 32];
    let mut grade = 0;
    let mut job = 0;
    let mut level = 0;
    let mut offer = 0;
    let mut general_flag = 0;
    
    let result = py_bridge_guild_info::m2_py_guild_get_member_data(
        5, // Only 2 members
        &mut pid,
        name.as_mut_ptr(),
        name.len(),
        &mut grade,
        &mut job,
        &mut level,
        &mut offer,
        &mut general_flag,
    );
    
    assert!(!result);
}

#[test]
fn test_m2_py_guild_get_grade_data_success() {
    reset_and_setup_guild_state();
    
    let mut name = [0u8; 32];
    let mut auth_flag = 0;
    
    let result = py_bridge_guild_info::m2_py_guild_get_grade_data(
        1,
        name.as_mut_ptr(),
        name.len(),
        &mut auth_flag,
    );
    
    assert!(result);
    assert_eq!(auth_flag, 15);
    
    let c_name = unsafe { CStr::from_ptr(name.as_ptr() as *const i8) }.to_str().unwrap();
    assert_eq!(c_name, "Master");
}

#[test]
fn test_m2_py_guild_get_grade_name_success() {
    reset_and_setup_guild_state();
    
    let mut name = [0u8; 32];
    
    let result = py_bridge_guild_info::m2_py_guild_get_grade_name(
        2,
        name.as_mut_ptr(),
        name.len(),
    );
    
    assert!(result);
    
    let c_name = unsafe { CStr::from_ptr(name.as_ptr() as *const i8) }.to_str().unwrap();
    assert_eq!(c_name, "Rookie");
}
