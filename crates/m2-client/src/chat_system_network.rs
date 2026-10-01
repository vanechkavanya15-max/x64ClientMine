use std::ffi::{CStr, CString};
use std::ptr;
use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

pub const CG_CHAT: u16 = 0x0601;
pub const CG_WHISPER: u16 = 0x0602;
pub const GC_CHAT: u16 = 0x0603;
pub const GC_WHISPER: u16 = 0x0604;

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const CHAT_HISTORY_MAX_LEN: usize = 300;

pub const CHAT_TYPE_TALKING: u8 = 0;
pub const CHAT_TYPE_INFO: u8 = 1;
pub const CHAT_TYPE_NOTICE: u8 = 2;
pub const CHAT_TYPE_PARTY: u8 = 3;
pub const CHAT_TYPE_GUILD: u8 = 4;
pub const CHAT_TYPE_COMMAND: u8 = 5;
pub const CHAT_TYPE_SHOUT: u8 = 6;
pub const CHAT_TYPE_WHISPER: u8 = 7;
pub const CHAT_TYPE_BIG_NOTICE: u8 = 8;
pub const CHAT_TYPE_MAX_NUM: u8 = 9;

#[repr(C, packed)]
pub struct TPacketCGChat {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
}

#[repr(C, packed)]
pub struct TPacketCGWhisper {
    pub header: u16,
    pub length: u16,
    pub sz_name_to: [u8; CHARACTER_NAME_MAX_LEN + 1],
}

#[repr(C, packed)]
pub struct TPacketGCChat {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub dw_vid: u32,
    pub b_empire: u8,
}

#[repr(C, packed)]
pub struct TPacketGCWhisper {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub sz_name_from: [u8; CHARACTER_NAME_MAX_LEN + 1],
}

pub struct ChatMessage {
    pub chat_type: u8,
    pub message: String,
    pub sender: Option<String>,
}

static CHAT_HISTORY: OnceLock<Mutex<VecDeque<ChatMessage>>> = OnceLock::new();

fn get_chat_history() -> &'static Mutex<VecDeque<ChatMessage>> {
    CHAT_HISTORY.get_or_init(|| Mutex::new(VecDeque::with_capacity(CHAT_HISTORY_MAX_LEN)))
}

pub fn push_chat_message(chat_type: u8, message: String, sender: Option<String>) {
    let mut history = get_chat_history().lock().unwrap();
    if history.len() >= CHAT_HISTORY_MAX_LEN {
        history.pop_front();
    }
    history.push_back(ChatMessage {
        chat_type,
        message,
        sender,
    });
}

pub fn get_chat_history_clone() -> Vec<ChatMessage> {
    let history = get_chat_history().lock().unwrap();
    history.iter().map(|msg| ChatMessage {
        chat_type: msg.chat_type,
        message: msg.message.clone(),
        sender: msg.sender.clone(),
    }).collect()
}

