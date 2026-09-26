use std::collections::HashMap;
use std::mem::size_of;
use std::sync::RwLock;
use once_cell::sync::Lazy;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMessenger {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMessengerListOffline {
    pub connected: u8,
    pub length: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMessengerListOnline {
    pub connected: u8,
    pub length: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMessengerLogin {
    pub length: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMessengerLogout {
    pub length: u8,
}

pub const MESSENGER_SUBHEADER_GC_LIST: u8 = 0;
pub const MESSENGER_SUBHEADER_GC_LOGIN: u8 = 1;
pub const MESSENGER_SUBHEADER_GC_LOGOUT: u8 = 2;
pub const MESSENGER_SUBHEADER_GC_REMOVE_FRIEND: u8 = 4;

pub const MESSENGER_CONNECTED_STATE_OFFLINE: u8 = 0;
pub const MESSENGER_CONNECTED_STATE_ONLINE: u8 = 1;

#[derive(Default)]
pub struct MessengerState {
    pub friends: HashMap<Vec<u8>, u8>,
}

pub static MESSENGER_FRIENDS_V2: Lazy<RwLock<MessengerState>> = Lazy::new(|| RwLock::new(MessengerState::default()));

pub extern "C" fn m2_client_parse_messenger(buf: *const u8, len: usize) -> bool {
    if len < size_of::<TPacketGCMessenger>() {
        return false;
    }

    let p = unsafe { std::ptr::read_unaligned(buf as *const TPacketGCMessenger) };
    
    if (p.length as usize) > len {
        return false;
    }
    
    let mut offset = size_of::<TPacketGCMessenger>();
    let mut i_size = p.length as i32 - size_of::<TPacketGCMessenger>() as i32;

    match p.subheader {
        MESSENGER_SUBHEADER_GC_LIST => {
            while i_size > 0 {
                if i_size < size_of::<TPacketGCMessengerListOffline>() as i32 {
                    return false;
                }
                
                let on = unsafe { std::ptr::read_unaligned(buf.add(offset) as *const TPacketGCMessengerListOffline) };
                offset += size_of::<TPacketGCMessengerListOffline>();
                i_size -= size_of::<TPacketGCMessengerListOffline>() as i32;
                
                if i_size < on.length as i32 {
                    return false;
                }
                
                let name_len = on.length as usize;
                let name_slice = unsafe { std::slice::from_raw_parts(buf.add(offset), name_len) };
                let name_vec = name_slice.to_vec();
                
                offset += name_len;
                i_size -= name_len as i32;
                
                if let Ok(mut state) = MESSENGER_FRIENDS_V2.write() {
                    if (on.connected & MESSENGER_CONNECTED_STATE_ONLINE) != 0 {
                        state.friends.insert(name_vec, MESSENGER_CONNECTED_STATE_ONLINE);
                    } else {
                        state.friends.insert(name_vec, MESSENGER_CONNECTED_STATE_OFFLINE);
                    }
                }
            }
        },
        MESSENGER_SUBHEADER_GC_LOGIN => {
            if i_size < size_of::<TPacketGCMessengerLogin>() as i32 {
                return false;
            }
            
            let login_packet = unsafe { std::ptr::read_unaligned(buf.add(offset) as *const TPacketGCMessengerLogin) };
            offset += size_of::<TPacketGCMessengerLogin>();
            i_size -= size_of::<TPacketGCMessengerLogin>() as i32;
            
            if i_size < login_packet.length as i32 {
                return false;
            }
            
            let name_len = login_packet.length as usize;
            let name_slice = unsafe { std::slice::from_raw_parts(buf.add(offset), name_len) };
            let name_vec = name_slice.to_vec();
            
            if let Ok(mut state) = MESSENGER_FRIENDS_V2.write() {
                state.friends.insert(name_vec, MESSENGER_CONNECTED_STATE_ONLINE);
            }
        },
        MESSENGER_SUBHEADER_GC_LOGOUT => {
            if i_size < size_of::<TPacketGCMessengerLogout>() as i32 {
                return false;
            }
            
            let logout_packet = unsafe { std::ptr::read_unaligned(buf.add(offset) as *const TPacketGCMessengerLogout) };
            offset += size_of::<TPacketGCMessengerLogout>();
            i_size -= size_of::<TPacketGCMessengerLogout>() as i32;
            
            if i_size < logout_packet.length as i32 {
                return false;
            }
            
            let name_len = logout_packet.length as usize;
            let name_slice = unsafe { std::slice::from_raw_parts(buf.add(offset), name_len) };
            let name_vec = name_slice.to_vec();
            
            if let Ok(mut state) = MESSENGER_FRIENDS_V2.write() {
                state.friends.insert(name_vec, MESSENGER_CONNECTED_STATE_OFFLINE);
            }
        },
        MESSENGER_SUBHEADER_GC_REMOVE_FRIEND => {
            if i_size < 1 {
                return false;
            }
            
            let b_length = unsafe { *buf.add(offset) };
            offset += 1;
            i_size -= 1;
            
            if i_size < b_length as i32 {
                return false;
            }
            
            let name_len = b_length as usize;
            let name_slice = unsafe { std::slice::from_raw_parts(buf.add(offset), name_len) };
            let name_vec = name_slice.to_vec();
            
            if let Ok(mut state) = MESSENGER_FRIENDS_V2.write() {
                state.friends.remove(&name_vec);
            }
        },
        _ => {
            // unknown subheader
        }
    }

    true
}

#[no_mangle]
pub extern "C" fn m2_messenger_is_friend_online_v2(name: *const u8) -> bool {
    if name.is_null() {
        return false;
    }
    
    let c_str = unsafe { std::ffi::CStr::from_ptr(name as *const std::os::raw::c_char) };
    let bytes = c_str.to_bytes();
    
    if let Ok(state) = MESSENGER_FRIENDS_V2.read() {
        if let Some(&status) = state.friends.get(bytes) {
            return status == MESSENGER_CONNECTED_STATE_ONLINE;
        }
    }
    
    false
}
