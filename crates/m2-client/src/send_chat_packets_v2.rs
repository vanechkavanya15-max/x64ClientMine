use std::ptr;

pub const CG_CHAT: u16 = 0x0601;
pub const CG_WHISPER: u16 = 0x0602;
pub const CHARACTER_NAME_MAX_LEN: usize = 24;

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

#[no_mangle]
pub unsafe extern "C" fn m2_send_chat(
    chat_type: u8,
    msg: *const u8,
    len: usize,
    out_buf: *mut u8,
) -> usize {
    if msg.is_null() || out_buf.is_null() {
        return 0;
    }

    if len == 0 || len >= 512 {
        return 0;
    }

    // Include null terminator in the length if it wasn't already included
    // Wait, the C++ code does:
    // int iTextLen = strlen(c_szChat) + 1;
    // ChatPacket.length = sizeof(ChatPacket) + iTextLen;
    // Let's assume `len` passed to this function is the length of the string WITHOUT the null terminator, matching strlen.
    // If len >= 512, it returns 0.
    
    let text_len = len + 1;
    let packet_size = std::mem::size_of::<TPacketCGChat>();
    
    let packet = TPacketCGChat {
        header: CG_CHAT,
        length: (packet_size + text_len) as u16,
        type_: chat_type,
    };

    let total_size = packet_size + text_len;

    // Copy packet header
    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        packet_size,
    );

    // Copy message
    ptr::copy_nonoverlapping(
        msg,
        out_buf.add(packet_size),
        len,
    );

    // Null terminator
    *out_buf.add(packet_size + len) = 0;

    total_size
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_whisper(
    name: *const u8,
    name_len: usize,
    msg: *const u8,
    msg_len: usize,
    out_buf: *mut u8,
) -> usize {
    if name.is_null() || msg.is_null() || out_buf.is_null() {
        return 0;
    }

    if msg_len >= 255 {
        return 0;
    }

    let text_len = msg_len + 1;
    let packet_size = std::mem::size_of::<TPacketCGWhisper>();
    
    let mut packet = TPacketCGWhisper {
        header: CG_WHISPER,
        length: (packet_size + text_len) as u16,
        sz_name_to: [0; CHARACTER_NAME_MAX_LEN + 1],
    };

    // strncpy semantics for name
    let copy_len = std::cmp::min(name_len, CHARACTER_NAME_MAX_LEN);
    ptr::copy_nonoverlapping(
        name,
        packet.sz_name_to.as_mut_ptr(),
        copy_len,
    );
    packet.sz_name_to[copy_len] = 0; // Ensure null termination (already 0, but good practice)

    let total_size = packet_size + text_len;

    // Copy packet header
    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        packet_size,
    );

    // Copy message
    ptr::copy_nonoverlapping(
        msg,
        out_buf.add(packet_size),
        msg_len,
    );

    // Null terminator
    *out_buf.add(packet_size + msg_len) = 0;

    total_size
}
