//! Game Party Group Manager Module
//!
//! Zarzadzanie druzyna / grupa (Party System):
//! Zapraszanie, akceptacja, opuszczanie, tryby podzialu doswiadczenia (EXP) i dropu
//! (Po rowno vs Zaleznie od poziomu), role w grupie oraz paski zdrowia HP czlonkow.

pub use crate::recv_party_logic_v2::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyExpDistributionMode {
    Evenly,
    ByLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyMemberRole {
    Normal,
    Attacker,
    Defender,
    Buffer,
    SkillMaster,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyMemberInfo {
    pub pid: u32,
    pub name: String,
    pub hp_percent: u8,
    pub role: PartyMemberRole,
    pub is_leader: bool,
}

#[derive(Debug, Default, Clone)]
pub struct GamePartyGroupManager {
    pub leader_pid: u32,
    pub exp_mode: Option<PartyExpDistributionMode>,
    pub members: HashMap<u32, PartyMemberInfo>,
}

impl GamePartyGroupManager {
    pub fn new() -> Self {
        Self {
            leader_pid: 0,
            exp_mode: Some(PartyExpDistributionMode::Evenly),
            members: HashMap::new(),
        }
    }

    /// Dodaje czlonka do druzyny.
    pub fn add_member(&mut self, pid: u32, name: &str, is_leader: bool) {
        if is_leader {
            self.leader_pid = pid;
        }
        self.members.insert(
            pid,
            PartyMemberInfo {
                pid,
                name: name.to_string(),
                hp_percent: 100,
                role: PartyMemberRole::Normal,
                is_leader,
            },
        );
    }

    /// Usuwa czlonka z druzyny.
    pub fn remove_member(&mut self, pid: u32) -> bool {
        let removed = self.members.remove(&pid).is_some();
        if self.leader_pid == pid {
            self.leader_pid = 0;
        }
        if self.members.is_empty() {
            self.leader_pid = 0;
        }
        removed
    }

    /// Aktualizuje pasek zycia czlonka grupy.
    pub fn update_member_hp(&mut self, pid: u32, hp_percent: u8) -> bool {
        if let Some(m) = self.members.get_mut(&pid) {
            m.hp_percent = hp_percent.min(100);
            true
        } else {
            false
        }
    }

    /// Ustawia role bojowa czlonka grupy.
    pub fn set_member_role(&mut self, pid: u32, role: PartyMemberRole) -> bool {
        if let Some(m) = self.members.get_mut(&pid) {
            m.role = role;
            true
        } else {
            false
        }
    }

    /// Ustawia tryb podzialu EXP.
    pub fn set_exp_mode(&mut self, mode: PartyExpDistributionMode) {
        self.exp_mode = Some(mode);
    }

    pub fn is_in_party(&self) -> bool {
        !self.members.is_empty()
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    pub fn get_member(&self, pid: u32) -> Option<&PartyMemberInfo> {
        self.members.get(&pid)
    }
}
