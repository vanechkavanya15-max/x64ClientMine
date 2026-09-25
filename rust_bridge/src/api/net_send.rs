use crate::ffi_guard::safe_ffi_call;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_chat_ex(chat_type: u8, msg: *const c_char) -> bool {
    crate::net::cg::chat::m2_net_send_chat(chat_type, msg)
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_whisper_packet(target: *const c_char, msg: *const c_char) -> bool {
    crate::net::cg::whisper::m2_net_send_whisper(target, msg)
}
