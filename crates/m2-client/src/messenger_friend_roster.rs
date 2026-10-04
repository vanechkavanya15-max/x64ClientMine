use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use std::os::raw::{c_char, c_int};
use std::ffi::CStr;

pub const MESSENGER_GROUP_INDEX_FRIEND: u8 = 0;
pub const MESSENGER_GROUP_INDEX_GUILD: u8 = 1;

pub const MESSENGER_CONNECTED_STATE_OFFLINE: u8 = 0;
pub const MESSENGER_CONNECTED_STATE_ONLINE: u8 = 1;

#[derive(Default)]
pub struct MessengerFriendRoster {
    pub friend_state_map: HashMap<String, u8>,
    pub guild_member_state_map: HashMap<String, u8>,
}

impl MessengerFriendRoster {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn destroy(&mut self) {
        self.friend_state_map.clear();
        self.guild_member_state_map.clear();
    }

    pub fn add_friend(&mut self, key: &str) {
        if !self.friend_state_map.contains_key(key) {
            self.friend_state_map.insert(key.to_string(), MESSENGER_CONNECTED_STATE_OFFLINE);
        }
    }

    pub fn remove_friend(&mut self, key: &str) {
        self.friend_state_map.remove(key);
    }

    pub fn on_friend_login(&mut self, key: &str) {
        self.friend_state_map.insert(key.to_string(), MESSENGER_CONNECTED_STATE_ONLINE);
    }

    pub fn on_friend_logout(&mut self, key: &str) {
        self.friend_state_map.insert(key.to_string(), MESSENGER_CONNECTED_STATE_OFFLINE);
    }

    pub fn is_friend_by_key(&self, key: &str) -> bool {
        self.friend_state_map.contains_key(key)
    }

    pub fn is_friend_by_name(&self, name: &str) -> bool {
        self.is_friend_by_key(name)
    }

    pub fn append_guild_member(&mut self, name: &str) {
        if self.guild_member_state_map.contains_key(name) {
            return;
        }
        self.logout_guild_member(name);
    }

    pub fn remove_guild_member(&mut self, name: &str) {
        self.guild_member_state_map.remove(name);
    }

    pub fn remove_all_guild_member(&mut self) {
        self.guild_member_state_map.clear();
    }

    pub fn login_guild_member(&mut self, name: &str) {
        self.guild_member_state_map.insert(name.to_string(), MESSENGER_CONNECTED_STATE_ONLINE);
    }

    pub fn logout_guild_member(&mut self, name: &str) {
        self.guild_member_state_map.insert(name.to_string(), MESSENGER_CONNECTED_STATE_OFFLINE);
    }
}

pub static MESSENGER_FRIEND_ROSTER: Lazy<RwLock<MessengerFriendRoster>> = Lazy::new(|| {
    RwLock::new(MessengerFriendRoster::new())
});

#[no_mangle]
pub extern "C" fn m2_messenger_roster_remove_friend(name_ptr: *const c_char) {
    if name_ptr.is_null() {
        return;
    }
    if let Ok(c_str) = unsafe { CStr::from_ptr(name_ptr) }.to_str() {
        if let Ok(mut roster) = MESSENGER_FRIEND_ROSTER.write() {
            roster.remove_friend(c_str);
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_messenger_is_friend_by_name(name_ptr: *const c_char) -> c_int {
    if name_ptr.is_null() {
        return 0;
    }
    if let Ok(c_str) = unsafe { CStr::from_ptr(name_ptr) }.to_str() {
        if let Ok(roster) = MESSENGER_FRIEND_ROSTER.read() {
            return if roster.is_friend_by_name(c_str) { 1 } else { 0 };
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_messenger_destroy() {
    if let Ok(mut roster) = MESSENGER_FRIEND_ROSTER.write() {
        roster.destroy();
    }
}
