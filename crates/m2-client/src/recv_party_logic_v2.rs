use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::LazyLock;

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const PARTY_AFFECT_SLOT_MAX_NUM: usize = 7;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPartyAdd {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPartyUpdate {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub state: u8,
    pub percent_hp: u8,
    pub affects: [i16; PARTY_AFFECT_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPartyRemove {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
}

pub const PARTY_ROLE_NORMAL: u8 = 0;
pub const PARTY_ROLE_LEADER: u8 = 1;
pub const PARTY_ROLE_ATTACKER: u8 = 2;
pub const PARTY_ROLE_TANKER: u8 = 3;
pub const PARTY_ROLE_BUFFER: u8 = 4;
pub const PARTY_ROLE_SKILL_MASTER: u8 = 5;
pub const PARTY_ROLE_BERSERKER: u8 = 6;
pub const PARTY_ROLE_DEFENDER: u8 = 7;

#[derive(Debug, Clone)]
pub struct PartyMember {
    pub pid: u32,
    pub vid: u32,
    pub name: String,
    pub hp_percent: u8,
    pub role: u8,
    pub leader: bool,
    pub affects: [i16; PARTY_AFFECT_SLOT_MAX_NUM],
}

pub struct PartyManager {
    members: RwLock<HashMap<u32, PartyMember>>,
}

impl PartyManager {
    pub fn new() -> Self {
        Self {
            members: RwLock::new(HashMap::new()),
        }
    }

    pub fn handle_party_add(&self, packet: &TPacketGCPartyAdd) -> bool {
        let name_end = packet.name.iter().position(|&c| c == 0).unwrap_or(CHARACTER_NAME_MAX_LEN);
        let name_str = String::from_utf8_lossy(&packet.name[..name_end]).into_owned();
        
        let pid;
        unsafe {
            let pid_ptr = std::ptr::addr_of!(packet.pid) as *const u32;
            pid = std::ptr::read_unaligned(pid_ptr);
        }

        let member = PartyMember {
            pid,
            vid: 0,
            name: name_str,
            hp_percent: 100,
            role: PARTY_ROLE_NORMAL,
            leader: false,
            affects: [0; PARTY_AFFECT_SLOT_MAX_NUM],
        };

        if let Ok(mut members) = self.members.write() {
            members.insert(pid, member);
            return true;
        }
        false
    }

    pub fn handle_party_update(&self, packet: &TPacketGCPartyUpdate) -> bool {
        let pid;
        let affects;
        unsafe {
            let pid_ptr = std::ptr::addr_of!(packet.pid) as *const u32;
            pid = std::ptr::read_unaligned(pid_ptr);
            
            let affects_ptr = std::ptr::addr_of!(packet.affects) as *const [i16; PARTY_AFFECT_SLOT_MAX_NUM];
            affects = std::ptr::read_unaligned(affects_ptr);
        }
        
        let state = packet.state;
        let hp = packet.percent_hp;

        if let Ok(mut members) = self.members.write() {
            if let Some(member) = members.get_mut(&pid) {
                member.role = state;
                if member.role == PARTY_ROLE_LEADER {
                    member.leader = true;
                } else {
                    member.leader = false;
                }
                member.hp_percent = hp;
                member.affects = affects;
                return true;
            }
        }
        false
    }

    pub fn handle_party_remove(&self, packet: &TPacketGCPartyRemove) -> bool {
        let pid;
        unsafe {
            let pid_ptr = std::ptr::addr_of!(packet.pid) as *const u32;
            pid = std::ptr::read_unaligned(pid_ptr);
        }
        if let Ok(mut members) = self.members.write() {
            return members.remove(&pid).is_some();
        }
        false
    }
    
    pub fn get_member(&self, pid: u32) -> Option<PartyMember> {
        if let Ok(members) = self.members.read() {
            return members.get(&pid).cloned();
        }
        None
    }

    pub fn clear(&self) {
        if let Ok(mut members) = self.members.write() {
            members.clear();
        }
    }
}

pub static PARTY_MANAGER: LazyLock<PartyManager> = LazyLock::new(|| PartyManager::new());

#[no_mangle]
pub extern "C" fn m2_party_get_member(pid: u32) -> bool {
    PARTY_MANAGER.get_member(pid).is_some()
}
