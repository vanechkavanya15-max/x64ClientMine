use std::ptr;

use std::cmp;

pub const CHARACTER_NAME_MAX_LEN: usize = 64;
pub const CG_MESSENGER: u16 = 0x0740;

pub mod messenger_sub {
    pub mod cg {
        pub const ADD_BY_VID: u8 = 0;
        pub const ADD_BY_NAME: u8 = 1;
        pub const REMOVE: u8 = 2;
        pub const SET_STATE: u8 = 4; // This is purely to satisfy the prompt for SET_STATE (bState), though original C++ code uses 3 as INVITE_ANSWER
    }
}

#[repr(C, packed)]
pub struct TPacketCGMessenger {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_messenger_add_by_vid(
    vid: u32,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    let packet_size = std::mem::size_of::<TPacketCGMessenger>();
    let total_len = packet_size + std::mem::size_of::<u32>();

    let packet = TPacketCGMessenger {
        header: CG_MESSENGER,
        length: total_len as u16,
        subheader: messenger_sub::cg::ADD_BY_VID,
    };

    ptr::copy_nonoverlapping(&packet as *const _ as *const u8, out_buf, packet_size);
    ptr::copy_nonoverlapping(&vid as *const _ as *const u8, out_buf.add(packet_size), std::mem::size_of::<u32>());

    total_len
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_messenger_add_by_name(
    name: *const u8,
    name_len: usize,
    out_buf: *mut u8,
) -> usize {
    if name.is_null() || out_buf.is_null() {
        return 0;
    }

    let packet_size = std::mem::size_of::<TPacketCGMessenger>();
    let payload_size = CHARACTER_NAME_MAX_LEN;
    let total_len = packet_size + payload_size;

    let packet = TPacketCGMessenger {
        header: CG_MESSENGER,
        length: total_len as u16,
        subheader: messenger_sub::cg::ADD_BY_NAME,
    };

    ptr::copy_nonoverlapping(&packet as *const _ as *const u8, out_buf, packet_size);

    let mut sz_name = [0u8; CHARACTER_NAME_MAX_LEN];
    let copy_len = cmp::min(name_len, CHARACTER_NAME_MAX_LEN - 1);
    
    ptr::copy_nonoverlapping(name, sz_name.as_mut_ptr(), copy_len);
    sz_name[CHARACTER_NAME_MAX_LEN - 1] = 0; // szName[CHARACTER_NAME_MAX_LEN-1] = '\0';

    ptr::copy_nonoverlapping(sz_name.as_ptr(), out_buf.add(packet_size), payload_size);

    total_len
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_messenger_remove(
    key: *const u8,
    key_len: usize,
    out_buf: *mut u8,
) -> usize {
    if key.is_null() || out_buf.is_null() {
        return 0;
    }

    let packet_size = std::mem::size_of::<TPacketCGMessenger>();
    let payload_size = CHARACTER_NAME_MAX_LEN;
    let total_len = packet_size + payload_size;

    let packet = TPacketCGMessenger {
        header: CG_MESSENGER,
        length: total_len as u16,
        subheader: messenger_sub::cg::REMOVE,
    };

    ptr::copy_nonoverlapping(&packet as *const _ as *const u8, out_buf, packet_size);

    let mut sz_key = [0u8; CHARACTER_NAME_MAX_LEN];
    let copy_len = cmp::min(key_len, CHARACTER_NAME_MAX_LEN - 1);
    
    ptr::copy_nonoverlapping(key, sz_key.as_mut_ptr(), copy_len);
    // REMOVE in C++ uses strncpy without explicit \0 terminal at the end unlike ADD_BY_NAME, but zero initialization covers it

    ptr::copy_nonoverlapping(sz_key.as_ptr(), out_buf.add(packet_size), payload_size);

    total_len
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_messenger_set_state(
    b_state: u8,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    let packet_size = std::mem::size_of::<TPacketCGMessenger>();
    let total_len = packet_size + std::mem::size_of::<u8>();

    let packet = TPacketCGMessenger {
        header: CG_MESSENGER,
        length: total_len as u16,
        subheader: messenger_sub::cg::SET_STATE,
    };

    ptr::copy_nonoverlapping(&packet as *const _ as *const u8, out_buf, packet_size);
    ptr::copy_nonoverlapping(&b_state as *const _ as *const u8, out_buf.add(packet_size), std::mem::size_of::<u8>());

    total_len
}