// For testing purposes
pub fn clear_chat_history() {
    let mut history = get_chat_history().lock().unwrap();
    history.clear();
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_chat_v2(
    chat_type: u8,
    msg: *const std::ffi::c_char,
    out_buf: *mut u8,
    out_buf_len: usize,
) -> usize {
    if msg.is_null() || out_buf.is_null() {
        return 0;
    }

    let c_str = CStr::from_ptr(msg);
    let bytes = c_str.to_bytes();
    
    if bytes.is_empty() || bytes.len() >= 512 {
        return 0;
    }

    let mut actual_type = chat_type;
    let mut actual_msg = bytes;

    // Filter console commands
    if bytes[0] == b'/' {
        if bytes.len() > 1 {
            actual_type = CHAT_TYPE_COMMAND;
            actual_msg = &bytes[1..];
        } else {
            // C++ code handling for strlen == 1 is complex (last command etc.), 
            // for now just map to command
            actual_type = CHAT_TYPE_COMMAND;
        }
    }

    let text_len = actual_msg.len() + 1; // +1 for null terminator
    let packet_size = std::mem::size_of::<TPacketCGChat>();
    let total_size = packet_size + text_len;

    if total_size > out_buf_len {
        return 0;
    }

    let packet = TPacketCGChat {
        header: CG_CHAT,
        length: total_size as u16,
        type_: actual_type,
    };

    ptr::copy_nonoverlapping(&packet as *const _ as *const u8, out_buf, packet_size);
    ptr::copy_nonoverlapping(actual_msg.as_ptr(), out_buf.add(packet_size), actual_msg.len());
    *out_buf.add(packet_size + actual_msg.len()) = 0; // null terminator

    total_size
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_whisper_v2(
    target_name: *const std::ffi::c_char,
    msg: *const std::ffi::c_char,
    out_buf: *mut u8,
    out_buf_len: usize,
) -> usize {
    if target_name.is_null() || msg.is_null() || out_buf.is_null() {
        return 0;
    }

    let c_str_target = CStr::from_ptr(target_name);
    let c_str_msg = CStr::from_ptr(msg);
    let target_bytes = c_str_target.to_bytes();
    let msg_bytes = c_str_msg.to_bytes();

    if msg_bytes.is_empty() || msg_bytes.len() >= 512 {
        return 0;
    }

    let text_len = msg_bytes.len() + 1; // +1 for null terminator
    let packet_size = std::mem::size_of::<TPacketCGWhisper>();
    let total_size = packet_size + text_len;

    if total_size > out_buf_len {
        return 0;
    }

    let mut packet = TPacketCGWhisper {
        header: CG_WHISPER,
        length: total_size as u16,
        sz_name_to: [0; CHARACTER_NAME_MAX_LEN + 1],
    };

    let name_len = std::cmp::min(target_bytes.len(), CHARACTER_NAME_MAX_LEN);
    ptr::copy_nonoverlapping(target_bytes.as_ptr(), packet.sz_name_to.as_mut_ptr(), name_len);
    // null terminator already set by initialization

    ptr::copy_nonoverlapping(&packet as *const _ as *const u8, out_buf, packet_size);
    ptr::copy_nonoverlapping(msg_bytes.as_ptr(), out_buf.add(packet_size), msg_bytes.len());
    *out_buf.add(packet_size + msg_bytes.len()) = 0;

    total_size
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_recv_chat(
    buf: *const u8,
    buf_len: usize,
) -> bool {
    let header_size = std::mem::size_of::<TPacketGCChat>();
    if buf.is_null() || buf_len < header_size {
        return false;
    }

    let packet = &*(buf as *const TPacketGCChat);
    let length = u16::from_le(packet.length) as usize;
    
    if buf_len < length || length < header_size {
        return false;
    }

    let text_len = length - header_size;
    let text_ptr = buf.add(header_size);

    let mut msg_bytes = Vec::with_capacity(text_len);
    for i in 0..text_len {
        let b = *text_ptr.add(i);
        if b == 0 {
            break;
        }
        msg_bytes.push(b);
    }

    let message = String::from_utf8_lossy(&msg_bytes).into_owned();
    
    // As per C++ implementation: if type >= MAX_NUM, ignore saving it or rendering it (but return true)
    if packet.type_ < CHAT_TYPE_MAX_NUM {
        push_chat_message(packet.type_, message, None);
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_recv_whisper(
    buf: *const u8,
    buf_len: usize,
) -> bool {
    let header_size = std::mem::size_of::<TPacketGCWhisper>();
    if buf.is_null() || buf_len < header_size {
        return false;
    }

    let packet = &*(buf as *const TPacketGCWhisper);
    let length = u16::from_le(packet.length) as usize;
    
    if buf_len < length || length < header_size {
        return false;
    }

    let text_len = length - header_size;
    let text_ptr = buf.add(header_size);

    let mut msg_bytes = Vec::with_capacity(text_len);
    for i in 0..text_len {
        let b = *text_ptr.add(i);
        if b == 0 {
            break;
        }
        msg_bytes.push(b);
    }

    let message = String::from_utf8_lossy(&msg_bytes).into_owned();

    let mut name_bytes = Vec::new();
    for i in 0..CHARACTER_NAME_MAX_LEN {
        let b = packet.sz_name_from[i];
        if b == 0 {
            break;
        }
        name_bytes.push(b);
    }
    
    let sender_name = String::from_utf8_lossy(&name_bytes).into_owned();

    push_chat_message(packet.type_, message, Some(sender_name));

    true
}
