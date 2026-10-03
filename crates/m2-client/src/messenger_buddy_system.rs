use std::collections::HashMap;
use std::sync::RwLock;
use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use once_cell::sync::Lazy;

pub const HEADER_CG_MESSENGER: u16 = 0x0740;

pub const CG_MESSENGER_SUBHEADER_ADD_BY_VID: u8 = 0;
pub const CG_MESSENGER_SUBHEADER_ADD_BY_NAME: u8 = 1;
pub const CG_MESSENGER_SUBHEADER_REMOVE: u8 = 2;

#[repr(C, packed)]
pub struct TPacketCGMessenger {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

impl TPacketCGMessenger {
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;
}

#[derive(Default)]
pub struct MessengerBuddySystem {
    pub buddies: HashMap<String, bool>, // true if online
}

pub static BUDDY_SYSTEM: Lazy<RwLock<MessengerBuddySystem>> = Lazy::new(|| RwLock::new(MessengerBuddySystem::default()));

#[no_mangle]
pub extern "C" fn m2_messenger_add_friend_by_name(name_ptr: *const std::os::raw::c_char) -> bool {
    safe_ffi_call(false, || {
        if name_ptr.is_null() {
            return false;
        }
        let c_str = unsafe { std::ffi::CStr::from_ptr(name_ptr) };
        if let Ok(name) = c_str.to_str() {
            let name_bytes = name.as_bytes();
            let total_len = TPacketCGMessenger::SIZE + 1 + name_bytes.len() as u16;
            
            let mut buf = Vec::with_capacity(total_len as usize);
            buf.extend_from_slice(&HEADER_CG_MESSENGER.to_le_bytes());
            buf.extend_from_slice(&total_len.to_le_bytes());
            buf.push(CG_MESSENGER_SUBHEADER_ADD_BY_NAME);
            
            // Append name length (1 byte) + name
            buf.push(name_bytes.len() as u8);
            buf.extend_from_slice(name_bytes);
            
            if let Ok(mut system) = BUDDY_SYSTEM.write() {
                system.buddies.insert(name.to_string(), false);
            }
            
            return enqueue_outgoing_packet(buf);
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_messenger_remove_friend(name_ptr: *const std::os::raw::c_char) -> bool {
    safe_ffi_call(false, || {
        if name_ptr.is_null() {
            return false;
        }
        let c_str = unsafe { std::ffi::CStr::from_ptr(name_ptr) };
        if let Ok(name) = c_str.to_str() {
            let name_bytes = name.as_bytes();
            let total_len = TPacketCGMessenger::SIZE + 1 + name_bytes.len() as u16;
            
            let mut buf = Vec::with_capacity(total_len as usize);
            buf.extend_from_slice(&HEADER_CG_MESSENGER.to_le_bytes());
            buf.extend_from_slice(&total_len.to_le_bytes());
            buf.push(CG_MESSENGER_SUBHEADER_REMOVE);
            
            buf.push(name_bytes.len() as u8);
            buf.extend_from_slice(name_bytes);
            
            if let Ok(mut system) = BUDDY_SYSTEM.write() {
                system.buddies.remove(name);
            }
            
            return enqueue_outgoing_packet(buf);
        }
        false
    })
}
