use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::recv_guild_logic_v2::get_guild_state;

#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_money_mgmt() -> i32 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }
        state.gold as i32
    })
}

#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_experience_mgmt() -> i32 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }
        state.exp as i32
    })
}

#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_level_mgmt() -> i32 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }
        state.level as i32
    })
}

#[no_mangle]
pub extern "C" fn m2_py_guild_get_grade_name_mgmt(
    grade_number: u8,
    out_name_buf: *mut u8,
    buf_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return false;
        }

        if let Some(grade) = state.grades.get(&grade_number) {
            if !out_name_buf.is_null() && buf_len > 0 {
                let name_bytes = grade.name.as_bytes();
                let copy_len = name_bytes.len().min(buf_len - 1);
                unsafe {
                    std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), out_name_buf, copy_len);
                    *out_name_buf.add(copy_len) = 0; // null-terminate
                }
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_guild_get_grade_authority(grade_number: u8) -> u8 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return 0;
        }

        if let Some(grade) = state.grades.get(&grade_number) {
            return grade.auth_flag;
        }
        0
    })
}

#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_board_comment_data(
    index: u32,
    out_id: *mut u32,
    out_name: *mut u8,
    out_name_len: usize,
    out_comment: *mut u8,
    out_comment_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        let state = get_guild_state().lock().unwrap();
        if !state.is_enabled {
            return false;
        }

        let idx = index as usize;
        if idx < state.comments.len() {
            let comment_data = &state.comments[idx];
            
            if !out_id.is_null() {
                unsafe { *out_id = comment_data.comment_id };
            }
            
            if !out_name.is_null() && out_name_len > 0 {
                let name_bytes = comment_data.name.as_bytes();
                let copy_len = name_bytes.len().min(out_name_len - 1);
                unsafe {
                    std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), out_name, copy_len);
                    *out_name.add(copy_len) = 0;
                }
            }

            if !out_comment.is_null() && out_comment_len > 0 {
                let comment_bytes = comment_data.comment.as_bytes();
                let copy_len = comment_bytes.len().min(out_comment_len - 1);
                unsafe {
                    std::ptr::copy_nonoverlapping(comment_bytes.as_ptr(), out_comment, copy_len);
                    *out_comment.add(copy_len) = 0;
                }
            }
            
            return true;
        }
        
        false
    })
}
