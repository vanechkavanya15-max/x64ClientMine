use crate::recv_guild_logic_v2::get_guild_state;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const MARK_TOTAL_COUNT: u32 = 1280; // from CGuildMarkImage::MARK_TOTAL_COUNT (40 * 32)
const MAX_IMAGE_COUNT: u32 = 5;

fn deterministic_guild_hash(guild_id: u32) -> u32 {
    let mut hasher = DefaultHasher::new();
    guild_id.hash(&mut hasher);
    let hash_val = hasher.finish() as u32;
    hash_val % (MAX_IMAGE_COUNT * MARK_TOTAL_COUNT)
}

#[no_mangle]
pub extern "C" fn m2_py_guild_is_guild_enable() -> bool {
    safe_ffi_call(false, || {
        let state = get_guild_state().lock().unwrap();
        state.is_enabled
    })
}

#[no_mangle]
pub extern "C" fn m2_py_guild_guild_id_to_mark_id(guild_id: u32) -> u32 {
    safe_ffi_call(u32::MAX, || {
        deterministic_guild_hash(guild_id)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_guild_get_mark_image_filename_by_mark_id(mark_id: u32, out_buf: *mut std::os::raw::c_char, max_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || max_len == 0 {
            return false;
        }

        let img_idx = mark_id / MARK_TOTAL_COUNT;
        
        let path = format!("mark/10_{}.tga", img_idx); // prefix is typically hardcoded/configurable based on server, using 10_ as placeholder fallback based on general structure
        
        let path_bytes = path.as_bytes();
        let copy_len = path_bytes.len().min(max_len - 1);
        unsafe {
            std::ptr::copy_nonoverlapping(path_bytes.as_ptr() as *const std::os::raw::c_char, out_buf, copy_len);
            *out_buf.add(copy_len) = 0;
        }
        
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_guild_get_mark_index_by_mark_id(mark_id: u32) -> u32 {
    safe_ffi_call(0, || {
        mark_id % MARK_TOTAL_COUNT
    })
}

#[no_mangle]
pub extern "C" fn m2_py_guild_get_guild_master_name(out_buf: *mut std::os::raw::c_char, max_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || max_len == 0 {
            return false;
        }

        let state = get_guild_state().lock().unwrap();
        let master_name = if state.is_enabled {
            state.members.get(&state.master_pid)
                .map(|m| m.name.as_str())
                .unwrap_or("Noname")
        } else {
            "Noname"
        };
        
        let name_bytes = master_name.as_bytes();
        let copy_len = name_bytes.len().min(max_len - 1);
        
        unsafe {
            std::ptr::copy_nonoverlapping(name_bytes.as_ptr() as *const std::os::raw::c_char, out_buf, copy_len);
            *out_buf.add(copy_len) = 0;
        }
        
        true
    })
}
