// Modul wysylania pakietow administracyjnych gildii w Rust.
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use std::ffi::CString;

pub const HEADER_CG_GUILD: u16 = 0x0730;
pub const GUILD_GRADE_NAME_MAX_LEN: usize = 8;

pub mod guild_sub {
    pub const ADD_MEMBER: u8 = 0;
    pub const REMOVE_MEMBER: u8 = 1;
    pub const CHANGE_GRADE_NAME: u8 = 2;
    pub const CHANGE_GRADE_AUTHORITY: u8 = 3;
    pub const POST_COMMENT: u8 = 5;
    pub const DELETE_COMMENT: u8 = 6;
}

#[derive(Debug, PartialEq, Eq)]
pub enum GuildPacketError {
    CommentTooLong,
    StringConversionError,
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

pub fn build_guild_add_member_packet(vid: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::ADD_MEMBER, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&vid.to_ne_bytes());
    Ok(packet_data)
}

pub fn build_guild_remove_member_packet(pid: u32) -> Result<Vec<u8>, GuildPacketError> {
    let mut packet_data = Vec::new();
    let packet_len = (std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>()) as u16;
    let packet = TPacketCGGuild::new(guild_sub::REMOVE_MEMBER, packet_len);

    packet_data.extend_from_slice(&packet.to_bytes());
    packet_data.extend_from_slice(&pid.to_ne_bytes());
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
    packet_data.extend_from_slice(&index.to_ne_bytes());
    Ok(packet_data)
}
