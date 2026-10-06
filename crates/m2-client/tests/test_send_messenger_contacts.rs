#[path = "../src/send_messenger_contacts.rs"]
mod send_messenger_contacts;

use send_messenger_contacts::{
    m2_net_send_messenger_add_by_vid,
    m2_net_send_messenger_add_by_name,
    m2_net_send_messenger_remove,
    m2_net_send_messenger_set_state,
    TPacketCGMessenger,
    CG_MESSENGER,
    CHARACTER_NAME_MAX_LEN,
    messenger_sub
};
use std::mem;

#[test]
fn test_messenger_struct_layout() {
    assert_eq!(mem::size_of::<TPacketCGMessenger>(), 5);
}

#[test]
fn test_send_messenger_add_by_vid() {
    unsafe {
        let mut out_buf = [0u8; 256];
        let vid: u32 = 12345;
        
        let written = m2_net_send_messenger_add_by_vid(vid, out_buf.as_mut_ptr());
        
        let expected_size = mem::size_of::<TPacketCGMessenger>() + mem::size_of::<u32>();
        assert_eq!(written, expected_size);
        
        let packet = &*(out_buf.as_ptr() as *const TPacketCGMessenger);
        assert_eq!({packet.header}, CG_MESSENGER);
        assert_eq!({packet.length} as usize, expected_size);
        assert_eq!({packet.subheader}, messenger_sub::cg::ADD_BY_VID);
        
        let mut payload = 0u32; std::ptr::copy_nonoverlapping(out_buf.as_ptr().add(mem::size_of::<TPacketCGMessenger>()), &mut payload as *mut u32 as *mut u8, 4);
        assert_eq!(payload, vid);
    }
}

#[test]
fn test_send_messenger_add_by_name() {
    unsafe {
        let mut out_buf = [0u8; 256];
        let name = b"Jules\0";
        
        let written = m2_net_send_messenger_add_by_name(name.as_ptr(), 5, out_buf.as_mut_ptr());
        
        let expected_size = mem::size_of::<TPacketCGMessenger>() + CHARACTER_NAME_MAX_LEN;
        assert_eq!(written, expected_size);
        
        let packet = &*(out_buf.as_ptr() as *const TPacketCGMessenger);
        assert_eq!({packet.header}, CG_MESSENGER);
        assert_eq!({packet.length} as usize, expected_size);
        assert_eq!({packet.subheader}, messenger_sub::cg::ADD_BY_NAME);
        
        let payload = std::slice::from_raw_parts(out_buf.as_ptr().add(mem::size_of::<TPacketCGMessenger>()), CHARACTER_NAME_MAX_LEN);
        assert_eq!(&payload[0..5], b"Jules");
        assert_eq!(payload[5], 0);
        assert_eq!(payload[CHARACTER_NAME_MAX_LEN - 1], 0);
    }
}

#[test]
fn test_send_messenger_remove() {
    unsafe {
        let mut out_buf = [0u8; 256];
        let key = b"SomeUser\0";
        
        let written = m2_net_send_messenger_remove(key.as_ptr(), 8, out_buf.as_mut_ptr());
        
        let expected_size = mem::size_of::<TPacketCGMessenger>() + CHARACTER_NAME_MAX_LEN;
        assert_eq!(written, expected_size);
        
        let packet = &*(out_buf.as_ptr() as *const TPacketCGMessenger);
        assert_eq!({packet.header}, CG_MESSENGER);
        assert_eq!({packet.length} as usize, expected_size);
        assert_eq!({packet.subheader}, messenger_sub::cg::REMOVE);
        
        let payload = std::slice::from_raw_parts(out_buf.as_ptr().add(mem::size_of::<TPacketCGMessenger>()), CHARACTER_NAME_MAX_LEN);
        assert_eq!(&payload[0..8], b"SomeUser");
    }
}

#[test]
fn test_send_messenger_set_state() {
    unsafe {
        let mut out_buf = [0u8; 256];
        let state: u8 = 1; // Online
        
        let written = m2_net_send_messenger_set_state(state, out_buf.as_mut_ptr());
        
        let expected_size = mem::size_of::<TPacketCGMessenger>() + mem::size_of::<u8>();
        assert_eq!(written, expected_size);
        
        let packet = &*(out_buf.as_ptr() as *const TPacketCGMessenger);
        assert_eq!({packet.header}, CG_MESSENGER);
        assert_eq!({packet.length} as usize, expected_size);
        assert_eq!({packet.subheader}, messenger_sub::cg::SET_STATE);
        
        let payload = *(out_buf.as_ptr().add(mem::size_of::<TPacketCGMessenger>()));
        assert_eq!(payload, state);
    }
}

#[test]
fn test_null_pointers() {
    unsafe {
        let mut out_buf = [0u8; 256];
        
        assert_eq!(m2_net_send_messenger_add_by_vid(1, std::ptr::null_mut()), 0);
        
        assert_eq!(m2_net_send_messenger_add_by_name(std::ptr::null(), 0, out_buf.as_mut_ptr()), 0);
        assert_eq!(m2_net_send_messenger_add_by_name(b"name".as_ptr(), 4, std::ptr::null_mut()), 0);
        
        assert_eq!(m2_net_send_messenger_remove(std::ptr::null(), 0, out_buf.as_mut_ptr()), 0);
        assert_eq!(m2_net_send_messenger_remove(b"name".as_ptr(), 4, std::ptr::null_mut()), 0);
        
        assert_eq!(m2_net_send_messenger_set_state(1, std::ptr::null_mut()), 0);
    }
}
