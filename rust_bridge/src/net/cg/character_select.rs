// Struktura pakietu wyboru postaci CG na podstawie Packet.h: command_player_select
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::CHARACTER_SELECT;

pub const HEADER_CG_PLAYER_SELECT: u16 = CHARACTER_SELECT;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGSelectCharacter {
    pub header: u16,
    pub length: u16,
    pub player_index: u8,
}

// Zachowanie kompatybilnosci z wczesniejsza nazwa typu
pub type CharacterSelectPacket = TPacketCGSelectCharacter;

impl TPacketCGSelectCharacter {
    pub const HEADER: u16 = CHARACTER_SELECT;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy nowa instancje pakietu z podanym indeksem gracza (slot 0-3).
    pub fn new(player_index: u8) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            player_index,
        }
    }

    // Serializuje pakiet do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.player_index);
        buf
    }
}
