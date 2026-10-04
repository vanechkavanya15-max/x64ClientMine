use std::collections::HashMap;

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const PARTY_AFFECT_SLOT_MAX_NUM: usize = 7;
pub const MAX_PARTY_MEMBERS: usize = 8;

pub const PARTY_ROLE_NORMAL: u8 = 0;
pub const PARTY_ROLE_LEADER: u8 = 1;
pub const PARTY_ROLE_ATTACKER: u8 = 2;
pub const PARTY_ROLE_TANKER: u8 = 3;
pub const PARTY_ROLE_BUFFER: u8 = 4;
pub const PARTY_ROLE_SKILL_MASTER: u8 = 5;
pub const PARTY_ROLE_BERSERKER: u8 = 6;
pub const PARTY_ROLE_DEFENDER: u8 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyRole {
    Normal = 0,
    Leader = 1,
    Attacker = 2,
    Tanker = 3,
    Buffer = 4,
    SkillMaster = 5,
    Berserker = 6,
    Defender = 7,
}

impl From<u8> for PartyRole {
    fn from(value: u8) -> Self {
        match value {
            1 => PartyRole::Leader,
            2 => PartyRole::Attacker,
            3 => PartyRole::Tanker,
            4 => PartyRole::Buffer,
            5 => PartyRole::SkillMaster,
            6 => PartyRole::Berserker,
            7 => PartyRole::Defender,
            _ => PartyRole::Normal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartyMember {
    pub pid: u32,
    pub vid: u32,
    pub name: String,
    pub hp_percent: u8,
    pub sp_percent: u8,
    pub role: PartyRole,
    pub affects: [i16; PARTY_AFFECT_SLOT_MAX_NUM],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyMinimapMarker {
    pub vid: u32,
    pub marker_type: &'static str,
}

pub struct PartySyncCoordinator {
    members: HashMap<u32, PartyMember>,
    distribute_mode: u8,
}

impl PartySyncCoordinator {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
            distribute_mode: 0,
        }
    }

    pub fn handle_party_add(&mut self, pid: u32, name: String) -> bool {
        if self.members.len() >= MAX_PARTY_MEMBERS {
            return false;
        }

        let member = PartyMember {
            pid,
            vid: 0,
            name,
            hp_percent: 100,
            sp_percent: 100, // Default for new member
            role: PartyRole::Normal,
            affects: [0; PARTY_AFFECT_SLOT_MAX_NUM],
        };

        self.members.insert(pid, member);
        true
    }

    pub fn handle_party_update(
        &mut self,
        pid: u32,
        state: u8,
        hp_percent: u8,
        sp_percent: u8,
        affects: [i16; PARTY_AFFECT_SLOT_MAX_NUM],
    ) -> bool {
        if let Some(member) = self.members.get_mut(&pid) {
            member.role = PartyRole::from(state);
            member.hp_percent = hp_percent;
            member.sp_percent = sp_percent;
            member.affects = affects;
            return true;
        }
        false
    }

    pub fn handle_party_remove(&mut self, pid: u32) -> bool {
        self.members.remove(&pid).is_some()
    }

    pub fn handle_party_link(&mut self, pid: u32, vid: u32) -> bool {
        if let Some(member) = self.members.get_mut(&pid) {
            member.vid = vid;
            return true;
        }
        false
    }

    pub fn handle_party_unlink(&mut self, pid: u32) -> bool {
        if let Some(member) = self.members.get_mut(&pid) {
            member.vid = 0;
            return true;
        }
        false
    }
    
    pub fn handle_party_parameter(&mut self, distribute_mode: u8) {
        self.distribute_mode = distribute_mode;
    }

    pub fn get_minimap_markers(&self) -> Vec<PartyMinimapMarker> {
        self.members
            .values()
            .filter(|m| m.vid != 0)
            .map(|m| PartyMinimapMarker {
                vid: m.vid,
                marker_type: "TYPE_PARTY",
            })
            .collect()
    }

    pub fn get_member(&self, pid: u32) -> Option<&PartyMember> {
        self.members.get(&pid)
    }
    
    pub fn get_distribute_mode(&self) -> u8 {
        self.distribute_mode
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

// Packet representations

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyAdd {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyUpdate {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub state: u8,
    pub percent_hp: u8,
    pub percent_sp: u8, // Adding SP parsing requirement
    pub affects: [i16; PARTY_AFFECT_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyRemove {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyLink {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyUnlink {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyParameter {
    pub header: u16,
    pub length: u16,
    pub b_distribute_mode: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PartyPacketParseError {
    BufferTooSmall,
}

pub fn parse_packet_add(buffer: &[u8]) -> Result<TPacketGCPartyAdd, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyAdd>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyAdd) };
    Ok(packet)
}

pub fn parse_packet_update(buffer: &[u8]) -> Result<TPacketGCPartyUpdate, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyUpdate>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyUpdate) };
    Ok(packet)
}

pub fn parse_packet_remove(buffer: &[u8]) -> Result<TPacketGCPartyRemove, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyRemove>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyRemove) };
    Ok(packet)
}

pub fn parse_packet_link(buffer: &[u8]) -> Result<TPacketGCPartyLink, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyLink>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyLink) };
    Ok(packet)
}

pub fn parse_packet_unlink(buffer: &[u8]) -> Result<TPacketGCPartyUnlink, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyUnlink>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyUnlink) };
    Ok(packet)
}

pub fn parse_packet_parameter(buffer: &[u8]) -> Result<TPacketGCPartyParameter, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyParameter>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyParameter) };
    Ok(packet)
}

