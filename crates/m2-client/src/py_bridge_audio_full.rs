use std::os::raw::c_char;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::sound_ffi::{
    m2_audio_play_sound_2d,
    m2_audio_play_bgm,
    m2_audio_stop_bgm,
    m2_audio_set_category_volume,
    m2_audio_play_sound_3d,
    m2_audio_stop_all_sound,
    m2_audio_set_master_volume,
};

#[no_mangle]
pub extern "C" fn m2_py_snd_play_bgm(filename_ptr: *const c_char) -> bool {
    safe_ffi_call(false, || {
        m2_audio_play_bgm(filename_ptr, true)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_stop_bgm() -> bool {
    safe_ffi_call(false, || {
        m2_audio_stop_bgm(true)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_set_bgm_volume(vol: f32) {
    safe_ffi_call((), || {
        m2_audio_set_category_volume(1, vol);
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_play_sound(filename_ptr: *const c_char) -> bool {
    safe_ffi_call(false, || {
        m2_audio_play_sound_2d(filename_ptr, 1.0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_play_sound_at_position(filename_ptr: *const c_char, x: f32, y: f32, z: f32) -> bool {
    safe_ffi_call(false, || {
        m2_audio_play_sound_3d(filename_ptr, x, y, z, 1.0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_set_sound_volume(vol: f32) {
    safe_ffi_call((), || {
        m2_audio_set_category_volume(0, vol);
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_set_master_volume(vol: f32) {
    safe_ffi_call((), || {
        m2_audio_set_master_volume(vol);
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_stop_all_sound() -> bool {
    safe_ffi_call(false, || {
        m2_audio_stop_all_sound()
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_fade_in_music(filename_ptr: *const c_char) -> bool {
    safe_ffi_call(false, || {
        m2_audio_play_bgm(filename_ptr, true)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_fade_out_music(_filename_ptr: *const c_char) -> bool {
    safe_ffi_call(false, || {
        m2_audio_stop_bgm(true)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_fade_out_all_music() -> bool {
    safe_ffi_call(false, || {
        m2_audio_stop_bgm(true)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_snd_fade_limit_out_music(_filename_ptr: *const c_char, limit_volume: f32) -> bool {
    safe_ffi_call(false, || {
        m2_audio_set_category_volume(1, limit_volume);
        m2_audio_stop_bgm(true)
    })
}
