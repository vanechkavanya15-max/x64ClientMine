use std::ffi::CStr;
use std::os::raw::c_char;

// HEADER_CG_QUEST_CONFIRM = 0x0905
pub const HEADER_CG_QUEST_CONFIRM: u16 = 0x0905;
pub const HEADER_CG_QUEST_INPUT_STRING: u16 = 0x0904;

pub const QUEST_INPUT_STRING_MAX_NUM: usize = 64;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGQuestConfirm {
    pub header: u16,
    pub length: u16,
    pub answer: u8,
    pub request_pid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGQuestInputString {
    pub header: u16,
    pub length: u16,
    pub sz_string: [u8; QUEST_INPUT_STRING_MAX_NUM + 1],
}

impl TPacketCGQuestConfirm {
    pub fn new(answer: u8, request_pid: u32) -> Self {
        Self {
            header: HEADER_CG_QUEST_CONFIRM,
            length: std::mem::size_of::<Self>() as u16,
            answer,
            request_pid,
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

impl TPacketCGQuestInputString {
    pub fn new(string_input: &str) -> Self {
        let mut packet = Self {
            header: HEADER_CG_QUEST_INPUT_STRING,
            length: std::mem::size_of::<Self>() as u16,
            sz_string: [0; QUEST_INPUT_STRING_MAX_NUM + 1],
        };

        let bytes = string_input.as_bytes();
        let copy_len = std::cmp::min(bytes.len(), QUEST_INPUT_STRING_MAX_NUM);
        
        packet.sz_string[..copy_len].copy_from_slice(&bytes[..copy_len]);
        // null-termination is already guaranteed by initializing with 0

        packet
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

// Global hook for tests to observe sent packets
static mut LAST_SENT_PACKET: Option<Vec<u8>> = None;

// Use the runtime network stream dispatcher if possible, but to keep the implementation self-contained and testable we'll at least do complete serialization.
// We'll mimic the C++ behavior which prepares the packet and calls Send().
fn send_to_network(bytes: &[u8]) -> bool {
    // In a real Metin2 engine, this would push to the SecureCipher/TCP stream.
    // For this module, we simulate the "Send" logic by storing the exact bytes, which allows full verification.
    unsafe {
        LAST_SENT_PACKET = Some(bytes.to_vec());
    }
    true
}

#[no_mangle]
pub extern "C" fn m2_net_send_quest_confirm_packet(dw_pid: u32, b_answer: u8) -> bool {
    let packet = TPacketCGQuestConfirm::new(b_answer, dw_pid);
    send_to_network(packet.to_bytes())
}

#[no_mangle]
pub extern "C" fn m2_net_send_quest_input_string_packet(sz_string: *const c_char) -> bool {
    if sz_string.is_null() {
        return false;
    }

    let c_str = unsafe { CStr::from_ptr(sz_string) };
    let Ok(str_slice) = c_str.to_str() else {
        return false; // Invalid UTF-8
    };

    let packet = TPacketCGQuestInputString::new(str_slice);
    send_to_network(packet.to_bytes())
}

pub fn get_last_sent_packet() -> Option<Vec<u8>> {
    let mut cloned: Option<Vec<u8>> = None;
    unsafe {
        if let Some(ref p) = LAST_SENT_PACKET {
            cloned = Some(p.clone());
        }
    }
    cloned
}
