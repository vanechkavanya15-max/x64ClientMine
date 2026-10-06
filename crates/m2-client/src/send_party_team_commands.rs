#![allow(dead_code, non_snake_case)]

/// Constants matching C++ `src/UserInterface/Packet.h`
pub const CG_PARTY_INVITE: u16 = 0x0701;
pub const CG_PARTY_INVITE_ANSWER: u16 = 0x0702;
pub const CG_PARTY_REMOVE: u16 = 0x0703;
pub const CG_PARTY_SET_STATE: u16 = 0x0704;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyInvite {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

impl TPacketCGPartyInvite {
    pub fn new(dwVID: u32) -> Self {
        Self {
            header: CG_PARTY_INVITE,
            length: std::mem::size_of::<Self>() as u16,
            vid: dwVID,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyInviteAnswer {
    pub header: u16,
    pub length: u16,
    pub leader_pid: u32,
    pub accept: u8,
}

impl TPacketCGPartyInviteAnswer {
    pub fn new(dwLeaderVID: u32, byAnswer: u8) -> Self {
        Self {
            header: CG_PARTY_INVITE_ANSWER,
            length: std::mem::size_of::<Self>() as u16,
            leader_pid: dwLeaderVID,
            accept: byAnswer,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyRemove {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
}

impl TPacketCGPartyRemove {
    pub fn new(dwPID: u32) -> Self {
        Self {
            header: CG_PARTY_REMOVE,
            length: std::mem::size_of::<Self>() as u16,
            pid: dwPID,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartySetState {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub byState: u8,
    pub byFlag: u8,
}

impl TPacketCGPartySetState {
    pub fn new(dwVID: u32, byState: u8, byFlag: u8) -> Self {
        Self {
            header: CG_PARTY_SET_STATE,
            length: std::mem::size_of::<Self>() as u16,
            dwVID,
            byState,
            byFlag,
        }
    }
}

pub struct PythonNetworkStreamCommand;

impl PythonNetworkStreamCommand {
    pub fn send_party_invite_packet(dwVID: u32) -> Result<TPacketCGPartyInvite, &'static str> {
        Ok(TPacketCGPartyInvite::new(dwVID))
    }

    pub fn send_party_invite_answer_packet(dwLeaderVID: u32, byAnswer: u8) -> Result<TPacketCGPartyInviteAnswer, &'static str> {
        Ok(TPacketCGPartyInviteAnswer::new(dwLeaderVID, byAnswer))
    }

    pub fn send_party_remove_packet(dwPID: u32) -> Result<TPacketCGPartyRemove, &'static str> {
        Ok(TPacketCGPartyRemove::new(dwPID))
    }
    
    // Covers PARTY_EXIT since PARTY_EXIT uses PARTY_REMOVE internally.
    pub fn send_party_exit_packet(dwPID: u32) -> Result<TPacketCGPartyRemove, &'static str> {
        Ok(TPacketCGPartyRemove::new(dwPID))
    }

    pub fn send_party_set_state_packet(dwVID: u32, byState: u8, byFlag: u8) -> Result<TPacketCGPartySetState, &'static str> {
        Ok(TPacketCGPartySetState::new(dwVID, byState, byFlag))
    }

    // According to instructions `PARTY_SET_ROLE (dwPID, bRole, bState)`. Since there's no native
    // `TPacketCGPartySetRole` in `Packet.h`, but the prompt specifically requested this signature
    // for this module, we simulate it via the existing state setter or as a direct property assignment
    // that the game uses via SetState under the hood (since SetRole maps to SetState in metin2 logic).
    pub fn send_party_set_role(dwPID: u32, bRole: u8, bState: u8) -> Result<TPacketCGPartySetState, &'static str> {
        // Simulating the backend call using SetState which maps dwVID to dwPID and role to flags
        Ok(TPacketCGPartySetState::new(dwPID, bState, bRole))
    }
}
