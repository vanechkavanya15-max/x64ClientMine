// Modul pakietow sieciowych gildii
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.
use std::ffi::CString;

pub const HEADER_CG_GUILD: u16 = 0x0720;
pub const HEADER_CG_ANSWER_MAKE_GUILD: u16 = 0x0721;
pub const GUILD_NAME_MAX_LEN: usize = 12;
pub const GUILD_GRADE_NAME_MAX_LEN: usize = 8;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGAnswerMakeGuild {
    pub header: u16,
    pub length: u16,
    pub guild_name: [u8; GUILD_NAME_MAX_LEN + 1],
}

impl TPacketCGAnswerMakeGuild {
    pub fn new(name: &str) -> Option<Self> {
        let mut packet = Self {
            header: HEADER_CG_ANSWER_MAKE_GUILD,
            length: std::mem::size_of::<Self>() as u16,
            guild_name: [0; GUILD_NAME_MAX_LEN + 1],
        };

        let bytes = name.as_bytes();
        if bytes.len() > GUILD_NAME_MAX_LEN {
            return None;
        }

        packet.guild_name[..bytes.len()].copy_from_slice(bytes);
        Some(packet)
    }

    pub fn to_bytes(&self) -> [u8; std::mem::size_of::<Self>()] {
        unsafe { std::mem::transmute(*self) }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GuildPacketError {
    CommentTooLong,
    StringConversionError,
    NameTooLong,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGGuild {
    pub header: u16,
    pub length: u16,
    pub by_sub_header: u8,
}

impl TPacketCGGuild {
    pub fn new(sub_header: u8, length: u16) -> Self {
        Self {
            header: HEADER_CG_GUILD,
            length,
            by_sub_header: sub_header,
        }
    }

    pub fn to_bytes(&self) -> [u8; std::mem::size_of::<Self>()] {
        unsafe { std::mem::transmute(*self) }
    }
}
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
}

pub fn build_guild_offer_packet(experience: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::OFFER, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&experience.to_le_bytes());
    Ok(packet_data)
}

pub fn build_guild_refresh_comments_packet(highest_index: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = std::mem::size_of::<TPacketCGGuild>() as u16;
    let packet = TPacketCGGuild::new(guild_sub::REFRESH_COMMENT, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    // highest_index is not sent according to C++ SendGuildRefreshCommentsPacket
    Ok(packet_data)
}

pub fn build_guild_change_member_grade_packet(pid: u32, grade: u8) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>() + std::mem::size_of::<u8>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::CHANGE_MEMBER_GRADE, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&pid.to_le_bytes());
    packet_data.extend_from_slice(&[grade]);
    Ok(packet_data)
}

pub fn build_guild_use_skill_packet(skill_id: u32, target_vid: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>() * 2) as u16;
    let packet = TPacketCGGuild::new(guild_sub::USE_SKILL, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&skill_id.to_le_bytes());
    packet_data.extend_from_slice(&target_vid.to_le_bytes());
    Ok(packet_data)
}

pub fn build_guild_change_member_general_packet(pid: u32, flag: u8) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>() + std::mem::size_of::<u8>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::CHANGE_MEMBER_GENERAL, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&pid.to_le_bytes());
    packet_data.extend_from_slice(&[flag]);
    Ok(packet_data)
}

pub fn build_guild_invite_answer_packet(guild_id: u32, answer: u8) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>() + std::mem::size_of::<u8>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::GUILD_INVITE_ANSWER, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&guild_id.to_le_bytes());
    packet_data.extend_from_slice(&[answer]);
    Ok(packet_data)
}

pub fn build_guild_charge_gsp_packet(money: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::CHARGE_GSP, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&money.to_le_bytes());
    Ok(packet_data)
}

pub fn build_guild_deposit_money_packet(money: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::DEPOSIT_MONEY, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&money.to_le_bytes());
    Ok(packet_data)
}

pub fn build_guild_withdraw_money_packet(money: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::WITHDRAW_MONEY, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&money.to_le_bytes());
    Ok(packet_data)
}


pub fn build_guild_post_comment_packet(message: &str) -> Result<Vec<u8>, GuildPacketError> {
    let c_str = CString::new(message).map_err(|_| GuildPacketError::StringConversionError)?;
    let bytes = c_str.into_bytes_with_nul();
    
    if bytes.len() > 255 {
        return Err(GuildPacketError::CommentTooLong);
    }
    
    let by_size = bytes.len() as u8;

    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u8>() + by_size as usize) as u16;
    let packet = TPacketCGGuild::new(guild_sub::POST_COMMENT, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&[by_size]);
    packet_data.extend_from_slice(&bytes);

    Ok(packet_data)
}

pub fn build_guild_delete_comment_packet(index: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::DELETE_COMMENT, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&index.to_le_bytes());
    Ok(packet_data)
}

pub fn build_guild_add_member_packet(vid: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::ADD_MEMBER, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&vid.to_le_bytes());
    Ok(packet_data)
}

pub fn build_guild_remove_member_packet(pid: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::REMOVE_MEMBER, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&pid.to_le_bytes());
    Ok(packet_data)
}

pub fn build_guild_change_grade_name_packet(
    grade_number: u8,
    name: &str,
) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let name_len = GUILD_GRADE_NAME_MAX_LEN + 1;
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u8>() + name_len) as u16;
    
    let packet = TPacketCGGuild::new(guild_sub::CHANGE_GRADE_NAME, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&[grade_number]);

    let mut name_bytes = vec![0u8; name_len];
    let bytes = name.as_bytes();
    let copy_len = std::cmp::min(bytes.len(), GUILD_GRADE_NAME_MAX_LEN);
    name_bytes[..copy_len].copy_from_slice(&bytes[..copy_len]);

    packet_data.extend_from_slice(&name_bytes);
    Ok(packet_data)
}

pub fn build_guild_change_grade_authority_packet(
    grade_number: u8,
    authority: u8,
) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u8>() * 2) as u16;
    let packet = TPacketCGGuild::new(guild_sub::CHANGE_GRADE_AUTHORITY, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&[grade_number, authority]);
    Ok(packet_data)
}

