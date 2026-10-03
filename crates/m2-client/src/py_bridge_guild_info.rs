use m2_client::recv_guild_logic_v2::{get_guild_state, GuildMemberData};
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::ptr;

// Return types must be correctly matching what Python bridging C API expects.
// `GetGuildLastExp` in C++ Locale.cpp returns `unsigned` (u32).
fn get_guild_last_exp(level: u8) -> u32 {
    const GUILD_LEVEL_MAX: usize = 20;
    const GUILD_EXP_LIST: [u32; GUILD_LEVEL_MAX + 1] = [
        0,
        6000,
        18000,
        36000,
        64000,
        94000,
        130000,
        172000,
        220000,
        274000,
        334000,
        400000,
        600000,
        840000,
        1120000,
        1440000,
        1800000,
        2600000,
        3200000,
        4000000,
        16800000,
    ];

    if level as usize > GUILD_LEVEL_MAX {
        return 0;
    }
    GUILD_EXP_LIST[level as usize]
}

/// m2_py_guild_get_guild_level
/// Returns the current guild level.
#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_level() -> i32 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }
        state.level as i32
    })
}

/// m2_py_guild_get_guild_experience
/// Returns true if successful and fills out_exp and out_last_exp.
#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_experience(out_exp: *mut i32, out_last_exp: *mut i32) -> bool {
    safe_ffi_call(false, || {
        if out_exp.is_null() || out_last_exp.is_null() {
            return false;
        }
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return false;
        }

        const GUILD_MAX_LEVEL: u8 = 20;
        if state.level >= GUILD_MAX_LEVEL {
            unsafe {
                *out_exp = 0;
                *out_last_exp = 0;
            }
            return true;
        }

        let last_exp = get_guild_last_exp(state.level);
        unsafe {
            *out_exp = state.exp as i32;
            *out_last_exp = (last_exp.saturating_sub(state.exp)) as i32;
        }
        true
    })
}

/// m2_py_guild_get_guild_member_count
/// Fills out_current_count and out_max_count
#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_member_count(out_current_count: *mut i32, out_max_count: *mut i32) -> bool {
    safe_ffi_call(false, || {
        if out_current_count.is_null() || out_max_count.is_null() {
            return false;
        }
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return false;
        }

        unsafe {
            *out_current_count = state.member_count as i32;
            *out_max_count = state.max_member_count as i32;
        }
        true
    })
}

/// m2_py_guild_get_member_data
#[no_mangle]
pub extern "C" fn m2_py_guild_get_member_data(
    index: i32,
    out_pid: *mut i32,
    out_name: *mut u8,
    name_max_len: usize,
    out_grade: *mut i32,
    out_job: *mut i32,
    out_level: *mut i32,
    out_offer: *mut i32,
    out_general_flag: *mut i32,
) -> bool {
    safe_ffi_call(false, || {
        if index < 0 || out_name.is_null() || name_max_len == 0 {
            return false;
        }
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return false;
        }

        let mut members: Vec<&GuildMemberData> = state.members.values().collect();
        // CPythonGuild::__SortMember() uses CPythonGuild_SLessMemberGrade which sorts by grade
        members.sort_by(|a, b| a.grade.cmp(&b.grade));

        if index as usize >= members.len() {
            return false;
        }

        let member = members[index as usize];
        unsafe {
            if !out_pid.is_null() { *out_pid = member.pid as i32; }
            if !out_grade.is_null() { *out_grade = member.grade as i32; }
            if !out_job.is_null() { *out_job = member.job as i32; }
            if !out_level.is_null() { *out_level = member.level as i32; }
            if !out_offer.is_null() { *out_offer = member.offer as i32; }
            if !out_general_flag.is_null() { *out_general_flag = member.is_general as i32; }

            let name_bytes = member.name.as_bytes();
            let copy_len = name_bytes.len().min(name_max_len - 1);
            ptr::copy_nonoverlapping(name_bytes.as_ptr(), out_name, copy_len);
            *out_name.add(copy_len) = 0;
        }

        true
    })
}

/// m2_py_guild_get_grade_data
#[no_mangle]
pub extern "C" fn m2_py_guild_get_grade_data(
    grade_number: i32,
    out_name: *mut u8,
    name_max_len: usize,
    out_auth_flag: *mut i32,
) -> bool {
    safe_ffi_call(false, || {
        if grade_number < 0 || grade_number > 255 || out_name.is_null() || name_max_len == 0 {
            return false;
        }

        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return false;
        }

        if let Some(grade_data) = state.grades.get(&(grade_number as u8)) {
            unsafe {
                if !out_auth_flag.is_null() {
                    *out_auth_flag = grade_data.auth_flag as i32;
                }

                let name_bytes = grade_data.name.as_bytes();
                let copy_len = name_bytes.len().min(name_max_len - 1);
                ptr::copy_nonoverlapping(name_bytes.as_ptr(), out_name, copy_len);
                *out_name.add(copy_len) = 0;
            }
            return true;
        }

        false
    })
}

/// m2_py_guild_get_grade_name
#[no_mangle]
pub extern "C" fn m2_py_guild_get_grade_name(
    grade_number: i32,
    out_name: *mut u8,
    name_max_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        if grade_number < 0 || grade_number > 255 || out_name.is_null() || name_max_len == 0 {
            return false;
        }

        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return false;
        }

        if let Some(grade_data) = state.grades.get(&(grade_number as u8)) {
            unsafe {
                let name_bytes = grade_data.name.as_bytes();
                let copy_len = name_bytes.len().min(name_max_len - 1);
                ptr::copy_nonoverlapping(name_bytes.as_ptr(), out_name, copy_len);
                *out_name.add(copy_len) = 0;
            }
            return true;
        }

        false
    })
}
