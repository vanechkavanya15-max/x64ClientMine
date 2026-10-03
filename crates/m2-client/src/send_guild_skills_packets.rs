// send_guild_skills_packets.rs
// Zgodnosc 1:1 z C++: src/UserInterface/Packet.h oraz src/UserInterface/PythonNetworkStreamPhaseGame.cpp

pub const HEADER_CG_GUILD: u16 = 0x0720;

pub mod guild_sub {
    pub const USE_SKILL: u8 = 9;
    pub const GUILD_INVITE_ANSWER: u8 = 11;
    pub const CHARGE_GSP: u8 = 12;
    pub const DEPOSIT_MONEY: u8 = 13;
    pub const WITHDRAW_MONEY: u8 = 14;
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGGuild {
    pub header: u16,
    pub length: u16,
    pub by_sub_header: u8,
}

pub fn serialize_guild_deposit_money(money: u32) -> [u8; 9] {
    let mut buf = [0u8; 9];
    buf[0..2].copy_from_slice(&HEADER_CG_GUILD.to_le_bytes());
    buf[2..4].copy_from_slice(&9u16.to_le_bytes());
    buf[4] = guild_sub::DEPOSIT_MONEY;
    buf[5..9].copy_from_slice(&money.to_le_bytes());
    buf
}

pub fn serialize_guild_withdraw_money(money: u32) -> [u8; 9] {
    let mut buf = [0u8; 9];
    buf[0..2].copy_from_slice(&HEADER_CG_GUILD.to_le_bytes());
    buf[2..4].copy_from_slice(&9u16.to_le_bytes());
    buf[4] = guild_sub::WITHDRAW_MONEY;
    buf[5..9].copy_from_slice(&money.to_le_bytes());
    buf
}

pub fn serialize_guild_use_skill(skill_id: u32, target_vid: u32) -> [u8; 13] {
    let mut buf = [0u8; 13];
    buf[0..2].copy_from_slice(&HEADER_CG_GUILD.to_le_bytes());
    buf[2..4].copy_from_slice(&13u16.to_le_bytes());
    buf[4] = guild_sub::USE_SKILL;
    buf[5..9].copy_from_slice(&skill_id.to_le_bytes());
    buf[9..13].copy_from_slice(&target_vid.to_le_bytes());
    buf
}

pub fn serialize_guild_charge_gsp(money: u32) -> [u8; 9] {
    let mut buf = [0u8; 9];
    buf[0..2].copy_from_slice(&HEADER_CG_GUILD.to_le_bytes());
    buf[2..4].copy_from_slice(&9u16.to_le_bytes());
    buf[4] = guild_sub::CHARGE_GSP;
    buf[5..9].copy_from_slice(&money.to_le_bytes());
    buf
}

pub fn serialize_guild_invite_answer(guild_id: u32, answer: u8) -> [u8; 10] {
    let mut buf = [0u8; 10];
    buf[0..2].copy_from_slice(&HEADER_CG_GUILD.to_le_bytes());
    buf[2..4].copy_from_slice(&10u16.to_le_bytes());
    buf[4] = guild_sub::GUILD_INVITE_ANSWER;
    buf[5..9].copy_from_slice(&guild_id.to_le_bytes());
    buf[9] = answer;
    buf
}
