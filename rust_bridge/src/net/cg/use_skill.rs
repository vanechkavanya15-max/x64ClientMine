// Struktura pakietu uzycia umiejetnosci CG na podstawie Packet.h: command_use_skill
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::USE_SKILL;

pub const HEADER_CG_USE_SKILL: u16 = USE_SKILL;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGUseSkill {
    pub header: u16,
    pub length: u16,
    pub dw_vnum: u32,
    pub dw_target_vid: u32,
}

impl TPacketCGUseSkill {
    pub const HEADER: u16 = USE_SKILL;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy nowy pakiet uzycia umiejetnosci o podanym identyfikatorze vnum i celu vid.
    pub fn new(dw_vnum: u32, dw_target_vid: u32) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            dw_vnum,
            dw_target_vid,
        }
    }

    // Serializuje strukture do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.dw_vnum.to_le_bytes());
        buf.extend_from_slice(&self.dw_target_vid.to_le_bytes());
        buf
    }
}
