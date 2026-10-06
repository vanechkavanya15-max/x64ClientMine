// Modul wysylania pakietow operacyjnych gildii w Rust.
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::mem::size_of;

pub const HEADER_CG_GUILD: u16 = 0x0730;

pub mod guild_sub {
    pub const ADD_MEMBER: u8 = 0; // INVITE
    pub const OFFER: u8 = 4; // EXP_DONATE
    pub const CHANGE_MEMBER_GRADE: u8 = 8;
    pub const USE_SKILL: u8 = 9;
    pub const GUILD_INVITE_ANSWER: u8 = 11;
    pub const DEPOSIT_MONEY: u8 = 13;
    pub const WITHDRAW_MONEY: u8 = 14;
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

    pub fn to_bytes(&self) -> [u8; size_of::<Self>()] {
        unsafe { std::mem::transmute(*self) }
    }
}

/// Wysyla pakiet zaproszenia do gildii (HEADER_CG_GUILD -> ADD_MEMBER).
#[no_mangle]
pub extern "C" fn SendGuildAddMemberPacket(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let packet_len = (size_of::<TPacketCGGuild>() + size_of::<u32>()) as u16;
        let packet = TPacketCGGuild::new(guild_sub::ADD_MEMBER, packet_len);

        let mut data = Vec::with_capacity(packet_len as usize);
        data.extend_from_slice(&packet.to_bytes());
        data.extend_from_slice(&vid.to_ne_bytes());
        
        enqueue_outgoing_packet(data)
    })
}

/// Wysyla pakiet wplaty doswiadczenia do gildii (HEADER_CG_GUILD -> OFFER).
#[no_mangle]
pub extern "C" fn SendGuildOfferPacket(experience: u32) -> bool {
    safe_ffi_call(false, || {
        let packet_len = (size_of::<TPacketCGGuild>() + size_of::<u32>()) as u16;
        let packet = TPacketCGGuild::new(guild_sub::OFFER, packet_len);

        let mut data = Vec::with_capacity(packet_len as usize);
        data.extend_from_slice(&packet.to_bytes());
        data.extend_from_slice(&experience.to_ne_bytes());
        
        enqueue_outgoing_packet(data)
    })
}

/// Wysyla pakiet zmiany stopnia czlonka gildii (HEADER_CG_GUILD -> CHANGE_MEMBER_GRADE).
#[no_mangle]
pub extern "C" fn SendGuildChangeMemberGradePacket(pid: u32, grade: u8) -> bool {
    safe_ffi_call(false, || {
        let packet_len = (size_of::<TPacketCGGuild>() + size_of::<u32>() + size_of::<u8>()) as u16;
        let packet = TPacketCGGuild::new(guild_sub::CHANGE_MEMBER_GRADE, packet_len);

        let mut data = Vec::with_capacity(packet_len as usize);
        data.extend_from_slice(&packet.to_bytes());
        data.extend_from_slice(&pid.to_ne_bytes());
        data.extend_from_slice(&[grade]);
        
        enqueue_outgoing_packet(data)
    })
}

/// Wysyla pakiet uzycia umiejetnosci gildyjnej (HEADER_CG_GUILD -> USE_SKILL).
#[no_mangle]
pub extern "C" fn SendGuildUseSkillPacket(skill_id: u32, target_vid: u32) -> bool {
    safe_ffi_call(false, || {
        let packet_len = (size_of::<TPacketCGGuild>() + size_of::<u32>() + size_of::<u32>()) as u16;
        let packet = TPacketCGGuild::new(guild_sub::USE_SKILL, packet_len);

        let mut data = Vec::with_capacity(packet_len as usize);
        data.extend_from_slice(&packet.to_bytes());
        data.extend_from_slice(&skill_id.to_ne_bytes());
        data.extend_from_slice(&target_vid.to_ne_bytes());
        
        enqueue_outgoing_packet(data)
    })
}

/// Wysyla pakiet odpowiedzi na zaproszenie do gildii (HEADER_CG_GUILD -> GUILD_INVITE_ANSWER).
#[no_mangle]
pub extern "C" fn SendGuildInviteAnswerPacket(guild_id: u32, answer: u8) -> bool {
    safe_ffi_call(false, || {
        let packet_len = (size_of::<TPacketCGGuild>() + size_of::<u32>() + size_of::<u8>()) as u16;
        let packet = TPacketCGGuild::new(guild_sub::GUILD_INVITE_ANSWER, packet_len);

        let mut data = Vec::with_capacity(packet_len as usize);
        data.extend_from_slice(&packet.to_bytes());
        data.extend_from_slice(&guild_id.to_ne_bytes());
        data.extend_from_slice(&[answer]);
        
        enqueue_outgoing_packet(data)
    })
}

/// Wysyla pakiet wplaty pieniedzy do gildii (HEADER_CG_GUILD -> DEPOSIT_MONEY).
#[no_mangle]
pub extern "C" fn SendGuildDepositMoneyPacket(money: u32) -> bool {
    safe_ffi_call(false, || {
        let packet_len = (size_of::<TPacketCGGuild>() + size_of::<u32>()) as u16;
        let packet = TPacketCGGuild::new(guild_sub::DEPOSIT_MONEY, packet_len);

        let mut data = Vec::with_capacity(packet_len as usize);
        data.extend_from_slice(&packet.to_bytes());
        data.extend_from_slice(&money.to_ne_bytes());
        
        enqueue_outgoing_packet(data)
    })
}

/// Wysyla pakiet wyplaty pieniedzy z gildii (HEADER_CG_GUILD -> WITHDRAW_MONEY).
#[no_mangle]
pub extern "C" fn SendGuildWithdrawMoneyPacket(money: u32) -> bool {
    safe_ffi_call(false, || {
        let packet_len = (size_of::<TPacketCGGuild>() + size_of::<u32>()) as u16;
        let packet = TPacketCGGuild::new(guild_sub::WITHDRAW_MONEY, packet_len);

        let mut data = Vec::with_capacity(packet_len as usize);
        data.extend_from_slice(&packet.to_bytes());
        data.extend_from_slice(&money.to_ne_bytes());
        
        enqueue_outgoing_packet(data)
    })
}
