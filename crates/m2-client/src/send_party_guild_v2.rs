// send_party_guild_v2.rs
// Transkrypcja 1:1 z C++ (PythonNetworkStreamPhaseGame.cpp, Packet.h)
// Obsluga pakietow wychodzacych druzyny i gildii

pub const HEADER_CG_PARTY_INVITE: u16 = 0x0701;
pub const HEADER_CG_PARTY_INVITE_ANSWER: u16 = 0x0702;
pub const HEADER_CG_PARTY_REMOVE: u16 = 0x0703;
pub const HEADER_CG_PARTY_SET_STATE: u16 = 0x0704;
pub const HEADER_CG_PARTY_USE_SKILL: u16 = 0x0705;
pub const HEADER_CG_PARTY_PARAMETER: u16 = 0x0706;

pub const HEADER_CG_GUILD: u16 = 0x0720;
pub const HEADER_CG_ANSWER_MAKE_GUILD: u16 = 0x0721;

pub mod guild_sub {
    pub const ADD_MEMBER: u8 = 0;
    pub const REMOVE_MEMBER: u8 = 1;
    pub const CHANGE_GRADE_NAME: u8 = 2;
    pub const CHANGE_GRADE_AUTHORITY: u8 = 3;
    pub const OFFER: u8 = 4;
    pub const POST_COMMENT: u8 = 5;
    pub const DELETE_COMMENT: u8 = 6;
    pub const REFRESH_COMMENT: u8 = 7;
    pub const CHANGE_MEMBER_GRADE: u8 = 8;
    pub const USE_SKILL: u8 = 9;
    pub const CHANGE_MEMBER_GENERAL: u8 = 10;
    pub const GUILD_INVITE_ANSWER: u8 = 11;
    pub const CHARGE_GSP: u8 = 12;
    pub const DEPOSIT_MONEY: u8 = 13;
    pub const WITHDRAW_MONEY: u8 = 14;
    pub const WAR_DECLARE: u8 = 15;
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyInvite {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyInviteAnswer {
    pub header: u16,
    pub length: u16,
    pub leader_pid: u32,
    pub accept: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyRemove {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartySetState {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
    pub by_state: u8,
    pub by_flag: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyUseSkill {
    pub header: u16,
    pub length: u16,
    pub by_skill_index: u8,
    pub dw_target_vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGGuild {
    pub header: u16,
    pub length: u16,
    pub by_sub_header: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGGuildWarDeclare {
    pub header: u16,
    pub length: u16,
    pub by_sub_header: u8,
    pub target_guild_id: u32,
    pub war_type: u8,
}

pub fn serialize_party_invite(vid: u32) -> [u8; 8] {
    let mut buf = [0u8; 8];
    buf[0..2].copy_from_slice(&HEADER_CG_PARTY_INVITE.to_le_bytes());
    buf[2..4].copy_from_slice(&8u16.to_le_bytes());
    buf[4..8].copy_from_slice(&vid.to_le_bytes());
    buf
}

pub fn serialize_party_invite_answer(leader_pid: u32, accept: u8) -> [u8; 9] {
    let mut buf = [0u8; 9];
    buf[0..2].copy_from_slice(&HEADER_CG_PARTY_INVITE_ANSWER.to_le_bytes());
    buf[2..4].copy_from_slice(&9u16.to_le_bytes());
    buf[4..8].copy_from_slice(&leader_pid.to_le_bytes());
    buf[8] = accept;
    buf
}

pub fn serialize_party_remove(pid: u32) -> [u8; 8] {
    let mut buf = [0u8; 8];
    buf[0..2].copy_from_slice(&HEADER_CG_PARTY_REMOVE.to_le_bytes());
    buf[2..4].copy_from_slice(&8u16.to_le_bytes());
    buf[4..8].copy_from_slice(&pid.to_le_bytes());
    buf
}

pub fn serialize_party_set_state(dw_vid: u32, by_state: u8, by_flag: u8) -> [u8; 10] {
    let mut buf = [0u8; 10];
    buf[0..2].copy_from_slice(&HEADER_CG_PARTY_SET_STATE.to_le_bytes());
    buf[2..4].copy_from_slice(&10u16.to_le_bytes());
    buf[4..8].copy_from_slice(&dw_vid.to_le_bytes());
    buf[8] = by_state;
    buf[9] = by_flag;
    buf
}

pub fn serialize_party_use_skill(by_skill_index: u8, dw_target_vid: u32) -> [u8; 9] {
    let mut buf = [0u8; 9];
    buf[0..2].copy_from_slice(&HEADER_CG_PARTY_USE_SKILL.to_le_bytes());
    buf[2..4].copy_from_slice(&9u16.to_le_bytes());
    buf[4] = by_skill_index;
    buf[5..9].copy_from_slice(&dw_target_vid.to_le_bytes());
    buf
}

pub fn serialize_guild_add_member(vid: u32) -> [u8; 9] {
    let mut buf = [0u8; 9];
    buf[0..2].copy_from_slice(&HEADER_CG_GUILD.to_le_bytes());
    buf[2..4].copy_from_slice(&9u16.to_le_bytes());
    buf[4] = guild_sub::ADD_MEMBER;
    buf[5..9].copy_from_slice(&vid.to_le_bytes());
    buf
}

pub fn serialize_guild_remove_member(pid: u32) -> [u8; 9] {
    let mut buf = [0u8; 9];
    buf[0..2].copy_from_slice(&HEADER_CG_GUILD.to_le_bytes());
    buf[2..4].copy_from_slice(&9u16.to_le_bytes());
    buf[4] = guild_sub::REMOVE_MEMBER;
    buf[5..9].copy_from_slice(&pid.to_le_bytes());
    buf
}

pub fn serialize_guild_invite_answer(guild_id: u32, accept: u8) -> [u8; 10] {
    let mut buf = [0u8; 10];
    buf[0..2].copy_from_slice(&HEADER_CG_GUILD.to_le_bytes());
    buf[2..4].copy_from_slice(&10u16.to_le_bytes());
    buf[4] = guild_sub::GUILD_INVITE_ANSWER;
    buf[5..9].copy_from_slice(&guild_id.to_le_bytes());
    buf[9] = accept;
    buf
}

pub fn serialize_guild_war_declare(target_guild_id: u32, war_type: u8) -> [u8; 10] {
    let mut buf = [0u8; 10];
    buf[0..2].copy_from_slice(&HEADER_CG_GUILD.to_le_bytes());
    buf[2..4].copy_from_slice(&10u16.to_le_bytes());
    buf[4] = guild_sub::WAR_DECLARE;
    buf[5..9].copy_from_slice(&target_guild_id.to_le_bytes());
    buf[9] = war_type;
    buf
}

// ============================================================================
// Eksporty FFI dla C++
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn m2_send_party_invite(vid: u32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_party_invite(vid);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_party_invite_answer(leader_pid: u32, accept: u8, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_party_invite_answer(leader_pid, accept);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_party_remove(pid: u32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_party_remove(pid);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_guild_add_member(vid: u32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_guild_add_member(vid);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_guild_war_declare(target_guild_id: u32, war_type: u8, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_guild_war_declare(target_guild_id, war_type);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}
