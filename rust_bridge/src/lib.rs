pub mod ffi_guard;
pub mod timer;
pub mod crc32;
pub mod random;
pub mod tea;
pub mod utils;

use ffi_guard::safe_ffi_call;
use std::slice;

// ============================================================================
// C FFI Exports - Timer
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_timer_init() -> u32 {
    safe_ffi_call(1, || {
        timer::init();
        1
    })
}

#[no_mangle]
pub extern "C" fn m2_timer_get_msec() -> u32 {
    safe_ffi_call(0, || {
        timer::get_msec()
    })
}

#[no_mangle]
pub extern "C" fn m2_timer_set_server_msec(server_msec: u32) {
    safe_ffi_call((), || {
        timer::set_server_msec(server_msec);
    });
}

#[no_mangle]
pub extern "C" fn m2_timer_get_server_msec() -> u32 {
    safe_ffi_call(0, || {
        timer::get_server_msec()
    })
}

#[no_mangle]
pub extern "C" fn m2_timer_get_frame_msec() -> u32 {
    safe_ffi_call(0, || {
        timer::get_frame_msec()
    })
}

#[no_mangle]
pub extern "C" fn m2_timer_set_frame_msec() {
    safe_ffi_call((), || {
        timer::set_frame_msec();
    });
}

// ============================================================================
// C FFI Exports - CRC32
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_crc32(buf: *const u8, len: usize) -> u32 {
    safe_ffi_call(0, || {
        if buf.is_null() || len == 0 {
            return 0;
        }
        let slice = unsafe { slice::from_raw_parts(buf, len) };
        crc32::get_crc32(slice)
    })
}

#[no_mangle]
pub extern "C" fn m2_case_crc32(buf: *const u8, len: usize) -> u32 {
    safe_ffi_call(0, || {
        if buf.is_null() || len == 0 {
            return 0;
        }
        let slice = unsafe { slice::from_raw_parts(buf, len) };
        crc32::get_case_crc32(slice)
    })
}

// ============================================================================
// C FFI Exports - Random
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_srandom(seed: u32) {
    safe_ffi_call((), || {
        random::srandom(seed);
    });
}

#[no_mangle]
pub extern "C" fn m2_random() -> u32 {
    safe_ffi_call(0, || {
        random::random()
    })
}

#[no_mangle]
pub extern "C" fn m2_frandom(low: f32, high: f32) -> f32 {
    safe_ffi_call(low, || {
        random::frandom(low, high)
    })
}

#[no_mangle]
pub extern "C" fn m2_random_range(from: i32, to: i32) -> i32 {
    safe_ffi_call(from, || {
        random::random_range(from, to)
    })
}

// ============================================================================
// C FFI Exports - TEA Cipher
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_tea_code(sz: u32, sy: u32, key: *const u32, dest: *mut u32) {
    safe_ffi_call((), || {
        if key.is_null() || dest.is_null() {
            return;
        }
        let key_arr = unsafe { &*(key as *const [u32; 4]) };
        let (y, z) = tea::tea_code(sz, sy, key_arr);
        unsafe {
            *dest = y;
            *dest.add(1) = z;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_tea_decode(sz: u32, sy: u32, key: *const u32, dest: *mut u32) {
    safe_ffi_call((), || {
        if key.is_null() || dest.is_null() {
            return;
        }
        let key_arr = unsafe { &*(key as *const [u32; 4]) };
        let (y, z) = tea::tea_decode(sz, sy, key_arr);
        unsafe {
            *dest = y;
            *dest.add(1) = z;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_tea_encrypt(dest: *mut u8, src: *const u8, key: *const u32, size: usize) -> usize {
    safe_ffi_call(0, || {
        if dest.is_null() || src.is_null() || key.is_null() || size == 0 {
            return 0;
        }
        let key_arr = unsafe { &*(key as *const [u32; 4]) };
        let src_slice = unsafe { slice::from_raw_parts(src, size) };
        let resize = if size % 8 != 0 { size + 8 - (size % 8) } else { size };
        let dest_slice = unsafe { slice::from_raw_parts_mut(dest, resize) };
        tea::tea_encrypt(dest_slice, src_slice, key_arr)
    })
}

#[no_mangle]
pub extern "C" fn m2_tea_decrypt(dest: *mut u8, src: *const u8, key: *const u32, size: usize) -> usize {
    safe_ffi_call(0, || {
        if dest.is_null() || src.is_null() || key.is_null() || size == 0 {
            return 0;
        }
        let key_arr = unsafe { &*(key as *const [u32; 4]) };
        let src_slice = unsafe { slice::from_raw_parts(src, size) };
        let resize = if size % 8 != 0 { size + 8 - (size % 8) } else { size };
        let dest_slice = unsafe { slice::from_raw_parts_mut(dest, resize) };
        tea::tea_decrypt(dest_slice, src_slice, key_arr)
    })
}
pub mod vfs;
