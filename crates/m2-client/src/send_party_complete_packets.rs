#[allow(dead_code)]
pub const CG_PARTY_INVITE: u16 = 0x0710;
#[allow(dead_code)]
pub const CG_PARTY_INVITE_ANSWER: u16 = 0x0711;
#[allow(dead_code)]
pub const CG_PARTY_EXIT: u16 = 0x0712;
#[allow(dead_code)]
pub const CG_PARTY_REMOVE: u16 = 0x0713;
#[allow(dead_code)]
pub const CG_PARTY_SET_STATE: u16 = 0x0714;
#[allow(dead_code)]
pub const CG_PARTY_USE_SKILL: u16 = 0x0715;
#[allow(dead_code)]
pub const CG_PARTY_PARAMETER: u16 = 0x0716;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TPacketCGPartyInvite {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

impl TPacketCGPartyInvite {
    pub fn new(vid: u32) -> Self {
        Self {
            header: CG_PARTY_INVITE,
            length: std::mem::size_of::<Self>() as u16,
            vid,
        }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TPacketCGPartyInviteAnswer {
    pub header: u16,
    pub length: u16,
    pub leader_pid: u32,
    pub accept: u8,
}

impl TPacketCGPartyInviteAnswer {
    pub fn new(leader_pid: u32, accept: u8) -> Self {
        Self {
            header: CG_PARTY_INVITE_ANSWER,
            length: std::mem::size_of::<Self>() as u16,
            leader_pid,
            accept,
        }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TPacketCGPartyRemove {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
}

impl TPacketCGPartyRemove {
    pub fn new(pid: u32) -> Self {
        Self {
            header: CG_PARTY_REMOVE,
            length: std::mem::size_of::<Self>() as u16,
            pid,
        }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TPacketCGPartySetState {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
    pub by_state: u8,
    pub by_flag: u8,
}

impl TPacketCGPartySetState {
    pub fn new(dw_vid: u32, by_state: u8, by_flag: u8) -> Self {
        Self {
            header: CG_PARTY_SET_STATE,
            length: std::mem::size_of::<Self>() as u16,
            dw_vid,
            by_state,
            by_flag,
        }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TPacketCGPartyUseSkill {
    pub header: u16,
    pub length: u16,
    pub by_skill_index: u8,
    pub dw_target_vid: u32,
}

impl TPacketCGPartyUseSkill {
    pub fn new(by_skill_index: u8, dw_target_vid: u32) -> Self {
        Self {
            header: CG_PARTY_USE_SKILL,
            length: std::mem::size_of::<Self>() as u16,
            by_skill_index,
            dw_target_vid,
        }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TPacketCGPartyParameter {
    pub header: u16,
    pub length: u16,
    pub b_distribute_mode: u8,
}

impl TPacketCGPartyParameter {
    pub fn new(b_distribute_mode: u8) -> Self {
        Self {
            header: CG_PARTY_PARAMETER,
            length: std::mem::size_of::<Self>() as u16,
            b_distribute_mode,
        }
    }
}
