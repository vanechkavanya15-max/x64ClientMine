use m2_client::recv_messenger_friends_v2::*;
use std::mem::size_of;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

fn setup_state() {
    if let Ok(mut state) = MESSENGER_FRIENDS_V2.write() {
        state.friends.clear();
    }
}

#[test]
fn test_parse_list() {
    let _lock = TEST_LOCK.lock().unwrap();
    setup_state();

    let mut buf = Vec::new();
    
    let friend1 = "Alice";
    let friend2 = "Bob";
    
    let total_len = size_of::<TPacketGCMessenger>() 
        + size_of::<TPacketGCMessengerListOffline>() + friend1.len()
        + size_of::<TPacketGCMessengerListOffline>() + friend2.len();
        
    let p = TPacketGCMessenger {
        header: 0,
        length: total_len as u16,
        subheader: MESSENGER_SUBHEADER_GC_LIST,
    };
    
    buf.extend_from_slice(unsafe { std::slice::from_raw_parts(&p as *const _ as *const u8, size_of::<TPacketGCMessenger>()) });
    
    let f1_pkt = TPacketGCMessengerListOffline {
        connected: MESSENGER_CONNECTED_STATE_ONLINE,
        length: friend1.len() as u8,
    };
    buf.extend_from_slice(unsafe { std::slice::from_raw_parts(&f1_pkt as *const _ as *const u8, size_of::<TPacketGCMessengerListOffline>()) });
    buf.extend_from_slice(friend1.as_bytes());

    let f2_pkt = TPacketGCMessengerListOffline {
        connected: MESSENGER_CONNECTED_STATE_OFFLINE,
        length: friend2.len() as u8,
    };
    buf.extend_from_slice(unsafe { std::slice::from_raw_parts(&f2_pkt as *const _ as *const u8, size_of::<TPacketGCMessengerListOffline>()) });
    buf.extend_from_slice(friend2.as_bytes());

    assert!(m2_client_parse_messenger(buf.as_ptr(), buf.len()));
    
    let name_c = std::ffi::CString::new(friend1).unwrap();
    assert!(m2_messenger_is_friend_online_v2(name_c.as_ptr() as *const u8));
    
    let name_c2 = std::ffi::CString::new(friend2).unwrap();
    assert!(!m2_messenger_is_friend_online_v2(name_c2.as_ptr() as *const u8));
    
    if let Ok(state) = MESSENGER_FRIENDS_V2.read() {
        assert_eq!(state.friends.get(friend2.as_bytes()), Some(&MESSENGER_CONNECTED_STATE_OFFLINE));
        assert_eq!(state.friends.len(), 2);
    }
}

#[test]
fn test_parse_login() {
    let _lock = TEST_LOCK.lock().unwrap();
    setup_state();

    let mut buf = Vec::new();
    let friend_name = "Charlie";
    
    let total_len = size_of::<TPacketGCMessenger>() 
        + size_of::<TPacketGCMessengerLogin>() + friend_name.len();
        
    let p = TPacketGCMessenger {
        header: 0,
        length: total_len as u16,
        subheader: MESSENGER_SUBHEADER_GC_LOGIN,
    };
    
    buf.extend_from_slice(unsafe { std::slice::from_raw_parts(&p as *const _ as *const u8, size_of::<TPacketGCMessenger>()) });
    
    let login_pkt = TPacketGCMessengerLogin {
        length: friend_name.len() as u8,
    };
    buf.extend_from_slice(unsafe { std::slice::from_raw_parts(&login_pkt as *const _ as *const u8, size_of::<TPacketGCMessengerLogin>()) });
    buf.extend_from_slice(friend_name.as_bytes());

    assert!(m2_client_parse_messenger(buf.as_ptr(), buf.len()));
    
    let name_c = std::ffi::CString::new(friend_name).unwrap();
    assert!(m2_messenger_is_friend_online_v2(name_c.as_ptr() as *const u8));
}

