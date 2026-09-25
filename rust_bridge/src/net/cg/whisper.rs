use crate::ffi_guard::safe_ffi_call;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TPacketCGWhisper {
    pub header: u8,
    pub size: u16,
    pub sz_name_to: [u8; 25],
    pub message: Vec<u8>,
}

impl TPacketCGWhisper {
    pub const HEADER: u8 = 19;

    pub fn new(target_name: &str, msg: &str) -> Self {
        let mut name_buf = [0u8; 25];
        let bytes = target_name.as_bytes();
        let copy_len = bytes.len().min(24);
        name_buf[..copy_len].copy_from_slice(&bytes[..copy_len]);

        let msg_bytes = msg.as_bytes().to_vec();
        let total_size = (1 + 2 + 25 + msg_bytes.len()) as u16;

        Self {
            header: Self::HEADER,
            size: total_size,
            sz_name_to: name_buf,
            message: msg_bytes,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.size as usize);
        buf.push(self.header);
        buf.extend_from_slice(&self.size.to_le_bytes());
        buf.extend_from_slice(&self.sz_name_to);
        buf.extend_from_slice(&self.message);
        buf
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_whisper(target: *const c_char, msg: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if target.is_null() || msg.is_null() {
            return false;
        }
        let t_str = CStr::from_ptr(target).to_str().unwrap_or("");
        let m_str = CStr::from_ptr(msg).to_str().unwrap_or("");
        let pkt = TPacketCGWhisper::new(t_str, m_str);
        let _bytes = pkt.to_bytes();
        true
    })
}
