// Struktura pakietu szeptu CG na podstawie Packet.h: command_whisper
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::WHISPER;
use crate::ffi_guard::safe_ffi_call;
use std::ffi::CStr;
use std::os::raw::c_char;
use crate::net::send_queue::enqueue_outgoing_packet;

pub const CHARACTER_NAME_MAX_LEN: usize = 64;

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TPacketCGWhisper {
    pub header: u16,
    pub length: u16,
    pub sz_name_to: [u8; CHARACTER_NAME_MAX_LEN + 1],
}

impl TPacketCGWhisper {
    pub const HEADER: u16 = WHISPER;
    pub const BASE_SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy naglowek pakietu szeptu dla docelowego odbiorcy i podanej calkowitej dlugosci.
    pub fn new(target_name: &str, length: u16) -> Self {
        let mut name_buf = [0u8; CHARACTER_NAME_MAX_LEN + 1];
        let bytes = target_name.as_bytes();
        let copy_len = bytes.len().min(CHARACTER_NAME_MAX_LEN);
        name_buf[..copy_len].copy_from_slice(&bytes[..copy_len]);

        Self {
            header: Self::HEADER,
            length,
            sz_name_to: name_buf,
        }
    }

    // Serializuje sam naglowek pakietu (69 bajtow).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::BASE_SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.sz_name_to);
        buf
    }

    // Buduje pelny pakiet z dolaczona trescia wiadomosci tekstowej.
    pub fn build_with_message(target_name: &str, msg: &str) -> Vec<u8> {
        let msg_bytes = msg.as_bytes();
        let total_size = (Self::BASE_SIZE as usize + msg_bytes.len()) as u16;
        let pkt = Self::new(target_name, total_size);
        let mut bytes = pkt.to_bytes();
        bytes.extend_from_slice(msg_bytes);
        bytes
    }
}

// Funkcja C FFI wysylajaca wiadomosc szeptu.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_whisper(target: *const c_char, msg: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if target.is_null() || msg.is_null() {
            return false;
        }
        let t_str = CStr::from_ptr(target).to_str().unwrap_or("");
        let m_str = CStr::from_ptr(msg).to_str().unwrap_or("");
        let bytes = TPacketCGWhisper::build_with_message(t_str, m_str);
        enqueue_outgoing_packet(bytes)
    })
}