#[test]
fn test_parse_logout() {
    let _lock = TEST_LOCK.lock().unwrap();
    setup_state();
    
    let mut buf = Vec::new();
    let friend_name = "Dave";
    
    if let Ok(mut state) = MESSENGER_FRIENDS_V2.write() {
        state.friends.insert(friend_name.as_bytes().to_vec(), MESSENGER_CONNECTED_STATE_ONLINE);
    }
    
    let total_len = size_of::<TPacketGCMessenger>() 
        + size_of::<TPacketGCMessengerLogout>() + friend_name.len();
        
    let p = TPacketGCMessenger {
        header: 0,
        length: total_len as u16,
        subheader: MESSENGER_SUBHEADER_GC_LOGOUT,
    };
    
    buf.extend_from_slice(unsafe { std::slice::from_raw_parts(&p as *const _ as *const u8, size_of::<TPacketGCMessenger>()) });
    
    let logout_pkt = TPacketGCMessengerLogout {
        length: friend_name.len() as u8,
    };
    buf.extend_from_slice(unsafe { std::slice::from_raw_parts(&logout_pkt as *const _ as *const u8, size_of::<TPacketGCMessengerLogout>()) });
    buf.extend_from_slice(friend_name.as_bytes());

    assert!(m2_client_parse_messenger(buf.as_ptr(), buf.len()));
    
    let name_c = std::ffi::CString::new(friend_name).unwrap();
    assert!(!m2_messenger_is_friend_online_v2(name_c.as_ptr() as *const u8));
    
    if let Ok(state) = MESSENGER_FRIENDS_V2.read() {
        assert_eq!(state.friends.get(friend_name.as_bytes()), Some(&MESSENGER_CONNECTED_STATE_OFFLINE));
    }
}

#[test]
fn test_parse_remove() {
    let _lock = TEST_LOCK.lock().unwrap();
    setup_state();
    
    let mut buf = Vec::new();
    let friend_name = "Eve";
    
    if let Ok(mut state) = MESSENGER_FRIENDS_V2.write() {
        state.friends.insert(friend_name.as_bytes().to_vec(), MESSENGER_CONNECTED_STATE_ONLINE);
    }
    
    let total_len = size_of::<TPacketGCMessenger>() + 1 + friend_name.len();
        
    let p = TPacketGCMessenger {
        header: 0,
        length: total_len as u16,
        subheader: MESSENGER_SUBHEADER_GC_REMOVE_FRIEND,
    };
    
    buf.extend_from_slice(unsafe { std::slice::from_raw_parts(&p as *const _ as *const u8, size_of::<TPacketGCMessenger>()) });
    
    buf.push(friend_name.len() as u8);
    buf.extend_from_slice(friend_name.as_bytes());

    assert!(m2_client_parse_messenger(buf.as_ptr(), buf.len()));
    
    if let Ok(state) = MESSENGER_FRIENDS_V2.read() {
        assert!(state.friends.get(friend_name.as_bytes()).is_none());
    }
}

#[test]
fn test_ffi_is_friend_online() {
    let _lock = TEST_LOCK.lock().unwrap();
    setup_state();
    
    if let Ok(mut state) = MESSENGER_FRIENDS_V2.write() {
        state.friends.insert(b"Frank".to_vec(), MESSENGER_CONNECTED_STATE_ONLINE);
        state.friends.insert(b"Grace".to_vec(), MESSENGER_CONNECTED_STATE_OFFLINE);
    }
    
    let frank_c = std::ffi::CString::new("Frank").unwrap();
    let grace_c = std::ffi::CString::new("Grace").unwrap();
    let henry_c = std::ffi::CString::new("Henry").unwrap();
    
    assert!(m2_messenger_is_friend_online_v2(frank_c.as_ptr() as *const u8));
    assert!(!m2_messenger_is_friend_online_v2(grace_c.as_ptr() as *const u8));
    assert!(!m2_messenger_is_friend_online_v2(henry_c.as_ptr() as *const u8));
    assert!(!m2_messenger_is_friend_online_v2(std::ptr::null()));
}

#[test]
fn test_invalid_length_buffer() {
    let buf = vec![0u8; 1];
    assert!(!m2_client_parse_messenger(buf.as_ptr(), buf.len()));
    
    let p = TPacketGCMessenger {
        header: 0,
        length: 100, // larger than buffer
        subheader: MESSENGER_SUBHEADER_GC_LIST,
    };
    let buf = unsafe { std::slice::from_raw_parts(&p as *const _ as *const u8, size_of::<TPacketGCMessenger>()) };
    assert!(!m2_client_parse_messenger(buf.as_ptr(), buf.len()));
}
