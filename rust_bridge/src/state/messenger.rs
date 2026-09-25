use crate::ffi_guard::safe_ffi_call;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FriendEntry {
    pub name: String,
    pub is_online: bool,
}

pub struct MessengerManager {
    friends: HashMap<String, FriendEntry>,
}

impl MessengerManager {
    pub fn new() -> Self {
        Self {
            friends: HashMap::new(),
        }
    }

    pub fn add_friend(&mut self, name: &str) {
        self.friends.insert(
            name.to_lowercase(),
            FriendEntry {
                name: name.to_string(),
                is_online: false,
            },
        );
    }

    pub fn remove_friend(&mut self, name: &str) {
        self.friends.remove(&name.to_lowercase());
    }

    pub fn set_online(&mut self, name: &str, online: bool) {
        if let Some(f) = self.friends.get_mut(&name.to_lowercase()) {
            f.is_online = online;
        }
    }

    pub fn is_friend_online(&self, name: &str) -> bool {
        self.friends
            .get(&name.to_lowercase())
            .map(|f| f.is_online)
            .unwrap_or(false)
    }

    pub fn count(&self) -> usize {
        self.friends.len()
    }
}

static MESSENGER: Lazy<RwLock<MessengerManager>> = Lazy::new(|| RwLock::new(MessengerManager::new()));

#[no_mangle]
pub unsafe extern "C" fn m2_messenger_add_friend(name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let s = CStr::from_ptr(name).to_str().unwrap_or("");
        if s.is_empty() {
            return false;
        }
        if let Ok(mut mgr) = MESSENGER.write() {
            mgr.add_friend(s);
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_messenger_is_friend_online(name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let s = CStr::from_ptr(name).to_str().unwrap_or("");
        if let Ok(mgr) = MESSENGER.read() {
            mgr.is_friend_online(s)
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_messenger_count() -> u32 {
    safe_ffi_call(0, || {
        MESSENGER.read().map(|m| m.count() as u32).unwrap_or(0)
    })
}
