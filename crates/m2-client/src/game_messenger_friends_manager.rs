//! Game Messenger Friends Manager Module
//!
//! Zarzadzanie systemem kontaktow Messenger: lista znajomych (Friends),
//! czlonkowie gildii (Guild Members) oraz lista zablokowanych (Block/Blacklist).
//! Sledzenie statusu Online/Offline i serializacja pakietow sieciowych.

pub use crate::messenger_buddy_system::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendOnlineStatus {
    Offline,
    Online,
    Busy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FriendEntry {
    pub name: String,
    pub status: FriendOnlineStatus,
    pub is_guild_member: bool,
}

#[derive(Debug, Default, Clone)]
pub struct GameMessengerFriendsManager {
    friends: HashMap<String, FriendEntry>,
    blacklist: HashSet<String>,
}

impl GameMessengerFriendsManager {
    pub fn new() -> Self {
        Self {
            friends: HashMap::new(),
            blacklist: HashSet::new(),
        }
    }

    /// Dodaje gracza do listy przyjaciol.
    pub fn add_friend(&mut self, name: &str, is_guild: bool) {
        if self.blacklist.contains(name) {
            self.blacklist.remove(name);
        }
        self.friends.insert(
            name.to_string(),
            FriendEntry {
                name: name.to_string(),
                status: FriendOnlineStatus::Offline,
                is_guild_member: is_guild,
            },
        );
    }

    /// Usuwa gracza z listy przyjaciol.
    pub fn remove_friend(&mut self, name: &str) -> bool {
        self.friends.remove(name).is_some()
    }

    /// Aktualizuje status sieciowy przyjaciela.
    pub fn update_status(&mut self, name: &str, status: FriendOnlineStatus) -> bool {
        if let Some(entry) = self.friends.get_mut(name) {
            entry.status = status;
            true
        } else {
            false
        }
    }

    /// Dodaje gracza do czarnej listy (ignorowani).
    pub fn block_player(&mut self, name: &str) {
        self.friends.remove(name);
        self.blacklist.insert(name.to_string());
    }

    /// Usuwa gracza z czarnej listy.
    pub fn unblock_player(&mut self, name: &str) -> bool {
        self.blacklist.remove(name)
    }

    pub fn is_blocked(&self, name: &str) -> bool {
        self.blacklist.contains(name)
    }

    pub fn get_friend(&self, name: &str) -> Option<&FriendEntry> {
        self.friends.get(name)
    }

    pub fn count_friends(&self) -> usize {
        self.friends.len()
    }

    pub fn count_online(&self) -> usize {
        self.friends.values().filter(|f| f.status == FriendOnlineStatus::Online).count()
    }
}
